use std::fmt::{self};
use std::fs::read_to_string;

#[derive(Debug)]
pub(crate) struct ATmemory {
    registers: [u8; 32], // 32 x 8 General Purpose Working Registers
    sreg: u8,            // Status register
    pc: u8,              // Program Counter register
    sp: u8,              // Stack Pointer register
    flash: [u8; 16384],  // 16K Bytes of In-System Self-Programmable Flash
    sram: [u8; 1024],    // 1K Byte Internal SRAM
}

struct HexRecord {
    address: u16,
    data: Vec<u8>,
    byte_count: u8,
}

#[derive(Debug)]
enum Instruction {
    ADD { dest: u8, src: u8 },   // Add without Carry
    CLC,                         // Clear Carry Flag
    DEC { reg: u8 },             // Decrement
    INC { reg: u8 },             // Increment
    LDI { dest: u8, value: u8 }, // Load Immediate
    NOP,                         // No Operation
    RJMP { offset: i16 },        // Relative Jump
    SEC,                         // Set Carry Flag
    SUB { dest: u8, src: u8 },   // Subtract without Carry
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn parse_hex_line(line: &str) -> Result<Option<HexRecord>, String> {
    let hex_string = line.trim_start_matches(':');

    if !hex_string.len().is_multiple_of(2) {
        return Err(String::from("Cannot parse uneven hex lines."));
    }

    let bytes: Result<Vec<u8>, String> = (0..hex_string.len())
        .step_by(2)
        .map(|i| hex_byte(&hex_string[i..i + 2]))
        .collect();

    let bytes = bytes?;

    if bytes.len() < 5 {
        return Err(String::from("HEX line too short."));
    }

    let byte_count = bytes[0];
    let address = ((bytes[1] as u16) << 8) | (bytes[2] as u16);
    let record_type = bytes[3];

    let expected_len = 5 + byte_count;
    if bytes.len() != (expected_len as usize) {
        return Err(format!(
            "Length mismatch: expected {}, got {}",
            expected_len,
            bytes.len()
        ));
    }

    let data = bytes[4..bytes.len() - 1].to_vec();
    let _checksum = bytes[bytes.len() - 1];

    match record_type {
        0x00 => {
            // Data record
            Ok(Some(HexRecord {
                address,
                data,
                byte_count,
            }))
        }
        0x01 => {
            // End of file
            Ok(None)
        }
        _ => Err(format!("Unsuported record type: {:02X}", record_type)),
    }
}

fn hex_byte(s: &str) -> Result<u8, String> {
    if s.len() > 2 {
        return Err(String::from("Hex string is longer than expected."));
    }

    u8::from_str_radix(s, 16)
        .map_err(|e| format!("Failed to convert hex {} to an integer: {}", s, e))
}

impl ATmemory {
    pub fn registers(&self) -> &[u8; 32] {
        &self.registers
    }
    pub fn sreg(&self) -> u8 {
        self.sreg
    }
    pub fn pc(&self) -> u8 {
        self.pc
    }
    pub fn sp(&self) -> u8 {
        self.sp
    }
    pub fn flash(&self) -> &[u8; 16384] {
        &self.flash
    }
    pub fn sram(&self) -> &[u8; 1024] {
        &self.sram
    }

    pub fn init() -> Self {
        Self {
            registers: [0; 32],
            sreg: 0,
            pc: 0,
            sp: 0,
            flash: [0; 16384],
            sram: [0; 1024],
        }
    }

    pub fn load_bin(&mut self, filename: &str) -> Result<(), String> {
        let buffer = std::fs::read(filename).map_err(|e| format!("Failed to read file: {}", e))?;
        if buffer.len() > self.flash.len() {
            return Err(format!(
                "Binary too large: {} bytes (max: {})",
                buffer.len(),
                self.flash.len()
            ));
        }

        self.flash[..buffer.len()].copy_from_slice(&buffer);
        Ok(())
    }

    pub fn load_hex(&mut self, filename: &str) -> Result<(), String> {
        for line in read_to_string(filename).unwrap().lines() {
            match parse_hex_line(line) {
                Ok(Some(record)) => {
                    for (offset, &byte) in record.data.iter().enumerate() {
                        let flash_addr = record.address as usize + offset;
                        if flash_addr < self.flash.len() {
                            self.flash[flash_addr] = byte;
                        } else {
                            return Err(format!(
                                "Hex out of bounds: address {:#04X} (addressable to {:#04X})",
                                flash_addr,
                                self.flash.len() - 1
                            ));
                        }
                    }
                }
                Ok(None) => break,
                Err(_) => (),
            }
        }

        Ok(())
    }

    pub fn erase_flash(&mut self) {
        self.flash = [0; 16384];
        self.pc = 0;
    }

    pub fn step(&mut self) -> Result<(), String> {
        let opcode = self.fetch();
        let instruction = self.decode(opcode)?;
        self.execute(instruction)?;
        Ok(())
    }

    pub fn get_instruction(&self) -> String {
        let opcode = self.fetch();
        let instruction = self.decode(opcode).unwrap_or(Instruction::NOP);
        format!("{}", instruction)
    }

    fn fetch(&self) -> u16 {
        let mut flash_bytes = [0u8; 2];
        let range_s: usize = (self.pc).into();
        let range_e: usize = (self.pc + 2).into();
        let mut result: u16;
        flash_bytes[0..2].copy_from_slice(&self.flash[range_s..range_e]);
        result = flash_bytes[1] as u16;
        result <<= 8;
        result += flash_bytes[0] as u16;
        result
    }

    fn decode(&self, opcode: u16) -> Result<Instruction, String> {
        match opcode {
            0x0000 => Ok(Instruction::NOP),
            x if (x & 0xF000) == 0xE000 => Ok(Instruction::LDI {
                dest: ( 0x10 | ((x >> 4) & 0x0F)) as u8,
                value: (((x >> 4) & 0xF0) | (x & 0x0F)) as u8,
            }),
            0x4A08 => Ok(Instruction::SEC),
            x if (x & 0xFE0F) == 0x9403 => Ok(Instruction::INC {
                reg: ((x >> 4) & 0x1F) as u8,
            }),
            x if (x & 0xFE0F) == 0x940A => Ok(Instruction::DEC {
                reg: ((x >> 4) & 0x1F) as u8,
            }),
            0x9488 => Ok(Instruction::CLC),
            _ => Err(String::from("Unable to decode instruction")),
        }
    }
    fn execute(&mut self, instruction: Instruction) -> Result<(), String> {
        match instruction {
            Instruction::CLC => {
                self.sreg &= 0b11111110;
                self.pc += 2;
                Ok(())
            }
            Instruction::DEC { reg } => {
                // TODO: Implement Negative and oVerflow flags
                self.registers[reg as usize] = self.registers[reg as usize].wrapping_sub_signed(1);
                if self.registers[reg as usize] == 0 {
                    self.sreg |= 0b00000010;
                } else {
                    self.sreg &= 0b11111101;
                }
                self.pc += 2;
                Ok(())
            }
            Instruction::LDI { dest, value } => {
                self.registers[dest as usize] = value;
                self.pc += 2;
                Ok(())
            }
            Instruction::INC { reg } => {
                // TODO: Implement Negative and oVerflow flags
                self.registers[reg as usize] = self.registers[reg as usize].wrapping_add(1);
                if self.registers[reg as usize] == 0 {
                    self.sreg |= 0b00000010;
                } else {
                    self.sreg &= 0b11111101;
                }
                self.pc += 2;
                Ok(())
            }
            Instruction::NOP => {
                self.pc += 2;
                Ok(())
            }
            Instruction::SEC => {
                self.sreg |= 0b00000001;
                self.pc += 2;
                Ok(())
            }
            _ => Err(String::from("Unable to execute instruction")),
        }
    }
}

// 16(10) = 1|0000(2)
// 31(10) = 1|1111(2)

// ( x & 0xFE0F ) == 0x9403
//    INC = 1001|010d|dddd|0011
// 0xFE0F = 1111|1110|0000|1111 => mask
// 0x9403 = 1001|0100|0000|0011 => mask result
// 0x9453 = 1001|0100|0101|0011 => RESULT

// ( x & 0xF000 ) == 0xE000
//    DEC = 1110|KKKK|dddd|KKKK
// 0xF000 = 1111|0000|0000|0000 => mask
// 0xE000 = 1110|0000|0000|0000 => mask result
// 0x9453 = 1001|0100|0101|1010 => RESULT
//
// 1110 KKKK dddd KKKK
// 0000 1110 KKKK dddd => >>4
// 0000 0000 1111 0000 => maskH (F0)
// 0000 0000 0000 1111 => maskL (0F)
