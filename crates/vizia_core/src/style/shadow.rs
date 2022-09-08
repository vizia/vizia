use crate::prelude::*;
use crate::style::fmt_units;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct BoxShadow {
    pub horizontal_offset: Units,
    pub vertical_offset: Units,
    pub blur_radius: Units,
    pub color: Color,
}

impl Default for BoxShadow {
    fn default() -> Self {
        BoxShadow {
            horizontal_offset: Units::Auto,
            vertical_offset: Units::Auto,
            blur_radius: Units::Auto,
            color: Color::rgba(0, 0, 0, 128),
        }
    }
}

impl std::fmt::Display for BoxShadow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            fmt_units(&self.horizontal_offset),
            fmt_units(&self.vertical_offset),
            fmt_units(&self.blur_radius),
            &self.color
        )
    }
}
