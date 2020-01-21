use crate::button::ButtonId;
use std::sync::{Arc, Mutex};

struct RadioGroup {
    buttons: Vec<ButtonId>,
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
