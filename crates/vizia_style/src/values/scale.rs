use crate::{impl_parse, traits::Parse, PercentageOrNumber};

/// A scale defining a scale value on the x and the y axis.
#[derive(Debug, Clone, PartialEq)]
pub struct Scale {
    /// The scale value on the x axis.
    pub x: PercentageOrNumber,
    /// The scale value on the y axis.
    pub y: PercentageOrNumber,
}

impl Scale {
    /// Creates a new scale.
    pub fn new(x: PercentageOrNumber, y: PercentageOrNumber) -> Self {
        Self { x, y }
    }
}

impl_parse! {
    Scale,

    custom {
        |input| {
            let x = PercentageOrNumber::parse(input)?;
            input.expect_comma()?;
            let y = PercentageOrNumber::parse(input)?;
            Ok(Scale { x, y })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;
    use crate::PercentageOrNumber::*;

    assert_parse! {
        Scale, parse_scale,

        success {
            "10%, 20%" => Scale::new(Percentage(0.1), Percentage(0.2)),
            "10%, 20" => Scale::new(Percentage(0.1), Number(20.0)),
            "10, 20%" => Scale::new(Number(10.0), Percentage(0.2)),
            "10, 20" => Scale::new(Number(10.0), Number(20.0)),
        }

        failure {
            "10a, 10b",
            "123",
            "123%",
            "10% 20%",
            "test",
        }
    }
}
