use crate::CustomParseError;
use cssparser::*;

pub trait Parse<'i>: Sized {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>;
}

pub trait TryAdd<T> {
    fn try_add(&self, other: &T) -> Option<T>;
}
