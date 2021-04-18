use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

use std::thread;
use crate::bindings::message::Message;

pub struct CliState {
	stdin_channel: Receiver<String>
}

impl CliState {

	pub fn new() -> Self {
		let (tx, rx) = mpsc::channel::<String>();
		thread::spawn(move || loop {
			let mut buffer = String::new();
			io::stdin().read_line(&mut buffer).unwrap();
			tx.send(buffer).unwrap();
		});
		
		CliState {
			stdin_channel: rx
		}
	}
	
	pub fn fetch(&mut self) -> Message {
		match self.stdin_channel.try_recv() {
			Ok(_key) => Message::KeyPressed,
            _ => Message::None,
		}
	}
}