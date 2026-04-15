use std::{cell::RefCell, rc::Rc};

use appcui::prelude::{Label, Window};

#[Window(events = [MenuEvents, AppBarEvents], commands=[OpenBin, OpenHex, ShowAscii])]
pub struct FlashWindow {
    config: Rc<RefCell<crate::config::Config>>,
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
    menu_file: Handle<appbar::MenuButton>,
    menu_view: Handle<appbar::MenuButton>,
}

impl FlashWindow {
    pub fn new(
        config: Rc<RefCell<crate::config::Config>>,
        cpu: Rc<RefCell<crate::memory::ATmemory>>,
    ) -> Self {
        let mut win = Self {
            base: window!("'Flash',a:c,w:32,h:32,flags:sizeable"),
            config,
            cpu,
            menu_file: Handle::None,
            menu_view: Handle::None,
        };
        Self::render_flash_memory(&mut win);

        // File menu
        let mut menu_file = Menu::new();
        menu_file.add(menu::Command::new(
            "Open .&bin",
            key!("Alt+O"),
            flashwindow::Commands::OpenBin,
        ));
        menu_file.add(menu::Command::new(
            "Open .&hex",
            key!("Ctrl+O"),
            flashwindow::Commands::OpenHex,
        ));
        win.menu_file = win.appbar().add(appbar::MenuButton::new(
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
            flashwindow::Commands::ShowAscii,
        ));
        win.menu_view = win.appbar().add(appbar::MenuButton::new(
            "&View",
            menu_view,
            2,
            appbar::Side::Left,
        ));

        win
    }

    fn get_memory_window_boundary(&self) -> (usize, usize) {
        let pc = self.cpu.borrow().pc() as i32;
        let half_window = self.config.borrow().display.memory_bytes_per_column as i32;

        let start = pc - half_window;
        let end = pc + half_window + 1;

        let start = start.max(0) as usize;
        let end = end.min(self.cpu.borrow().flash().len() as i32) as usize;

        (start, end)
    }

    fn format_memory_row(&self, addr: usize) -> String {
        let mut row = String::new();

        row.push_str(&format!("{:04X}: ", addr));

        for seg in addr..addr + self.config.borrow().display.memory_bytes_per_row {
            let seg_byte = &format!(" {:02X}", self.cpu.borrow().flash()[seg]);
            row.push_str(seg_byte);
        }
        row
    }

    fn render_flash_memory(window: &mut FlashWindow) {
        let (start, end) = window.get_memory_window_boundary();

        let memory_bytes_per_row = window.config.borrow().display.memory_bytes_per_row;

        for (idx, addr) in (start..end).step_by(memory_bytes_per_row).enumerate() {
            let row = window.format_memory_row(addr);
            window.add(Label::new(
                &row,
                LayoutBuilder::new().x(0).y(idx as u32).width(32).build(),
            ));
        }
    }
}

impl MenuEvents for FlashWindow {
    fn on_command(
        &mut self,
        menu: Handle<Menu>,
        item: Handle<menu::Command>,
        command: flashwindow::Commands,
    ) {
        match command {
            cmd if cmd == flashwindow::Commands::OpenBin => {
                self.cpu.borrow_mut().load_bin("/home/pisek/Obrázky/2P.png");
            }
            cmd if cmd == flashwindow::Commands::OpenHex => {
                self.cpu
                    .borrow_mut()
                    .load_bin("/home/pisek/Projekty/Rust/Breadboard/tests/template-tst/test.hex");
            }
            cmd if cmd == flashwindow::Commands::ShowAscii => {}
            _ => {}
        }
    }
}

impl AppBarEvents for FlashWindow {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.menu_file);
        appbar.show(self.menu_view);
    }
}
