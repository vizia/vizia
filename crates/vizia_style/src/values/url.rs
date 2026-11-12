use crate::error::CustomParseError;
use crate::traits::Parse;
use cssparser::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Url<'a> {
    pub url: CowRcStr<'a>,
}

impl<'i> Parse<'i> for Url<'i> {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        let url = input.expect_url_or_string()?;
        Ok(Url { url })
    }
}

impl<'a> From<&'a str> for Url<'a> {
    fn from(s: &'a str) -> Self {
        let mut parser = Parser::new(s);
        Url::parse(&mut parser).unwrap_or_default()
    }
}
