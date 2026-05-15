mod config;
mod memory;
mod port;
mod tests;

#[cfg(feature = "gui")]
mod gui;

#[cfg(feature = "tui")]
mod tui;

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
        use crate::tui::TUInterface;
        let app = TUInterface::init();
        app.run();
    }
}
