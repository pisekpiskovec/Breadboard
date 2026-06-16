use std::{cell::RefCell, rc::Rc};

use crate::{
    config::Config,
    memory::ATmemory,
    tui::{desktop::TDesktop, memory::MemoryWindow, ports::PortsWindow, status::StatusWindow},
};

mod config;
mod desktop;
mod flash;
mod memory;
mod ports;
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
        let mut interface = Self::new();

        let _ = interface
            .cpu
            .connect_to_hw(&interface.config.bridge_address);

        let cpu_shared = Rc::new(RefCell::new(interface.cpu));
        let config_shared = Rc::new(RefCell::new(interface.config));

        let mut app = appcui::system::App::new()
            .title("Breadboard")
            .app_bar()
            .desktop(TDesktop::new(
                Rc::clone(&config_shared),
                Rc::clone(&cpu_shared),
            ))
            .build()
            .unwrap();

        app.add_window(StatusWindow::new(Rc::clone(&cpu_shared)));
        app.add_window(MemoryWindow::new(
            Rc::clone(&config_shared),
            Rc::clone(&cpu_shared),
        ));
        app.add_window(PortsWindow::new(Rc::clone(&cpu_shared)));

        app
    }
}
