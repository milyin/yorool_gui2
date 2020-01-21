use crate::button::{ButtonMode, ButtonSkin, ButtonState};
use ggez::event::EventHandler;
use ggez::graphics::{self, Align, DrawMode, DrawParam, MeshBuilder, Rect, Text};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

#[derive(Default, Debug)]
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
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
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
