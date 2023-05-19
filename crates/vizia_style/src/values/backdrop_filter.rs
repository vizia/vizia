use crate::{CustomParseError, Length, Parse};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    Blur(Length),
}

impl Default for Filter {
    fn default() -> Self {
        Filter::Blur(Length::default())
    }
}

impl<'i> Parse<'i> for Filter {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
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
