use crate::{AutoKeyword, Length, Parse, Rect};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Clip {
    Auto,
    Shape(Rect<Length>),
}

impl Default for Clip {
    fn default() -> Self {
        Clip::Auto
    }
}

impl<'i> Parse<'i> for Clip {
    fn parse<'t>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, crate::CustomParseError<'i>>> {
        if let Ok(_) = input.try_parse(AutoKeyword::parse) {
            Ok(Clip::Auto)
        } else {
            let function = input.expect_function()?.clone();

            input.parse_nested_block(|input| {
                let location = input.current_source_location();
                match_ignore_ascii_case! { &function,
                    "rect" => {
                        let top = Length::parse(input)?;
                        input.expect_comma()?;
                        let right = Length::parse(input)?;
                        input.expect_comma()?;
                        let bottom = Length::parse(input)?;
                        input.expect_comma()?;
                        let left = Length::parse(input)?;
                        return Ok(Clip::Shape(Rect(top, right, bottom, left)));
                    },

                    _ => {
                        Err(location.new_unexpected_token_error(Token::Ident(function)))
                    }
                }
            })
        }
    }
}
