use crate::{
    macros::impl_parse, AutoKeyword, Length, LengthOrPercentage, LengthValue, Parse, Percentage,
};
use cssparser::*;
use morphorm::Units;

/// A length or a percentage value.
#[derive(Debug, Clone, PartialEq)]
pub enum LengthPercentageOrAuto {
    LengthPercentage(LengthOrPercentage),
    Auto,
}

impl Default for LengthPercentageOrAuto {
    fn default() -> Self {
        Self::LengthPercentage(LengthOrPercentage::Length(Length::default()))
    }
}

impl LengthPercentageOrAuto {
    // TODO - Function to return the length in pixels given the necessary input parameters
    // > dpi, font_size, size of 0 char, viewport size, min of bounds
    pub fn to_pixels(&self, min_bounds: f32, scale: f32) -> f32 {
        match self {
            Self::LengthPercentage(length) => length.to_pixels(min_bounds, scale),

            Self::Auto => 0.0,
        }
    }

    pub fn px(val: f32) -> Self {
        Self::LengthPercentage(LengthOrPercentage::Length(Length::px(val)))
    }
}

impl_parse! {
    LengthPercentageOrAuto,

    try_parse {
        LengthOrPercentage,
        AutoKeyword,
    }
}

impl From<LengthValue> for LengthPercentageOrAuto {
    fn from(value: LengthValue) -> Self {
        Self::LengthPercentage(LengthOrPercentage::Length(Length::Value(value)))
    }
}

impl From<Length> for LengthPercentageOrAuto {
    fn from(length: Length) -> Self {
        Self::LengthPercentage(LengthOrPercentage::Length(length))
    }
}

impl From<LengthOrPercentage> for LengthPercentageOrAuto {
    fn from(length: LengthOrPercentage) -> Self {
        Self::LengthPercentage(length)
    }
}

impl From<AutoKeyword> for LengthPercentageOrAuto {
    fn from(_: AutoKeyword) -> Self {
        Self::Auto
    }
}

impl From<Percentage> for LengthPercentageOrAuto {
    fn from(percentage: Percentage) -> Self {
        Self::LengthPercentage(LengthOrPercentage::Percentage(percentage.0))
    }
}

impl From<&str> for LengthPercentageOrAuto {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        Self::parse(&mut parser).unwrap_or_default()
    }
}

impl From<Units> for LengthPercentageOrAuto {
    fn from(units: Units) -> Self {
        match units {
            Units::Pixels(val) => Length::Value(LengthValue::Px(val)).into(),
            Units::Percentage(val) => Self::LengthPercentage(LengthOrPercentage::Percentage(val)),
            _ => Self::Auto,
        }
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
