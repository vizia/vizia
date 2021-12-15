
use crate::Color;

use crate::Units;

#[derive(Copy, Clone, Debug, PartialEq)]
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