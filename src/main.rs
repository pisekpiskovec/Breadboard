use std::fmt::{self, format};
use std::fs::File;
use std::io::{BufReader, Read};

struct ATmemory {
    registers: [u8; 32], // 32 x 8 General Purpose Working Registers
    sreg: u8,            // Status register
    pc: u8,              // Program Counter register
    sp: u8,              // Stack Pointer register
    flash: [u8; 16384],  // 16K Bytes of In-System Self-Programmable Flash
    sram: [u8; 1024],    // 1K Byte Internal SRAM
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

impl ATmemory {
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

    pub fn load_hex(&mut self, filename: &str) -> Result<(), String> {}

    pub fn step(&mut self) -> Result<(), String> {
        let opcode = self.fetch();
        println!("fetched {:#04x}", opcode);
        let instruction = self.decode(opcode)?;
        println!("executing {}", instruction);
        println!("serg {}", self.sreg);
        self.execute(instruction)?;
        Ok(())
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

fn main() {
    let mut cpu = ATmemory::init();
}
// ( x & 0xFE0F ) == 0x9403
//    INC = 1001|010d|dddd|0011
// 0xFE0F = 1111|1110|0000|1111 => mask
// 0x9403 = 1001|0100|0000|0011 => mask result
// 0x9453 = 1001|0100|0101|0011 => RESULT

// ( x & 0xFE0F ) == 0x9403
//    DEC = 1001|010d|dddd|1010
// 0xFE0F = 1111|1110|0000|1111 => mask
// 0x9403 = 1001|0100|0000|1010 => mask result
// 0x9453 = 1001|0100|0101|1010 => RESULT
