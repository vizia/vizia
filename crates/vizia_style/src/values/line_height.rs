use cssparser::{Parser, ParserInput};

use crate::{Length, Parse, Percentage, define_enum, impl_parse};

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

impl_parse! {
    LineHeight,

    try_parse {
        LineHeightKeyword,
        Percentage,
        f32,
        Length,
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
                "24px" => LineHeight::Length(Length::Value(LengthValue::Px(24.0))),
            }

            failure {
                "auto",
                "none",
                "abc",
            }
        }
    }
}
