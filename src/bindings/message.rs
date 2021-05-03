use std::net::{TcpStream, SocketAddr};
use std::vec::Vec;

// Messages that can be fetched from all bindings
// Those messages are events
pub enum Message {
  None,
  KeyPressed,
  
  TcpListenerAccept(TcpStream, SocketAddr),
  TcpRead(usize, Vec<u8>),
  TcpEnd,
}