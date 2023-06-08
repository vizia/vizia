use crate::{impl_parse, traits::Parse, LengthOrPercentage};

/// A translate defining a translate value on the x and the y axis.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Translate {
    /// The translate value on the x axis.
    pub x: LengthOrPercentage,
    /// The translate value on the y axis.
    pub y: LengthOrPercentage,
}

impl Translate {
    /// Creates a new translation.
    pub fn new<L1: Into<LengthOrPercentage>, L2: Into<LengthOrPercentage>>(x: L1, y: L2) -> Self {
        Self { x: x.into(), y: y.into() }
    }
}

impl_parse! {
    Translate,

    custom {
        |input| {
            let x = LengthOrPercentage::parse(input)?;
            let y = input.try_parse(LengthOrPercentage::parse).ok().unwrap_or_default();
            Ok(Translate { x, y })
        }
    }
}

impl<T: Into<LengthOrPercentage>> From<T> for Translate {
    fn from(value: T) -> Translate {
        let l = value.into();
        Translate { x: l.clone(), y: l }
    }
}

impl<T1: Into<LengthOrPercentage>, T2: Into<LengthOrPercentage>> From<(T1, T2)> for Translate {
    fn from(value: (T1, T2)) -> Translate {
        Translate { x: value.0.into(), y: value.1.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::assert_parse, Length, LengthOrPercentage::*};

    assert_parse! {
        Translate, parse_translate,

        success {
            "10% 20%" => Translate::new(Percentage(10.0), Percentage(20.0)),
            "10% 20px" => Translate::new(Percentage(10.0), Length(Length::px(20.0))),
            "10px 20%" => Translate::new(Length(Length::px(10.0)), Percentage(20.0)),
            "10px 20px" => Translate::new(Length(Length::px(10.0)), Length(Length::px(20.0))),

        }

        failure {
            "10a, 10b",
            "test",
        }
    }
}
