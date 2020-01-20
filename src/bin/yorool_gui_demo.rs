use async_std::task;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use yorool_gui2::button::ButtonMode;
use yorool_gui2::default_skin::{DefaultButtonBuilder, DefaultRibbonBuilder};
use yorool_gui2::ribbon::RibbonOrientation;
use yorool_gui2::Widget;

struct GuiDemoState {
    root: Box<dyn Widget>,
}

impl GuiDemoState {
    fn new() -> Self {
        let checkbox1 = DefaultButtonBuilder::new()
            .mode(ButtonMode::Checkbox(false))
            .build();
        let checkbox2 = DefaultButtonBuilder::new()
            .mode(ButtonMode::Checkbox(true))
            .build();
        let checkbox1id = checkbox1.id();
        let checkbox2id = checkbox2.id();
        let checkboxes = DefaultRibbonBuilder::new()
            .orientation(RibbonOrientation::Horizontal)
            .add_widget(checkbox1)
            .add_widget(checkbox2)
            .build();
        let checkboxesid = checkboxes.id();
        let button = DefaultButtonBuilder::new()
            .mode(ButtonMode::Button)
            .on_click({
                move |_| {
                    task::spawn(async move {
                        checkbox1id.set_mode(ButtonMode::Checkbox(false)).await;
                        checkbox2id.set_mode(ButtonMode::Checkbox(false)).await;
                        checkboxesid
                            .add_widget(
                                DefaultButtonBuilder::new()
                                    .mode(ButtonMode::Checkbox(false))
                                    .build(),
                            )
                            .await;
                    });
                }
            })
            .build();
        let root = DefaultRibbonBuilder::new()
            .orientation(RibbonOrientation::Vertical)
            .add_widget(checkboxes)
            .add_widget(button)
            .build();
        Self {
            root: Box::new(root),
        }
    }
}

impl EventHandler for GuiDemoState {
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
