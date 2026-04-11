mod config;
mod memory;
mod port;
mod tests;

#[cfg(feature = "gui")]
mod gui;

#[cfg(feature = "tui")]
#[path = "tui/flash.rs"]
mod flash;

#[cfg(feature = "tui")]
#[path = "tui/status.rs"]
mod status;

fn main() {
    #[cfg(all(feature = "gui", feature = "tui"))]
    compile_error!("Cannot enable both 'gui' and 'tui' features simultaneously");

    #[cfg(not(any(feature = "gui", feature = "tui")))]
    compile_error!("Must enable either 'gui' or 'tui' feature");

    #[cfg(feature = "gui")]
    {
        use crate::gui::GUInterface;
        iced::application(GUInterface::new, GUInterface::update, GUInterface::view)
            .title("Breadboard")
            .theme(GUInterface::theme)
            .subscription(GUInterface::subscription)
            .run()
            .expect("GUI initialisation error")
    }

    #[cfg(feature = "tui")]
    {
        use crate::memory::ATmemory;
        let mut cpu = ATmemory::init();
        cpu.load_hex("/home/pisek/Projekty/Rust/Breadboard/tests/rcall-tst/test.hex").ok();

        use crate::config::Config;
        let config = Config::load().ok().unwrap();

        use appcui::system::App;
        let mut app = App::new().title("Breadboard").build().unwrap();

        use flash::FlashWindow;
        let flash_window = FlashWindow::new(config, cpu);

        use status::StatusWindow;
        let status_window = StatusWindow::new(cpu);

        app.add_window(flash_window);
        app.add_window(status_window);
        app.run();
    }
}
