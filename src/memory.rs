use std::fmt::{self};
use std::fs::read_to_string;

#[derive(Debug)]
pub(crate) struct ATmemory {
    registers: [u8; 32], // 32 x 8 General Purpose Working Registers
    sreg: u8,            // Status register
    pc: u16,             // Program Counter register
    sp: u16,             // Stack Pointer register
    flash: [u8; 16384],  // 16K Bytes of In-System Self-Programmable Flash
    sram: [u8; 1024],    // 1K Byte Internal SRAM
    memory: [u8; 1120]   // EEPROM
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
    RCALL { offset: i16 },       // Relative Call to Subroutine
    RET,                         // Return from Subroutine
    RETI,                        // Return from Interrupt
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
    pub fn pc(&self) -> u16 {
        self.pc
    }
    pub fn sp(&self) -> u16 {
        self.sp
    }
    pub fn flash(&self) -> &[u8; 16384] {
        &self.flash
    }
    pub fn sram(&self) -> &[u8; 1024] {
        &self.sram
    }
    pub fn memory(&self) -> &[u8; 1120] {
        &self.memory
    }

    pub fn init() -> Self {
        Self {
            registers: [0; 32],
            sreg: 0,
            pc: 0,
            sp: 0x3FF,
            flash: [0; 16384],
            sram: [0; 1024],
            memory: [0; 1120]
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

    /// Clears current flash and loads content from vector
    ///
    /// # Errors
    ///
    /// Vector is bigger than flash.
    pub fn load_flash_from_vec(&mut self, content: Vec<u8>) -> Result<(), String> {
        self.erase_flash();

        if content.len() > self.flash.len() {
            return Err(format!(
                "Binary too large: {} bytes (max: {})",
                content.len(),
                self.flash.len()
            ));
        }

        self.flash[..content.len()].copy_from_slice(&content);
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
            x if (x & 0xFC00) == 0x1800 => Ok(Instruction::SUB {
                dest: ((x >> 4) & 0x1F) as u8,
                src: (((x >> 5) & 0x10) | (x & 0x0F)) as u8,
            }),
            x if (x & 0xF000) == 0xE000 => Ok(Instruction::LDI {
                dest: (0x10 | ((x >> 4) & 0x0F)) as u8,
                value: (((x >> 4) & 0xF0) | (x & 0x0F)) as u8,
            }),
            x if (x & 0xFC00) == 0x0C00 => Ok(Instruction::ADD {
                dest: ((x >> 4) & 0x1F) as u8,
                src: (((x >> 5) & 0x10) | (x & 0x0F)) as u8,
            }),
            0x4A08 => Ok(Instruction::SEC),
            x if (x & 0xFE0F) == 0x9403 => Ok(Instruction::INC {
                reg: ((x >> 4) & 0x1F) as u8,
            }),
            x if (x & 0xFE0F) == 0x940A => Ok(Instruction::DEC {
                reg: ((x >> 4) & 0x1F) as u8,
            }),
            0x9488 => Ok(Instruction::CLC),
            0x9508 => Ok(Instruction::RET),
            0x9518 => Ok(Instruction::RETI),
            x if (x & 0xF000) == 0xC000 => Ok(Instruction::RJMP {
                offset: ((((x & 0xFFF) << 4) as i16) >> 4),
            }),
            x if (x & 0xF000) == 0xD000 => Ok(Instruction::RCALL {
                offset: ((((x & 0xFFF) << 4) as i16) >> 4),
            }),
            _ => Err(String::from("Unable to decode instruction")),
        }
    }
    fn execute(&mut self, instruction: Instruction) -> Result<(), String> {
        match instruction {
            Instruction::ADD { dest, src } => {
                let rd3 = Self::bit(self.registers[dest as usize], 3);
                let rr3 = Self::bit(self.registers[src as usize], 3);
                let rd7 = Self::bit(self.registers[dest as usize], 7);
                let rr7 = Self::bit(self.registers[src as usize], 7);

                self.registers[dest as usize] =
                    self.registers[dest as usize].wrapping_add(self.registers[src as usize]);

                let r3 = Self::bit(self.registers[dest as usize], 3);
                let r7 = Self::bit(self.registers[dest as usize], 7);
                let n = r7 == 1;
                let v = (rd7 & rr7 & !r7 | !rd7 & !rr7 & r7) != 0;

                // H - Half-Carry flag
                self.update_flag(0b00100000, (rd3 & rr3 | rr3 & !r3 | !r3 & rd3) != 0);
                // S - Signed Tests flag
                self.update_flag(0b00010000, n ^ v);
                // V - Two Complements flag
                self.update_flag(0b00001000, v);
                // N - Negative flag
                self.update_flag(0b00000100, n);
                // Z - Zero flag
                self.update_flag(0b00000010, self.registers[dest as usize] == 0);
                // C - Carry flag
                self.update_flag(0b00000001, (rd7 & rr7 | rr7 & !r7 | !r7 & rd7) != 0);

                self.pc += 2;
                Ok(())
            }
            Instruction::CLC => {
                self.clear_flag(0b00000001);
                self.pc += 2;
                Ok(())
            }
            Instruction::DEC { reg } => {
                self.registers[reg as usize] = self.registers[reg as usize].wrapping_sub_signed(1);
                let r7 = Self::bit(self.registers[reg as usize], 7);

                // S - Signed Tests flag
                self.update_flag(
                    0b00010000,
                    (r7 == 1) ^ (self.registers[reg as usize] == 0x7F),
                );
                // V - Two Complements flag
                self.update_flag(0b00001000, self.registers[reg as usize] == 0x7F);
                // N - Negative flag
                self.update_flag(0b00000100, r7 == 1);
                // Z - Zero flag
                self.update_flag(0b00000010, self.registers[reg as usize] == 0);

                self.pc += 2;
                Ok(())
            }
            Instruction::INC { reg } => {
                self.registers[reg as usize] = self.registers[reg as usize].wrapping_add(1);
                let r7 = Self::bit(self.registers[reg as usize], 7);

                // S - Signed Tests flag
                self.update_flag(
                    0b00010000,
                    (r7 == 1) ^ (self.registers[reg as usize] == 0x80),
                );
                // V - Two Complements flag
                self.update_flag(0b00001000, self.registers[reg as usize] == 0x80);
                // N - Negative flag
                self.update_flag(0b00000100, r7 == 1);
                // Z - Zero flag
                self.update_flag(0b00000010, self.registers[reg as usize] == 0);

                self.pc += 2;
                Ok(())
            }
            Instruction::LDI { dest, value } => {
                self.registers[dest as usize] = value;
                self.pc += 2;
                Ok(())
            }
            Instruction::NOP => {
                self.pc += 2;
                Ok(())
            }
            Instruction::RCALL { offset } => {
                let future_pc = self.pc + 2;
                let st_h = (future_pc >> 8) as u8;
                let st_l = (future_pc & 0x00FF) as u8;
                self.shrink_stack_pointer(None);
                self.sram[self.sp as usize] = st_l;
                self.shrink_stack_pointer(None);
                self.sram[self.sp as usize] = st_h;

                let pc_in_words = (self.pc / 2) as i32;
                let new_pc_in_words = pc_in_words + offset as i32 + 1;
                self.pc = (new_pc_in_words * 2) as u16;
                Ok(())
            }
            Instruction::RET => {
                let mut new_pc: u16;
                new_pc = self.sram[self.sp as usize] as u16;
                new_pc <<= 8;
                self.shrink_stack_pointer(Some(-1));
                new_pc += self.sram[self.sp as usize] as u16;
                self.shrink_stack_pointer(Some(-1));
                self.pc = new_pc;
                Ok(())
            }
            Instruction::RETI => {
                let mut new_pc: u16;
                new_pc = self.sram[self.sp as usize] as u16;
                new_pc <<= 8;
                self.shrink_stack_pointer(Some(-1));
                new_pc += self.sram[self.sp as usize] as u16;
                self.shrink_stack_pointer(Some(-1));
                self.set_flag(0b10000000);
                self.pc = new_pc;
                Ok(())
            },
            Instruction::RJMP { offset } => {
                let pc_in_words = (self.pc / 2) as i32;
                let new_pc_in_words = pc_in_words + offset as i32 + 1;
                self.pc = (new_pc_in_words * 2) as u16;
                Ok(())
            }
            Instruction::SEC => {
                self.set_flag(0b00000001);
                self.pc += 2;
                Ok(())
            }
            Instruction::SUB { dest, src } => {
                let rd3 = Self::bit(self.registers[dest as usize], 3);
                let rr3 = Self::bit(self.registers[src as usize], 3);
                let rd7 = Self::bit(self.registers[dest as usize], 7);
                let rr7 = Self::bit(self.registers[src as usize], 7);

                self.registers[dest as usize] =
                    self.registers[dest as usize].wrapping_sub(self.registers[src as usize]);

                let r3 = Self::bit(self.registers[dest as usize], 3);
                let r7 = Self::bit(self.registers[dest as usize], 7);
                let n = r7 == 1;
                let v = (rd7 & !rr7 & !r7 | !rd7 & rr7 & r7) != 0;

                // H - Half-Carry flag
                self.update_flag(0b00100000, (!rd3 & rr3 | rr3 & r3 | r3 & !rd3) != 0);
                // S - Signed Tests flag
                self.update_flag(0b00010000, n ^ v);
                // V - Two Complements flag
                self.update_flag(0b00001000, v);
                // N - Negative flag
                self.update_flag(0b00000100, n);
                // Z - Zero flag
                self.update_flag(0b00000010, self.registers[dest as usize] == 0);
                // C - Carry flag
                self.update_flag(0b00000001, (!rd7 & rr7 | rr7 & r7 | r7 & !rd7) != 0);

                self.pc += 2;
                Ok(())
            }
            _ => Err(String::from("Unable to execute instruction")),
        }
    }

    fn set_flag(&mut self, mask: u8) {
        self.sreg |= mask;
    }

    fn clear_flag(&mut self, mask: u8) {
        self.sreg &= !mask;
    }

    fn update_flag(&mut self, mask: u8, condition: bool) {
        match condition {
            true => self.set_flag(mask),
            false => self.clear_flag(mask),
        }
    }

    fn bit(value: u8, position: u8) -> u8 {
        (value >> position) & 1
    }

    fn shrink_stack_pointer(&mut self, amount: Option<i16>) {
        self.sp = self.sp.wrapping_sub(amount.unwrap_or(1) as u16);
        if self.sp == u16::MAX {
            self.sp = 0x3FF;
        } else if self.sp >= 1024 {
            self.sp = 0x000;
        }
    }

    fn write_memory(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    fn read_memory(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn push_stack(&mut self, value: u8) {
        self.sp = self.sp.wrapping_sub(1);
        if self.sp < 0x0060 {
            eprintln!("Stack overflow! SP={:#04X}", self.sp);
        } else if self.sp > 0x045F {
            eprintln!("Stack overflow! SP={:#04X}", self.sp);
            self.sp = 0x045F;
        }
        self.write_memory(self.sp, value);
    }

    fn pop_stack(&mut self) -> u8 {
        let ret = self.read_memory(self.sp);
        self.sp = self.sp.wrapping_add(1);
        if self.sp > 0x045F {
            eprintln!("Stack underflow! SP={:#04X}", self.sp);
            self.sp = 0x0000;
        }
        ret
    }
}

// (x & 0xFE0F) == 0x9403
//    INC = 1001|010d|dddd|0011
// 0xFE0F = 1111|1110|0000|1111 => mask
// 0x9403 = 1001|0100|0000|0011 => mask result
// 0x9453 = 1001|0100|0101|0011 => RESULT

// (x & 0xF000) == 0xD000
//  RCALL = 1101|kkkk|kkkk|kkkk
// 0xF000 = 1111|0000|0000|0000 => mask
// 0x1800 = 1101|0000|0000|0000 => mask result
// 0x9453 = 1001|0100|0101|1010 => RESULT
//
// 1110 KKKK dddd KKKK
// 0000 1110 KKKK dddd => >>4
// 0000 0000 1111 0000 => maskH (F0)
// 0000 0000 0000 1111 => maskL (0F)
// 0000111111111111
