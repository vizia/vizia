use morphorm::Units;

use crate::{macros::impl_parse, Parse, Percentage};

/// A number or a percentage value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PercentageOrNumber {
    /// A percentage value.
    Percentage(f32),
    /// A number.
    Number(f32),
}

impl PercentageOrNumber {
    pub fn to_factor(&self) -> f32 {
        match self {
            PercentageOrNumber::Percentage(val) => *val / 100.0,
            PercentageOrNumber::Number(val) => *val,
        }
    }

    pub fn to_number(&self, min_bounds: f32) -> f32 {
        match self {
            PercentageOrNumber::Number(num) => *num,

            PercentageOrNumber::Percentage(val) => (val / 100.0) * min_bounds,
        }
    }
}

impl Default for PercentageOrNumber {
    fn default() -> Self {
        Self::Number(0.0)
    }
}

impl_parse! {
    PercentageOrNumber,

    try_parse {
        Percentage,
        f32,
    }
}

impl From<Percentage> for PercentageOrNumber {
    fn from(percentage: Percentage) -> Self {
        PercentageOrNumber::Percentage(percentage.0)
    }
}

impl From<f32> for PercentageOrNumber {
    fn from(number: f32) -> Self {
        PercentageOrNumber::Number(number)
    }
}

impl From<f64> for PercentageOrNumber {
    fn from(number: f64) -> Self {
        PercentageOrNumber::Number(number as f32)
    }
}

impl From<Units> for PercentageOrNumber {
    fn from(value: Units) -> Self {
        match value {
            Units::Percentage(val) => PercentageOrNumber::Percentage(val),
            _ => PercentageOrNumber::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        PercentageOrNumber, parse_length_percentage,

        number {
            PercentageOrNumber::Number,
        }

        percentage {
            PercentageOrNumber::Percentage,
        }
    }
}
