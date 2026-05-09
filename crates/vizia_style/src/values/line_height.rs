use cssparser::{ParseError, ParseErrorKind, Parser, ParserInput};

use crate::{CustomParseError, Length, Parse, Percentage, define_enum};

define_enum! {
    #[derive(Default)]
    pub enum LineHeightKeyword {
        #[default]
        "normal": Normal,
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LineHeight {
    #[default]
    Normal,
    Number(f32),
    Percentage(f32),
    Length(Length),
}

impl<'i> Parse<'i> for LineHeight {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();

        if let Ok(keyword) = input.try_parse(LineHeightKeyword::parse) {
            return Ok(keyword.into());
        }

        if let Ok(percentage) = input.try_parse(Percentage::parse) {
            if percentage.0 < 0.0 {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                    location,
                });
            }

            return Ok(LineHeight::Percentage(percentage.0));
        }

        if let Ok(number) = input.try_parse(f32::parse) {
            if number < 0.0 {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                    location,
                });
            }

            return Ok(LineHeight::Number(number));
        }

        let length = input.try_parse(Length::parse)?;
        if let Length::Value(length_value) = &length {
            let (value, _) = length_value.to_unit_value();
            if value < 0.0 {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                    location,
                });
            }
        }

        Ok(LineHeight::Length(length))
    }
}

impl From<LineHeightKeyword> for LineHeight {
    fn from(keyword: LineHeightKeyword) -> Self {
        match keyword {
            LineHeightKeyword::Normal => LineHeight::Normal,
        }
    }
}

impl From<Percentage> for LineHeight {
    fn from(percentage: Percentage) -> Self {
        LineHeight::Percentage(percentage.0)
    }
}

impl From<f32> for LineHeight {
    fn from(number: f32) -> Self {
        LineHeight::Number(number)
    }
}

impl From<f64> for LineHeight {
    fn from(number: f64) -> Self {
        LineHeight::Number(number as f32)
    }
}

impl From<Length> for LineHeight {
    fn from(length: Length) -> Self {
        LineHeight::Length(length)
    }
}

impl From<&str> for LineHeight {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        LineHeight::parse(&mut parser).unwrap_or_default()
    }
}

#[cfg(test)]
mod line_height_tests {
    use super::*;
    use crate::{LengthValue, tests::assert_parse};

    assert_parse! {
        LineHeight, parse_line_height,

        number {
            LineHeight::Number,
        }

        percentage {
            LineHeight::Percentage,
        }

        custom {
            success {
                "normal" => LineHeight::Normal,
                "0" => LineHeight::Number(0.0),
                "24px" => LineHeight::Length(Length::Value(LengthValue::Px(24.0))),
            }

            failure {
                "auto",
                "none",
                "abc",
                "-1",
                "-10%",
                "-2px",
            }
        }
    }
}
