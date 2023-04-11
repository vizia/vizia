use crate::{CustomParseError, LengthPercentageOrAuto, Parse};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundSize {
    Explicit { width: LengthPercentageOrAuto, height: LengthPercentageOrAuto },

    Cover,

    Contain,
}

impl Default for BackgroundSize {
    fn default() -> BackgroundSize {
        BackgroundSize::Explicit {
            width: LengthPercentageOrAuto::Auto,
            height: LengthPercentageOrAuto::Auto,
        }
    }
}

impl<'i> Parse<'i> for BackgroundSize {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if let Ok(width) = input.try_parse(LengthPercentageOrAuto::parse) {
            let height = input
                .try_parse(LengthPercentageOrAuto::parse)
                .unwrap_or(LengthPercentageOrAuto::Auto);
            return Ok(BackgroundSize::Explicit { width, height });
        }

        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
          "cover" => BackgroundSize::Cover,
          "contain" => BackgroundSize::Contain,
          _ => return Err(location.new_unexpected_token_error(
            cssparser::Token::Ident(ident.clone())
          ))
        })
    }
}

impl<'i> Parse<'i> for Vec<BackgroundSize> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(BackgroundSize::parse)
    }
}
