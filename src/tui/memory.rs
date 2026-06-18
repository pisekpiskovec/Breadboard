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
            base: window!("'Internal Memory',a:bl,w:28,h:36"),
            config,
            cpu,
            list: Handle::None,
            g_reg: listview::Group::None,
            g_stc: listview::Group::None,
        };

        let mut list = listview!(
            "MemoryItem,d:f,view:Details,flags:ScrollBars+NoSelection+ShowGroups+SearchBar"
        );

        // Registers
        let g_reg = list.add_group("Registers");
        let mut registers = Vec::new();
        for reg in 0..32 {
            registers.push(MemoryItem {
                address: format!("R{:02}", reg),
                value: "000".to_string(),
            });
        }
        list.add_to_group(registers, g_reg);

        // Stack
        let g_stc = list.add_group("Stack");
        let mut stack = Vec::new();
        for addr in (0x0060..0x0460).rev() {
            stack.push(MemoryItem {
                address: format!("{:#05X}", addr),
                value: "0x00".to_string(),
            });
        }
        list.add_to_group(stack, g_stc);

        win.list = win.add(list);

        if let Some(timer) = win.timer() {
            timer.start(Duration::from_millis(100));
        }

        win
    }

    pub fn update(&mut self) {
        let registers = *self.cpu.borrow().memory();

        let mut reg_values = Vec::new();
        for reg in 0..32 {
            reg_values.push(self.format_registers(registers[reg]));
        }

        let mut stack_values = Vec::new();
        for addr in (0x0060..0x0460).rev() {
            stack_values.push(self.format_stack(registers[addr]));
        }

        let handle = self.list;
        if let Some(list) = self.control_mut(handle) {
            for (reg, value) in reg_values.into_iter().enumerate() {
                if let Some(item) = list.item_mut(reg) {
                    item.value = value;
                }
            }

            for (idx, value) in stack_values.into_iter().enumerate() {
                if let Some(item) = list.item_mut(32 + idx) {
                    item.value = value;
                }
            }
        }
    }

    fn format_registers(&self, value: u8) -> String {
        match self.config.borrow().display_base.registers {
            crate::config::DisplayBase::Binary => format!("{:#010b}", value),
            crate::config::DisplayBase::Decimal => format!("{:03}", value),
            crate::config::DisplayBase::Hexadecimal => format!("{:#04X}", value),
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

impl TimerEvents for MemoryWindow {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        self.update();
        EventProcessStatus::Processed
    }
}

#[derive(Clone, ListItem)]
struct MemoryItem {
    #[Column(name: "&Address", width: 12, align: Left)]
    address: String,

    #[Column(name: "&Value", width: 12, align: Right)]
    value: String,
}
