mod config;
mod memory;
mod port;
mod tests;

#[cfg(feature = "gui")]
mod gui;

#[cfg(feature = "tui")]
mod tui;

fn main() {
    #[cfg(not(any(feature = "gui", feature = "tui")))]
    compile_error!("Mut enable either 'gui' or 'tui' feature");

    #[cfg(feature = "gui")]
    {
        use crate::gui::GUInterface;
        // Try to run GUI
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            iced::application(GUInterface::new, GUInterface::update, GUInterface::view)
                .title("Breadboard")
                .theme(GUInterface::theme)
                .subscription(GUInterface::subscription)
                .window(GUInterface::window_settings())
                .run()
        }));
        match result {
            Ok(gui_result) => {
                gui_result.expect("GUI initialisation error");
            }
            Err(_) => {
                eprintln!(
                    "GUI failed to initialize (headless environment?). Falling back to TUI..."
                );
                #[cfg(feature = "tui")]
                {
                    run_tui();
                }
                #[cfg(not(feature = "tui"))]
                {
                    eprintln!("TUI not available. Compile with --features tui");
                    std::process::exit(1);
                }
            }
        }
    }

    // Only TUI available
    #[cfg(all(feature = "tui", not(feature = "gui")))]
    {
        run_tui();
    }
}

#[cfg(feature = "tui")]
fn run_tui() {
    use crate::tui::TUInterface;
    let app = TUInterface::init();
    app.run();
}
