use std::{cell::RefCell, rc::Rc, time::Duration};

use appcui::prelude::{Label, Window};

#[Window(events = TimerEvents)]
pub struct FlashWindow {
    config: Rc<RefCell<crate::config::Config>>,
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
    pub flash: Vec<Handle<Label>>,
}

impl FlashWindow {
    pub fn new(
        config: Rc<RefCell<crate::config::Config>>,
        cpu: Rc<RefCell<crate::memory::ATmemory>>,
    ) -> Self {
        let max_window: usize = config.borrow().display.memory_bytes_per_column * 2;
        let max_rows: usize = max_window / config.borrow().display.memory_bytes_per_row;
        let mut win = Self {
            base: Window::new(
                "Flash",
                LayoutBuilder::new()
                    .alignment(Alignment::Center)
                    .width(32)
                    .height((max_rows as u8) + 2)
                    .build(),
                window::Flags::Sizeable,
            ),
            config,
            cpu,
            flash: vec![Handle::None; max_rows],
        };

        for byte in 0..win.flash.len() {
            win.flash[byte] = win.add(Label::new(
                "",
                LayoutBuilder::new().x(0).y(byte as u8).width(32).build(),
            ));
        }

        Self::render_flash_memory(&mut win);

        if let Some(timer) = win.timer() {
            timer.start(Duration::from_millis(100));
        }

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

    fn render_flash_memory(&mut self) {
        let (start, end) = self.get_memory_window_boundary();

        let memory_bytes_per_row = self.config.borrow().display.memory_bytes_per_row;

        for (idx, addr) in (start..end).step_by(memory_bytes_per_row).enumerate() {
            let row = self.format_memory_row(addr);
            let h = self.flash[idx];
            if let Some(lb) = self.control_mut(h) {
                lb.set_caption(&row);
            }
        }
    }
}

impl TimerEvents for FlashWindow {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        Self::render_flash_memory(self);
        EventProcessStatus::Processed
    }
}
