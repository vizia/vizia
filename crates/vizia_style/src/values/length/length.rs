use std::fmt::Debug;

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
        Length::Value(LengthValue::default())
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

    fn mul(self, other: f32) -> Length {
        match self {
            Length::Value(a) => Length::Value(a * other),
            Length::Calc(a) => Length::Calc(Box::new(*a * other)),
        }
    }
}

impl std::ops::Add<Length> for Length {
    type Output = Self;

    fn add(self, other: Length) -> Length {
        match self.try_add(&other) {
            Some(r) => r,
            None => self.add(other),
        }
    }
}

impl Length {
    pub fn zero() -> Length {
        Length::Value(LengthValue::Px(0.0))
    }

    pub fn px(px: f32) -> Length {
        Length::Value(LengthValue::Px(px))
    }

    pub fn to_px(&self) -> Option<f32> {
        match self {
            Length::Value(a) => a.to_px(),
            _ => None,
        }
    }

    fn add(self, other: Length) -> Length {
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
            (Length::Calc(a), Length::Calc(b)) => Length::Calc(Box::new(*a + *b)),
            (Length::Calc(calc), b) => {
                if let Calc::Value(a) = *calc {
                    a.add(b)
                } else {
                    Length::Calc(Box::new(Calc::Sum(Box::new(*calc), Box::new(b.into()))))
                }
            }
            (a, Length::Calc(calc)) => {
                if let Calc::Value(b) = *calc {
                    a.add(*b)
                } else {
                    Length::Calc(Box::new(Calc::Sum(Box::new(a.into()), Box::new(*calc))))
                }
            }
            (a, b) => Length::Calc(Box::new(Calc::Sum(Box::new(a.into()), Box::new(b.into())))),
        }
    }
}

impl TryAdd<Length> for Length {
    fn try_add(&self, other: &Length) -> Option<Length> {
        match (self, other) {
            (Length::Value(a), Length::Value(b)) => a.try_add(b).map(Length::Value),
            (Length::Calc(a), other) => match &**a {
                Calc::Value(v) => v.try_add(other),
                Calc::Sum(a, b) => {
                    if let Some(res) = Length::Calc(Box::new(*a.clone())).try_add(other) {
                        return Some(res.add(Length::from(*b.clone())));
                    }

                    if let Some(res) = Length::Calc(Box::new(*b.clone())).try_add(other) {
                        return Some(Length::from(*a.clone()).add(res));
                    }

                    None
                }
                _ => None,
            },
            (other, Length::Calc(b)) => match &**b {
                Calc::Value(v) => other.try_add(v),
                Calc::Sum(a, b) => {
                    if let Some(res) = other.try_add(&Length::Calc(Box::new(*a.clone()))) {
                        return Some(res.add(Length::from(*b.clone())));
                    }

                    if let Some(res) = other.try_add(&Length::Calc(Box::new(*b.clone()))) {
                        return Some(Length::from(*a.clone()).add(res));
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
            b => Calc::Value(Box::new(b)),
        }
    }
}

impl std::convert::From<Calc<Length>> for Length {
    fn from(calc: Calc<Length>) -> Length {
        Length::Calc(Box::new(calc))
    }
}

impl std::cmp::PartialEq<f32> for Length {
    fn eq(&self, other: &f32) -> bool {
        match self {
            Length::Value(a) => *a == *other,
            Length::Calc(_) => false,
        }
    }
}

impl std::cmp::PartialOrd<f32> for Length {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        match self {
            Length::Value(a) => a.partial_cmp(other),
            Length::Calc(_) => None,
        }
    }
}

impl std::cmp::PartialOrd<Length> for Length {
    fn partial_cmp(&self, other: &Length) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Length::Value(a), Length::Value(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl From<f32> for Length {
    fn from(value: f32) -> Self {
        Length::px(value)
    }
}

impl From<Units> for Length {
    fn from(value: Units) -> Self {
        match value {
            Units::Pixels(val) => Length::px(val),
            _ => Length::default(),
        }
    }
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Length::Value(length_value) => std::fmt::Display::fmt(&length_value, f),
            Length::Calc(_calc) => todo!(),
        }
    }
}
