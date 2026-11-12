use crate::CustomParseError;
use cssparser::*;

pub trait Parse<'i>: Sized {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>>;
}

pub trait TryAdd<T> {
    fn try_add(&self, other: &T) -> Option<T>;
}
