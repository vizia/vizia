use crate::{macros::impl_parse, Length, LengthOrPercentage, Parse, Rect};

/// Defines the border radius of every corner of a rectangle.
#[derive(Debug, Clone, PartialEq)]
pub struct BorderRadius {
    /// The border radius of the top-left corner.
    pub top_left: LengthOrPercentage,
    /// The border radius of the top-right corner.
    pub top_right: LengthOrPercentage,
    /// The border radius of the bottom-right corner.
    pub bottom_right: LengthOrPercentage,
    /// The border radius of the bottom-left corner.
    pub bottom_left: LengthOrPercentage,
}

impl BorderRadius {
    pub fn new(
        top_left: LengthOrPercentage,
        top_right: LengthOrPercentage,
        bottom_right: LengthOrPercentage,
        bottom_left: LengthOrPercentage,
    ) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
}

impl_parse! {
    BorderRadius,

    try_parse {
        Rect<LengthOrPercentage>,
    }
}

impl From<Rect<LengthOrPercentage>> for BorderRadius {
    fn from(rect: Rect<LengthOrPercentage>) -> Self {
        BorderRadius::new(rect.0, rect.1, rect.2, rect.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        BorderRadius, assert_border_radius,

        success {
            "10px" => BorderRadius::new(LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(10.0))),
            "10px 20px" =>  BorderRadius::new(LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(20.0)), LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(20.0))),
            "10px 20px 30px" =>  BorderRadius::new(LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(20.0)), LengthOrPercentage::Length(Length::px(30.0)), LengthOrPercentage::Length(Length::px(20.0))),
            "10px 20px 30px 40px" =>  BorderRadius::new(LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(20.0)), LengthOrPercentage::Length(Length::px(30.0)), LengthOrPercentage::Length(Length::px(40.0))),
        }

        failure {
            "px",
            "10px 20px 30px 40px 50px",
        }
    }
}
