use crate::{macros::impl_parse, Parse, Percentage};

/// A number or a percentage value.
#[derive(Debug, Clone, PartialEq)]
pub enum PercentageOrNumber {
    /// A percentage value.
    Percentage(f32),
    /// A number.
    Number(f32),
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
