use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

use appcui::prelude::Window;
use appcui::ui::hline::Flags;

#[Window(events = TimerEvents)]
pub struct PortsWindow {
    cpu: Rc<RefCell<crate::memory::ATmemory>>,
    status_label: Handle<Label>,
    port_a_label: Handle<Label>,
    port_b_label: Handle<Label>,
    port_c_label: Handle<Label>,
    port_d_label: Handle<Label>,
}

impl PortsWindow {
    pub fn new(cpu: Rc<RefCell<crate::memory::ATmemory>>) -> Self {
        let mut win = Self {
            base: window!("'Ports',x:30,y:1,w:30,h:19,pivot:topright"),
            cpu,
            status_label: Handle::None,
            port_a_label: Handle::None,
            port_b_label: Handle::None,
            port_c_label: Handle::None,
            port_d_label: Handle::None,
        };

        // Network status
        win.status_label = win.add(Label::new(
            "Network status: Disconnected",
            LayoutBuilder::new().x(0).y(0).width(28).build(),
        ));

        win.add(HLine::new(
            "",
            LayoutBuilder::new().x(0).y(1).width(28).build(),
            Flags::None,
        ));

        // Port A
        win.port_a_label = win.add(Label::new(
            "PortA | ░░░░░░░░\n DDRA | ░░░░░░░░\n PinA | ░░░░░░░░",
            LayoutBuilder::new().x(0).y(2).width(28).height(3).build(),
        ));
        win.add(HLine::new(
            "",
            LayoutBuilder::new().x(0).y(5).width(28).build(),
            Flags::None,
        ));

        // Port B
        win.port_b_label = win.add(Label::new(
            "PortB | ░░░░░░░░\n DDRB | ░░░░░░░░\n PinB | ░░░░░░░░",
            LayoutBuilder::new().x(0).y(6).width(28).height(3).build(),
        ));
        win.add(HLine::new(
            "",
            LayoutBuilder::new().x(0).y(9).width(28).build(),
            Flags::None,
        ));

        // Port C
        win.port_c_label = win.add(Label::new(
            "PortC | ░░░░░░░░\n DDRC | ░░░░░░░░\n PinC | ░░░░░░░░",
            LayoutBuilder::new().x(0).y(10).width(28).height(3).build(),
        ));
        win.add(HLine::new(
            "",
            LayoutBuilder::new().x(0).y(13).width(28).build(),
            Flags::None,
        ));

        // Port D
        win.port_d_label = win.add(Label::new(
            "PortD | ░░░░░░░░\n DDRD | ░░░░░░░░\n PinD | ░░░░░░░░",
            LayoutBuilder::new().x(0).y(14).width(28).height(3).build(),
        ));
        win.add(HLine::new(
            "",
            LayoutBuilder::new().x(0).y(17).width(28).build(),
            Flags::None,
        ));

        if let Some(timer) = win.timer() {
            timer.start(Duration::from_millis(500));
        }

        win
    }

    fn format_bits(value: u8) -> String {
        let mut bits = String::new();

        for idx in 0..8 {
            if (value << idx) & 0x80 == 128 {
                bits.push('█');
            } else {
                bits.push('░');
            }
        }

        bits
    }

    fn update_port(
        &mut self,
        handle: Handle<Label>,
        port_name: &str,
        port_addr: u16,
        ddr_addr: u16,
        pin_addr: u16,
    ) {
        let cpu = self.cpu.borrow();
        let port_val = cpu.memory()[port_addr as usize];
        let ddr_val = cpu.memory()[ddr_addr as usize];
        let pin_val = cpu.memory()[pin_addr as usize];

        let port_bits = Self::format_bits(port_val);
        let ddr_bits = Self::format_bits(ddr_val);
        let pin_bits = Self::format_bits(pin_val);

        let text = format!(
            "Port{0} | {1}\n DDR{0} | {2}\n Pin{0} | {3}",
            port_name, port_bits, ddr_bits, pin_bits,
        );
        drop(cpu);

        if let Some(lb) = self.control_mut(handle) {
            lb.set_caption(&text);
        }
    }
}

impl TimerEvents for PortsWindow {
    fn on_start(&mut self) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    fn on_resume(&mut self, _ticks: u64) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    fn on_pause(&mut self, _ticks: u64) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        // Network status
        let status_text = format!(
            "Network status: {}",
            match self.cpu.borrow().is_bridge_connected() {
                true => "Connected",
                false => "Disconnected",
            }
        );
        let handle = self.status_label;
        if let Some(lb) = self.control_mut(handle) {
            lb.set_caption(&status_text);
        }
        drop(handle);

        self.update_port(self.port_a_label, "A", 0x3B, 0x3A, 0x39);

        self.update_port(self.port_b_label, "B", 0x38, 0x37, 0x36);

        self.update_port(self.port_c_label, "C", 0x35, 0x34, 0x33);

        self.update_port(self.port_d_label, "D", 0x32, 0x31, 0x30);

        EventProcessStatus::Processed
    }
}
