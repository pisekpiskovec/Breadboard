mod config;
mod memory;
mod ui;
mod tests;

use crate::ui::UInterface;

fn main() -> iced::Result {
    iced::application(UInterface::new, UInterface::update, UInterface::view)
        .title("Breadboard")
        .theme(UInterface::theme)
        .subscription(UInterface::subscription)
        .run()
}
