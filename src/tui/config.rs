use std::{cell::RefCell, rc::Rc};

use appcui::prelude::{ButtonEvents, ModalWindow};

#[ModalWindow(events=[WindowEvents, ButtonEvents])]
pub struct ConfigDialog {
    config: Rc<RefCell<crate::config::Config>>,
    close_btn: Handle<Button>,
    save_btn: Handle<Button>,
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
        };
        win.close_btn = win.add(button!("Close,d:b"));
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
