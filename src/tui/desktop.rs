use std::{cell::RefCell, rc::Rc};

use appcui::prelude::Desktop;

use crate::tui::ascii::AsciiFlashWindow;

#[Desktop(events = [MenuEvents, AppBarEvents, DesktopEvents], commands=[OpenBin, OpenHex, ShowAscii])]
pub struct TDesktop {
    config: Rc<RefCell<crate::config::Config>>,
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
    menu_file: Handle<appbar::MenuButton>,
    menu_view: Handle<appbar::MenuButton>,
}

impl TDesktop {
    pub fn new(
        config: Rc<RefCell<crate::config::Config>>,
        cpu: Rc<RefCell<crate::memory::ATmemory>>,
    ) -> Self {
        Self {
            base: Desktop::new(),
            config,
            cpu,
            menu_file: Handle::None,
            menu_view: Handle::None,
        }
    }
}

impl MenuEvents for TDesktop {
    fn on_command(
        &mut self,
        _menu: Handle<Menu>,
        _item: Handle<menu::Command>,
        command: tdesktop::Commands,
    ) {
        match command {
            tdesktop::Commands::OpenBin => {
                let file = dialogs::open(
                    "Open binary file",
                    "",
                    dialogs::Location::Last,
                    Some("Binaries = [bin]"),
                    dialogs::OpenFileDialogFlags::Icons
                        | dialogs::OpenFileDialogFlags::CheckIfFileExists,
                );

                if let Some(path) = file
                    && let Some(path_str) = path.to_str()
                {
                    log!("INFO", "Loading: {}", path_str);
                    match self.cpu.borrow_mut().load_bin(path_str) {
                        Ok(_) => {
                            log!("INFO", "File loaded succesfully")
                        }
                        Err(e) => {
                            log!("ERROR", "Failed to load file: {}", e)
                        }
                    }
                }
            }
            tdesktop::Commands::OpenHex => {
                let file = dialogs::open(
                    "Open hex file",
                    "",
                    dialogs::Location::Last,
                    Some("Hex file = [hex]"),
                    dialogs::OpenFileDialogFlags::Icons
                        | dialogs::OpenFileDialogFlags::CheckIfFileExists,
                );

                if let Some(path) = file
                    && let Some(path_str) = path.to_str()
                {
                    log!("INFO", "Loading: {}", path_str);
                    match self.cpu.borrow_mut().load_hex(path_str) {
                        Ok(_) => {
                            log!("INFO", "File loaded succesfully")
                        }
                        Err(e) => {
                            log!("ERROR", "Failed to load file: {}", e)
                        }
                    }
                }
            }
            tdesktop::Commands::ShowAscii => {
                let ascii = AsciiFlashWindow::new(Rc::clone(&self.config), Rc::clone(&self.cpu));
                self.add_window(ascii);
            }
        }
    }
}

impl AppBarEvents for TDesktop {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.menu_file);
        appbar.show(self.menu_view);
    }
}

impl DesktopEvents for TDesktop {
    fn on_start(&mut self) {
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
        self.menu_file = self.appbar().add(appbar::MenuButton::new(
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
        self.menu_view = self.appbar().add(appbar::MenuButton::new(
            "&View",
            menu_view,
            2,
            appbar::Side::Left,
        ));
    }
}
