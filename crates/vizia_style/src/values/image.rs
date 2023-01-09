use cssparser::CowRcStr;

use crate::{Gradient, Parse};

#[derive(Debug, Clone, PartialEq)]
pub enum Image<'i> {
    Name(CowRcStr<'i>),
    Gradient(Box<Gradient>),
}

impl<'i> Parse<'i> for Image<'i> {
    fn parse<'t>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, crate::CustomParseError<'i>>> {
        if let Ok(gradient) = input.try_parse(Gradient::parse) {
            return Ok(Image::Gradient(Box::new(gradient)));
        }

        Err(input.new_error_for_next_token())
    }
}
