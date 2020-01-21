use async_std::task;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use yorool_gui2::button::ButtonMode::{Button, Checkbox};
use yorool_gui2::default_skin::{DefaultButtonBuilder, DefaultRibbonBuilder};
use yorool_gui2::ribbon::RibbonOrientation::{Horizontal, Vertical};
use yorool_gui2::Widget;

struct GuiDemoState {
    root: Box<dyn Widget>,
}

impl GuiDemoState {
    fn button_panel() -> impl Widget {
        let add_button = DefaultButtonBuilder::new()
            .set_label("Add")
            .set_mode(Button)
            .build();
        let remove_button = DefaultButtonBuilder::new()
            .set_label("Remove")
            .set_mode(Button)
            .build();
        DefaultRibbonBuilder::new()
            .set_orientation(Horizontal)
            .add_widget(add_button)
            .add_widget(remove_button)
            .build()
    }

    fn radio_panel() -> impl Widget {
        DefaultRibbonBuilder::new()
            .set_orientation(Horizontal)
            .build()
    }

    fn panel() -> impl Widget {
        DefaultRibbonBuilder::new()
            .set_orientation(Vertical)
            .add_widget(Self::radio_panel())
            .add_widget(Self::button_panel())
            .build()
    }

    fn new() -> Self {
        Self {
            root: Box::new(Self::panel()),
        }
    }
    /*    fn new() -> Self {
        let checkbox1 = DefaultButtonBuilder::new()
            .set_mode(ButtonMode::Checkbox(false))
            .build();
        let checkbox2 = DefaultButtonBuilder::new()
            .set_mode(ButtonMode::Checkbox(true))
            .build();
        let checkbox1id = checkbox1.id();
        let checkboxes = DefaultRibbonBuilder::new()
            .set_orientation(RibbonOrientation::Horizontal)
            .add_widget(checkbox1)
            .add_widget(checkbox2)
            .build();
        let checkboxesid = checkboxes.id();
        let button_add = DefaultButtonBuilder::new()
            .set_mode(ButtonMode::Button)
            .on_click(move |_| {
                task::spawn(async move {
                    checkbox1id
                        .on_click(move |_| {
                            task::spawn(async move {
                                let rotation = checkboxesid.get_orientation().await;
                                checkboxesid.set_orientation(!rotation).await;
                            });
                        })
                        .await;
                    checkboxesid
                        .add_widget(
                            DefaultButtonBuilder::new()
                                .set_mode(ButtonMode::Checkbox(false))
                                .on_click(move |checkbox| {
                                    if let ButtonMode::Checkbox(false) = checkbox.get_mode() {
                                        let srv_id = checkbox.srv_id();
                                        task::spawn(async move {
                                            checkboxesid.remove_widget(srv_id).await;
                                        });
                                    }
                                })
                                .build(),
                        )
                        .await;
                });
            })
            .build();
        let button_rotate = DefaultButtonBuilder::new()
            .set_mode(ButtonMode::Button)
            .on_click(move |_| {
                task::spawn(async move {
                    checkbox1id.remove_on_click(0).await;
                    let rotation = checkboxesid.get_orientation().await;
                    checkboxesid.set_orientation(!rotation).await;
                });
            })
            .build();
        let root = DefaultRibbonBuilder::new()
            .set_orientation(RibbonOrientation::Vertical)
            .add_widget(checkboxes)
            .add_widget(
                DefaultRibbonBuilder::new()
                    .set_orientation(RibbonOrientation::Horizontal)
                    .add_widget(button_add)
                    .add_widget(button_rotate)
                    .build(),
            )
            .build();
        Self {
            root: Box::new(root),
        }
    }
    */
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
