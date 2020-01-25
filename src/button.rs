use crate::{add_to_indexmap, EventHandlerProxy, Layout, Widget};
use async_call::{register_service, send_request, serve_requests, ServiceRegistration, SrvId};
use ggez::event::MouseButton;
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use indexmap::map::IndexMap;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

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

#[derive(Clone, Default, Debug)]
pub struct ButtonState {
    pub mode: ButtonMode,
    pub touched: bool,
    pub label: String,
    pub rect: Rect,
}

pub trait ButtonSkin: EventHandlerProxy + Default + Debug + Send {
    fn set_state(&mut self, state: &ButtonState);
    fn is_hot_area(&self, x: f32, y: f32) -> bool;
}

pub struct ButtonId<S: ButtonSkin>(SrvId, PhantomData<S>);

impl<S: ButtonSkin> Clone for ButtonId<S> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}
impl<S: ButtonSkin> Copy for ButtonId<S> {}

enum ButtonOp<S: ButtonSkin> {
    GetMode,
    SetMode(ButtonMode),
    GetLabel,
    SetLabel(String),
    OnClick(Box<dyn Fn(&mut Button<S>) + Send>),
    RemoveOnClick(usize),
}

impl<S: ButtonSkin> Debug for ButtonOp<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonOp::GetMode => write!(f, "GetMode"),
            ButtonOp::SetMode(mode) => write!(f, "SetMode({:?}", mode),
            ButtonOp::GetLabel => write!(f, "GetLabel"),
            ButtonOp::SetLabel(label) => write!(f, "SetLabel({:?})", label),
            ButtonOp::OnClick(_) => write!(f, "OnClick"),
            ButtonOp::RemoveOnClick(handler_id) => write!(f, "RemoveOnClick({:?})", handler_id),
        }
    }
}

impl<S: ButtonSkin + 'static> ButtonId<S> {
    pub async fn get_mode(self) -> ButtonMode {
        send_request(self.0, ButtonOp::<S>::GetMode).await.unwrap()
    }
    pub async fn set_mode(self, mode: ButtonMode) {
        send_request(self.0, ButtonOp::<S>::SetMode(mode))
            .await
            .unwrap()
    }
    pub async fn get_label(self) -> String {
        send_request(self.0, ButtonOp::<S>::GetLabel).await.unwrap()
    }
    pub async fn set_label(self, label: String) {
        send_request(self.0, ButtonOp::<S>::SetLabel(label))
            .await
            .unwrap()
    }
    pub async fn on_click<F: Fn(&mut Button<S>) + Send + 'static>(self, f: F) -> usize {
        send_request(self.0, ButtonOp::<S>::OnClick(Box::new(f)))
            .await
            .unwrap()
    }
    pub async fn remove_on_click(self, handler_id: usize) {
        send_request(self.0, ButtonOp::<S>::RemoveOnClick(handler_id))
            .await
            .unwrap()
    }
}

pub struct Button<S: ButtonSkin> {
    state: ButtonState,
    skin: S,
    reg: ServiceRegistration,
    on_click_handlers: IndexMap<usize, Box<dyn Fn(&mut Self) + Send>>,
}

impl<S: ButtonSkin> Button<S> {
    pub fn new() -> Self {
        Self {
            state: ButtonState::default(),
            skin: S::default(),
            reg: register_service(),
            on_click_handlers: IndexMap::new(),
        }
    }
    pub fn id(&self) -> ButtonId<S> {
        ButtonId(self.reg.id(), PhantomData)
    }
    pub fn set_mode(&mut self, mode: ButtonMode) {
        self.state.mode = mode
    }
    pub fn get_mode(&mut self) -> ButtonMode {
        self.state.mode
    }
    pub fn set_label(&mut self, label: String) {
        self.state.label = label
    }
    pub fn get_label(&self) -> &str {
        self.state.label.as_str()
    }
    pub fn on_click_box(&mut self, handler: Box<dyn Fn(&mut Self) + Send>) -> usize {
        add_to_indexmap(&mut self.on_click_handlers, handler)
    }
    pub fn on_click<F: Fn(&mut Button<S>) + Send + 'static>(&mut self, f: F) -> usize {
        self.on_click_box(Box::new(f))
    }
    pub fn remove_on_click(&mut self, handler_id: usize) {
        self.on_click_handlers.remove(&handler_id);
    }
}

impl<S: ButtonSkin + 'static> Widget for Button<S> {
    fn srv_id(&self) -> SrvId {
        self.reg.id()
    }
}

impl<S: ButtonSkin> Layout for Button<S> {
    fn set_rect(&mut self, rect: Rect) {
        self.state.rect = rect;
    }
    fn get_rect(&self) -> Rect {
        self.state.rect
    }
}

impl<S: ButtonSkin + 'static> EventHandlerProxy for Button<S> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        serve_requests(self.reg.id(), |req| match req {
            ButtonOp::<S>::GetMode => Some(Box::new(self.get_mode())),
            ButtonOp::<S>::SetMode(mode) => {
                self.set_mode(mode);
                Some(Box::new(()))
            }
            ButtonOp::<S>::GetLabel => Some(Box::new(self.get_label().to_string())),
            ButtonOp::<S>::SetLabel(label) => {
                self.set_label(label);
                Some(Box::new(()))
            }
            ButtonOp::<S>::OnClick(handler) => {
                let handler_id = self.on_click_box(handler);
                Some(Box::new(handler_id))
            }
            ButtonOp::<S>::RemoveOnClick(handler_id) => {
                self.remove_on_click(handler_id);
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
                    let handlers = std::mem::replace(&mut self.on_click_handlers, IndexMap::new());
                    for (_, handler) in &handlers {
                        handler(self);
                    }
                    self.on_click_handlers = handlers;
                }
            }
        }
    }
}

pub struct ButtonBuilder<S: ButtonSkin> {
    button: Button<S>,
}

impl<S: ButtonSkin> ButtonBuilder<S> {
    pub fn new() -> Self {
        Self {
            button: Button::new(),
        }
    }
    pub fn set_mode(mut self, mode: ButtonMode) -> Self {
        self.button.set_mode(mode);
        self
    }
    pub fn set_label<T: Into<String>>(mut self, label: T) -> Self {
        self.button.set_label(label.into());
        self
    }
    pub fn on_click<F: Fn(&mut Button<S>) + Send + 'static>(mut self, f: F) -> Self {
        self.button.on_click(f);
        self
    }
    pub fn build(self) -> Button<S> {
        self.button
    }
}
