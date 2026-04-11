use std::io::{Read, Write};
use std::net::TcpStream;
use wire::{Message, CMD_REQUEST, CMD_RESET, CMD_RESPONSE, CMD_WRITE, PROTOCOL_VERSION};

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
            // Wire protocol: send 4 bytes [version, command, port, value]
            let data = Message {
                version: PROTOCOL_VERSION,
                command: CMD_WRITE,
                address: port,
                value,
            }
            .to_bytes();
            if stream.write_all(&data).is_err() {
                self.tcp_connection = None;
            }
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
            let mut buf = [0u8; 4];

            match stream.read_exact(&mut buf) {
                Ok(_) => {
                    let data = Message::from_bytes(buf);

                    // Protocol version check
                    if data.version != PROTOCOL_VERSION {
                        return Err("Protocol version mismatch".to_string());
                    }

                    // Command parsing
                    match data.command {
                        CMD_WRITE => {
                            memory[usize::from(data.address)] = data.value;
                            Ok(())
                        }
                        CMD_REQUEST => Err("Unexpected REQUEST command from server".to_string()),
                        CMD_RESPONSE => {
                            memory[usize::from(data.address)] = data.value;
                            Ok(())
                        }
                        CMD_RESET => {
                            self.reset_hold = data.value != 0;
                            Ok(())
                        }
                        _ => Err("No Wire command found".to_string()),
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    Ok(()) // No data available
                }
                Err(_) => {
                    self.tcp_connection = None;
                    Err("Connection lost".to_string())
                }
            }
        } else {
            Ok(())
        }
    }

    pub fn request_port_state(&mut self, address: u8) {
        if let Some(ref mut stream) = self.tcp_connection {
            // Wire protocol: send 4 bytes [version, command, port, value]
            let data = Message {
                version: PROTOCOL_VERSION,
                command: CMD_REQUEST,
                address,
                value: 0,
            }
            .to_bytes();
            if stream.write_all(&data).is_err() {
                self.tcp_connection = None;
            }
        }
    }
}
