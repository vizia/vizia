use crate::{Color, CustomParseError, InsetKeyword, Length, Parse};
use cssparser::{ParseError, Parser};

/// A box shadow adding a shadow effect around an element's frame.
#[derive(Debug, Clone, PartialEq)]
pub struct BoxShadow {
    /// The horizontal offset of the box shadow.
    pub x_offset: Length,
    /// The vertical offset of the box shadow.
    pub y_offset: Length,
    /// The blur radius used for making the box shadow bigger and lighter.
    pub blur_radius: Option<Length>,
    /// The spread radius used for expanding and growing the box shadow.
    pub spread_radius: Option<Length>,
    /// The color of the box shadow.
    pub color: Option<Color>,
    /// Determines if the box shadow should be an outer shadow (outset) or an inner shadow (inset).
    pub inset: bool,
}

impl BoxShadow {
    /// Creates a new box shadow.
    pub fn new(
        x_offset: Length,
        y_offset: Length,
        blur_radius: Option<Length>,
        spread_radius: Option<Length>,
        color: Option<Color>,
        inset: bool,
    ) -> Self {
        Self {
            x_offset,
            y_offset,
            blur_radius,
            spread_radius,
            color,
            inset,
        }
    }
}

impl<'i> Parse<'i> for BoxShadow {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let x_offset = Length::parse(input)?;
        let y_offset = Length::parse(input)?;
        let blur_radius = input.try_parse(Length::parse).ok();
        let spread_radius = input.try_parse(Length::parse).ok();
        let color = input.try_parse(Color::parse).ok();
        let inset = input
            .try_parse(InsetKeyword::parse)
            .map(|_| true)
            .unwrap_or(false);

        Ok(BoxShadow::new(
            x_offset,
            y_offset,
            blur_radius,
            spread_radius,
            color,
            inset,
        ))
    }
}

impl<'i> Parse<'i> for Vec<BoxShadow> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(BoxShadow::parse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        BoxShadow, parse_box_shadow,

        custom {
            success {
                "10px 20px" => BoxShadow::new(
                    Length::px(10.0),
                    Length::px(20.0),
                    None,
                    None,
                    None,
                    false,
                ),
                "10px 20px 30px 40px red inset" => BoxShadow::new(
                    Length::px(10.0),
                    Length::px(20.0),
                    Some(Length::px(30.0)),
                    Some(Length::px(40.0)),
                    Some(Color::rgb(255, 0, 0)),
                    true,
                ),
            }

            failure {
                "test",
                "123",
            }
        }
    }

    assert_parse! {
        Vec<BoxShadow>, parse_vec_box_shadow,

        custom {
            success {
                "10px 20px, 10px 20px 30px 40px red inset" => vec![
                    BoxShadow::new(
                        Length::px(10.0),
                        Length::px(20.0),
                        None,
                        None,
                        None,
                        false,
                    ),
                    BoxShadow::new(
                        Length::px(10.0),
                        Length::px(20.0),
                        Some(Length::px(30.0)),
                        Some(Length::px(40.0)),
                        Some(Color::rgb(255, 0, 0)),
                        true,
                    ),
                ],
            }

            failure {
                "10px, 10px, 10px",
                "test",
                "123",
            }
        }
    }
}
