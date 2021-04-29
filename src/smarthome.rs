use crate::CliState;
use crate::bindings::message::*;
use crate::devices::doorlock::DoorLock;
use std::{thread, time};

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}

pub struct SmartHome {
    // bindings:
    cli: CliState,

    // devices:
    doorlock: DoorLock

    // doorlock: DoorLock,
    // button: Button
}

impl SmartHome {

    pub fn new() -> Self {
        return SmartHome {
            cli: CliState::new(),
            doorlock: DoorLock::new(),
        };
    }
  
    pub fn start(&mut self) {
        loop {
            let message: Message = self.cli.fetch();

            match message {
                Message::KeyPressed => self.doorlock.toggle(),
                _ => {}
            }
            sleep(1000);
        }
    }
}
