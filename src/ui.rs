use std::path::PathBuf;

use iced::theme::Mode;
use iced::widget::{button, column, container, row, rule, scrollable, slider, text};
use iced::Length::Fill;
use iced::{system, window, Element, Font, Task, Theme};
use rfd::FileDialog;

use crate::config::Config;
use crate::memory::ATmemory;

#[derive(Debug)]
pub struct UInterface {
    cpu: ATmemory,
    cycle_counter: usize,
    flash_file: Option<PathBuf>,
    memory_bytes_per_row: usize,
    memory_bytes_per_column: usize,
    theme: Theme,
    theme_mode: Mode,
    show_settings: bool,
    instructions_per_tick: u8,
    ticks_per_second: u8,

    // Temp settings value
    temp_memory_bytes_per_row: usize,
    temp_memory_bytes_per_column: usize,
    temp_instructions_per_tick: u8,
    temp_ticks_per_second: u8,
}

#[derive(Debug, Clone)]
pub enum Message {
    CPUstep,
    Exit,
    LoadBinToFlash,
    LoadHexToFlash,
    Restart,
    ThemeChanged(Mode),
    OpenSettings,
    CloseSettings,
    SettingsRowChanged(usize),
    SettingsColumnChanged(usize),
    SettingsInsTickChanged(u8),
    SettingsTickSecChanged(u8),
    SaveSettings,
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
            let seg_byte =
                if usize::from(self.cpu.pc() * 2) == seg || usize::from((self.cpu.pc() * 2) + 1) == seg {
                    text!(" {:02X}", self.cpu.flash()[seg]).style(text::primary)
                } else {
                    text!(" {:02X}", self.cpu.flash()[seg])
                };
            row = row.push(seg_byte.font(Font::MONOSPACE));
        }

        row = row.push(text("        ").font(Font::MONOSPACE));

        for seg in addr..addr + self.memory_bytes_per_row {
            let seg_char =
                if usize::from(self.cpu.pc() * 2) == seg || usize::from((self.cpu.pc() * 2) + 1) == seg {
                    text!("{}", Self::byte_to_ascii(self.cpu.flash()[seg])).style(text::primary)
                } else {
                    text!("{}", Self::byte_to_ascii(self.cpu.flash()[seg]))
                };
            row = row.push(seg_char.font(Font::MONOSPACE));
        }

        row.spacing(2).into()
    }

    fn get_memory_window_boundary(&self) -> (usize, usize) {
        let pc = self.cpu.pc() as i32;
        let half_window = self.memory_bytes_per_column as i32;

        let start = pc - half_window;
        let end = pc + half_window + 1;

        let start = start.max(0) as usize;
        let end = end.min(self.cpu.flash().len() as i32) as usize;

        (start, end)
    }

    fn mode_to_theme(mode: Mode) -> Theme {
        match mode {
            Mode::None => Theme::Ferra,
            Mode::Light => Theme::GruvboxLight,
            Mode::Dark => Theme::SolarizedDark,
        }
    }

    pub fn new() -> Self {
        let config = Config::load().unwrap_or_default();

        Self {
            theme_mode: match config.theme.mode.as_str() {
                "Light" => Mode::Light,
                "Dark" => Mode::Dark,
                _ => Mode::None,
            },
            theme: Theme::Dark,
            cpu: ATmemory::init(),
            flash_file: None,
            memory_bytes_per_row: config.display.memory_bytes_per_row,
            memory_bytes_per_column: config.display.memory_bytes_per_column,
            show_settings: false,
            temp_memory_bytes_per_row: config.display.memory_bytes_per_row,
            temp_memory_bytes_per_column: config.display.memory_bytes_per_column,
            instructions_per_tick: 1,
            ticks_per_second: 1,
            temp_instructions_per_tick: 1,
            temp_ticks_per_second: 1,
            cycle_counter: 0,
        }
    }

    fn save_config(&self) -> Result<(), String> {
        let config = Config {
            display: crate::config::DisplayConfig {
                memory_bytes_per_row: self.memory_bytes_per_row,
                memory_bytes_per_column: self.memory_bytes_per_column,
            },
            theme: crate::config::ThemeConfig {
                mode: match self.theme_mode {
                    Mode::Light => "Light".to_string(),
                    Mode::Dark => "Dark".to_string(),
                    Mode::None => String::new(),
                },
            },
        };
        config.save()
    }

    fn render_flash_memory(&self) -> Element<'_, Message> {
        let (start, end) = Self::get_memory_window_boundary(self);
        let mut rows = column![].spacing(2);

        for addr in (start..end).step_by(self.memory_bytes_per_row) {
            let row = self.format_memory_row(addr);
            rows = rows.push(row);
        }

        scrollable(rows.padding(4)).width(Fill).into()
    }

    fn render_registers(&self) -> Element<'_, Message> {
        let mut rows = column![].spacing(2);
        for reg in 0..32 {
            rows = rows.push(text!("R{:02}={:03} | {1:#04X} | {1:#08b}", reg, self.cpu.memory()[reg]));
        }

        scrollable(rows.padding(4)).width(Fill).into()
    }

    fn render_sram(&self) -> Element<'_, Message> {
        let mut rows = column![].spacing(2);
        for sp in (0x0060..0x0460).rev() {
            match sp == self.cpu.sp() as usize {
                true => {
                    rows = rows.push(
                        text!("{:#05X}={:#04X}", sp, self.cpu.memory()[sp])
                            .font(Font::MONOSPACE)
                            .style(text::primary),
                    );
                }
                false => {
                    rows = rows.push(
                        text!("{:#05X}={:#04X}", sp, self.cpu.memory()[sp]).font(Font::MONOSPACE),
                    );
                }
            }
        }

        scrollable(rows.padding(4)).width(Fill).into()
    }

    fn render_sreg(&self) -> Element<'_, Message> {
        let mut cols = row![text("Status Register | ")].spacing(2);
        let flags = ["I", "T", "H", "S", "V", "N", "Z", "C"];

        for (idx, val) in flags.iter().enumerate() {
            match (self.cpu.sreg() << idx & 0x80) == 128 {
                true => cols = cols.push(text!("{}", val).style(text::primary)),
                false => cols = cols.push(text!("{}", val)),
            }
        }

        scrollable(cols).height(Fill).into()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        system::theme_changes().map(Message::ThemeChanged)
    }

    pub fn theme(&self) -> Theme {
        match self.theme_mode {
            Mode::None => Theme::Ferra,
            Mode::Light => Theme::GruvboxLight,
            Mode::Dark => Theme::SolarizedDark,
        }
    }

    pub fn update(state: &mut UInterface, message: Message) -> Task<Message> {
        match message {
            Message::Exit => window::latest().and_then(window::close),
            Message::ThemeChanged(mode) => {
                state.theme = UInterface::mode_to_theme(mode);
                state.theme_mode = mode;
                Task::none()
            }
            Message::LoadBinToFlash => {
                state.cpu = ATmemory::init();
                state.cycle_counter = 0;
                state.flash_file = None;
                let file = FileDialog::new()
                    .add_filter("Binary file", &["bin", "obj"])
                    .set_directory(std::env::current_dir().unwrap_or(std::env::home_dir().unwrap()))
                    .set_title("Open binary file")
                    .pick_file();
                state.flash_file = file.clone();

                if let Some(path) = file {
                    if let Some(path_str) = path.to_str() {
                        let _ = state.cpu.load_bin(path_str);
                    } else {
                        eprintln!("Error: Path is not valid UTF-8.");
                    }
                } else {
                    eprintln!("Error: No file selected.");
                }
                Task::none()
            }
            Message::LoadHexToFlash => {
                state.cpu = ATmemory::init();
                state.cycle_counter = 0;
                state.flash_file = None;
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
                        eprintln!("Error: Path is not valid UTF-8.");
                    }
                } else {
                    eprintln!("Error: No file selected.");
                }
                Task::none()
            }
            Message::Restart => {
                state.cpu = ATmemory::init();
                state.cycle_counter = 0;
                state.flash_file = None;
                Task::none()
            }
            Message::CPUstep => {
                let _ = state.cpu.step();
                state.cycle_counter += 1;
                Task::none()
            }
            Message::OpenSettings => {
                state.temp_memory_bytes_per_column = state.memory_bytes_per_column;
                state.temp_memory_bytes_per_row = state.memory_bytes_per_row;
                state.show_settings = true;
                Task::none()
            }
            Message::CloseSettings => {
                state.temp_memory_bytes_per_column = state.memory_bytes_per_column;
                state.temp_memory_bytes_per_row = state.memory_bytes_per_row;
                state.show_settings = false;
                Task::none()
            }
            Message::SettingsRowChanged(val) => {
                state.temp_memory_bytes_per_row = val;
                Task::none()
            }
            Message::SettingsColumnChanged(val) => {
                state.temp_memory_bytes_per_column = val;
                Task::none()
            }
            Message::SaveSettings => {
                state.memory_bytes_per_column = state.temp_memory_bytes_per_column;
                state.memory_bytes_per_row = state.temp_memory_bytes_per_row;
                state.instructions_per_tick = state.temp_instructions_per_tick;
                state.ticks_per_second = state.temp_ticks_per_second;
                state.show_settings = false;
                let _ = state.save_config();
                Task::none()
            }
            Message::SettingsInsTickChanged(val) => {
                state.temp_instructions_per_tick = val;
                Task::none()
            }
            Message::SettingsTickSecChanged(val) => {
                state.temp_ticks_per_second = val;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if self.show_settings {
            self.view_settings()
        } else {
            self.view_main()
        }
    }

    fn view_main(&self) -> Element<'_, Message> {
        let mut content = column![].spacing(2).padding(4);

        let header = row![
            text("Breadboard").size(36).width(Fill),
            button(text("Config")).on_press(Message::OpenSettings)
        ]
        .spacing(8);
        content = content.push(header);

        content = content.push(rule::horizontal(2));

        let toolbar = row![
            button(text("Load .bin")).on_press(Message::LoadBinToFlash),
            button(text("Load .hex")).on_press(Message::LoadHexToFlash),
            if self.flash_file.is_some() {
                button(text("Restart"))
                    .style(button::danger)
                    .on_press(Message::Restart)
            } else {
                button(text("Restart")).style(button::danger)
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
            scrollable(
                column![
                    text!("Program Counter | {:#06X}", self.cpu.pc()),
                    text!("Stack Pointer | {:#04X}", self.cpu.sp()),
                    text!("Cycle Counter | {:06}", self.cycle_counter),
                    Self::render_sreg(self)
                ]
                .padding(4)
            )
            .width(Fill),
            rule::horizontal(2),
            row![
                Self::render_registers(self),
                rule::vertical(2),
                Self::render_sram(self)
            ]
        ];

        // let right_sidebar = column![
        //     // text("PortA"),
        //     // text("PortB"),
        //     // text("PortC"),
        //     // text("PortD"),
        //     // text("Timer0"),
        //     // text("Timer1"),
        //     // text("Timer2"),
        // ]
        // .padding(2);

        let main_view = row![
            left_sidebar,
            rule::vertical(2),
            Self::render_flash_memory(self),
            // rule::vertical(2),
            // right_sidebar,
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

    fn view_settings(&self) -> Element<'_, Message> {
        let mut content = column![].spacing(2).padding(4);

        let header = row![
            text("Breadboard").size(36).width(Fill),
            button(text("Cancel")).on_press(Message::CloseSettings),
            button(text("Save")).on_press(Message::SaveSettings),
        ]
        .spacing(8);
        content = content.push(header);

        content = content.push(rule::horizontal(2));

        content = content.push(
            row![
                text("Bytes of memory per row:"),
                slider(1.0..=16.0, self.temp_memory_bytes_per_row as f64, |val| {
                    Message::SettingsRowChanged(val as usize)
                }),
                text!("{}", self.temp_memory_bytes_per_row)
            ]
            .spacing(4)
            .padding(4),
        );

        content = content.push(
            row![
                text("Bytes of memory per column:"),
                slider(
                    8.0..=256.0,
                    self.temp_memory_bytes_per_column as f64,
                    |val| { Message::SettingsColumnChanged(val as usize) }
                ),
                text!("{}", self.temp_memory_bytes_per_column)
            ]
            .spacing(4)
            .padding(4),
        );

        content = content.push(
            row![
                text("Instructions per Tick:"),
                slider(1.0..=64.0, self.temp_instructions_per_tick as f64, |val| {
                    Message::SettingsInsTickChanged(val as u8)
                }),
                text!("{} instructions/tick", self.temp_instructions_per_tick)
            ]
            .spacing(4)
            .padding(4),
        );

        content = content.push(
            row![
                text("Bytes of memory per column:"),
                slider(1.0..=20.0, self.temp_ticks_per_second as f64, |val| {
                    Message::SettingsTickSecChanged(val as u8)
                }),
                text!("{} ticks/second", self.temp_ticks_per_second)
            ]
            .spacing(4)
            .padding(4),
        );
        container(content).into()
    }
}
