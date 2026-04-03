mod config;
mod memory;
mod port;
mod tests;
mod gui;

use crate::gui::GUInterface;

fn main() -> iced::Result {
    iced::application(GUInterface::new, GUInterface::update, GUInterface::view)
        .title("Breadboard")
        .theme(GUInterface::theme)
        .subscription(GUInterface::subscription)
        .run()
}
