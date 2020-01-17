use crate::Layout;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Align, DrawMode, DrawParam, MeshBuilder, Rect, Text};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

#[derive(Clone)]
pub enum Mode {
    Button,
    Checkbox(bool),
    Radio(bool),
    //Checkbox3State(), - or even N-state, may be added later
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Checkbox(false)
    }
}

#[derive(Clone, Default)]
pub struct State {
    mode: Mode,
    touched: bool,
    label: String,
    rect: Rect,
}

pub trait Skin: EventHandler + Default {
    fn set_state(&mut self, state: &State);
    fn is_hot_area(&self, x: f32, y: f32) -> bool;
}

#[derive(Default)]
pub struct DefaultSkin {
    state: State,
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

impl EventHandler for DefaultSkin {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &self.state.mode {
            Mode::Button => {
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
            Mode::Checkbox(checked) => {
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

impl Skin for DefaultSkin {
    fn set_state(&mut self, state: &State) {
        self.state = state.clone();
    }
    fn is_hot_area(&self, x: f32, y: f32) -> bool {
        let rect = base_rect(self.state.rect);
        rect.contains(Point2::new(x, y))
    }
}

pub struct Button<S: Skin> {
    state: State,
    skin: S,
}

impl<S: Skin> Button<S> {
    pub fn new() -> Self {
        Self {
            state: State::default(),
            skin: S::default(),
        }
    }
}

impl<S: Skin> Layout for Button<S> {
    fn set_rect(&mut self, rect: Rect) {
        self.state.rect = rect;
    }
    fn get_rect(&self) -> Rect {
        self.state.rect
    }
}

impl<S: Skin> EventHandler for Button<S> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
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
                    Mode::Button => {}
                    Mode::Checkbox(check) => self.state.mode = Mode::Checkbox(!*check),
                    _ => panic!(),
                }
            }
        }
    }
}
