use cssparser::{ParseError, Parser};

use crate::{BorderColor, BorderStyle, BorderWidth, CustomParseError, Parse};

/// The border shorthand containing a border width, style and color.
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    /// The width of the border.
    pub width: Option<BorderWidth>,
    /// The style of the border.
    pub style: Option<BorderStyle>,
    /// The color of the border.
    pub color: Option<BorderColor>,
}

impl Border {
    /// Creates a new border.
    pub fn new(
        width: Option<BorderWidth>,
        style: Option<BorderStyle>,
        color: Option<BorderColor>,
    ) -> Self {
        Self {
            width,
            style,
            color,
        }
    }
}

impl<'i> Parse<'i> for Border {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let width = input.try_parse(BorderWidth::parse).ok();
        let style = input.try_parse(BorderStyle::parse).ok();
        let color = input.try_parse(BorderColor::parse).ok();

        if (width.is_some() || style.is_some() || color.is_some()) && input.is_exhausted() {
            Ok(Border::new(width, style, color))
        } else {
            return Err(cssparser::ParseError {
                kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                location,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::assert_parse, BorderStyleKeyword, Color, Length};

    assert_parse! {
        Border, assert_border,

        custom {
            success {
                "thin thick" => Border::new(
                    Some(BorderWidth::new(Length::px(1.0).into(), Length::px(5.0).into(), Length::px(1.0).into(), Length::px(5.0).into())),
                    None,
                    None,
                ),
                "solid dotted" => Border::new(
                    None,
                    Some(BorderStyle::new(BorderStyleKeyword::Solid, BorderStyleKeyword::Dotted, BorderStyleKeyword::Solid, BorderStyleKeyword::Dotted)),
                    None,
                ),
                "#00FF00 #FF00FF" => Border::new(
                    None,
                    None,
                    Some(BorderColor::new(Color::rgb(0, 255, 0), Color::rgb(255, 0, 255), Color::rgb(0, 255, 0), Color::rgb(255, 0, 255))),
                ),
                "thin thick solid dotted #00FF00 #FF00FF" => Border::new(
                    Some(BorderWidth::new(Length::px(1.0).into(), Length::px(5.0).into(), Length::px(1.0).into(), Length::px(5.0).into())),
                    Some(BorderStyle::new(BorderStyleKeyword::Solid, BorderStyleKeyword::Dotted, BorderStyleKeyword::Solid, BorderStyleKeyword::Dotted)),
                    Some(BorderColor::new(Color::rgb(0, 255, 0), Color::rgb(255, 0, 255), Color::rgb(0, 255, 0), Color::rgb(255, 0, 255))),
                ),
            }

            failure {
                "test",
                "123",
                "test #00FF00",
                "#00FF00 test",
                "thin solid #00FF00 x",
            }
        }
    }
}
