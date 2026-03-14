use std::io::{Read, Write};
use std::net::TcpStream;

const RESET_ADDRESS: u8 = 0xFF;

#[derive(Debug)]
pub struct ATport {
    reset_hold: bool,
    tcp_connection: Option<TcpStream>,
}

impl ATport {
    pub fn new() -> Self {
        Self {
            reset_hold: false,
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
            Err(e) => Err(format!("Connection failed: {}", e)),
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
            let mut buf = [0u8; 2];
            if stream.read_exact(&mut buf).is_err() {
                self.tcp_connection = None;
            }
            Some((buf[0], buf[1]))
        } else {
            None
        }
    }

    pub fn is_connected(&self) -> bool {
        self.tcp_connection.is_some()
    }

    pub fn is_reset_holded(&self) -> bool {
        self.reset_hold
    }

    pub fn update_io(&mut self, memory: &mut [u8; 1120]) -> Result<(), String> {
        if let Some(ref mut stream) = self.tcp_connection {
            let mut buf = [0u8; 2];

            match stream.read_exact(&mut buf) {
                Ok(_) => {
                    if buf[0] == 0xFF {
                        self.reset_hold = buf[1] == 1;
                    } else {
                        let memory_address = 0x20 + buf[0];
                        memory[usize::from(memory_address)] = buf[1];
                    }
                    Ok(())
                },
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    Ok(()) // No data available
                },
                Err(_) => {
                    self.tcp_connection = None;
                    Err("Connection lost".to_string())
                }
            }
        } else {
            Ok(())
        }
    }
}
