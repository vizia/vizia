use crate::{CustomParseError, Length, NoneKeyword, Parse};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    /// No filter applied.
    None,
    Blur(Length),
}

impl Default for Filter {
    fn default() -> Self {
        Filter::None
    }
}

impl<'i> Parse<'i> for Filter {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if input.try_parse(NoneKeyword::parse).is_ok() {
            return Ok(Filter::None);
        }

        let function = input.expect_function()?.clone();

        input.parse_nested_block(|input| {
            let location = input.current_source_location();
            match_ignore_ascii_case! { &function,
                "blur" => {
                    Ok(Filter::Blur(input.try_parse(Length::parse).unwrap_or(Length::px(0.0))))
                },

                _ => {
                    Err(location.new_unexpected_token_error(Token::Ident(function)))
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parse;
    use cssparser::{Parser, ParserInput};

    fn parse_filter(
        input: &str,
    ) -> Result<Filter, cssparser::ParseError<'_, crate::CustomParseError<'_>>> {
        let mut pi = ParserInput::new(input);
        let mut p = Parser::new(&mut pi);
        Filter::parse(&mut p)
    }

    #[test]
    fn parses_none_keyword() {
        assert_eq!(parse_filter("none"), Ok(Filter::None));
    }

    #[test]
    fn parses_blur_function() {
        assert!(matches!(parse_filter("blur(4px)"), Ok(Filter::Blur(_))));
    }
}
