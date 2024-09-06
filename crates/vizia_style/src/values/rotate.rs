use crate::{impl_parse, traits::Parse, Angle};

/// A scale defining a scale value on the x and the y axis.
#[derive(Debug, Clone, PartialEq)]
pub struct Rotate {
    pub angle: Angle,
}

impl Rotate {
    /// Creates a new scale.
    pub fn new(angle: Angle) -> Self {
        Self { angle }
    }
}

impl_parse! {
    Rotate,

    custom {
        |input| {
            let angle = Angle::parse(input)?;
            Ok(Rotate { angle })
        }
    }
}

impl<T: Into<Angle>> From<T> for Rotate {
    fn from(value: T) -> Rotate {
        Rotate { angle: value.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Rotate, parse_rotate,

        success {
            "-45deg" => Rotate::new(Angle::Deg(-45.0)),
            "0.25turn" => Rotate::new(Angle::Turn(0.25)),
            "1.57rad" => Rotate::new(Angle::Rad(1.57)),
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
