use crate::error::CustomParseError;
use crate::traits::Parse;
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Url<'a> {
    pub url: CowRcStr<'a>,
}

impl<'i> Parse<'i> for Url<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let url = input.expect_url()?;
        Ok(Url { url })
    }
}
