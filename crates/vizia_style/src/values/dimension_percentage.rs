use crate::{Calc, CustomParseError, Parse, Percentage, TryAdd};
use cssparser::{ParseError, Parser};

/// A generic type that allows any kind of dimension and percentage to be
/// used standalone or mixed within a calc() expression.
/// https://drafts.csswg.org/css-values-4/#mixed-percentages
#[derive(Debug, Clone, PartialEq)]
pub enum DimensionPercentage<D> {
    Dimension(D),
    Percentage(Percentage),
    Calc(Box<Calc<DimensionPercentage<D>>>),
}

impl<
        'i,
        D: Parse<'i>
            + std::ops::Mul<f32, Output = D>
            + TryAdd<D>
            + Clone
            + std::cmp::PartialEq<f32>
            + std::cmp::PartialOrd<f32>
            + std::cmp::PartialOrd<D>
            + std::fmt::Debug,
    > Parse<'i> for DimensionPercentage<D>
{
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        match input.try_parse(Calc::parse) {
            Ok(Calc::Value(v)) => return Ok(*v),
            Ok(calc) => return Ok(DimensionPercentage::Calc(Box::new(calc))),
            _ => {}
        }

        if let Ok(length) = input.try_parse(D::parse) {
            return Ok(DimensionPercentage::Dimension(length));
        }

        if let Ok(percent) = input.try_parse(Percentage::parse) {
            return Ok(DimensionPercentage::Percentage(percent));
        }

        Err(input.new_error_for_next_token())
    }
}

impl<D: std::ops::Mul<f32, Output = D>> std::ops::Mul<f32> for DimensionPercentage<D> {
    type Output = Self;

    fn mul(self, other: f32) -> DimensionPercentage<D> {
        match self {
            DimensionPercentage::Dimension(l) => DimensionPercentage::Dimension(l * other),
            DimensionPercentage::Percentage(p) => {
                DimensionPercentage::Percentage(Percentage(p.0 * other))
            }
            DimensionPercentage::Calc(c) => DimensionPercentage::Calc(Box::new(*c * other)),
        }
    }
}

impl<
        D: TryAdd<D> + Clone + std::cmp::PartialEq<f32> + std::cmp::PartialOrd<f32> + std::fmt::Debug,
    > std::ops::Add<DimensionPercentage<D>> for DimensionPercentage<D>
{
    type Output = Self;

    fn add(self, other: DimensionPercentage<D>) -> DimensionPercentage<D> {
        match self.add_recursive(&other) {
            Some(r) => r,
            None => self.add(other),
        }
    }
}

impl<
        D: TryAdd<D> + Clone + std::cmp::PartialEq<f32> + std::cmp::PartialOrd<f32> + std::fmt::Debug,
    > DimensionPercentage<D>
{
    fn add_recursive(&self, other: &DimensionPercentage<D>) -> Option<DimensionPercentage<D>> {
        match (self, other) {
            (DimensionPercentage::Dimension(a), DimensionPercentage::Dimension(b)) => {
                if let Some(res) = a.try_add(b) {
                    Some(DimensionPercentage::Dimension(res))
                } else {
                    None
                }
            }
            (DimensionPercentage::Percentage(a), DimensionPercentage::Percentage(b)) => {
                Some(DimensionPercentage::Percentage(Percentage(a.0 + b.0)))
            }
            (DimensionPercentage::Calc(a), other) => match &**a {
                Calc::Value(v) => v.add_recursive(other),
                Calc::Sum(a, b) => {
                    if let Some(res) =
                        DimensionPercentage::Calc(Box::new(*a.clone())).add_recursive(other)
                    {
                        return Some(res.add(DimensionPercentage::from(*b.clone())));
                    }

                    if let Some(res) =
                        DimensionPercentage::Calc(Box::new(*b.clone())).add_recursive(other)
                    {
                        return Some(DimensionPercentage::from(*a.clone()).add(res));
                    }

                    None
                }
                _ => None,
            },
            (other, DimensionPercentage::Calc(b)) => match &**b {
                Calc::Value(v) => other.add_recursive(&*v),
                Calc::Sum(a, b) => {
                    if let Some(res) =
                        other.add_recursive(&DimensionPercentage::Calc(Box::new(*a.clone())))
                    {
                        return Some(res.add(DimensionPercentage::from(*b.clone())));
                    }

                    if let Some(res) =
                        other.add_recursive(&DimensionPercentage::Calc(Box::new(*b.clone())))
                    {
                        return Some(DimensionPercentage::from(*a.clone()).add(res));
                    }

                    None
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn add(self, other: DimensionPercentage<D>) -> DimensionPercentage<D> {
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
            (DimensionPercentage::Calc(a), DimensionPercentage::Calc(b)) => {
                return DimensionPercentage::Calc(Box::new(*a + *b))
            }
            (DimensionPercentage::Calc(calc), b) => {
                if let Calc::Value(a) = *calc {
                    a.add(b)
                } else {
                    DimensionPercentage::Calc(Box::new(Calc::Sum(
                        Box::new((*calc).into()),
                        Box::new(b.into()),
                    )))
                }
            }
            (a, DimensionPercentage::Calc(calc)) => {
                if let Calc::Value(b) = *calc {
                    a.add(*b)
                } else {
                    DimensionPercentage::Calc(Box::new(Calc::Sum(
                        Box::new(a.into()),
                        Box::new((*calc).into()),
                    )))
                }
            }
            (a, b) => DimensionPercentage::Calc(Box::new(Calc::Sum(
                Box::new(a.into()),
                Box::new(b.into()),
            ))),
        }
    }
}

impl<D> std::convert::Into<Calc<DimensionPercentage<D>>> for DimensionPercentage<D> {
    fn into(self) -> Calc<DimensionPercentage<D>> {
        match self {
            DimensionPercentage::Calc(c) => *c,
            b => Calc::Value(Box::new(b)),
        }
    }
}

impl<D> std::convert::From<Calc<DimensionPercentage<D>>> for DimensionPercentage<D> {
    fn from(calc: Calc<DimensionPercentage<D>>) -> DimensionPercentage<D> {
        DimensionPercentage::Calc(Box::new(calc))
    }
}

impl<D: std::cmp::PartialEq<f32>> std::cmp::PartialEq<f32> for DimensionPercentage<D> {
    fn eq(&self, other: &f32) -> bool {
        match self {
            DimensionPercentage::Dimension(a) => *a == *other,
            DimensionPercentage::Percentage(a) => *a == *other,
            DimensionPercentage::Calc(_) => false,
        }
    }
}

impl<D: std::cmp::PartialOrd<f32>> std::cmp::PartialOrd<f32> for DimensionPercentage<D> {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        match self {
            DimensionPercentage::Dimension(a) => a.partial_cmp(other),
            DimensionPercentage::Percentage(a) => a.partial_cmp(other),
            DimensionPercentage::Calc(_) => None,
        }
    }
}

impl<D: std::cmp::PartialOrd<D>> std::cmp::PartialOrd<DimensionPercentage<D>>
    for DimensionPercentage<D>
{
    fn partial_cmp(&self, other: &DimensionPercentage<D>) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (DimensionPercentage::Dimension(a), DimensionPercentage::Dimension(b)) => {
                a.partial_cmp(b)
            }
            (DimensionPercentage::Percentage(a), DimensionPercentage::Percentage(b)) => {
                a.partial_cmp(b)
            }
            _ => None,
        }
    }
}
