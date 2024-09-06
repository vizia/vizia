use morphorm::Units;

use crate::{
    calc::Calc,
    impl_parse,
    traits::{Parse, TryAdd},
    LengthValue,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Length {
    Value(LengthValue),
    Calc(Box<Calc<Length>>),
}

impl Default for Length {
    fn default() -> Self {
        Self::Value(LengthValue::default())
    }
}

impl_parse! {
    Length,

    custom {
        |input| {
            match input.try_parse(Calc::parse) {
                Ok(Calc::Value(v)) => return Ok(*v),
                Ok(calc) => return Ok(Length::Calc(Box::new(calc))),
                _ => {}
            }

            let length = LengthValue::parse(input)?;
            Ok(Length::Value(length))
        }
    }
}

impl std::ops::Mul<f32> for Length {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        match self {
            Self::Value(a) => Self::Value(a * other),
            Self::Calc(a) => Self::Calc(Box::new(*a * other)),
        }
    }
}

impl std::ops::Add<Self> for Length {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self.try_add(&other) {
            Some(r) => r,
            None => self.add(other),
        }
    }
}

impl Length {
    pub fn zero() -> Self {
        Self::Value(LengthValue::Px(0.0))
    }

    pub fn px(px: f32) -> Self {
        Self::Value(LengthValue::Px(px))
    }

    pub fn to_px(&self) -> Option<f32> {
        match self {
            Self::Value(a) => a.to_px(),
            _ => None,
        }
    }

    fn add(self, other: Self) -> Self {
        let mut a = self;
        let mut b = other;

        if a == 0.0 {
            return b;
        }

        if b == 0.0 {
            return a;
        }

        if a < 0.0 && b > 0.0 {
            std::mem::swap(&mut a, &mut b);
        }

        match (a, b) {
            (Self::Calc(a), Self::Calc(b)) => Self::Calc(Box::new(*a + *b)),
            (Self::Calc(calc), b) => {
                if let Calc::Value(a) = *calc {
                    a.add(b)
                } else {
                    Self::Calc(Box::new(Calc::Sum(Box::new(*calc), Box::new(b.into()))))
                }
            }
            (a, Self::Calc(calc)) => {
                if let Calc::Value(b) = *calc {
                    a.add(*b)
                } else {
                    Self::Calc(Box::new(Calc::Sum(Box::new(a.into()), Box::new(*calc))))
                }
            }
            (a, b) => Self::Calc(Box::new(Calc::Sum(Box::new(a.into()), Box::new(b.into())))),
        }
    }
}

impl TryAdd<Self> for Length {
    fn try_add(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a.try_add(b).map(Length::Value),
            (Self::Calc(a), other) => match &**a {
                Calc::Value(v) => v.try_add(other),
                Calc::Sum(a, b) => {
                    if let Some(res) = Self::Calc(Box::new(*a.clone())).try_add(other) {
                        return Some(res.add(Self::from(*b.clone())));
                    }

                    if let Some(res) = Self::Calc(Box::new(*b.clone())).try_add(other) {
                        return Some(Self::from(*a.clone()).add(res));
                    }

                    None
                }
                _ => None,
            },
            (other, Self::Calc(b)) => match &**b {
                Calc::Value(v) => other.try_add(v),
                Calc::Sum(a, b) => {
                    if let Some(res) = other.try_add(&Self::Calc(Box::new(*a.clone()))) {
                        return Some(res.add(Self::from(*b.clone())));
                    }

                    if let Some(res) = other.try_add(&Self::Calc(Box::new(*b.clone()))) {
                        return Some(Self::from(*a.clone()).add(res));
                    }

                    None
                }
                _ => None,
            },
        }
    }
}

impl From<Length> for Calc<Length> {
    fn from(value: Length) -> Self {
        match value {
            Length::Calc(c) => *c,
            b => Self::Value(Box::new(b)),
        }
    }
}

impl std::convert::From<Calc<Self>> for Length {
    fn from(calc: Calc<Self>) -> Self {
        Self::Calc(Box::new(calc))
    }
}

impl std::cmp::PartialEq<f32> for Length {
    fn eq(&self, other: &f32) -> bool {
        match self {
            Self::Value(a) => *a == *other,
            Self::Calc(_) => false,
        }
    }
}

impl std::cmp::PartialOrd<f32> for Length {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        match self {
            Self::Value(a) => a.partial_cmp(other),
            Self::Calc(_) => None,
        }
    }
}

impl std::cmp::PartialOrd<Self> for Length {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl From<f32> for Length {
    fn from(value: f32) -> Self {
        Self::px(value)
    }
}

impl From<Units> for Length {
    fn from(value: Units) -> Self {
        match value {
            Units::Pixels(val) => Self::px(val),
            _ => Self::default(),
        }
    }
}
