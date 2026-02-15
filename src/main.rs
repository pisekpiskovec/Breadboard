use std::fmt::{self};
use std::fs::read_to_string;
use std::path::PathBuf;

use iced::theme::Mode;
use iced::widget::{button, column, container, row, rule, scrollable, text};
use iced::Length::Fill;
use iced::{Element, Font, Task, Theme, system, window};
use rfd::FileDialog;

// === ATmega16 part ===

#[derive(Debug)]
struct ATmemory {
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

    if hex_string.len() % 2 != 0 {
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

fn main() -> iced::Result {
    iced::application(UInterface::new, UInterface::update, UInterface::view)
        .title("Breadboard")
        .theme(UInterface::theme)
        .subscription(UInterface::subscription)
        .run()
}

// === User Inteface part ===
#[derive(Debug)]
struct UInterface {
    cpu: ATmemory,
    flash_file: Option<PathBuf>,
    memory_bytes_per_row: usize,
    theme: Theme,
    theme_mode: Mode,
}

#[derive(Debug, Clone)]
enum Message {
    CPUstep,
    EraseFlash,
    Exit,
    LoadBinToFlash,
    LoadHexToFlash,
    ThemeChanged(Mode)
}

impl UInterface {
    fn byte_to_ascii(byte: u8) -> char {
        let range = 32..126;
        if range.contains(&byte) {
            char::from(byte)
        } else {
            '.'
        }
    }

    fn format_memory_row(&self, addr: usize) -> Element<'_, Message> {
        let mut row = row![];

        row = row.push(text!("{:04X}:", addr).font(Font::MONOSPACE));

        for seg in addr..addr + self.memory_bytes_per_row {
            let seg_byte = if usize::from(self.cpu.pc) == seg || usize::from(self.cpu.pc + 1) == seg {
                text!(" {:02X}", self.cpu.flash[seg]).style(text::primary)
            } else {
                text!(" {:02X}", self.cpu.flash[seg])
            };
            row = row.push(seg_byte.font(Font::MONOSPACE));
        }

        row = row.push(text("        ").font(Font::MONOSPACE));

        for seg in addr..addr + self.memory_bytes_per_row {
            let seg_char = if usize::from(self.cpu.pc) == seg || usize::from(self.cpu.pc + 1) == seg {
                text!("{}", Self::byte_to_ascii(self.cpu.flash[seg])).style(text::primary)
            } else {
                text!("{}", Self::byte_to_ascii(self.cpu.flash[seg]))
            };
            row = row.push(seg_char.font(Font::MONOSPACE));
        }

        row.spacing(2).into()
    }

    fn get_memory_window_boundary(&self) -> (usize, usize) {
        let pc = self.cpu.pc as i32;
        let half_window = 128;

        let start = pc - half_window;
        let end = pc + half_window;

        let start = start.max(0) as usize;
        let end = end.min(self.cpu.flash.len() as i32) as usize;

        (start, end)
    }

    fn mode_to_theme(mode: Mode) -> Theme {
        match mode {
            Mode::None => Theme::Ferra,
            Mode::Light => Theme::GruvboxLight,
            Mode::Dark => Theme::SolarizedDark,
        }
    }

    fn new() -> Self {
        Self {
            theme_mode: Mode::Light,
            theme: Theme::Dark,
            cpu: ATmemory::init(),
            flash_file: None,
            memory_bytes_per_row: 8,
        }
    }

    fn render_flash_memory(&self) -> Element<'_, Message> {
        let (start, end) = Self::get_memory_window_boundary(self);
        let mut rows = column![].spacing(2);

        for addr in (start..end).step_by(self.memory_bytes_per_row) {
            let row = self.format_memory_row(addr);
            rows = rows.push(row);
        }

        scrollable(rows).into()
    }

    fn render_registers(&self) -> Element<'_, Message> {
        let mut rows = column![].spacing(2);
        for reg in (0..self.cpu.registers.len()) {
            rows = rows.push(text!("R{:02}={:03}", reg, self.cpu.registers[reg]).width(Fill));
        }

        scrollable(rows).into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        system::theme_changes().map(Message::ThemeChanged)
    }

    fn theme(&self) -> Theme {
        match self.theme_mode {
            Mode::None => Theme::Ferra,
            Mode::Light => Theme::GruvboxLight,
            Mode::Dark => Theme::SolarizedDark,
        }
    }

    fn update(state: &mut UInterface, message: Message) -> Task<Message> {
        match message {
            Message::Exit => window::latest().and_then(window::close),
            Message::ThemeChanged(mode) => {
                state.theme = UInterface::mode_to_theme(mode);
                state.theme_mode = mode;
                Task::none()
            }
            Message::LoadBinToFlash => {
                let file = FileDialog::new()
                    .add_filter("Binary file", &["bin"])
                    .set_directory(std::env::current_dir().unwrap_or(std::env::home_dir().unwrap()))
                    .set_title("Open binary file")
                    .pick_file();
                state.flash_file = file.clone();

                if let Some(path) = file {
                    if let Some(path_str) = path.to_str() {
                        let _ = state.cpu.load_bin(path_str);
                    } else {
                        state.cpu.erase_flash();
                        state.flash_file = None;
                        eprintln!("Error: Path is not valid UTF-8.");
                    }
                } else {
                    state.cpu.erase_flash();
                    state.flash_file = None;
                    eprintln!("Error: No file selected.");
                }
                Task::none()
            }
            Message::LoadHexToFlash => {
                let file = FileDialog::new()
                    .add_filter("Hex file", &["hex"])
                    .set_directory(std::env::current_dir().unwrap_or(std::env::home_dir().unwrap()))
                    .set_title("Open hex file")
                    .pick_file();
                state.flash_file = file.clone();

                if let Some(path) = file {
                    if let Some(path_str) = path.to_str() {
                        let _ = state.cpu.load_hex(path_str);
                    } else {
                        state.cpu.erase_flash();
                        state.flash_file = None;
                        eprintln!("Error: Path is not valid UTF-8.");
                    }
                } else {
                    state.cpu.erase_flash();
                    state.flash_file = None;
                    eprintln!("Error: No file selected.");
                }
                Task::none()
            }
            Message::EraseFlash => {
                state.cpu.erase_flash();
                state.flash_file = None;
                Task::none()
            }
            Message::CPUstep => {
                let _ = state.cpu.step();
                Task::none()
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut content = column![].spacing(2).padding(4);

        let header = row![
            text("Breadboard").size(36).width(Fill),
            button(text("Exit")).on_press(Message::Exit)
        ]
        .spacing(8);
        content = content.push(header);

        content = content.push(rule::horizontal(2));

        let toolbar = row![
            button(text("Load .bin")).on_press(Message::LoadBinToFlash),
            button(text("Load .hex")).on_press(Message::LoadHexToFlash),
            if self.flash_file.is_some() {
                button(text("Erase flash"))
                    .style(button::danger)
                    .on_press(Message::EraseFlash)
            } else {
                button(text("Erase flash")).style(button::danger)
            },
            if self.flash_file.is_some() {
                button(text("Step")).on_press(Message::CPUstep)
            } else {
                button(text("Step"))
            }
        ]
        .spacing(8)
        .padding(4);
        content = content.push(toolbar);
        content = content.push(rule::horizontal(2));

        let left_sidebar = column![
            text(format!("Program Counter | {:#06X}", self.cpu.pc)),
            text(format!("Stack Pointer | {:#04X}", self.cpu.sp)),
            text(format!("Status Register | {:#04X}", self.cpu.sreg)),
            rule::horizontal(2),
            Self::render_registers(self)
        ]
        .padding(2);

        let right_sidebar = column![
            text("PortA"),
            text("PortB"),
            text("PortC"),
            text("PortD"),
            text("Timer0"),
            text("Timer1"),
            text("Timer2"),
        ]
        .padding(2);

        let main_view = row![
            left_sidebar,
            rule::vertical(2),
            Self::render_flash_memory(self),
            rule::vertical(2),
            right_sidebar,
        ];

        content = content.push(main_view);
        content = content.push(rule::horizontal(2));

        let mut status_bar = row![];
        if let Some(path) = self.flash_file.as_ref() {
            if let Some(path_str) = path.to_str() {
                status_bar = status_bar.push(text(path_str).width(Fill));
            }
        } else {
            status_bar = status_bar.push(text("").width(Fill));
        }
        status_bar = status_bar.push(text!("Current instruction: {}", self.cpu.get_instruction()));
        content = content.push(status_bar);

        container(content).into()
    }
}
