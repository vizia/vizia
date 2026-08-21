use crate::{CustomParseError, Length, Parse};
use cssparser::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Filter {
    #[default]
    None,
    Blur(Length),
    List(Vec<Filter>),
}

impl Filter {
    fn parse_single<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let function = input.expect_function()?.clone();
        input.parse_nested_block(|input| {
            let location = input.current_source_location();
            match_ignore_ascii_case! { &function,
                "blur" => {
                    Ok(Filter::Blur(input.try_parse(Length::parse).unwrap_or(Length::px(0.0))))
                },
                _ => Err(location.new_unexpected_token_error(Token::Ident(function)))
            }
        })
    }

    pub fn as_list(&self) -> &[Filter] {
        match self {
            Filter::List(filters) => filters,
            _ => std::slice::from_ref(self),
        }
    }
}

impl<'i> Parse<'i> for Filter {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if input.try_parse(|i| i.expect_ident_matching("none")).is_ok() {
            return Ok(Filter::None);
        }

        let mut filters = Vec::new();
        while !input.is_exhausted() {
            filters.push(Self::parse_single(input)?);
        }
        match filters.len() {
            0 => {
                let location = input.current_source_location();
                Err(ParseError {
                    kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                    location,
                })
            }
            1 => Ok(filters.pop().unwrap()),
            _ => Ok(Filter::List(filters)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::ParserInput;

    fn parse(text: &str) -> Filter {
        let mut input = ParserInput::new(text);
        let mut parser = Parser::new(&mut input);
        Filter::parse(&mut parser).unwrap()
    }

    #[test]
    fn parses_none_single_and_filter_lists() {
        assert_eq!(parse("none"), Filter::None);
        assert_eq!(parse("blur(5px)"), Filter::Blur(Length::px(5.0)));
        assert_eq!(
            parse("blur(2px) blur(8px)"),
            Filter::List(vec![Filter::Blur(Length::px(2.0)), Filter::Blur(Length::px(8.0))])
        );
    }
}
