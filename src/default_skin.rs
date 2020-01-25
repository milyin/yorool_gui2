use crate::default_skin::button::DefaultButtonSkin;

pub mod button;

pub type ButtonId = crate::button::ButtonId<DefaultButtonSkin>;
pub type ButtonBuilder = crate::button::ButtonBuilder<DefaultButtonSkin>;
pub type RibbonBuilder = crate::ribbon::RibbonBuilder;
