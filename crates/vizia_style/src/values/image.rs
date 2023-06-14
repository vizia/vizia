use cssparser::*;

use crate::{CustomParseError, Gradient, Parse, Url};

#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundImage<'i> {
    None,
    Url(Url<'i>),
    Gradient(Box<Gradient>),
}

impl<'i> BackgroundImage<'i> {
    pub fn is_gradient(&self) -> bool {
        matches!(self, BackgroundImage::Gradient(_))
    }
}

impl<'i> Parse<'i> for BackgroundImage<'i> {
    fn parse<'t>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, crate::CustomParseError<'i>>> {
        if input.try_parse(|i| i.expect_ident_matching("none")).is_ok() {
            return Ok(BackgroundImage::None);
        }

        if let Ok(url) = input.try_parse(Url::parse) {
            return Ok(BackgroundImage::Url(url));
        }

        if let Ok(gradient) = input.try_parse(Gradient::parse) {
            return Ok(BackgroundImage::Gradient(Box::new(gradient)));
        }

        Err(input.new_error_for_next_token())
    }
}

impl<'i> From<Gradient> for BackgroundImage<'i> {
    fn from(gradient: Gradient) -> Self {
        BackgroundImage::Gradient(Box::new(gradient))
    }
}

impl<'i> From<&'i str> for BackgroundImage<'i> {
    fn from(s: &'i str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        BackgroundImage::parse(&mut parser).expect("Failed to parse background-image")
    }
}

impl<'i> Parse<'i> for Vec<BackgroundImage<'i>> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(BackgroundImage::parse)
    }
}
