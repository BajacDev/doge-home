use crate::bindings::cli::*;
use crate::bindings::gpio::gpio_controller::GpioController;
use crate::bindings::gpio::*;
use crate::bindings::message::*;
use crate::bindings::tcpconnection::*;
use crate::bindings::tcpserver::*;
use crate::devices::doorlock::DoorLock;

use std::{thread, time};

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}

pub struct SmartHome {
    // bindings:
    cli: CliState,
    tcp_server: TcpServer,
    tcp_connection: Option<TcpConnection>,
    gpio_controller: GpioController,
    // devices:
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
            tcp_server: TcpServer::new().expect("could not create tcpServer"),
            tcp_connection: None,
            gpio_controller: gpio_controller,
            doorlock: DoorLock::new(gpio_output_pin),
        };
    }

    pub fn start(&mut self) {
        loop {
            // receive messages from all bindings and process them
            let mut message = self.cli.fetch();
            self.process_message(message);
            message = self.tcp_server.fetch();
            self.process_message(message);
            if let Some(connection) = &mut self.tcp_connection {
                message = connection.fetch();
                self.process_message(message);
            }
            sleep(100);
        }
    }

    fn process_message(&mut self, message: Message) {
        match message {
            Message::KeyPressed => {
                self.doorlock.toggle(&mut self.gpio_controller);
            }
            Message::TcpListenerAccept(stream, addr) => {
                println!("new connection at {}", addr);
                self.tcp_connection = Some(TcpConnection::new(stream).unwrap())
            }
            Message::TcpEnd => {
                println!("connection end");
                self.tcp_connection = None;
            }
            Message::TcpRead(size, vec) => {
                println!("receive {:?} bytes: {:?}", size, vec);
                if vec[0] == 49 { // check if received "1" from tcp client
                    self.doorlock.toggle(&mut self.gpio_controller);
                 }
 
             },

            Message::None => {}
        }
        sleep(100);
    }
}
