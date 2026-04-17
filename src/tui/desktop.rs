use std::{cell::RefCell, rc::Rc};

use appcui::prelude::Desktop;

#[Desktop(events = [MenuEvents, AppBarEvents], commands=[OpenBin, OpenHex, ShowAscii])]
pub struct TDesktop {
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
}

impl TDesktop {
    pub fn new(cpu: Rc<RefCell<crate::memory::ATmemory>>) -> Self {
        Self {
            base: Desktop::new(),
            cpu,
        }
    }
}

impl MenuEvents for TDesktop {
    fn on_command(
        &mut self,
        menu: Handle<Menu>,
        item: Handle<menu::Command>,
        command: tdesktop::Commands,
    ) {
        match command {
            tdesktop::Commands::OpenBin => {
                let _ = self.cpu.borrow_mut().load_bin("/home/pisek/Obrázky/2P.png");
            }
            tdesktop::Commands::OpenHex => {
                let _ = self
                    .cpu
                    .borrow_mut()
                    .load_bin("/home/pisek/Projekty/Rust/Breadboard/tests/template-tst/test.hex");
            }
            tdesktop::Commands::ShowAscii => {}
        }
    }
}

impl AppBarEvents for TDesktop {
    fn on_update(&self, appbar: &mut AppBar) {
        // File menu
        let mut menu_file = Menu::new();
        menu_file.add(menu::Command::new(
            "Open .&bin",
            key!("Alt+O"),
            tdesktop::Commands::OpenBin,
        ));
        menu_file.add(menu::Command::new(
            "Open .&hex",
            key!("Ctrl+O"),
            tdesktop::Commands::OpenHex,
        ));
        appbar.add(appbar::MenuButton::new(
            "&File",
            menu_file,
            1,
            appbar::Side::Left,
        ));

        // View menu
        let mut menu_view = Menu::new();
        menu_view.add(menu::Command::new(
            "Show Flash as &ASCII",
            Key::None,
            tdesktop::Commands::ShowAscii,
        ));
        appbar.add(appbar::MenuButton::new(
            "&View",
            menu_view,
            2,
            appbar::Side::Left,
        ));
    }
}
