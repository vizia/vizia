use crate::{Color, CustomParseError, NoneKeyword, Parse};
use cssparser::{ParseError, Parser};

/// Parsed value for the `fill` property.
///
/// The CSS keyword `none` is treated as fully transparent.
#[derive(Debug, Clone, PartialEq)]
pub struct FillColor(pub Color);

impl<'i> Parse<'i> for FillColor {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if input.try_parse(NoneKeyword::parse).is_ok() {
            return Ok(FillColor(Color::transparent()));
        }

        Color::parse(input).map(FillColor)
    }
}

impl From<FillColor> for Color {
    fn from(fill: FillColor) -> Self {
        fill.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::{Parser, ParserInput};

    fn parse_fill(
        input: &str,
    ) -> Result<FillColor, cssparser::ParseError<'_, crate::CustomParseError<'_>>> {
        let mut pi = ParserInput::new(input);
        let mut p = Parser::new(&mut pi);
        FillColor::parse(&mut p)
    }

    #[test]
    fn parses_none_as_transparent() {
        let result = parse_fill("none").unwrap();
        assert_eq!(result, FillColor(Color::transparent()));
    }

    #[test]
    fn parses_named_color() {
        let result = parse_fill("red").unwrap();
        assert_eq!(result, FillColor(Color::rgb(255, 0, 0)));
    }
}
