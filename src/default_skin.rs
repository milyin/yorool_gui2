use crate::default_skin::button::DefaultButtonSkin;

pub mod button;

pub type Button = crate::button::Button<DefaultButtonSkin>;
pub type ButtonId = crate::button::ButtonId;
pub type ButtonBuilder = crate::button::ButtonBuilder<DefaultButtonSkin>;
pub type RibbonBuilder = crate::ribbon::RibbonBuilder;
