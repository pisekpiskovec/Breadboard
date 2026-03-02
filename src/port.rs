use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct ATport {
    tcp_connection: Option<TcpStream>,
}

impl ATport {
    pub fn new() -> Self {
        Self {
            tcp_connection: None,
            }
    }

    pub fn connect(&mut self, addr: &str) -> Result<(), String> {
        match TcpStream::connect(addr) {
            Ok(stream) => {
                let _ = stream
                    .set_nonblocking(true)
                    .map_err(|e| format!("Failed to set nonblocking: {}", e));
                self.tcp_connection = Some(stream);
                Ok(())
            }
            Err(e) => {
                Err(format!("Connection failed: {}", e))
            }
        }
    }

    pub fn send_port_write(&mut self, port: u8, value: u8) {
        if let Some(ref mut stream) = self.tcp_connection {
            // Simple protocol: send 2 bytes [port, value]
            let data = [port, value];
            if stream.write_all(&data).is_err() {
                self.tcp_connection = None;
            }
        }
    }

    pub fn recive_port_read(&mut self) -> Option<(u8, u8)> {
        if let Some(ref mut stream) = self.tcp_connection {
            // Simple protocol: recive 2 bytes [port, value]
            let mut buf = Vec::new();
            if stream.read_to_end(&mut buf).is_err() {
                self.tcp_connection = None;
            }
            Some((buf[0], buf[1]))
        } else {
            None
        }
    }
}
