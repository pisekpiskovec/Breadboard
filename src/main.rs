mod config;
mod memory;
mod port;
mod tests;
mod gui;

use crate::gui::UInterface;

fn main() -> iced::Result {
    iced::application(UInterface::new, UInterface::update, UInterface::view)
        .title("Breadboard")
        .theme(UInterface::theme)
        .subscription(UInterface::subscription)
        .run()
}
