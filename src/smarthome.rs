use crate::bindings::message::*;
use crate::bindings::tcpserver::*;
use crate::devices::doorlock::DoorLock;
use crate::bindings::cli::*;

use std::{thread, time};

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}

pub struct SmartHome {
    // bindings:
    cli: CliState,
    tcp: TcpServer,

    // devices:
    doorlock: DoorLock

    // doorlock: DoorLock,
    // button: Button
}

impl SmartHome {

    pub fn new() -> Self {
        return SmartHome {
            cli: CliState::new(),
            tcp: TcpServer::new().unwrap(), // panic if failure
            doorlock: DoorLock::new(),
        };
    }
  
    pub fn start(&mut self) {
        loop {
            
            let mut message = self.cli.fetch();
            
            self.process_messages(message);
            
            message = self.tcp.fetch();

            self.process_messages(message);

            sleep(100);
        }
    }

    fn process_messages(&mut self, message: Message) {
        match message {
            Message::KeyPressed => self.doorlock.toggle(),
            Message::TcpListenerAccept(_stream, _addr) => println!("new connection"),
            Message::None => {}
        }
        sleep(1000);
    }
}
