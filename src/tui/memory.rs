use std::{cell::RefCell, rc::Rc, time::Duration};

use appcui::prelude::Window;

#[Window(events = [AccordionEvents, TimerEvents])]
pub struct MemoryWindow {
    config: Rc<RefCell<crate::config::Config>>,
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
    panels: Handle<Accordion>,
    ta_reg: Handle<TextArea>,
    ta_stc: Handle<TextArea>,
}

const REGISTER_PANEL_ID: usize = 0;
const STACK_PANEL_ID: usize = 1;

impl MemoryWindow {
    pub fn new(
        config: Rc<RefCell<crate::config::Config>>,
        cpu: Rc<RefCell<crate::memory::ATmemory>>,
    ) -> Self {
        let mut win = Self {
            base: window!("'Internal Memory',a:bl,w:31,h:34"),
            config,
            cpu,
            panels: Handle::None,
            ta_reg: Handle::None,
            ta_stc: Handle::None,
        };

        let mut accordion = Accordion::new(
            LayoutBuilder::new().dock(Dock::Fill).build(),
            accordion::Flags::TransparentBackground,
        );

        accordion.add_panel("&Registers");
        let ta_reg = TextArea::new(
            "R00 = 000",
            LayoutBuilder::new().dock(Dock::Fill).build(),
            textarea::Flags::ReadOnly | textarea::Flags::ScrollBars,
        );
        win.ta_reg = accordion.add(0, ta_reg);

        accordion.add_panel("&Stack");
        let ta_stc = TextArea::new(
            "0x45F = 0x00",
            LayoutBuilder::new().dock(Dock::Fill).build(),
            textarea::Flags::ReadOnly | textarea::Flags::ScrollBars,
        );
        win.ta_stc = accordion.add(1, ta_stc);

        win.panels = win.add(accordion);

        if let Some(timer) = win.timer() {
            timer.start(Duration::from_millis(100_00));
        }

        win
    }

    pub fn update(&mut self) {
        let handle = self.panels;
        if let Some(pa) = self.control_mut(handle) {
            match pa.current_panel() {
                Some(REGISTER_PANEL_ID) => self.update_registers(),
                Some(STACK_PANEL_ID) => self.update_stack(),
                _ => {}
            }
        }
    }

    fn update_registers(&mut self) {
        let registers = *self.cpu.borrow().memory();
        let mut text = String::new();
        for reg in 0..32 {
            text.push_str(
                format!("R{:02} = {}\n", reg, self.format_registers(registers[reg])).as_str(),
            );
        }
        text.push('\u{0008}');
        let handle = self.ta_reg;
        if let Some(ta) = self.control_mut(handle) {
            ta.set_text(&text);
        }
    }

    fn format_registers(&self, value: u8) -> String {
        match self.config.borrow().display_base.registers {
            crate::config::DisplayBase::Binary => format!("{:#010b}", value),
            crate::config::DisplayBase::Decimal => format!("{:03}", value),
            crate::config::DisplayBase::Hexadecimal => format!("{:#04X}", value),
        }
    }

    fn update_stack(&mut self) {
        let stack = *self.cpu.borrow().memory();
        let mut text = String::new();

        for sp in (0x0060..0x0460).rev() {
            text.push_str(format!("{:#05X} = {}\n", sp, self.format_stack(stack[sp])).as_str());
        }

        text.push('\u{0008}');
        let handle = self.ta_stc;
        if let Some(ta) = self.control_mut(handle) {
            ta.set_text(&text);
        }
    }

    fn format_stack(&self, value: u8) -> String {
        match self.config.borrow().display_base.stack {
            crate::config::DisplayBase::Binary => format!("{:#010b}", value),
            crate::config::DisplayBase::Decimal => format!("{:03}", value),
            crate::config::DisplayBase::Hexadecimal => format!("{:#04X}", value),
        }
    }
}

impl AccordionEvents for MemoryWindow {
    fn on_panel_changed(&mut self, _handle: Handle<Accordion>, _new_panel_index: u32, _old_panel_index: u32) -> EventProcessStatus {
        self.update();
        EventProcessStatus::Processed
    }
}

impl TimerEvents for MemoryWindow {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        self.update();
        EventProcessStatus::Processed
    }
}
