use async_call::SrvId;
use ggez::event::EventHandler;
use ggez::graphics::Rect;
use std::fmt::{Debug, Formatter};

pub mod button;
pub mod default_skin;
pub mod ribbon;

pub trait Layout {
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;
}

pub trait Widget: Layout + EventHandler + Send {
    fn srv_id(&self) -> SrvId;
}
impl Debug for dyn Widget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WIDGET {:?}", self.srv_id())
    }
}
