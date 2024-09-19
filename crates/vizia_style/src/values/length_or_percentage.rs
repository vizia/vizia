use crate::{macros::impl_parse, Length, LengthValue, Parse, Percentage};
use cssparser::*;
use morphorm::Units;

/// A length or a percentage value.
#[derive(Debug, Clone, PartialEq)]
pub enum LengthOrPercentage {
    Length(Length),
    Percentage(f32),
}

impl Default for LengthOrPercentage {
    fn default() -> Self {
        Self::Length(Length::default())
    }
}

impl LengthOrPercentage {
    // TODO - Function to return the length in pixels given the necessary input parameters
    // > dpi, font_size, size of 0 char, viewport size, min of bounds
    pub fn to_pixels(&self, min_bounds: f32, scale: f32) -> f32 {
        match self {
            Self::Length(length) => {
                match length {
                    Length::Value(val) => {
                        if let LengthValue::Px(pixels) = val {
                            return *pixels * scale;
                        }
                    }

                    // TODO
                    Length::Calc(_l) => {
                        todo!();
                    }
                }
            }

            Self::Percentage(val) => {
                return (val / 100.0) * min_bounds;
            }
        }

        0.0
    }

    pub fn px(val: f32) -> Self {
        Self::Length(Length::px(val))
    }
}

impl_parse! {
    LengthOrPercentage,

    try_parse {
        Length,
        Percentage,
    }
}

impl From<LengthValue> for LengthOrPercentage {
    fn from(value: LengthValue) -> Self {
        Self::Length(Length::Value(value))
    }
}

impl From<Length> for LengthOrPercentage {
    fn from(length: Length) -> Self {
        Self::Length(length)
    }
}

impl From<Percentage> for LengthOrPercentage {
    fn from(percentage: Percentage) -> Self {
        Self::Percentage(percentage.0)
    }
}

impl From<&str> for LengthOrPercentage {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        Self::parse(&mut parser).unwrap_or_default()
    }
}

impl From<Units> for LengthOrPercentage {
    fn from(units: Units) -> Self {
        match units {
            Units::Pixels(val) => Length::Value(LengthValue::Px(val)).into(),
            Units::Percentage(val) => Self::Percentage(val),
            _ => Self::default(),
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
