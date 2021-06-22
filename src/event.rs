use std::net::{SocketAddr, TcpStream};
use std::vec::Vec;

// Events that can be fetched from all bindings
pub enum Event {
    None,
    KeyPressed,

    TcpListenerAccept(TcpStream, SocketAddr),
    TcpNewConnection(SocketAddr),
    TcpRead(usize, Vec<u8>),
    TcpEnd,
}
