use cssparser::*;

use crate::{CustomParseError, Gradient, NoneKeyword, Parse, Url};

#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundImage<'i> {
    None,
    Url(Url<'i>),
    Gradient(Box<Gradient>),
}

impl BackgroundImage<'_> {
    pub fn is_gradient(&self) -> bool {
        matches!(self, BackgroundImage::Gradient(_))
    }
}

impl<'i> Parse<'i> for BackgroundImage<'i> {
    fn parse<'t>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, crate::CustomParseError<'i>>> {
        if input.try_parse(NoneKeyword::parse).is_ok() {
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

impl From<Gradient> for BackgroundImage<'_> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_background_image(
        input: &str,
    ) -> Result<BackgroundImage<'_>, ParseError<'_, CustomParseError<'_>>> {
        let mut parser_input = ParserInput::new(input);
        let mut parser = Parser::new(&mut parser_input);
        BackgroundImage::parse(&mut parser)
    }

    #[test]
    fn parses_none_keyword() {
        assert_eq!(parse_background_image("none"), Ok(BackgroundImage::None));
    }

    #[test]
    fn parses_url_value() {
        let parsed = parse_background_image("url(\"foo.png\")");
        assert!(matches!(parsed, Ok(BackgroundImage::Url(_))));
    }
}
