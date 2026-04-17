use std::{cell::RefCell, rc::Rc};

use appcui::prelude::Desktop;

#[Desktop(events = [MenuEvents, AppBarEvents], commands=[OpenBin, OpenHex, ShowAscii])]
pub struct TDesktop {
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
    menu_file: Handle<appbar::MenuButton>,
    menu_view: Handle<appbar::MenuButton>,
}

impl TDesktop {
    pub fn new(
        cpu: Rc<RefCell<crate::memory::ATmemory>>,
    ) -> Self {
        let mut desk = Self {
            base: Desktop::new(),
            cpu,
            menu_file: Handle::None,
            menu_view: Handle::None,
        };
        
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
        desk.menu_file = desk.appbar().add(appbar::MenuButton::new(
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
        desk.menu_view = desk.appbar().add(appbar::MenuButton::new(
            "&View",
            menu_view,
            2,
            appbar::Side::Left,
        ));

        desk
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
            _ => {}
        }
    }
}

impl AppBarEvents for TDesktop {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.menu_file);
        appbar.show(self.menu_view);
    }
}
