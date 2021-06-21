use crate::bindings::cli::*;
use crate::bindings::gpio::gpio_controller::GpioController;
use crate::bindings::gpio::*;
use crate::bindings::tcp_binding::*;
use crate::devices::doorlock::DoorLock;
use crate::event::Event;

use std::{thread, time};

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}

pub struct SmartHome {
    // bindings:
    // should appear stateless from the smart home
    cli: CliState,
    tcp_binding: TcpBinding,
    gpio_controller: GpioController,
    gpio_output_pin: GpioOutputPin,

    // devices ie smart home state:
    doorlock: DoorLock,
}

impl SmartHome {
    pub fn new() -> SmartHome {
        let mut gpio_controller = GpioController::get_the_gpio_controller();

        let gpio_output_pin = GpioOutputPin::new(
            GpioPin::new(&GpioPinAvailable::Gpio21),
            &mut gpio_controller,
        );
        return SmartHome {
            cli: CliState::new(),
            tcp_binding: TcpBinding::new().expect("could not create tcpServer"),
            gpio_controller: gpio_controller,
            gpio_output_pin: gpio_output_pin,
            doorlock: DoorLock::new(),
        };
    }

    pub fn start(&mut self) {
        loop {
            // receive events from all bindings and process them
            let mut event = self.cli.fetch();
            self.process_event(event);
            event = self.tcp_binding.fetch();
            self.process_event(event);
            sleep(100);
        }
    }

    fn process_event(&mut self, event: Event) {
        match event {
            Event::KeyPressed => {
                self.doorlock
                    .toggle(&mut self.gpio_controller, &mut self.gpio_output_pin);
            }
            Event::TcpNewConnection(addr) => {
                println!("new connection at {}", addr);
            }
            Event::TcpEnd => {
                println!("connection end");
            }
            Event::TcpRead(size, vec) => {
                println!("receive {:?} bytes: {:?}", size, vec);
                if vec[0] == 49 {
                    // check if received "1" from tcp client
                    self.doorlock
                        .toggle(&mut self.gpio_controller, &mut self.gpio_output_pin);
                } else if vec[0] == 48 {
                    // check if received "0" from tcp client
                    self.doorlock
                        .open(&mut self.gpio_controller, &mut self.gpio_output_pin);
                }
            }

            _ => {}
        }
    }
}
