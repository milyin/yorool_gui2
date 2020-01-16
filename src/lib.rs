use ggez::graphics::Rect;
use ggez::event::EventHandler;

pub mod button;
pub mod ribbon;

pub trait Layout {
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;
}

pub trait Widget : Layout + EventHandler {}
impl<W> Widget for W where W: Layout + EventHandler {}

