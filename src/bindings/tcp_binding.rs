use crate::bindings::tcp_connection::TcpConnection;
use crate::bindings::tcp_server::TcpServer;
use crate::event::Event;

// group TcpServer and TcpConnection in a single struct to be tested all together
pub struct TcpBinding {
    tcp_server: TcpServer,
    tcp_connection: Option<TcpConnection>,
}

impl TcpBinding {
    pub fn new() -> std::io::Result<Self> {
        Ok(TcpBinding {
            tcp_server: TcpServer::new()?,
            tcp_connection: None,
        })
    }

    pub fn fetch(&mut self) -> Event {
        let event = self.tcp_server.fetch();
        match event {
            Event::TcpListenerAccept(stream, addr) => {
                self.tcp_connection = Some(TcpConnection::new(stream).unwrap());
                Event::TcpNewConnection(addr)
            }
            Event::TcpEnd => {
                self.tcp_connection = None;
                Event::TcpEnd
            }
            _ => match &mut self.tcp_connection {
                Some(connection) => connection.fetch(),
                _ => Event::None,
            },
        }
    }
}
