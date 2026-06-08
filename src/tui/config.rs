use std::{cell::RefCell, rc::Rc};

use appcui::prelude::{ButtonEvents, ModalWindow};

#[ModalWindow(events=[WindowEvents, ButtonEvents])]
pub struct ConfigDialog {
    config: Rc<RefCell<crate::config::Config>>,
    close_btn: Handle<Button>,
    save_btn: Handle<Button>,
    memory_bytes_per_row: Handle<NumericSelector<usize>>,
    memory_bytes_per_column: Handle<NumericSelector<usize>>,
}

impl ConfigDialog {
    pub fn new(config: Rc<RefCell<crate::config::Config>>) -> Self {
        let mut win = Self {
            base: ModalWindow::new(
                "Config",
                layout!("x:0,y:1,w:32,h:32"),
                window::Flags::NoCloseButton,
            ),
            config,
            close_btn: Handle::None,
            save_btn: Handle::None,
            memory_bytes_per_row: Handle::None,
            memory_bytes_per_column: Handle::None,
        };

        // Panels
        //// Display
        let mut flash_disp_panel = Panel::new(
            "Flash Display",
            LayoutBuilder::new().dock(Dock::Top).height(6).build(),
        );
        flash_disp_panel.add(Label::new(
            "Bytes of memory per row",
            layout!("x:0,y:0,w:23"),
        ));
        let mbp_row: NumericSelector<usize> = NumericSelector::new(
            8,
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
            128,
            8,
            256,
            8,
            layout!("x:1,y:3,w:20"),
            numericselector::Flags::None,
        );
        win.memory_bytes_per_column = flash_disp_panel.add(mbp_column);

        win.add(flash_disp_panel);

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
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}
