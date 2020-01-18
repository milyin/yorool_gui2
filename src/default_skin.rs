use crate::button::ButtonBuilder;
use crate::default_skin::button::DefaultButtonSkin;
use crate::ribbon::RibbonBuilder;

pub mod button;

pub type DefaultButtonBuilder<'a> = ButtonBuilder<'a, DefaultButtonSkin>;
pub type DefaultRibbonBuilder<'a> = RibbonBuilder<'a>;
