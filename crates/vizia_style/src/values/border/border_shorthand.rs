use cssparser::{ParseError, Parser};

use crate::{BorderStyle, BorderWidthValue, Color, CustomParseError, Parse};

/// The border shorthand containing a border width, style and color.
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    /// The width of the border.
    pub width: Option<BorderWidthValue>,
    /// The style of the border.
    pub style: Option<BorderStyle>,
    /// The color of the border.
    pub color: Option<Color>,
}

impl Border {
    /// Creates a new border.
    pub fn new(
        width: Option<BorderWidthValue>,
        style: Option<BorderStyle>,
        color: Option<Color>,
    ) -> Self {
        Self { width, style, color }
    }
}

impl<'i> Parse<'i> for Border {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let mut width = None;
        let mut style = None;
        let mut color = None;
        let mut any = false;
        loop {
            let mut consumed = false;

            if width.is_none() {
                if let Ok(value) = input.try_parse(|i| BorderWidthValue::parse(i)) {
                    width = Some(value);
                    any = true;
                    consumed = true;
                }
            }
            if style.is_none() {
                if let Ok(value) = input.try_parse(BorderStyle::parse) {
                    style = Some(value);
                    any = true;
                    consumed = true;
                }
            }
            if color.is_none() {
                if let Ok(value) = input.try_parse(|i| Color::parse(i)) {
                    color = Some(value);
                    any = true;
                    consumed = true;
                }
            }

            if !consumed {
                break;
            }
        }

        if any && input.is_exhausted() {
            Ok(Border { width, style, color })
        } else {
            Err(cssparser::ParseError {
                kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                location,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BorderStyleKeyword, BorderWidthKeyword, Color, tests::assert_parse};

    assert_parse! {
        Border, assert_border,

        custom {
            success {
                "thin" => Border::new(
                    Some(BorderWidthKeyword::Thin.into()),
                    None,
                    None,
                ),
                "solid dotted" => Border::new(
                    None,
                    Some(BorderStyle::new(BorderStyleKeyword::Solid, BorderStyleKeyword::Dotted, BorderStyleKeyword::Solid, BorderStyleKeyword::Dotted)),
                    None,
                ),
                "#FF00FF" => Border::new(
                    None,
                    None,
                    Some(Color::rgb(255, 0, 255)),
                ),
                "thin solid #00FF00" => Border::new(
                    Some(BorderWidthKeyword::Thin.into()),
                    Some(BorderStyle::new(BorderStyleKeyword::Solid, BorderStyleKeyword::Solid, BorderStyleKeyword::Solid, BorderStyleKeyword::Solid)),
                    Some(Color::rgb(0, 255, 0)),
                ),
            }

            failure {
                "test",
                "123",
                "test #00FF00",
                // "#00FF00 test",
                // "thin solid #00FF00 x",
            }
        }
    }
}
