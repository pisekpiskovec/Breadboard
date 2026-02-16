mod memory;
mod main_ui;

use crate::main_ui::UInterface;

fn main() -> iced::Result {
    iced::application(UInterface::new, UInterface::update, UInterface::view)
        .title("Breadboard")
        .theme(UInterface::theme)
        .subscription(UInterface::subscription)
        .run()
}
