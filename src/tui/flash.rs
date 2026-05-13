use std::{cell::RefCell, rc::Rc};

use appcui::prelude::{ActionRequest, Window, WindowEvents};

use crate::memory::ATmemory;

#[Window(events=WindowEvents)]
pub struct FlashWindow {
    config: Rc<RefCell<crate::config::Config>>,
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
    flash: Handle<TextArea>,
}

impl FlashWindow {
    pub fn new(
        config: Rc<RefCell<crate::config::Config>>,
        cpu: Rc<RefCell<crate::memory::ATmemory>>,
    ) -> Self {
        let max_window: usize = config.borrow().display.memory_bytes_per_column * 2;
        let bytes_per_row: usize = config.borrow().display.memory_bytes_per_row;
        let max_rows: usize = (max_window / bytes_per_row) + 1;

        let mut win = Self {
            base: Window::new(
                "Flash",
                LayoutBuilder::new()
                    .alignment(Alignment::Center)
                    .width(33)
                    .height((max_rows as u8) + 2)
                    .build(),
                window::Flags::Sizeable,
            ),
            config,
            cpu,
            flash: Handle::None,
        };

        win.flash = win.add(TextArea::new(
            "0000:  00 00 00 00 00 00 00 00  . . . . . . . .",
            LayoutBuilder::new().dock(Dock::Fill).build(),
            textarea::Flags::ReadOnly | textarea::Flags::ScrollBars,
        ));

        win
    }

    pub fn load_flash(&mut self) -> String {
        let mut flash = String::new();
        let bytes_per_row = self.config.borrow().display.memory_bytes_per_row;

        for addr in (0..16384).step_by(bytes_per_row) {
            flash.push_str(&format!("{:04X}: ", addr));

            // Hex
            for seg in addr..addr + bytes_per_row {
                let cpu_seg = self.cpu.borrow().flash()[seg];

                let seg_byte = &format!(" {:02X}", cpu_seg);
                flash.push_str(seg_byte);
            }

            // ASCII
            for seg in addr..addr + bytes_per_row {
                let cpu_seg = self.cpu.borrow().flash()[seg];
                let seg_byte = &format!(
                    " {}",
                    if (32..126).contains(&cpu_seg) {
                        char::from(cpu_seg)
                    } else {
                        '.'
                    }
                );
                flash.push_str(seg_byte);
            }

            flash.push('\n');
        }

        let handle = self.flash;
        if let Some(ta) = self.control_mut(handle) {
            ta.set_text(&flash);
        }

        flash
    }
}

impl WindowEvents for FlashWindow {
    fn on_cancel(&mut self) -> ActionRequest {
        self.cpu.replace(ATmemory::init());
        ActionRequest::Allow
    }
}
