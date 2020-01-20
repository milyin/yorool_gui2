use crate::ribbon::RibbonOp::AddWidet;
use crate::ribbon::RibbonOrientation::Horizontal;
use crate::{Layout, Widget};
use async_call::{register_service, send_request, serve_requests, ServiceRegistration, SrvId};
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

pub struct Ribbon {
    widgets: Vec<Box<dyn Widget>>,
    rect: Rect,
    orientation: RibbonOrientation,
    reg: ServiceRegistration,
}

#[derive(Copy, Clone)]
pub struct RibbonId(SrvId);

#[derive(Debug)]
pub enum RibbonOp {
    None,
    AddWidet(Box<dyn Widget>),
}

impl Default for RibbonOp {
    fn default() -> Self {
        RibbonOp::None
    }
}

impl RibbonId {
    pub async fn add_widget(self, widget: impl Widget + 'static) {
        send_request(self.0, AddWidet(Box::new(widget)))
            .await
            .unwrap()
    }
}

impl Layout for Ribbon {
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

impl Ribbon {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            rect: Rect::default(),
            orientation: RibbonOrientation::default(),
            reg: register_service(),
        }
    }
    pub fn orientation(&mut self, orientation: RibbonOrientation) {
        self.orientation = orientation
    }
    pub fn add_widget_box(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget)
    }
    pub fn add_widget(&mut self, widget: impl Widget + 'static) {
        self.add_widget_box(Box::new(widget))
    }
}

impl EventHandler for Ribbon {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        serve_requests(self.reg.id(), |req| match req {
            RibbonOp::AddWidet(w) => {
                self.add_widget_box(w);
                Some(Box::new(()))
            }
            RibbonOp::None => panic!("RibbonOp::None request not expected"),
        });

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

pub struct RibbonBuilder {
    ribbon: Ribbon,
}

impl RibbonBuilder {
    pub fn new() -> Self {
        Self {
            ribbon: Ribbon::new(),
        }
    }
    pub fn orientation(mut self, orientation: RibbonOrientation) -> Self {
        self.ribbon.orientation(orientation);
        self
    }
    pub fn add_widget(mut self, widget: impl Widget + 'static) -> Self {
        self.ribbon.add_widget(widget);
        self
    }
    pub fn build(self) -> Ribbon {
        self.ribbon
    }
}
