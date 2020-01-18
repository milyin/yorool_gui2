use crate::ribbon::RibbonOrientation::Horizontal;
use crate::{Layout, Widget};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::Rect;
use ggez::{Context, GameResult};

pub enum RibbonOrientation {
    Horizontal,
    Vertical,
}

impl Default for RibbonOrientation {
    fn default() -> Self {
        Horizontal
    }
}

pub struct Ribbon<'a> {
    widgets: Vec<Box<dyn Widget + 'a>>,
    rect: Rect,
    orientation: RibbonOrientation,
}

impl Layout for Ribbon<'_> {
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
        match &self.orientation {
            RibbonOrientation::Horizontal => {
                let dw = rect.w / self.widgets.len() as f32;
                let mut x = rect.x;
                for w in &mut self.widgets {
                    w.set_rect(Rect::new(x, rect.y, dw, rect.h));
                    x += dw;
                }
            }
            RibbonOrientation::Vertical => {
                let dh = rect.h / self.widgets.len() as f32;
                let mut y = rect.y;
                for w in &mut self.widgets {
                    w.set_rect(Rect::new(rect.x, y, rect.w, dh));
                    y += dh;
                }
            }
        }
    }
    fn get_rect(&self) -> Rect {
        self.rect
    }
}

impl<'a> Ribbon<'a> {
    fn new() -> Self {
        Self {
            widgets: Vec::new(),
            rect: Rect::default(),
            orientation: RibbonOrientation::default(),
        }
    }
    fn orientation(&mut self, orientation: RibbonOrientation) {
        self.orientation = orientation
    }
    fn add_widget<W: Widget + 'a>(&mut self, widget: W) {
        self.widgets.push(Box::new(widget))
    }
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

pub struct RibbonBuilder<'a> {
    ribbon: Ribbon<'a>,
}

impl<'a> RibbonBuilder<'a> {
    pub fn new() -> Self {
        Self {
            ribbon: Ribbon::new(),
        }
    }
    pub fn orientation(mut self, orientation: RibbonOrientation) -> Self {
        self.ribbon.orientation(orientation);
        self
    }
    pub fn add_widget<W: Widget + 'a>(mut self, widget: W) -> Self {
        self.ribbon.add_widget(widget);
        self
    }
    pub fn build(self) -> Ribbon<'a> {
        self.ribbon
    }
}
