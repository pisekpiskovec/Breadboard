use std::{cell::RefCell, rc::Rc};

use appcui::prelude::{Label, Window};

#[Window]
pub struct RegisterWindow {}

impl RegisterWindow {
    pub fn new(cpu: Rc<RefCell<crate::memory::ATmemory>>) -> Self {
        let mut win = Self {
            base: window!("'Registers',a:l,w:31,h:34"),
        };

        let cpu_ref = cpu.borrow();

        for reg in 0..32 {
            let displayed_part = format!("R{:02} = {:#010b} / {:03} / {:#04X}", reg, cpu_ref.memory()[reg], cpu_ref.memory()[reg], cpu_ref.memory()[reg]);
            win.add(Label::new(
                &displayed_part,
                LayoutBuilder::new().x(0).y(reg as u8).width(32).build(),
            ));
        }
        
        win
    }
}
