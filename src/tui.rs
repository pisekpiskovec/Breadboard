use crate::{config::Config, memory::ATmemory, tui::{flash::FlashWindow, status::StatusWindow}};

mod flash;
mod status;

pub struct TUInterface {
    cpu: ATmemory,
    config: Config,
}

impl TUInterface {
    pub fn new() -> Self {
        let config = Config::load().unwrap_or_default();
        let mut cpu = ATmemory::init();
        cpu.connect_to_hw(&config.bridge_address).ok();

        Self { cpu, config }
    }

    pub fn init() -> appcui::prelude::App {
        let mut app = appcui::system::App::new().title("Breadboard").build().unwrap();
        let mut interface = Self::new();

        app.add_window(FlashWindow::new(interface.config, interface.cpu));
        app.add_window(StatusWindow::new(interface.cpu));

        app
    }
}
