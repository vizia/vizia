pub mod style;
pub use style::*;

pub mod keyframes;
pub use keyframes::*;

pub mod property;
pub use property::*;

#[derive(Debug, PartialEq, Clone)]
pub struct CssRuleList<'i>(pub Vec<CssRule<'i>>);

#[derive(Debug, PartialEq, Clone)]
pub enum CssRule<'i> {
    Style(StyleRule<'i>),
    Property(PropertyRule<'i>),
    Ignored,
    Keyframes(KeyframesRule<'i>),
}
