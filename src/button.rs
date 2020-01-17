use crate::Layout;
use async_call::{
    register_service, send_request_typed, serve_requests_typed, ServiceRegistration, SrvId,
};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Align, DrawMode, DrawParam, MeshBuilder, Rect, Text};
use ggez::nalgebra::Point2;
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
        ButtonMode::Checkbox(false)
    }
}

#[derive(Clone, Default)]
pub struct ButtonState {
    mode: ButtonMode,
    touched: bool,
    label: String,
    rect: Rect,
}

pub trait ButtonSkin: EventHandler + Default {
    fn set_state(&mut self, state: &ButtonState);
    fn is_hot_area(&self, x: f32, y: f32) -> bool;
}

#[derive(Default)]
pub struct DefaultButtonSkin {
    state: ButtonState,
}

const MARGIN: f32 = 5.;
const PRESS_OFFSET: f32 = 10.;

fn base_rect(mut rect: Rect) -> Rect {
    rect.x += MARGIN;
    rect.y += MARGIN;
    rect.w -= MARGIN * 2.;
    rect.h -= MARGIN * 2.;
    rect
}

fn button_rect(mut rect: Rect, touched: bool) -> Rect {
    let dxy = if touched { PRESS_OFFSET } else { 0. };
    rect.x += MARGIN + dxy;
    rect.y += MARGIN + dxy;
    rect.w -= MARGIN * 2. + PRESS_OFFSET;
    rect.h -= MARGIN * 2. + PRESS_OFFSET;
    rect
}

impl EventHandler for DefaultButtonSkin {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &self.state.mode {
            ButtonMode::Button => {
                let rect = button_rect(self.state.rect, self.state.touched);
                let mesh = MeshBuilder::new()
                    .rectangle(DrawMode::fill(), rect, graphics::WHITE)
                    .build(ctx)?;
                graphics::draw(ctx, &mesh, DrawParam::default())?;
                let mut text = Text::new(self.state.label.clone());
                text.set_bounds([rect.w, rect.h], Align::Center);
                let tdh = (rect.h - text.height(ctx) as f32) / 2.;
                graphics::draw(
                    ctx,
                    &text,
                    (Point2::new(rect.x, rect.y + tdh), graphics::BLACK),
                )
            }
            ButtonMode::Checkbox(checked) => {
                let rect = base_rect(self.state.rect);
                let mesh = MeshBuilder::new()
                    .rectangle(
                        if *checked {
                            DrawMode::fill()
                        } else {
                            DrawMode::stroke(1.)
                        },
                        rect,
                        graphics::WHITE,
                    )
                    .build(ctx)?;
                graphics::draw(ctx, &mesh, DrawParam::default())
            }

            _ => panic!(),
        }
    }
}

impl ButtonSkin for DefaultButtonSkin {
    fn set_state(&mut self, state: &ButtonState) {
        self.state = state.clone();
    }
    fn is_hot_area(&self, x: f32, y: f32) -> bool {
        let rect = base_rect(self.state.rect);
        rect.contains(Point2::new(x, y))
    }
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
        send_request_typed(self.0, ButtonOp::GetMode).await.unwrap()
    }
    pub async fn set_mode(self, mode: ButtonMode) {
        send_request_typed(self.0, ButtonOp::SetMode(mode))
            .await
            .unwrap()
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
    pub fn on_click(&mut self, handler: Rc<dyn Fn(&mut Self) + 'a>) {
        self.on_click_handlers.push(handler)
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
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        serve_requests_typed(self.reg.id(), |req| match req {
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

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if self.skin.is_hot_area(x, y) {
            self.state.touched = true;
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if self.state.touched {
            self.state.touched = false;
            if self.skin.is_hot_area(x, y) {
                match &self.state.mode {
                    ButtonMode::Button => {}
                    ButtonMode::Checkbox(check) => self.state.mode = ButtonMode::Checkbox(!*check),
                    _ => panic!(),
                }
                for handler in self.on_click_handlers.clone() {
                    handler(self)
                }
            }
        }
    }
}

pub struct ButtonBuilder<'a, S: ButtonSkin = DefaultButtonSkin> {
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
    pub fn on_click<F: Fn(&mut Button<'a, S>) + 'a>(mut self, f: F) -> Self {
        self.button.on_click(Rc::new(f));
        self
    }
    pub fn build(self) -> Button<'a, S> {
        self.button
    }
}
