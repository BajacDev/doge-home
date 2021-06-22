use crate::event::Event;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::vec::Vec;

pub struct TcpConnection {
    receiver: Receiver<(usize, Vec<u8>)>,
}

pub const BUFFER_SIZE: usize = 64;

impl TcpConnection {
    pub fn new(mut stream: TcpStream) -> std::io::Result<Self> {
        let (sender, receiver) = mpsc::channel::<(usize, Vec<u8>)>();
        thread::spawn(move || loop {
            let mut buf = vec![0u8; BUFFER_SIZE];
            match stream.read(&mut buf) {
                Ok(size) => match sender.send((size, buf)) {
                    Err(_) => break, // the stream has been deleted (TcpEnd event)
                    _ => (),
                },
                _ => (),
            }
        });

        Ok(TcpConnection { receiver: receiver })
    }

    pub fn fetch(&mut self) -> Event {
        match self.receiver.try_recv() {
            Ok((size, _buf)) if size == 0 => Event::TcpEnd, // todo is this equivalent to a connection end
            Ok((size, buf)) => Event::TcpRead(size, buf),
            _ => Event::None,
        }
    }
}
