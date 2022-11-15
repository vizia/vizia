use crate::{macros::impl_parse, Length, Parse, Percentage};

/// A length or a percentage value.
#[derive(Debug, Clone, PartialEq)]
pub enum LengthOrPercentage {
    Length(Length),
    Percentage(f32),
}

impl Default for LengthOrPercentage {
    fn default() -> Self {
        LengthOrPercentage::Length(Length::default())
    }
}

impl_parse! {
    LengthOrPercentage,

    try_parse {
        Length,
        Percentage,
    }
}

impl From<Length> for LengthOrPercentage {
    fn from(length: Length) -> Self {
        LengthOrPercentage::Length(length)
    }
}

impl From<Percentage> for LengthOrPercentage {
    fn from(percentage: Percentage) -> Self {
        LengthOrPercentage::Percentage(percentage.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        LengthOrPercentage, parse_length_percentage,

        percentage {
            LengthOrPercentage::Percentage,
        }

        length {
            LengthOrPercentage::Length,
        }
    }
}
