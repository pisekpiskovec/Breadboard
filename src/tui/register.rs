use std::{cell::RefCell, rc::Rc, time::Duration};

use appcui::prelude::Window;

#[Window(events = TimerEvents)]
pub struct RegisterWindow {
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
    reg_lb: [Handle<Label>; 32],
}

impl RegisterWindow {
    pub fn new(cpu: Rc<RefCell<crate::memory::ATmemory>>) -> Self {
        let mut win = Self {
            base: window!("'Registers',a:bl,w:31,h:34"),
            cpu,
            reg_lb: [Handle::None; 32],
        };

        for reg in 0..32 {
            win.reg_lb[reg] = win.add(Label::new(
                "",
                LayoutBuilder::new().x(0).y(reg as u8).width(32).build(),
            ));
        }

        if let Some(timer) = win.timer() {
            timer.start(Duration::from_millis(100));
        }

        win
    }
}

impl TimerEvents for RegisterWindow {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        for reg in 0..32 {
            let displayed_part = format!(
                "R{0:02} = {1:#010b} / {1:03} / {1:#04X}",
                reg,
                self.cpu.borrow().memory()[reg]
            );
            let h = self.reg_lb[reg];
            if let Some(lb) = self.control_mut(h) {
                lb.set_caption(&displayed_part);
            }
        }
        EventProcessStatus::Processed
    }
}
