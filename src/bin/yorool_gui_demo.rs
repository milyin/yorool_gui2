use async_std::task;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use yorool_gui2::button::{Button, ButtonBuilder, ButtonMode, DefaultButtonSkin};
use yorool_gui2::ribbon::{RibbonBuilder, RibbonOrientation};
use yorool_gui2::Widget;

struct GuiDemoState<'a> {
    root: Box<dyn Widget + 'a>,
}

impl<'a> GuiDemoState<'a> {
    fn new() -> Self {
        let checkbox1 = ButtonBuilder::<DefaultButtonSkin>::new()
            .mode(ButtonMode::Checkbox(false))
            .build();
        let checkbox2 = ButtonBuilder::<DefaultButtonSkin>::new()
            .mode(ButtonMode::Checkbox(true))
            .build();
        let pcheckbox1 = checkbox1.id();
        let pcheckbox2 = checkbox2.id();
        let button = ButtonBuilder::<DefaultButtonSkin>::new()
            .mode(ButtonMode::Button)
            .on_click({
                //    let checkbox1 = checkbox1.id();
                //    let checkbox2 = checkbox2.id();
                |_| {
                    task::spawn(async move {
                        pcheckbox1.set_mode(ButtonMode::Checkbox(false)).await;
                        pcheckbox2.set_mode(ButtonMode::Checkbox(false)).await;
                    });
                }
            })
            .build();
        let checkboxes = RibbonBuilder::new()
            .orientation(RibbonOrientation::Horizontal)
            .add_widget(checkbox1)
            .add_widget(checkbox2)
            .build();
        let root = RibbonBuilder::new()
            .orientation(RibbonOrientation::Vertical)
            .add_widget(checkboxes)
            .add_widget(button)
            .build();
        Self {
            root: Box::new(root),
        }
    }
}

impl<'a> EventHandler for GuiDemoState<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.root.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0., 0., 0., 0.));
        self.root.draw(ctx)?;
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.root.mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.root.mouse_button_up_event(ctx, button, x, y)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0., 0., width, height);
        self.root.set_rect(new_rect.clone());
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cb = ContextBuilder::new("yorool_gui_demo", "milyin")
        .window_setup(WindowSetup::default().title("Yorool GUI demo"))
        .window_mode(WindowMode::default().resizable(true));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GuiDemoState::new();
    event::run(ctx, event_loop, state)?;
    Ok(())
}
