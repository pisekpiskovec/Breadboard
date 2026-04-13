use std::{cell::RefCell, rc::Rc};

use appcui::prelude::{Label, Window};

#[Window]
pub struct FlashWindow {
    config: Rc<RefCell<crate::config::Config>>,
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
}

impl FlashWindow {
    pub fn new(config: Rc<RefCell<crate::config::Config>>, cpu: Rc<RefCell<crate::memory::ATmemory>>) -> Self {
        let mut win = Self {
            base: window!("'Flash',a:c,w:30,h:30,flags:sizeable"),
            config,
            cpu,
        };
        Self::render_flash_memory(&mut win);
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
        eprintln!("Formatting row at addr: {:#06X}, PC: {:#06X}", addr, self.cpu.borrow().pc());
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

        for addr in (start..end).step_by(memory_bytes_per_row) {
            let row = window.format_memory_row(addr);
            window.add(Label::new(&row, LayoutBuilder::new().alignment(Alignment::TopLeft).build()));
        }
    }
}
