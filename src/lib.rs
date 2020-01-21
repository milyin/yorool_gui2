use async_call::SrvId;
use ggez::event::EventHandler;
use ggez::graphics::Rect;
use indexmap::map::IndexMap;
use std::fmt::{Debug, Formatter};

pub mod button;
pub mod default_skin;
//pub mod radiogroup;
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

pub(crate) fn add_to_indexmap<T>(indexmap: &mut IndexMap<usize, T>, value: T) -> usize {
    let key = if indexmap.is_empty() {
        0 as usize
    } else {
        indexmap.get_index(indexmap.len() - 1).unwrap().0 + 1
    };
    indexmap.insert(key, value);
    key
}
