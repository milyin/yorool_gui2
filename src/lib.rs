use ggez::event::EventHandler;
use ggez::graphics::Rect;

pub mod button;
pub mod default_skin;
pub mod ribbon;

pub trait Layout {
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;
}

pub trait Widget: Layout + EventHandler {}
impl<W> Widget for W where W: Layout + EventHandler {}
