use std::{cell::RefCell, rc::Rc, time::Duration};

use appcui::prelude::Window;

#[Window(events = [TimerEvents])]
pub struct MemoryWindow {
    config: Rc<RefCell<crate::config::Config>>,
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
    list: Handle<ListView<MemoryItem>>,
    g_reg: listview::Group,
    g_stc: listview::Group,
}

impl MemoryWindow {
    pub fn new(
        config: Rc<RefCell<crate::config::Config>>,
        cpu: Rc<RefCell<crate::memory::ATmemory>>,
    ) -> Self {
        let mut win = Self {
            base: window!("'Internal Memory',a:bl,w:31,h:36"),
            config,
            cpu,
            list: Handle::None,
            g_reg: listview::Group::None,
            g_stc: listview::Group::None,
        };

        let mut list: ListView<MemoryItem> = ListView::new(
            LayoutBuilder::new().dock(Dock::Fill).build(),
            listview::Flags::ScrollBars | listview::Flags::NoSelection | listview::Flags::ShowGroups,
        );

        win.g_reg = list.add_group("Registers");
        win.g_stc = list.add_group("Stack");

        list.add_item(listview::Item::new(MemoryItem { address: "h", value: "h" }, false, None, [' ', ' '], win.g_reg));
        
        win.list = win.add(list);

        if let Some(timer) = win.timer() {
            timer.start(Duration::from_millis(100_00));
        }

        win
    }

    pub fn update(&mut self) {
        self.update_registers();
        self.update_stack();
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
    }

    fn format_stack(&self, value: u8) -> String {
        match self.config.borrow().display_base.stack {
            crate::config::DisplayBase::Binary => format!("{:#010b}", value),
            crate::config::DisplayBase::Decimal => format!("{:03}", value),
            crate::config::DisplayBase::Hexadecimal => format!("{:#04X}", value),
        }
    }
}

impl TimerEvents for MemoryWindow {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        self.update();
        EventProcessStatus::Processed
    }
}

struct MemoryItem {
    address: &'static str,
    value: &'static str,
}

impl listview::ListItem for MemoryItem {
    fn render_method(&'_ self, column_index: u16) -> Option<RenderMethod<'_>> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.address)),
            1 => Some(listview::RenderMethod::Text(self.value)),
            _ => None,
        }
    }
}
