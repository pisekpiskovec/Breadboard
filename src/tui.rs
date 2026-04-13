use std::{cell::RefCell, rc::Rc};

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
        let interface = Self::new();

        let cpu_shared = Rc::new(RefCell::new(interface.cpu));
        let config_shared = Rc::new(RefCell::new(interface.config));

        app.add_window(FlashWindow::new(Rc::clone(&config_shared), Rc::clone(&cpu_shared)));
        app.add_window(StatusWindow::new(Rc::clone(&cpu_shared)));

        app
    }
}
