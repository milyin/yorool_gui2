use async_std::task;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use std::sync::{Arc, Mutex};
use yorool_gui2::button::ButtonMode::{Checkbox, PressButton};
use yorool_gui2::default_skin::{ButtonBuilder, ButtonId, RibbonBuilder};
use yorool_gui2::ribbon::RibbonId;
use yorool_gui2::ribbon::RibbonOrientation::{Horizontal, Vertical};
use yorool_gui2::{EventHandlerProxy, Layout, Widget, WidgetGroup};

struct DemoPanelState {
    ribbon_id: RibbonId,
    button_ids: Vec<ButtonId>,
}

fn add_radio(state: Arc<Mutex<DemoPanelState>>) {
    task::spawn(async move {
        let mut radio = ButtonBuilder::new().set_mode(Checkbox(false)).build();
        let (n, ribbon_id) = {
            let mut state = state.lock().unwrap();
            state.button_ids.push(radio.id());
            (state.button_ids.len(), state.ribbon_id)
        };
        radio.set_label(n.to_string());
        ribbon_id.add_widget(radio).await;
    });
}

fn remove_selected_radio(state: Arc<Mutex<DemoPanelState>>) {
    task::spawn(async move {
        let (button_ids, ribbon_id) = {
            let state = state.lock().unwrap();
            (state.button_ids.clone(), state.ribbon_id)
        };
        for radio_id in button_ids {
            if let Checkbox(true) = radio_id.get_mode().await {
                ribbon_id.remove_widget(radio_id.into()).await;
            }
        }
    });
}

struct DemoPanel {
    _state: Arc<Mutex<DemoPanelState>>,
    root: Box<dyn Widget>,
}

impl DemoPanel {
    fn button_panel(state: &Arc<Mutex<DemoPanelState>>) -> impl Widget {
        let add_button = ButtonBuilder::new()
            .set_label("Add")
            .set_mode(PressButton)
            .on_click({
                let state = state.clone();
                move |_| add_radio(state.clone())
            })
            .build();
        let remove_button = ButtonBuilder::new()
            .set_label("Remove selected")
            .set_mode(PressButton)
            .on_click({
                let state = state.clone();
                move |_| remove_selected_radio(state.clone())
            })
            .build();
        RibbonBuilder::new()
            .set_orientation(Horizontal)
            .add_widget(add_button)
            .add_widget(remove_button)
            .build()
    }

    fn panel(radios: impl Widget + 'static, buttons: impl Widget + 'static) -> impl Widget {
        RibbonBuilder::new()
            .set_orientation(Vertical)
            .add_widget(radios)
            .add_widget(buttons)
            .build()
    }

    fn new() -> Self {
        let radios = RibbonBuilder::new().set_orientation(Horizontal).build();
        let state = Arc::new(Mutex::new(DemoPanelState {
            ribbon_id: radios.id(),
            button_ids: Vec::new(),
        }));
        let buttons = Self::button_panel(&state);
        Self {
            _state: state,
            root: Box::new(Self::panel(radios, buttons)),
        }
    }
}

impl WidgetGroup for DemoPanel {
    fn mut_root(&mut self) -> &mut Box<dyn Widget> {
        &mut self.root
    }
    fn root(&self) -> &Box<dyn Widget> {
        &self.root
    }
}

struct GuiDemoState {
    panel: DemoPanel,
}

impl GuiDemoState {
    fn new() -> Self {
        Self {
            panel: DemoPanel::new(),
        }
    }
    /*    fn new() -> Self {
        let checkbox1 = ButtonBuilder::new()
            .set_mode(ButtonMode::Checkbox(false))
            .build();
        let checkbox2 = ButtonBuilder::new()
            .set_mode(ButtonMode::Checkbox(true))
            .build();
        let checkbox1id = checkbox1.id();
        let checkboxes = RibbonBuilder::new()
            .set_orientation(RibbonOrientation::Horizontal)
            .add_widget(checkbox1)
            .add_widget(checkbox2)
            .build();
        let checkboxesid = checkboxes.id();
        let button_add = ButtonBuilder::new()
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
                            ButtonBuilder::new()
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
        let button_rotate = ButtonBuilder::new()
            .set_mode(ButtonMode::Button)
            .on_click(move |_| {
                task::spawn(async move {
                    checkbox1id.remove_on_click(0).await;
                    let rotation = checkboxesid.get_orientation().await;
                    checkboxesid.set_orientation(!rotation).await;
                });
            })
            .build();
        let root = RibbonBuilder::new()
            .set_orientation(RibbonOrientation::Vertical)
            .add_widget(checkboxes)
            .add_widget(
                RibbonBuilder::new()
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
        self.panel.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0., 0., 0., 0.));
        self.panel.draw(ctx)?;
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.panel.mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.panel.mouse_button_up_event(ctx, button, x, y)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0., 0., width, height);
        self.panel.set_rect(new_rect.clone());
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
