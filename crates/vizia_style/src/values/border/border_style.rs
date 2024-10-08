use crate::{impl_parse, BorderStyleKeyword, Parse, Rect};

/// Defines the style of every border of a rectangle.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BorderStyle {
    /// The style of the top border.
    pub top: BorderStyleKeyword,
    /// The style of the right border.
    pub right: BorderStyleKeyword,
    /// The style of the bottom border.
    pub bottom: BorderStyleKeyword,
    /// The style of the left border.
    pub left: BorderStyleKeyword,
}

impl BorderStyle {
    /// Creates a new border style.
    pub fn new(
        top: BorderStyleKeyword,
        right: BorderStyleKeyword,
        bottom: BorderStyleKeyword,
        left: BorderStyleKeyword,
    ) -> Self {
        Self { top, right, bottom, left }
    }
}

impl_parse! {
    BorderStyle,

    try_parse {
        Rect<BorderStyleKeyword>,
    }
}

impl From<Rect<BorderStyleKeyword>> for BorderStyle {
    fn from(rect: Rect<BorderStyleKeyword>) -> Self {
        Self::new(rect.0, rect.1, rect.2, rect.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::assert_parse, BorderStyleKeyword::*};

    assert_parse! {
        BorderStyle, assert_border_style,

        custom {
            success {
                "solid" => BorderStyle::new(Solid, Solid, Solid, Solid),
                "solid solid" => BorderStyle::new(Solid, Solid, Solid, Solid),
                "solid solid dashed" => BorderStyle::new(Solid, Solid, Dashed, Solid),
                "solid solid dashed solid" => BorderStyle::new(Solid, Solid, Dashed, Solid),
            }

            failure {
                "test",
                "123",
                "solid solid dashed dashed dotted",
            }
        }
    }
}
