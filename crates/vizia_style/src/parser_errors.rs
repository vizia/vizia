use cssparser::{ParseError, Parser, SourceLocation, Token};

use crate::CustomParseError;

pub(crate) trait ParserErrorExt<'i> {
    fn new_unexpected_token_error(&self, token: Token<'i>) -> ParseError<CustomParseError<'i>>;

    fn new_custom_error<E>(&self, error: E) -> ParseError<CustomParseError<'i>>
    where
        E: Into<CustomParseError<'i>>;
}

impl<'i> ParserErrorExt<'i> for Parser<'i> {
    fn new_unexpected_token_error(&self, token: Token<'i>) -> ParseError<CustomParseError<'i>> {
        ParseError::custom(CustomParseError::UnexpectedToken(Some(token)))
    }

    fn new_custom_error<E>(&self, error: E) -> ParseError<CustomParseError<'i>>
    where
        E: Into<CustomParseError<'i>>,
    {
        ParseError::custom(error.into())
    }
}

impl<'i> ParserErrorExt<'i> for SourceLocation {
    fn new_unexpected_token_error(&self, token: Token<'i>) -> ParseError<CustomParseError<'i>> {
        ParseError::custom(CustomParseError::UnexpectedToken(Some(token)))
    }

    fn new_custom_error<E>(&self, error: E) -> ParseError<CustomParseError<'i>>
    where
        E: Into<CustomParseError<'i>>,
    {
        ParseError::custom(error.into())
    }
}
