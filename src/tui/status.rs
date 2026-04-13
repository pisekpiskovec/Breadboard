use std::{cell::RefCell, rc::Rc};

use appcui::prelude::{Label, Window};

#[Window]
pub struct StatusWindow {}

impl StatusWindow {
    pub fn new(cpu: Rc<RefCell<crate::memory::ATmemory>>) -> Self {
        let cpu_ref = cpu.borrow();
        let pc = cpu_ref.pc();
        let sp = cpu_ref.sp();
        let xp = cpu_ref.xp();
        let yp = cpu_ref.yp();
        let zp = cpu_ref.zp();
        let cycle_cnt = cpu_ref.cycle_cnt();
        let mut win = Self {
            base: window!("'Status',a:tl,w:32,h:8"),
        };
        win.add(Label::new(
            format!("Program Counter | {:#08X}", pc).as_str(),
            LayoutBuilder::new().x(0).y(0).width(32).build(),
        ));
        win.add(Label::new(
            format!("Stack Pointer | {:#06X}", sp).as_str(),
            LayoutBuilder::new().x(0).y(1).width(32).build(),
        ));
        win.add(Label::new(
            format!("X Pointer | {:#06X}", xp).as_str(),
            LayoutBuilder::new().x(0).y(2).width(32).build(),
        ));
        win.add(Label::new(
            format!("Y Pointer | {:#06X}", yp).as_str(),
            LayoutBuilder::new().x(0).y(3).width(32).build(),
        ));
        win.add(Label::new(
            format!("Z Pointer | {:#06X}", zp).as_str(),
            LayoutBuilder::new().x(0).y(4).width(32).build(),
        ));
        win.add(Label::new(
            format!("Cycle Counter | {:06}", cycle_cnt).as_str(),
            LayoutBuilder::new().x(0).y(5).width(32).build(),
        ));
        win
    }
}
