use crate::{AutoKeyword, Length, LengthOrPercentage, Parse, Rect};
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

impl From<Rect<LengthOrPercentage>> for ClipPath {
    fn from(value: Rect<LengthOrPercentage>) -> Self {
        ClipPath::Shape(value)
    }
}

impl From<LengthOrPercentage> for ClipPath {
    fn from(value: LengthOrPercentage) -> Self {
        ClipPath::Shape(Rect(value.clone(), value.clone(), value.clone(), value))
    }
}

impl From<Length> for ClipPath {
    fn from(value: Length) -> Self {
        ClipPath::Shape(Rect(
            value.clone().into(),
            value.clone().into(),
            value.clone().into(),
            value.into(),
        ))
    }
}

impl<'i> Parse<'i> for ClipPath {
    fn parse<'t>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, crate::CustomParseError<'i>>> {
        if input.try_parse(AutoKeyword::parse).is_ok() {
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
