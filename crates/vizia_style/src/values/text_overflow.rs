use cssparser::{Parser, ParserInput};

use crate::{define_enum, impl_parse, Parse};

define_enum! {
    /// Determines how overflowed content that is not displayed should be signaled to the user.
    #[derive(Default)]
    pub enum TextOverflow {
        /// The text is clipped and not accessible.
        #[default]
        "clip": Clip,
        /// Renders an ellipsis ("…") to represent the clipped text.
        "ellipsis": Ellipsis,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineClamp(pub u32);

impl Default for LineClamp {
    fn default() -> Self {
        LineClamp(1)
    }
}

impl_parse! {
    LineClamp,
    try_parse {
        u32,
    }
}

impl From<&str> for LineClamp {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        LineClamp::parse(&mut parser).unwrap_or_default()
    }
}

impl From<u32> for LineClamp {
    fn from(number: u32) -> Self {
        LineClamp(number)
    }
}

impl From<i32> for LineClamp {
    fn from(number: i32) -> Self {
        LineClamp(number as u32)
    }
}
