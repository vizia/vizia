use cssparser::{ParseError, Parser};

use crate::{CustomParseError, Length, Parse, define_enum};

define_enum! {
    #[derive(Default)]
    pub enum LetterSpacingKeyword {
        #[default]
        "normal": Normal,
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LetterSpacing {
    #[default]
    Normal,
    Length(Length),
}

impl<'i> Parse<'i> for LetterSpacing {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        if let Ok(keyword) = input.try_parse(LetterSpacingKeyword::parse) {
            return Ok(keyword.into());
        }

        let length = input.try_parse(Length::parse)?;
        if let Length::Value(length_value) = &length {
            let (value, _) = length_value.to_unit_value();
            if value == 0.0 {
                return Ok(LetterSpacing::Length(length));
            }
        }

        match length {
            Length::Value(_) => Ok(LetterSpacing::Length(length)),
            _ => Err(ParseError::custom(CustomParseError::InvalidValue)),
        }
    }
}

impl From<LetterSpacingKeyword> for LetterSpacing {
    fn from(keyword: LetterSpacingKeyword) -> Self {
        match keyword {
            LetterSpacingKeyword::Normal => LetterSpacing::Normal,
        }
    }
}

impl From<Length> for LetterSpacing {
    fn from(length: Length) -> Self {
        LetterSpacing::Length(length)
    }
}

impl From<&str> for LetterSpacing {
    fn from(s: &str) -> Self {
        let mut parser = Parser::new(s);
        LetterSpacing::parse(&mut parser).unwrap_or_default()
    }
}

#[cfg(test)]
mod letter_spacing_tests {
    use super::*;
    use crate::{LengthValue, tests::assert_parse};

    assert_parse! {
        LetterSpacing, parse_letter_spacing,

        custom {
            success {
                "normal" => LetterSpacing::Normal,
                "1px" => LetterSpacing::Length(Length::Value(LengthValue::Px(1.0))),
                "0" => LetterSpacing::Length(Length::Value(LengthValue::Px(0.0))),
                "-0.4px" => LetterSpacing::Length(Length::Value(LengthValue::Px(-0.4))),
            }

            failure {
                "auto",
                "none",
                "25%",
            }
        }
    }
}
