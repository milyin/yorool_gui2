use crate::button::{ButtonId, ButtonMode};
use crate::radiogroup::RadioGroupOp::{AddRadio, RemoveRadio};
use crate::{EventHandlerProxy, Widget};
use async_call::{register_service, send_request, serve_requests, ServiceRegistration, SrvId};
use async_std::task;
use ggez::{Context, GameResult};
use std::sync::{Arc, Mutex};

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

pub struct RadioGroup {
    reg: ServiceRegistration,
    state: Arc<Mutex<RadioGroupState>>,
}

#[derive(Copy, Clone)]
pub struct RadioGroupId(SrvId);

#[derive(Debug)]
enum RadioGroupOp {
    AddRadio(ButtonId),
    RemoveRadio(ButtonId),
}

impl RadioGroup {
    pub fn new() -> Self {
        Self {
            reg: register_service(),
            state: Arc::new(Mutex::new(RadioGroupState::new())),
        }
    }
    pub fn id(&self) -> RadioGroupId {
        RadioGroupId(self.reg.id())
    }
    pub fn add_radio(&mut self, button: ButtonId) {
        self.state.lock().unwrap().buttons.push(button);
        task::spawn({
            let state = self.state.clone();
            async move {
                button
                    .on_click(move |b| {
                        let current_id = b.srv_id();
                        let buttons = state.lock().unwrap().buttons.clone();
                        task::spawn(async move {
                            for button_id in buttons {
                                if current_id != button_id.into() {
                                    button_id.set_mode(ButtonMode::Radio(false)).await;
                                }
                            }
                        });
                    })
                    .await;
            }
        });
    }
    pub fn remove_radio(&mut self, button: ButtonId) {
        self.state
            .lock()
            .unwrap()
            .buttons
            .retain(|id| *id != button);
    }
}

impl RadioGroupId {
    pub async fn add_radio(self, button: ButtonId) {
        send_request(self.0, AddRadio(button)).await.unwrap()
    }
    pub async fn remove_radio(self, button: ButtonId) {
        send_request(self.0, RemoveRadio(button)).await.unwrap()
    }
}

impl EventHandlerProxy for RadioGroup {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        serve_requests(self.reg.id(), |req| match req {
            AddRadio(button) => {
                self.add_radio(button);
                Some(Box::new(()))
            }
            RemoveRadio(button) => {
                self.remove_radio(button);
                Some(Box::new(()))
            }
        });
        Ok(())
    }
}

impl Widget for RadioGroup {
    fn srv_id(&self) -> SrvId {
        self.reg.id()
    }
}

pub struct RadioGroupBuilder {
    radio_group: RadioGroup,
}

impl RadioGroupBuilder {
    pub fn new() -> Self {
        RadioGroupBuilder {
            radio_group: RadioGroup::new(),
        }
    }
    pub fn add_radio(mut self, button: ButtonId) -> Self {
        self.radio_group.add_radio(button);
        self
    }
    pub fn remove_radio(mut self, button: ButtonId) -> Self {
        self.radio_group.remove_radio(button);
        self
    }
    pub fn build(self) -> RadioGroup {
        self.radio_group
    }
}
