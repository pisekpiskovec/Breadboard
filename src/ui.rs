use std::path::PathBuf;
use std::time::Duration;

use iced::theme::Mode;
use iced::widget::{
    button, column, container, pick_list, row, rule, scrollable, slider, text, text_input,
};
use iced::Length::Fill;
use iced::{system, Element, Font, Task, Theme};
use rfd::FileDialog;

use crate::config::{Config, DisplayBase};
use crate::memory::ATmemory;

#[derive(Debug)]
pub struct UInterface {
    cpu: ATmemory,
    cycle_counter: usize,
    display_base_registers: DisplayBase,
    display_base_stack: DisplayBase,
    flash_file: Option<PathBuf>,
    instructions_per_second: u32,
    memory_bytes_per_column: usize,
    memory_bytes_per_row: usize,
    show_settings: bool,
    status_message: Option<String>,
    temp_display_base_registers: DisplayBase,
    temp_display_base_stack: DisplayBase,
    temp_instructions_per_second: u32,
    temp_memory_bytes_per_column: usize,
    temp_memory_bytes_per_row: usize,
    theme: Theme,
    theme_mode: Mode,
    run_active: bool,
    bridge_address: String,
    temp_bridge_address: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    CPUstep,
    CloseSettings,
    LoadBinToFlash,
    LoadHexToFlash,
    OpenSettings,
    Reset,
    Restart,
    RunTick,
    RunToggle,
    SaveSettings,
    SettingsColumnChanged(usize),
    SettingsDisplayBaseRegistersChanged(DisplayBase),
    SettingsDisplayBaseStackChanged(DisplayBase),
    SettingsInsSecChanged(u32),
    SettingsRowChanged(usize),
    SettingsBridgeChanged(String),
    ThemeChanged(Mode),
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
            let seg_byte = if usize::from(self.cpu.pc() * 2) == seg
                || usize::from((self.cpu.pc() * 2) + 1) == seg
            {
                text!(" {:02X}", self.cpu.flash()[seg]).style(text::primary)
            } else {
                text!(" {:02X}", self.cpu.flash()[seg])
            };
            row = row.push(seg_byte.font(Font::MONOSPACE));
        }

        row = row.push(text("        ").font(Font::MONOSPACE));

        for seg in addr..addr + self.memory_bytes_per_row {
            let seg_char = if usize::from(self.cpu.pc() * 2) == seg
                || usize::from((self.cpu.pc() * 2) + 1) == seg
            {
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
        let mut cpu = ATmemory::init();
        cpu.connect_to_hw(&config.bridge_address).ok();

        Self {
            theme_mode: match config.theme.mode.as_str() {
                "Light" => Mode::Light,
                "Dark" => Mode::Dark,
                _ => Mode::None,
            },
            theme: Theme::Dark,
            cpu,
            flash_file: None,
            memory_bytes_per_row: config.display.memory_bytes_per_row,
            memory_bytes_per_column: config.display.memory_bytes_per_column,
            show_settings: false,
            temp_memory_bytes_per_row: config.display.memory_bytes_per_row,
            temp_memory_bytes_per_column: config.display.memory_bytes_per_column,
            instructions_per_second: 1,
            temp_instructions_per_second: 1,
            cycle_counter: 0,
            temp_display_base_registers: DisplayBase::Decimal,
            display_base_registers: config.display_base.registers,
            temp_display_base_stack: DisplayBase::Hexadecimal,
            display_base_stack: config.display_base.stack,
            run_active: false,
            status_message: None,
            bridge_address: config.bridge_address.clone(),
            temp_bridge_address: config.bridge_address.clone(),
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
            display_base: crate::config::DisplayBaseConfig {
                registers: self.display_base_registers,
                stack: self.display_base_stack,
            },
            bridge_address: self.bridge_address.clone(),
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
            rows = rows.push(
                text!(
                    "R{:02}={}",
                    reg,
                    Self::format_value(self.cpu.memory()[reg], self.display_base_registers)
                )
                .font(Font::MONOSPACE),
            );
        }

        scrollable(rows.padding(4)).width(Fill).into()
    }

    fn render_sram(&self) -> Element<'_, Message> {
        let mut rows = column![].spacing(2);
        for sp in (0x0060..0x0460).rev() {
            match sp == self.cpu.sp() as usize {
                true => {
                    rows = rows.push(
                        text!(
                            "{:#05X}={}",
                            sp,
                            Self::format_value(self.cpu.memory()[sp], self.display_base_stack)
                        )
                        .font(Font::MONOSPACE)
                        .style(text::primary),
                    );
                }
                false => {
                    rows = rows.push(
                        text!(
                            "{:#05X}={}",
                            sp,
                            Self::format_value(self.cpu.memory()[sp], self.display_base_stack)
                        )
                        .font(Font::MONOSPACE),
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

    fn render_bits(label: &str, value: u8) -> Element<'_, Message> {
        let mut cols = row![text!("{label} | ")].spacing(2);
        for idx in 0..8 {
            match ((value << idx) & 0x80) == 128 {
                true => cols = cols.push(text("▪").style(text::primary)),
                false => cols = cols.push(text("▪")),
            }
        }
        cols.into()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let theme_sub = system::theme_changes().map(Message::ThemeChanged);

        if self.run_active {
            let interval_ms: u64 = (1000.0 / self.instructions_per_second as f64) as u64;
            let timer_sub =
                iced::time::every(Duration::from_millis(interval_ms)).map(|_| Message::RunTick);
            iced::Subscription::batch(vec![theme_sub, timer_sub])
        } else {
            theme_sub
        }
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
            Message::ThemeChanged(mode) => {
                state.theme = UInterface::mode_to_theme(mode);
                state.theme_mode = mode;
                Task::none()
            }
            Message::LoadBinToFlash => {
                state.run_active = false;
                state.cpu = ATmemory::init();
                state.cycle_counter = 0;
                state.flash_file = None;
                let file = FileDialog::new()
                    .add_filter("Binary file", &["bin"])
                    .set_directory(std::env::current_dir().unwrap_or(std::env::home_dir().unwrap()))
                    .set_title("Open binary file")
                    .pick_file();

                if let Some(path) = file.clone() {
                    if let Some(path_str) = path.to_str() {
                        let _ = state.cpu.load_bin(path_str);
                    } else {
                        state.status_message = Some("Error: Path is not valid UTF-8.".to_string());
                        return Task::none();
                    }
                } else {
                    state.status_message = Some("Error: No file selected.".to_string());
                    return Task::none();
                }
                state.flash_file = file.clone();
                state.cpu.connect_to_hw(&state.bridge_address).ok();
                state.status_message = Some(format!(
                    "Loaded {}",
                    state.flash_file.clone().unwrap().as_os_str().display()
                ));
                Task::none()
            }
            Message::LoadHexToFlash => {
                state.run_active = false;
                state.cpu = ATmemory::init();
                state.cycle_counter = 0;
                state.flash_file = None;
                let file = FileDialog::new()
                    .add_filter("Hex file", &["hex"])
                    .set_directory(std::env::current_dir().unwrap_or(std::env::home_dir().unwrap()))
                    .set_title("Open hex file")
                    .pick_file();

                if let Some(path) = file.clone() {
                    if let Some(path_str) = path.to_str() {
                        let _ = state.cpu.load_hex(path_str);
                    } else {
                        state.status_message = Some("Error: Path is not valid UTF-8.".to_string());
                        return Task::none();
                    }
                } else {
                    state.status_message = Some("Error: No file selected.".to_string());
                    return Task::none();
                }

                state.flash_file = file.clone();
                state.cpu.connect_to_hw(&state.bridge_address).ok();
                state.status_message = Some(format!(
                    "Loaded {}",
                    state.flash_file.clone().unwrap().as_os_str().display()
                ));
                Task::none()
            }
            Message::Reset => {
                state.run_active = false;
                state.cpu.reset();
                state.cycle_counter = 0;
                state.cpu.connect_to_hw(&state.bridge_address).ok();
                Task::none()
            }
            Message::Restart => {
                state.run_active = false;
                state.cpu = ATmemory::init();
                state.cycle_counter = 0;
                state.flash_file = None;
                state.cpu.connect_to_hw(&state.bridge_address).ok();
                Task::none()
            }
            Message::CPUstep => {
                state.run_active = false;
                state.cpu.update_io();
                if let Err(e) = state.cpu.step() {
                    state.status_message = Some(format!("Execution error: {}", e));
                };
                state.cycle_counter += 1;
                Task::none()
            }
            Message::OpenSettings => {
                state.run_active = false;
                state.temp_memory_bytes_per_column = state.memory_bytes_per_column;
                state.temp_memory_bytes_per_row = state.memory_bytes_per_row;
                state.show_settings = true;
                Task::none()
            }
            Message::CloseSettings => {
                state.temp_memory_bytes_per_column = state.memory_bytes_per_column;
                state.temp_memory_bytes_per_row = state.memory_bytes_per_row;
                state.show_settings = false;
                state.cpu.connect_to_hw(&state.bridge_address).ok();
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
                state.instructions_per_second = state.temp_instructions_per_second;
                state.display_base_registers = state.temp_display_base_registers;
                state.display_base_stack = state.temp_display_base_stack;
                state.bridge_address = state.temp_bridge_address.trim().to_string();
                state.show_settings = false;
                state.cpu.connect_to_hw(&state.bridge_address).ok();
                let _ = state.save_config();
                Task::none()
            }
            Message::SettingsInsSecChanged(val) => {
                state.temp_instructions_per_second = val;
                Task::none()
            }
            Message::SettingsDisplayBaseRegistersChanged(display_base) => {
                state.temp_display_base_registers = display_base;
                Task::none()
            }
            Message::SettingsDisplayBaseStackChanged(display_base) => {
                state.temp_display_base_stack = display_base;
                Task::none()
            }
            Message::RunTick => {
                state.cpu.update_io();
                if let Err(e) = state.cpu.step() {
                    state.run_active = false;
                    state.status_message = Some(format!("Execution error: {}", e));
                    return Task::none();
                }
                state.cycle_counter += 1;
                Task::none()
            }
            Message::RunToggle => {
                state.run_active = !state.run_active;
                Task::none()
            }
            Message::SettingsBridgeChanged(addr) => {
                state.temp_bridge_address = addr;
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
            },
            if self.flash_file.is_some() {
                match self.run_active {
                    true => button(text("Disable Auto Run"))
                        .style(button::secondary)
                        .on_press(Message::RunToggle),
                    false => button(text("Enable Auto Run")).on_press(Message::RunToggle),
                }
            } else {
                button(text("Auto Run"))
            },
            if self.flash_file.is_some() {
                button(text("Reset")).on_press(Message::Reset)
            } else {
                button(text("Reset"))
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
                    text!("Frequency | {:02} Hz", self.instructions_per_second),
                    Self::render_sreg(self),
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

        let right_sidebar = column![
            Self::render_bits("PortA", self.cpu.memory()[0x3B]),
            Self::render_bits("DDRA", self.cpu.memory()[0x3A]),
            Self::render_bits("PinA", self.cpu.memory()[0x39]),
            rule::horizontal(2),
            Self::render_bits("PortB", self.cpu.memory()[0x38]),
            Self::render_bits("DDRB", self.cpu.memory()[0x37]),
            Self::render_bits("PinB", self.cpu.memory()[0x36]),
            rule::horizontal(2),
            Self::render_bits("PortC", self.cpu.memory()[0x35]),
            Self::render_bits("DDRC", self.cpu.memory()[0x34]),
            Self::render_bits("PinC", self.cpu.memory()[0x33]),
            rule::horizontal(2),
            Self::render_bits("PortD", self.cpu.memory()[0x32]),
            Self::render_bits("DDRD", self.cpu.memory()[0x31]),
            Self::render_bits("PinD", self.cpu.memory()[0x30]),
            // text("Timer0"),
            // text("Timer1"),
            // text("Timer2"),
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
        if let Some(status) = self.status_message.as_ref() {
            status_bar = status_bar.push(text(status).width(Fill));
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
            text("Breadboard").size(36),
            text(env!("CARGO_PKG_VERSION")).width(Fill),
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
                text("CPU frequency:"),
                slider(
                    1.0..=10.0,
                    self.temp_instructions_per_second as f64,
                    |val| { Message::SettingsInsSecChanged(val as u32) }
                ),
                text!("{} instructions/second", self.temp_instructions_per_second)
            ]
            .spacing(4)
            .padding(4),
        );

        content = content.push(
            row![
                text("Display registers in:"),
                pick_list(
                    DisplayBase::ALL,
                    Some(self.temp_display_base_registers),
                    Message::SettingsDisplayBaseRegistersChanged
                )
            ]
            .spacing(4)
            .padding(4),
        );

        content = content.push(
            row![
                text("Display stack in:"),
                pick_list(
                    DisplayBase::ALL,
                    Some(self.temp_display_base_stack),
                    Message::SettingsDisplayBaseStackChanged
                )
            ]
            .spacing(4)
            .padding(4),
        );

        content = content.push(
            row![
                text("Hardware bridge address:"),
                text_input("", &self.temp_bridge_address).on_input(Message::SettingsBridgeChanged)
            ]
            .spacing(4)
            .padding(4),
        );
        container(content).into()
    }

    fn format_value(value: u8, base: DisplayBase) -> String {
        match base {
            DisplayBase::Binary => format!("{:#010b}", value),
            DisplayBase::Decimal => format!("{}", value),
            DisplayBase::Hexadecimal => format!("{:#04X}", value),
        }
    }
}
