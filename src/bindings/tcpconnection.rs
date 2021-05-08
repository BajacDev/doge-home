use crate::bindings::message::Message;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::vec::Vec;

pub struct TcpConnection {
    receiver: Receiver<(usize, Vec<u8>)>,
}

const BUFFER_SIZE: usize = 64;

impl TcpConnection {
    pub fn new(mut stream: TcpStream) -> std::io::Result<Self> {
        let (sender, receiver) = mpsc::channel::<(usize, Vec<u8>)>();
        thread::spawn(move || loop {
            let mut buf = vec![0u8; BUFFER_SIZE];
            match stream.read(&mut buf) {
                Ok(size) => match sender.send((size, buf)) {
                    Err(_) => break, // the stream has been deleted (TcpEnd message)
                    _ => (),
                },
                _ => (),
            }
        });

        Ok(TcpConnection { receiver: receiver })
    }

    pub fn fetch(&mut self) -> Message {
        match self.receiver.try_recv() {
            Ok((size, _buf)) if size == 0 => Message::TcpEnd, // todo is this equivalent to a connection end
            Ok((size, buf)) => Message::TcpRead(size, buf),
            _ => Message::None,
        }
    }
}
