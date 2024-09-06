use crate::{macros::define_enum, Parse, Rect};
use cssparser::*;

define_enum! {
    /// The shape the default view drawing algorithm should use for handling borders.
    #[derive(Default)]
    pub enum CornerShape {
        /// The round border corner shape.
        #[default]
        "round": Round,
        /// The bevel border corner shape.
        "bevel": Bevel,
    }
}

impl From<&str> for CornerShape {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        Self::parse(&mut parser).unwrap_or_default()
    }
}

impl From<CornerShape> for Rect<CornerShape> {
    fn from(value: CornerShape) -> Self {
        Self(value, value, value, value)
    }
}
