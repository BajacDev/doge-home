use std::net::{TcpStream, SocketAddr};

// message that can be fetched from all bindings
pub enum Message {
  None,
  KeyPressed,
  TcpListenerAccept(TcpStream, SocketAddr),
}