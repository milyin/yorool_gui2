use crate::Layout;
use async_call::{register_service, send_request, serve_requests, ServiceRegistration, SrvId};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub enum ButtonMode {
    Button,
    Checkbox(bool),
    Radio(bool),
    //Checkbox3State(), - or even N-state, may be added later
}

impl Default for ButtonMode {
    fn default() -> Self {
        ButtonMode::Button
    }
}

#[derive(Clone, Default)]
pub struct ButtonState {
    pub mode: ButtonMode,
    pub touched: bool,
    pub label: String,
    pub rect: Rect,
}

pub trait ButtonSkin: EventHandler + Default {
    fn set_state(&mut self, state: &ButtonState);
    fn is_hot_area(&self, x: f32, y: f32) -> bool;
}

#[derive(Copy, Clone)]
pub struct ButtonId(SrvId);

#[derive(Copy, Clone, Debug)]
enum ButtonOp {
    GetMode,
    SetMode(ButtonMode),
}

impl ButtonId {
    pub async fn get_mode(self) -> ButtonMode {
        send_request(self.0, ButtonOp::GetMode).await.unwrap()
    }
    pub async fn set_mode(self, mode: ButtonMode) {
        send_request(self.0, ButtonOp::SetMode(mode)).await.unwrap()
    }
}

pub struct Button<'a, S: ButtonSkin> {
    state: ButtonState,
    skin: S,
    reg: ServiceRegistration,
    on_click_handlers: Vec<Rc<dyn Fn(&mut Self) + 'a>>,
}

impl<'a, S: ButtonSkin> Button<'a, S> {
    pub fn new() -> Self {
        Self {
            state: ButtonState::default(),
            skin: S::default(),
            reg: register_service(),
            on_click_handlers: Vec::new(),
        }
    }
    pub fn id(&self) -> ButtonId {
        ButtonId(self.reg.id())
    }
    pub fn mode(&mut self, mode: ButtonMode) {
        self.state.mode = mode
    }
    pub fn on_click_rc(&mut self, handler: Rc<dyn Fn(&mut Self) + 'a>) {
        self.on_click_handlers.push(handler)
    }
    pub fn on_click<F: Fn(&mut Button<'a, S>) + 'a>(&mut self, f: F) {
        self.on_click_rc(Rc::new(f))
    }
}

impl<'a, S: ButtonSkin> Layout for Button<'a, S> {
    fn set_rect(&mut self, rect: Rect) {
        self.state.rect = rect;
    }
    fn get_rect(&self) -> Rect {
        self.state.rect
    }
}

impl<'a, S: ButtonSkin> EventHandler for Button<'a, S> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        serve_requests(self.reg.id(), |req| match req {
            ButtonOp::GetMode => Some(Box::new(self.state.mode)),
            ButtonOp::SetMode(mode) => {
                self.state.mode = mode;
                Some(Box::new(()))
            }
        });
        self.skin.set_state(&self.state);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.skin.draw(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if _button == MouseButton::Left {
            if self.skin.is_hot_area(x, y) {
                self.state.touched = true;
            }
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            if self.state.touched {
                self.state.touched = false;
                if self.skin.is_hot_area(x, y) {
                    match &self.state.mode {
                        ButtonMode::Button => {}
                        ButtonMode::Checkbox(check) => {
                            self.state.mode = ButtonMode::Checkbox(!*check)
                        }
                        _ => panic!(),
                    }
                    for handler in self.on_click_handlers.clone() {
                        handler(self)
                    }
                }
            }
        }
    }
}

pub struct ButtonBuilder<'a, S: ButtonSkin> {
    button: Button<'a, S>,
}

impl<'a, S: ButtonSkin> ButtonBuilder<'a, S> {
    pub fn new() -> Self {
        Self {
            button: Button::new(),
        }
    }
    pub fn mode(mut self, mode: ButtonMode) -> Self {
        self.button.mode(mode);
        self
    }
    pub fn on_click_rc(mut self, handler: Rc<dyn Fn(&mut Button<'a, S>) + 'a>) -> Self {
        self.button.on_click_rc(handler);
        self
    }
    pub fn on_click<F: Fn(&mut Button<'a, S>) + 'a>(mut self, f: F) -> Self {
        self.button.on_click(f);
        self
    }
    pub fn build(self) -> Button<'a, S> {
        self.button
    }
}
