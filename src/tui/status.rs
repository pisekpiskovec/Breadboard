use appcui::prelude::{Label, Window};

#[Window]
pub struct StatusWindow {
    cpu: crate::memory::ATmemory,
}

impl StatusWindow {
    pub fn new(cpu: crate::memory::ATmemory) -> Self {
        let pc = cpu.pc();
        let sp = cpu.sp();
        let xp = cpu.xp();
        let yp = cpu.yp();
        let zp = cpu.zp();
        let cycle_cnt = cpu.cycle_cnt();
        let mut win = Self {
            base: window!("'Status',a:c,w:10,h:30,flags:sizeable"),
            cpu,
        };
        win.add(Label::new(
            format!("Program Counter | {:#08X}", pc).as_str(),
            LayoutBuilder::new()
                .alignment(Alignment::CenterLeft)
                .build(),
        ));
        win.add(Label::new(
            format!("Stack Pointer | {:#06X}", sp).as_str(),
            LayoutBuilder::new()
                .alignment(Alignment::CenterLeft)
                .build(),
        ));
        win.add(Label::new(
            format!("X Pointer | {:#06X}", xp).as_str(),
            LayoutBuilder::new()
                .alignment(Alignment::CenterLeft)
                .build(),
        ));
        win.add(Label::new(
            format!("Y Pointer | {:#06X}", yp).as_str(),
            LayoutBuilder::new()
                .alignment(Alignment::CenterLeft)
                .build(),
        ));
        win.add(Label::new(
            format!("Z Pointer | {:#06X}", zp).as_str(),
            LayoutBuilder::new()
                .alignment(Alignment::CenterLeft)
                .build(),
        ));
        win.add(Label::new(
            format!("Cycle Counter | {:06}", cycle_cnt).as_str(),
            LayoutBuilder::new()
                .alignment(Alignment::CenterLeft)
                .build(),
        ));
        win
    }
}
