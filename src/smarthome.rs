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
    // Option None is for testing only
    pub cli: Option<CliState>,
    pub tcp_binding: Option<TcpBinding>,
    pub gpio_controller: Option<GpioController>,
    pub gpio_output_pin: Option<GpioOutputPin>,

    // devices ie smart home state:
    pub doorlock: DoorLock,
}

impl SmartHome {
    pub fn new() -> Self {
        let mut gpio_controller = GpioController::get_the_gpio_controller();

        let gpio_output_pin = GpioOutputPin::new(
            GpioPin::new(&GpioPinAvailable::Gpio21),
            &mut gpio_controller,
        );

        return SmartHome {
            cli: Some(CliState::new()),
            tcp_binding: Some(TcpBinding::new().expect("could not create tcpServer")),
            gpio_controller: Some(gpio_controller),
            gpio_output_pin: Some(gpio_output_pin),
            doorlock: DoorLock::new(),
        };
    }

    pub fn new_fake() -> Self {
        return SmartHome {
            cli: None,
            tcp_binding: None,
            gpio_controller: None,
            gpio_output_pin: None,
            doorlock: DoorLock::new(),
        };
    }

    pub fn start(&mut self) {
        loop {
            // receive events from all bindings and process them
            let mut event = self.cli.as_mut().unwrap().fetch();
            self.process_event(event);
            event = self.tcp_binding.as_mut().unwrap().fetch();
            self.process_event(event);
            sleep(100);
        }
    }

    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::KeyPressed => {
                self.doorlock
                    .toggle(self.gpio_controller.as_mut(), self.gpio_output_pin.as_mut());
            }
            Event::TcpNewConnection(addr) => {
                println!("new connection at {}", addr);
            }
            Event::TcpEnd => {
                println!("connection end");
            }
            Event::TcpRead(size, vec) => {

                
                println!("receive {:?} bytes: {:?}", size, vec);
                if size == 0 {
                    return
                }
                if vec[0] == 49 {
                    // check if received "1" from tcp client
                    self.doorlock
                        .toggle(self.gpio_controller.as_mut(), self.gpio_output_pin.as_mut());
                } else if vec[0] == 48 {
                    // check if received "0" from tcp client
                    self.doorlock
                        .open(self.gpio_controller.as_mut(), self.gpio_output_pin.as_mut());
                }
            }

            _ => {}
        }
    }
}
