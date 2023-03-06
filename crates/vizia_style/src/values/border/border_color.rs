use crate::{macros::impl_parse, Color, Parse, Rect};

/// Defines the color of every border of a rectangle.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BorderColor {
    /// The color of the top border.
    pub top: Color,
    /// The color of the right border.
    pub right: Color,
    /// The color of the bottom border.
    pub bottom: Color,
    /// The color of the left border.
    pub left: Color,
}

impl BorderColor {
    pub fn new(top: Color, right: Color, bottom: Color, left: Color) -> Self {
        Self { top, right, bottom, left }
    }
}

impl_parse! {
    BorderColor,

    try_parse {
        Rect<Color>,
    }
}

impl From<Rect<Color>> for BorderColor {
    fn from(rect: Rect<Color>) -> Self {
        BorderColor::new(rect.0, rect.1, rect.2, rect.3)
    }
}

impl From<Color> for BorderColor {
    fn from(col: Color) -> Self {
        BorderColor::new(col, col, col, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        BorderColor, assert_border_color,

        success {
            "#000000" => BorderColor::new(Color::rgb(0, 0, 0), Color::rgb(0, 0, 0), Color::rgb(0, 0, 0), Color::rgb(0, 0, 0)),
            "#FF00FF #00FF00" => BorderColor::new(Color::rgb(255, 0, 255), Color::rgb(0, 255, 0), Color::rgb(255, 0, 255), Color::rgb(0, 255, 0)),
            "#FF00FF #00FF00 #00FFFF" => BorderColor::new(Color::rgb(255, 0, 255), Color::rgb(0, 255, 0), Color::rgb(0, 255, 255), Color::rgb(0, 255, 0)),
            "#FF00FF #00FF00 #00FFFF #FFFFFF" => BorderColor::new(Color::rgb(255, 0, 255), Color::rgb(0, 255, 0), Color::rgb(0, 255, 255), Color::rgb(255, 255, 255)),
        }

        failure {
            "test",
            "123",
            "#000000 #000000 #000000 #000000 #000000",
        }
    }
}
