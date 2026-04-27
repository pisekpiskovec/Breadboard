use std::{cell::RefCell, rc::Rc, time::Duration};

use appcui::prelude::{Label, Window};

#[Window(events=[WindowEvents, TimerEvents])]
pub struct StatusWindow {
    cc_lb: Handle<Label>,
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
    pc_lb: Handle<Label>,
    sp_lb: Handle<Label>,
    xp_lb: Handle<Label>,
    yp_lb: Handle<Label>,
    zp_lb: Handle<Label>,
}

impl StatusWindow {
    pub fn new(cpu: Rc<RefCell<crate::memory::ATmemory>>) -> Self {
        let mut win = Self {
            base: window!("'Status',x:0,y:1,w:32,h:8,flags:NoCloseButton"),
            cc_lb: Handle::None,
            cpu,
            pc_lb: Handle::None,
            sp_lb: Handle::None,
            xp_lb: Handle::None,
            yp_lb: Handle::None,
            zp_lb: Handle::None,
        };
        win.set_hotkey(key!("Alt+S"));
        win.pc_lb = win.add(Label::new(
            "",
            LayoutBuilder::new().x(0).y(0).width(32).build(),
        ));
        win.sp_lb = win.add(Label::new(
            "",
            LayoutBuilder::new().x(0).y(1).width(32).build(),
        ));
        win.xp_lb = win.add(Label::new(
            "",
            LayoutBuilder::new().x(0).y(2).width(32).build(),
        ));
        win.yp_lb = win.add(Label::new(
            "",
            LayoutBuilder::new().x(0).y(3).width(32).build(),
        ));
        win.zp_lb = win.add(Label::new(
            "",
            LayoutBuilder::new().x(0).y(4).width(32).build(),
        ));
        win.cc_lb = win.add(Label::new(
            "",
            LayoutBuilder::new().x(0).y(5).width(32).build(),
        ));

        if let Some(timer) = win.timer() {
            timer.start(Duration::from_millis(100));
        }

        win
    }
}

impl WindowEvents for StatusWindow {
    fn on_cancel(&mut self) -> ActionRequest {
        ActionRequest::Deny
    }
}

impl TimerEvents for StatusWindow {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        let text = format!("Program Counter | {:#08X}", self.cpu.borrow().pc());
        let h = self.pc_lb;
        if let Some(lb) = self.control_mut(h) {
            lb.set_caption(&text);
        }

        let text = format!("Stack Pointer | {:#06X}", self.cpu.borrow().sp());
        let h = self.sp_lb;
        if let Some(lb) = self.control_mut(h) {
            lb.set_caption(&text);
        }

        let text = format!("X Pointer | {:#06X}", self.cpu.borrow().xp());
        let h = self.xp_lb;
        if let Some(lb) = self.control_mut(h) {
            lb.set_caption(&text);
        }

        let text = format!("Y Pointer | {:#06X}", self.cpu.borrow().yp());
        let h = self.yp_lb;
        if let Some(lb) = self.control_mut(h) {
            lb.set_caption(&text);
        }

        let text = format!("Z Pointer | {:#06X}", self.cpu.borrow().zp());
        let h = self.zp_lb;
        if let Some(lb) = self.control_mut(h) {
            lb.set_caption(&text);
        }

        let text = format!("Cycle Counter | {:06}", self.cpu.borrow().cycle_cnt());
        let h = self.cc_lb;
        if let Some(lb) = self.control_mut(h) {
            lb.set_caption(&text);
        }

        EventProcessStatus::Processed
    }
}
