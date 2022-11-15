use crate::{impl_parse, traits::Parse, LengthOrPercentage};

/// A translate defining a translate value on the x and the y axis.
#[derive(Debug, Clone, PartialEq)]
pub struct Translate {
    /// The translate value on the x axis.
    pub x: LengthOrPercentage,
    /// The translate value on the y axis.
    pub y: LengthOrPercentage,
}

impl Translate {
    /// Creates a new translate.
    pub fn new(x: LengthOrPercentage, y: LengthOrPercentage) -> Self {
        Self { x, y }
    }
}

impl_parse! {
    Translate,

    custom {
        |input| {
            let x = LengthOrPercentage::parse(input)?;
            input.expect_comma()?;
            let y = LengthOrPercentage::parse(input)?;
            Ok(Translate { x, y })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::assert_parse, Length, LengthOrPercentage::*};

    assert_parse! {
        Translate, parse_translate,

        success {
            "10%, 20%" => Translate::new(Percentage(0.1), Percentage(0.2)),
            "10%, 20px" => Translate::new(Percentage(0.1), Length(Length::px(20.0))),
            "10px, 20%" => Translate::new(Length(Length::px(10.0)), Percentage(0.2)),
            "10px, 20px" => Translate::new(Length(Length::px(10.0)), Length(Length::px(20.0))),
        }

        failure {
            "10a, 10b",
            "123px",
            "123%",
            "10% 20%",
            "10px 20px",
            "test",
        }
    }
}
