use std::net::{TcpListener};
use std::net::{TcpStream, SocketAddr};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use crate::bindings::message::Message;

pub struct TcpServer {
    stream_channel: Receiver<(TcpStream, SocketAddr)>,
}

impl TcpServer {
    pub fn new() -> std::io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:8080")?;

        let (sender, receiver) = mpsc::channel::<(TcpStream, SocketAddr)>();
		thread::spawn(move || loop {
            match listener.accept() {
                Ok(value) => sender.send(value).unwrap(),
                _ => (),
            }
		});

        Ok(TcpServer {
            stream_channel: receiver
        })
    }

    pub fn fetch(&mut self) -> Message {
        match self.stream_channel.try_recv() {
            Ok((socket, addr)) => Message::TcpListenerAccept(socket, addr),
            _ => Message::None,
        }
    }
}
