use std::{cell::RefCell, rc::Rc};

use appcui::prelude::{ButtonEvents, ModalWindow};

use crate::config::{Config, DisplayBase, DisplayBaseConfig};

#[ModalWindow(events=[WindowEvents, ButtonEvents], response=Config)]
pub struct ConfigDialog {
    config: Rc<RefCell<crate::config::Config>>,
    close_btn: Handle<Button>,
    save_btn: Handle<Button>,
    memory_bytes_per_row: Handle<NumericSelector<usize>>,
    memory_bytes_per_column: Handle<NumericSelector<usize>>,
    registers_base: Handle<DropDownList<DisplayBase>>,
    stack_base: Handle<DropDownList<DisplayBase>>,
    bridge_adderss: Handle<TextField>,
}

impl ConfigDialog {
    pub fn new(config: Rc<RefCell<crate::config::Config>>) -> Self {
        let mut win = Self {
            base: ModalWindow::new(
                "Config",
                layout!("x:0,y:1,w:32,h:32"),
                window::Flags::NoCloseButton,
            ),
            config: config.clone(),
            close_btn: Handle::None,
            save_btn: Handle::None,
            memory_bytes_per_row: Handle::None,
            memory_bytes_per_column: Handle::None,
            registers_base: Handle::None,
            stack_base: Handle::None,
            bridge_adderss: Handle::None,
        };

        let cfg = config.borrow();

        // Panels
        //// Display
        let mut flash_disp_panel = Panel::new(
            "Flash Display",
            LayoutBuilder::new().width(1.0).height(6).x(0).y(0).build(),
        );
        flash_disp_panel.add(Label::new(
            "Bytes of memory per row",
            layout!("x:0,y:0,w:23"),
        ));
        let mbp_row: NumericSelector<usize> = NumericSelector::new(
            cfg.display.memory_bytes_per_row,
            1,
            16,
            1,
            layout!("x:1,y:1,w:20"),
            numericselector::Flags::None,
        );
        win.memory_bytes_per_row = flash_disp_panel.add(mbp_row);

        flash_disp_panel.add(Label::new(
            "Bytes of memory per column",
            layout!("x:0,y:2,w:26"),
        ));
        let mbp_column: NumericSelector<usize> = NumericSelector::new(
            cfg.display.memory_bytes_per_column,
            8,
            256,
            8,
            layout!("x:1,y:3,w:20"),
            numericselector::Flags::None,
        );
        win.memory_bytes_per_column = flash_disp_panel.add(mbp_column);
        win.add(flash_disp_panel);

        //// Numeric bases
        let mut mem_disp_panel = Panel::new(
            "Memory Display",
            LayoutBuilder::new().width(1.0).height(6).x(0).y(6).build(),
        );
        mem_disp_panel.add(Label::new("Display registers in", layout!("x:0,y:0,w:20")));
        let mut db_reg: DropDownList<DisplayBase> =
            DropDownList::new(layout!("x:1,y:1,w:20"), dropdownlist::Flags::None);
        for db in DisplayBase::ALL.to_vec().iter() {
            db_reg.add(*db);
        }
        db_reg.set_index(
            DisplayBase::ALL
                .iter()
                .position(|&r| r == cfg.display_base.registers)
                .unwrap() as u32,
        );
        win.registers_base = mem_disp_panel.add(db_reg);

        mem_disp_panel.add(Label::new("Display stack in", layout!("x:0,y:2,w:20")));
        let mut db_stc: DropDownList<DisplayBase> =
            DropDownList::new(layout!("x:1,y:3,w:20"), dropdownlist::Flags::None);
        for db in DisplayBase::ALL.to_vec().iter() {
            db_stc.add(*db);
        }
        db_stc.set_index(
            DisplayBase::ALL
                .iter()
                .position(|&r| r == cfg.display_base.stack)
                .unwrap() as u32,
        );
        win.stack_base = mem_disp_panel.add(db_stc);
        win.add(mem_disp_panel);

        //// Bridge address
        let mut pinout_panel = Panel::new(
            "Pinout",
            LayoutBuilder::new().width(1.0).height(4).x(0).y(12).build(),
        );
        pinout_panel.add(Label::new("Bridge address", layout!("x:0,y:0,w:14")));
        win.bridge_adderss = pinout_panel.add(TextField::new(
            &cfg.bridge_address,
            layout!("x:1,y:1,w:20"),
            textfield::Flags::DisableAutoSelectOnFocus,
        ));
        win.add(pinout_panel);

        // Buttons
        win.save_btn = win.add(button!("&Save,a:bl,w:50%"));
        win.close_btn = win.add(button!("&Close,a:br,w:50%"));
        win
    }
}

impl WindowEvents for ConfigDialog {
    fn on_cancel(&mut self) -> ActionRequest {
        ActionRequest::Deny
    }
}

impl ButtonEvents for ConfigDialog {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.close_btn {
            self.exit();
            return EventProcessStatus::Processed;
        } else if handle == self.save_btn {
            let clonned_cfg = {
                let mut new_cfg = self.config.borrow_mut();
                if let Some(value) = self.control(self.memory_bytes_per_row) {
                    new_cfg.display.memory_bytes_per_row = value.value();
                }
                if let Some(value) = self.control(self.memory_bytes_per_column) {
                    new_cfg.display.memory_bytes_per_column = value.value();
                }
                if let Some(value) = self.control(self.registers_base) {
                    new_cfg.display_base.registers = *value.selected_item().unwrap();
                }
                if let Some(value) = self.control(self.stack_base) {
                    new_cfg.display_base.stack = *value.selected_item().unwrap();
                }
                if let Some(value) = self.control(self.stack_base) {
                    new_cfg.display_base.stack = *value.selected_item().unwrap();
                }
                if let Some(value) = self.control(self.bridge_adderss) {
                    new_cfg.bridge_address = value.text().to_string();
                }
                new_cfg.clone()
            };
            self.exit_with(clonned_cfg.clone());
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}

impl DropDownListType for DisplayBase {
    fn name(&self) -> &str {
        match self {
            DisplayBase::Binary => "Binary",
            DisplayBase::Decimal => "Decimal",
            DisplayBase::Hexadecimal => "Hexadecimal",
        }
    }
    fn description(&self) -> &str {
        match self {
            DisplayBase::Binary => "0b00000000",
            DisplayBase::Decimal => "000",
            DisplayBase::Hexadecimal => "0x00",
        }
    }
    fn symbol(&self) -> &str {
        match self {
            DisplayBase::Binary => "%",
            DisplayBase::Decimal => "",
            DisplayBase::Hexadecimal => "#",
        }
    }
}
