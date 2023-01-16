use cssparser::*;

use crate::{Gradient, Parse};

#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundImage<'i> {
    Name(CowRcStr<'i>),
    Gradient(Box<Gradient>),
}

impl<'i> Parse<'i> for BackgroundImage<'i> {
    fn parse<'t>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, crate::CustomParseError<'i>>> {
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
        let mut input = ParserInput::new(&s);
        let mut parser = Parser::new(&mut input);
        BackgroundImage::parse(&mut parser).expect("Failed to parse background-image")
    }
}
