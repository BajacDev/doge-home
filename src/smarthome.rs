use crate::bindings::cli::*;
use crate::bindings::message::*;
use crate::bindings::tcpconnection::*;
use crate::bindings::tcpserver::*;
use crate::devices::doorlock::DoorLock;
use std::option::*;

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

    // devices:
    doorlock: DoorLock,
}

impl SmartHome {
    pub fn new() -> Self {
        return SmartHome {
            cli: CliState::new(),
            tcp_server: TcpServer::new().unwrap(), // panic if failure
            tcp_connection: None,
            doorlock: DoorLock::new(),
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
            Message::KeyPressed => self.doorlock.toggle(),
            Message::TcpListenerAccept(stream, addr) => {
                println!("new connection at {}", addr);
                self.tcp_connection = Some(TcpConnection::new(stream).unwrap())
            }
            Message::TcpEnd => {
                println!("connection end");
                self.tcp_connection = None;
            }
            Message::TcpRead(size, vec) => println!("receive {:?} bytes: {:?}", size, vec),

            Message::None => {}
        }
        sleep(100);
    }
}
