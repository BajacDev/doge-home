use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

use crate::bindings::message::Message;
use std::thread;

pub struct CliState {
    stdin_channel: Receiver<String>,
}

impl CliState {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<String>();
        thread::spawn(move || loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            sender.send(buffer).unwrap();
        });

        CliState {
            stdin_channel: receiver,
        }
    }

    pub fn fetch(&mut self) -> Message {
        match self.stdin_channel.try_recv() {
            Ok(_key) => Message::KeyPressed,
            _ => Message::None,
        }
    }
}
