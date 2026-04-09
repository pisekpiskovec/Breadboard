mod config;
mod memory;
mod port;
mod tests;

#[cfg(feature = "gui")]
mod gui;

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

        use crate::config::Config;
        let config = Config::load().ok().unwrap();

        use appcui::system::App;
        let mut app = App::new().build().unwrap();

        use flash::FlashWindow;
        let flash_window = FlashWindow::new(config, cpu);

        app.add_window(flash_window);
        app.run();
    }
}
