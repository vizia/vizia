use crate::{AutoKeyword, LengthOrPercentage, Parse, Rect};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ClipPath {
    Auto,
    Shape(Rect<LengthOrPercentage>),
}

impl Default for ClipPath {
    fn default() -> Self {
        ClipPath::Auto
    }
}

impl<'i> Parse<'i> for ClipPath {
    fn parse<'t>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, crate::CustomParseError<'i>>> {
        if let Ok(_) = input.try_parse(AutoKeyword::parse) {
            Ok(ClipPath::Auto)
        } else {
            let function = input.expect_function()?.clone();

            input.parse_nested_block(|input| {
                let location = input.current_source_location();
                match_ignore_ascii_case! { &function,
                    "inset" | "rect" => {
                        let rect = Rect::parse(input)?;
                        Ok(ClipPath::Shape(rect))
                    },

                    _ => {
                        Err(location.new_unexpected_token_error(Token::Ident(function)))
                    }
                }
            })
        }
    }
}
