use cssparser::{Parser, ParserInput};

use crate::{impl_parse, traits::Parse, PercentageOrNumber};

/// A scale defining a scale value on the x and the y axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scale {
    /// The scale value on the x axis.
    pub x: PercentageOrNumber,
    /// The scale value on the y axis.
    pub y: PercentageOrNumber,
}

impl Scale {
    /// Creates a new scale.
    pub fn new<P1: Into<PercentageOrNumber>, P2: Into<PercentageOrNumber>>(x: P1, y: P2) -> Self {
        Self { x: x.into(), y: y.into() }
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self { x: PercentageOrNumber::Number(1.0), y: PercentageOrNumber::Number(1.0) }
    }
}

impl_parse! {
    Scale,

    custom {
        |input| {
            let x = PercentageOrNumber::parse(input)?;
            if let Ok(y) = input.try_parse(PercentageOrNumber::parse) {
                Ok(Scale {x , y})
            } else {
                Ok(Scale { x, y: x })
            }
        }
    }
}

impl From<&str> for Scale {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        Scale::parse(&mut parser).unwrap_or_default()
    }
}

impl<T1: Into<PercentageOrNumber>, T2: Into<PercentageOrNumber>> From<(T1, T2)> for Scale {
    fn from(value: (T1, T2)) -> Scale {
        Scale { x: value.0.into(), y: value.1.into() }
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
            "10% 20%" => Scale::new(Percentage(10.0), Percentage(20.0)),
            "10% 20" => Scale::new(Percentage(10.0), Number(20.0)),
            "10 20%" => Scale::new(Number(10.0), Percentage(20.0)),
            "10 20" => Scale::new(Number(10.0), Number(20.0)),
        }

        failure {
            "10a 10b",
            "test",
        }
    }
}
