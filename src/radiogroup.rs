use crate::button::ButtonId;
use crate::{EventHandlerProxy, Widget};
use async_call::{register_service, ServiceRegistration, SrvId};

pub struct RadioGroupState {
    buttons: Vec<ButtonId>,
}

impl RadioGroupState {
    fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }
}

struct RadioGroup {
    reg: ServiceRegistration,
    state: RadioGroupState,
}

impl RadioGroup {
    fn new() -> Self {
        Self {
            reg: register_service(),
            state: RadioGroupState::new(),
        }
    }
    fn add_radio(&mut self, button: ButtonId) {
        //        self.state.buttons.push(button)
    }
    fn remove_button(&mut self, button: ButtonId) {
        //        self.state.buttons.retain(|id| id != button);
    }
}

impl EventHandlerProxy for RadioGroup {}

impl Widget for RadioGroup {
    fn srv_id(&self) -> SrvId {
        self.reg.id()
    }
}
