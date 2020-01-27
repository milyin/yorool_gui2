use async_std::task;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use std::sync::{Arc, Mutex};
use yorool_gui2::button::ButtonMode::{Checkbox, PressButton, Radio};
use yorool_gui2::default_skin::{ButtonBuilder, ButtonId, RibbonBuilder};
use yorool_gui2::radiogroup::{RadioGroupBuilder, RadioGroupId};
use yorool_gui2::ribbon::RibbonId;
use yorool_gui2::ribbon::RibbonOrientation::{Horizontal, Vertical};
use yorool_gui2::{EventHandlerProxy, Widget, WidgetGroup};

struct DemoPanelState {
    button_ids: Vec<ButtonId>,
}

fn add_checkbox_to_ribbon(ribbon_id: RibbonId, state: Arc<Mutex<DemoPanelState>>) {
    task::spawn(async move {
        let mut checkbox = ButtonBuilder::new().set_mode(Checkbox(false)).build();
        let n = {
            let mut state = state.lock().unwrap();
            state.button_ids.push(checkbox.id());
            state.button_ids.len()
        };
        checkbox.set_label(n.to_string());
        ribbon_id.add_widget(checkbox).await;
    });
}

fn add_radio_to_ribbon(ribbon_id: RibbonId, radio_group_id: RadioGroupId) {
    task::spawn(async move {
        let radio = ButtonBuilder::new().set_mode(Radio(false)).build();
        radio_group_id.add_radio(radio.id()).await;
        ribbon_id.add_widget(radio).await;
    });
}

fn remove_selected(ribbon_id: RibbonId, state: Arc<Mutex<DemoPanelState>>) {
    task::spawn(async move {
        let button_ids = {
            let state = state.lock().unwrap();
            state.button_ids.clone()
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
    fn button_panel(
        state: &Arc<Mutex<DemoPanelState>>,
        radio_ribbon_id: RibbonId,
        radio_group_id: RadioGroupId,
        checkbox_ribbon_id: RibbonId,
    ) -> impl Widget {
        let add_checkbox_button = ButtonBuilder::new()
            .set_label("Add checkbox")
            .set_mode(PressButton)
            .on_click({
                let state = state.clone();
                move |_| add_checkbox_to_ribbon(checkbox_ribbon_id, state.clone())
            })
            .build();
        let add_radio_button = ButtonBuilder::new()
            .set_label("Add radio")
            .set_mode(PressButton)
            .on_click(move |_| add_radio_to_ribbon(radio_ribbon_id, radio_group_id))
            .build();
        let remove_button = ButtonBuilder::new()
            .set_label("Remove selected")
            .set_mode(PressButton)
            .on_click({
                let state = state.clone();
                move |_| remove_selected(radio_ribbon_id, state.clone())
            })
            .build();
        RibbonBuilder::new()
            .set_orientation(Horizontal)
            .add_widget(add_checkbox_button)
            .add_widget(add_radio_button)
            .add_widget(remove_button)
            .build()
    }

    fn panel(
        radios: impl Widget + 'static,
        checkboxes: impl Widget + 'static,
        buttons: impl Widget + 'static,
    ) -> impl Widget {
        RibbonBuilder::new()
            .set_orientation(Vertical)
            .add_widget(radios)
            .add_widget(checkboxes)
            .add_widget(buttons)
            .build()
    }

    fn new() -> Self {
        let radio_group = RadioGroupBuilder::new().build();
        let radio_group_id = radio_group.id();
        let radio_ribbon = RibbonBuilder::new()
            .set_orientation(Horizontal)
            .add_widget(radio_group)
            .build();
        let radio_ribbon_id = radio_ribbon.id();
        let checkbox_ribbon = RibbonBuilder::new().set_orientation(Horizontal).build();
        let checkbox_ribbon_id = checkbox_ribbon.id();
        let state = Arc::new(Mutex::new(DemoPanelState {
            button_ids: Vec::new(),
        }));
        let buttons =
            Self::button_panel(&state, radio_ribbon_id, radio_group_id, checkbox_ribbon_id);
        Self {
            _state: state,
            root: Box::new(Self::panel(radio_ribbon, checkbox_ribbon, buttons)),
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
