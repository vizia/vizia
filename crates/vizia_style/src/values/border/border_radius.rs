use crate::{macros::impl_parse, LengthOrPercentage, Parse, Rect};
/// Defines the border radius of every corner of a rectangle.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CornerRadius {
    /// The border radius of the top-left corner.
    pub top_left: LengthOrPercentage,
    /// The border radius of the top-right corner.
    pub top_right: LengthOrPercentage,
    /// The border radius of the bottom-right corner.
    pub bottom_right: LengthOrPercentage,
    /// The border radius of the bottom-left corner.
    pub bottom_left: LengthOrPercentage,
}

impl CornerRadius {
    pub fn new(
        top_left: LengthOrPercentage,
        top_right: LengthOrPercentage,
        bottom_right: LengthOrPercentage,
        bottom_left: LengthOrPercentage,
    ) -> Self {
        Self { top_left, top_right, bottom_right, bottom_left }
    }
}

impl_parse! {
    CornerRadius,

    try_parse {
        Rect<LengthOrPercentage>,
    }
}

impl From<Rect<LengthOrPercentage>> for CornerRadius {
    fn from(rect: Rect<LengthOrPercentage>) -> Self {
        CornerRadius::new(rect.0, rect.1, rect.2, rect.3)
    }
}

// impl From<LengthOrPercentage> for CornerRadius {
//     fn from(length: LengthOrPercentage) -> Self {
//         CornerRadius::new(length.clone(), length.clone(), length.clone(), length.clone())
//     }
// }

// impl From<Length> for CornerRadius {
//     fn from(length: Length) -> Self {
//         CornerRadius::new(
//             length.clone().into(),
//             length.clone().into(),
//             length.clone().into(),
//             length.clone().into(),
//         )
//     }
// }

// impl From<LengthValue> for CornerRadius {
//     fn from(length: LengthValue) -> Self {
//         CornerRadius::new(
//             length.clone().into(),
//             length.clone().into(),
//             length.clone().into(),
//             length.clone().into(),
//         )
//     }
// }

// impl From<Units> for CornerRadius {
//     fn from(value: Units) -> Self {
//         let length: LengthOrPercentage = value.into();
//         CornerRadius::new(length.clone(), length.clone(), length.clone(), length.clone())
//     }
// }

// impl From<&str> for CornerRadius {
//     fn from(s: &str) -> Self {
//         let mut input = ParserInput::new(&s);
//         let mut parser = Parser::new(&mut input);
//         CornerRadius::parse(&mut parser).unwrap_or_default()
//     }
// }

impl<T: Into<LengthOrPercentage>> From<T> for CornerRadius {
    fn from(value: T) -> Self {
        let length: LengthOrPercentage = value.into();
        CornerRadius::new(length.clone(), length.clone(), length.clone(), length)
    }
}

impl<T1: Into<LengthOrPercentage>, T2: Into<LengthOrPercentage>> From<(T1, T2)> for CornerRadius {
    fn from(value: (T1, T2)) -> Self {
        let length1: LengthOrPercentage = value.0.into();
        let length2: LengthOrPercentage = value.1.into();
        CornerRadius::new(length1.clone(), length2.clone(), length1, length2)
    }
}

impl<T1: Into<LengthOrPercentage>, T2: Into<LengthOrPercentage>, T3: Into<LengthOrPercentage>>
    From<(T1, T2, T3)> for CornerRadius
{
    fn from(value: (T1, T2, T3)) -> Self {
        let length1: LengthOrPercentage = value.0.into();
        let length2: LengthOrPercentage = value.1.into();
        let length3: LengthOrPercentage = value.2.into();
        CornerRadius::new(length1, length2.clone(), length3, length2)
    }
}

impl<
        T1: Into<LengthOrPercentage>,
        T2: Into<LengthOrPercentage>,
        T3: Into<LengthOrPercentage>,
        T4: Into<LengthOrPercentage>,
    > From<(T1, T2, T3, T4)> for CornerRadius
{
    fn from(value: (T1, T2, T3, T4)) -> Self {
        let length1: LengthOrPercentage = value.0.into();
        let length2: LengthOrPercentage = value.1.into();
        let length3: LengthOrPercentage = value.2.into();
        let length4: LengthOrPercentage = value.3.into();
        CornerRadius::new(length1, length2, length3, length4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;
    use crate::Length;

    assert_parse! {
        CornerRadius, assert_border_radius,

        success {
            "10px" => CornerRadius::new(LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(10.0))),
            "10px 20px" =>  CornerRadius::new(LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(20.0)), LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(20.0))),
            "10px 20px 30px" =>  CornerRadius::new(LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(20.0)), LengthOrPercentage::Length(Length::px(30.0)), LengthOrPercentage::Length(Length::px(20.0))),
            "10px 20px 30px 40px" =>  CornerRadius::new(LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Length(Length::px(20.0)), LengthOrPercentage::Length(Length::px(30.0)), LengthOrPercentage::Length(Length::px(40.0))),
        }

        failure {
            "px",
            "10px 20px 30px 40px 50px",
        }
    }
}
