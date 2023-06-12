use crate::{macros::define_enum, Parse, Rect};
use cssparser::*;

define_enum! {
    /// The shape the default view drawing algorithm should use for handling borders.
    pub enum BorderCornerShape {
        /// The round border corner shape.
        "round": Round,
        /// The bevel border corner shape.
        "bevel": Bevel,
    }
}

impl Default for BorderCornerShape {
    fn default() -> Self {
        BorderCornerShape::Round
    }
}

impl From<&str> for BorderCornerShape {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        BorderCornerShape::parse(&mut parser).unwrap_or_default()
    }
}

impl From<BorderCornerShape> for Rect<BorderCornerShape> {
    fn from(value: BorderCornerShape) -> Self {
        Rect(value, value, value, value)
    }
}
