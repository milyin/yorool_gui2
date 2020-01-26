use crate::button::ButtonId;
use std::sync::{Arc, Mutex};

pub struct RadioGroupState {
    buttons: Vec<ButtonId>,
}

struct RadioGroup {
    state: RadioGroupState,
}

impl RadioGroup {
    fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }
    fn add_button(&mut self, button: ButtonId) {
        self.buttons.push(button)
    }
    fn remove_button(&mut self, button: ButtonId) {
        self.buttons.retain(|id| id != button);
    }
}
