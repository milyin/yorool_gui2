use async_call::SrvId;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use indexmap::map::IndexMap;
use std::fmt::{Debug, Formatter};

pub mod button;
pub mod default_skin;
//pub mod radiogroup;
pub mod ribbon;

pub trait EventHandlerProxy {
    fn update(&mut self, ctx: &mut Context) -> GameResult;
    fn draw(&mut self, ctx: &mut Context) -> GameResult;
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }
}

pub trait Widget: EventHandlerProxy + Send {
    fn srv_id(&self) -> SrvId;
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;
}
impl Debug for dyn Widget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WIDGET {:?}", self.srv_id())
    }
}

impl EventHandler for dyn Widget {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_button_up_event(ctx, button, x, y)
    }
}

pub trait WidgetGroup: Send {
    fn mut_root(&mut self) -> &mut Box<dyn Widget>;
    fn root(&self) -> &Box<dyn Widget>;
}

impl<T> EventHandlerProxy for T
where
    T: WidgetGroup,
{
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.mut_root().update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.mut_root().draw(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mut_root().mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mut_root().mouse_button_up_event(ctx, button, x, y)
    }
}

impl<T> Widget for T
where
    T: WidgetGroup,
{
    fn srv_id(&self) -> SrvId {
        self.root().srv_id()
    }
    fn set_rect(&mut self, rect: Rect) {
        self.mut_root().set_rect(rect)
    }
    fn get_rect(&self) -> Rect {
        self.root().get_rect()
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
