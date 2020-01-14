use crate::Widget;
use ggez::graphics::Rect;
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameResult};

pub struct Ribbon<'a> {
    widgets: Vec<Box<dyn Widget + 'a>>,
    rect: Rect,
    horizontal: bool,
}

impl EventHandler for Ribbon<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for w in &mut self.widgets {
           w.update(ctx)?
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for w in &mut self.widgets {
            w.draw(ctx)?
        }
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        for w in &mut self.widgets {
            w.mouse_button_down_event(ctx, button, x, y)
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        for w in &mut self.widgets {
            w.mouse_button_up_event(ctx, button, x, y)
        }
    }
}

