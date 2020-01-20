use crate::ribbon::RibbonOp::{AddWidget, GetOrientation, RemoveWidget, SetOrientation};
use crate::ribbon::RibbonOrientation::{Horizontal, Vertical};
use crate::{Layout, Widget};
use async_call::{register_service, send_request, serve_requests, ServiceRegistration, SrvId};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use std::ops::Not;

#[derive(Copy, Clone, Debug)]
pub enum RibbonOrientation {
    Horizontal,
    Vertical,
}

impl Not for RibbonOrientation {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Horizontal => Vertical,
            Vertical => Horizontal,
        }
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
    AddWidget(Box<dyn Widget>),
    RemoveWidget(SrvId),
    SetOrientation(RibbonOrientation),
    GetOrientation,
}

impl RibbonId {
    pub async fn add_widget(self, widget: impl Widget + 'static) {
        send_request(self.0, AddWidget(Box::new(widget)))
            .await
            .unwrap()
    }
    pub async fn remove_widget(self, srv_id: SrvId) {
        send_request(self.0, RemoveWidget(srv_id)).await.unwrap()
    }
    pub async fn set_orientation(self, orientation: RibbonOrientation) {
        send_request(self.0, SetOrientation(orientation))
            .await
            .unwrap()
    }
    pub async fn get_orientation(self) -> RibbonOrientation {
        send_request(self.0, GetOrientation).await.unwrap()
    }
}

impl Layout for Ribbon {
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.update_widgets_rects();
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
            orientation: RibbonOrientation::Horizontal,
            reg: register_service(),
        }
    }
    pub fn set_orientation(&mut self, orientation: RibbonOrientation) {
        self.orientation = orientation;
        self.update_widgets_rects();
    }
    pub fn get_orientation(&self) -> RibbonOrientation {
        self.orientation
    }
    pub fn add_widget_box(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
        self.update_widgets_rects();
    }
    pub fn add_widget(&mut self, widget: impl Widget + 'static) {
        self.add_widget_box(Box::new(widget))
    }
    pub fn remove_widget(&mut self, id: SrvId) {
        self.widgets.retain(|w| w.srv_id() != id);
        self.update_widgets_rects();
    }
    pub fn id(&self) -> RibbonId {
        RibbonId(self.reg.id())
    }
    fn update_widgets_rects(&mut self) {
        let rect = self.rect;
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
}

impl Widget for Ribbon {
    fn srv_id(&self) -> SrvId {
        self.reg.id()
    }
}

impl EventHandler for Ribbon {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        serve_requests(self.reg.id(), |req| match req {
            AddWidget(w) => {
                self.add_widget_box(w);
                Some(Box::new(()))
            }
            RemoveWidget(srv_id) => {
                self.remove_widget(srv_id);
                Some(Box::new(()))
            }
            SetOrientation(orientation) => {
                self.set_orientation(orientation);
                Some(Box::new(()))
            }
            GetOrientation => Some(Box::new(self.get_orientation())),
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
    pub fn set_orientation(mut self, orientation: RibbonOrientation) -> Self {
        self.ribbon.set_orientation(orientation);
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
