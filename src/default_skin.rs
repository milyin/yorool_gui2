use crate::button::ButtonBuilder;
use crate::default_skin::button::DefaultButtonSkin;
use crate::ribbon::RibbonBuilder;

pub mod button;

pub type DefaultButtonBuilder = ButtonBuilder<DefaultButtonSkin>;
pub type DefaultRibbonBuilder = RibbonBuilder;
