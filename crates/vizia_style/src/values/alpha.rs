use crate::{impl_parse, traits::Parse, Percentage};

/// A value specifying the alpha channel or transparency of a color.
#[derive(Debug, Clone, PartialEq)]
pub struct AlphaValue(pub f32);

impl_parse! {
    AlphaValue,

    try_parse {
        Percentage,
        f32,
    }
}

impl From<f32> for AlphaValue {
    fn from(number: f32) -> Self {
        AlphaValue(number)
    }
}

impl From<Percentage> for AlphaValue {
    fn from(percentage: Percentage) -> Self {
        AlphaValue(percentage.0)
    }
}

impl From<AlphaValue> for f32 {
    fn from(alpha_value: AlphaValue) -> Self {
        alpha_value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        AlphaValue, parse_numbers,

        number {
            AlphaValue,
        }

        percentage {
            AlphaValue,
        }
    }
}
