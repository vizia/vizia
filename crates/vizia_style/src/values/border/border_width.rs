use crate::{impl_parse, BorderWidthValue, Parse, Rect};

/// Defines the width of every border of a rectangle.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BorderWidth {
    /// The width of the top border.
    pub top: BorderWidthValue,
    /// The width of the right border.
    pub right: BorderWidthValue,
    /// The width of the bottom border.
    pub bottom: BorderWidthValue,
    /// The width of the left border.
    pub left: BorderWidthValue,
}

impl BorderWidth {
    /// Creates a new border width.
    pub fn new(
        top: BorderWidthValue,
        right: BorderWidthValue,
        bottom: BorderWidthValue,
        left: BorderWidthValue,
    ) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl_parse! {
    BorderWidth,

    try_parse {
        Rect<BorderWidthValue>,
    }
}

impl From<Rect<BorderWidthValue>> for BorderWidth {
    fn from(rect: Rect<BorderWidthValue>) -> Self {
        BorderWidth::new(rect.0, rect.1, rect.2, rect.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::assert_parse, Length};

    assert_parse! {
        BorderWidth, assert_border_width,

        custom {
            success {
                "10px" => BorderWidth::new(Length::px(10.0).into(), Length::px(10.0).into(), Length::px(10.0).into(), Length::px(10.0).into()),
                "10px 20px" => BorderWidth::new(Length::px(10.0).into(), Length::px(20.0).into(), Length::px(10.0).into(), Length::px(20.0).into()),
                "10px 20px 30px" => BorderWidth::new(Length::px(10.0).into(), Length::px(20.0).into(), Length::px(30.0).into(), Length::px(20.0).into()),
                "10px 20px 30px 40px" => BorderWidth::new(Length::px(10.0).into(), Length::px(20.0).into(), Length::px(30.0).into(), Length::px(40.0).into()),

                "thin" => BorderWidth::new(Length::px(1.0).into(), Length::px(1.0).into(), Length::px(1.0).into(), Length::px(1.0).into()),
                "thin medium" => BorderWidth::new(Length::px(1.0).into(), Length::px(3.0).into(), Length::px(1.0).into(), Length::px(3.0).into()),
                "thin medium thick" => BorderWidth::new(Length::px(1.0).into(), Length::px(3.0).into(), Length::px(5.0).into(), Length::px(3.0).into()),
                "thin medium thick thin" => BorderWidth::new(Length::px(1.0).into(), Length::px(3.0).into(), Length::px(5.0).into(), Length::px(1.0).into()),
            }

            failure {
                "test",
                "123",
                "10px 20px 30px 40px 50px",
            }
        }
    }
}
