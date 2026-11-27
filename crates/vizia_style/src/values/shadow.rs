use std::fmt::Write;

use crate::{Color, CustomParseError, InsetKeyword, Length, Parse, ParserErrorExt, TokenOrValue};
use cssparser::{ParseError, Parser, ToCss};

/// A box shadow adding a shadow effect around an element's frame.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Shadow {
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

impl Shadow {
    /// Creates a new shadow.
    pub fn new(
        x_offset: impl Into<Length>,
        y_offset: impl Into<Length>,
        blur_radius: Option<Length>,
        spread_radius: Option<Length>,
        color: Option<Color>,
        inset: bool,
    ) -> Self {
        Self {
            x_offset: x_offset.into(),
            y_offset: y_offset.into(),
            blur_radius,
            spread_radius,
            color,
            inset,
        }
    }
}

impl<'i> Parse<'i> for Shadow {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        let mut lengths = Vec::new();
        let mut color = None;
        let mut inset = false;

        while !input.is_exhausted() {
            if !inset && input.try_parse(InsetKeyword::parse).is_ok() {
                inset = true;
            } else if color.is_none()
                && let Ok(value) = input.try_parse(Color::parse)
            {
                color = Some(value);
            } else if lengths.len() < 4 {
                let length = input
                    .try_parse(|input| {
                        let number = input.expect_number()?;
                        if number == 0.0 {
                            Ok(Length::zero())
                        } else {
                            Err(input.new_custom_error(CustomParseError::InvalidDeclaration))
                        }
                    })
                    .or_else(|_| Length::parse(input))?;
                lengths.push(length);
            } else {
                return Err(ParseError::custom(CustomParseError::InvalidDeclaration));
            }
        }

        let mut lengths = lengths.into_iter();
        let x_offset = lengths
            .next()
            .ok_or_else(|| ParseError::custom(CustomParseError::InvalidDeclaration))?;
        let y_offset = lengths
            .next()
            .ok_or_else(|| ParseError::custom(CustomParseError::InvalidDeclaration))?;
        Ok(Shadow::new(x_offset, y_offset, lengths.next(), lengths.next(), color, inset))
    }
}

impl<'i> Parse<'i> for Vec<Shadow> {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        if input
            .try_parse(|input| {
                input.expect_ident_matching("none")?;
                input.expect_exhausted()
            })
            .is_ok()
        {
            return Ok(Vec::new());
        }
        input.parse_comma_separated(Shadow::parse)
    }
}

/// Parses a shadow list from custom-property tokens, using the normal shadow grammar.
pub fn parse_shadow_tokens(tokens: &[TokenOrValue<'_>]) -> Option<Vec<Shadow>> {
    let mut css = String::new();
    for token in tokens {
        match token {
            TokenOrValue::Token(token) => token.to_css(&mut css).ok()?,
            TokenOrValue::Color(Color::CurrentColor) => css.push_str(" currentcolor "),
            TokenOrValue::Color(Color::RGBA(color)) => {
                write!(
                    css,
                    " #{:02x}{:02x}{:02x}{:02x} ",
                    color.r(),
                    color.g(),
                    color.b(),
                    color.a()
                )
                .ok()?;
            }
            TokenOrValue::Var(_) => return None,
        }
    }

    let mut parser = Parser::new(&css);
    let shadows = Vec::<Shadow>::parse(&mut parser).ok()?;
    if parser.is_exhausted() { Some(shadows) } else { None }
}

impl From<&str> for Shadow {
    fn from(s: &str) -> Self {
        let mut parser = Parser::new(s);
        Shadow::parse(&mut parser).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    #[test]
    fn standard_box_shadow_forms_and_none() {
        let expected = vec![Shadow::new(
            Length::px(0.0),
            Length::px(2.0),
            Some(Length::px(6.0)),
            None,
            Some(Color::rgb(255, 0, 0)),
            true,
        )];
        for value in ["inset 0 2px 6px red", "red 0 2px 6px inset", "0 2px 6px red inset"] {
            let mut parser = Parser::new(value);
            assert_eq!(Vec::<Shadow>::parse(&mut parser).unwrap(), expected, "{value}");
            assert!(parser.is_exhausted(), "{value}");
        }

        assert!(Vec::<Shadow>::parse(&mut Parser::new("none")).unwrap().is_empty());
        let mut parser = Parser::new("red 0 2px, inset 0 0 #000");
        let shadows = Vec::<Shadow>::parse(&mut parser).unwrap();
        assert_eq!(shadows.len(), 2);
        assert_eq!(shadows[0].color, Some(Color::rgb(255, 0, 0)));
        assert!(shadows[1].inset);
    }

    #[test]
    fn standard_box_shadow_rejects_invalid_lists() {
        for value in [
            "none, 0 2px red",
            "0 2px, none",
            "none 0 2px",
            "red blue 0 2px",
            "inset inset 0 2px",
            "0 2px 3px 4px 5px",
            "1 2px",
            "0",
            "0 2px,",
        ] {
            assert!(Vec::<Shadow>::parse(&mut Parser::new(value)).is_err(), "{value}");
        }
    }

    #[test]
    fn shadow_custom_tokens_preserve_calc_operators() {
        use crate::CustomProperty;

        for value in [
            "calc(1px + 2px) 0 red",
            "calc(5px - 2px) calc(2px + -1px) #f00",
            "calc(calc(1px + 2px) * 2) 0 red, inset 0 2px blue",
        ] {
            let custom = CustomProperty::parse("--shadow".into(), &mut Parser::new(value)).unwrap();
            let literal = Vec::<Shadow>::parse(&mut Parser::new(value)).unwrap();
            assert_eq!(parse_shadow_tokens(&custom.value.0), Some(literal), "{value}");
        }
        for invalid in ["calc(1px+2px) 0 red", "calc(1px+ 2px) 0 red", "calc(1px +2px) 0 red"] {
            let custom =
                CustomProperty::parse("--shadow".into(), &mut Parser::new(invalid)).unwrap();
            assert!(Vec::<Shadow>::parse(&mut Parser::new(invalid)).is_err(), "{invalid}");
            assert!(parse_shadow_tokens(&custom.value.0).is_none(), "{invalid}");
        }
    }

    assert_parse! {
        Shadow, parse_shadow,

        custom {
            success {
                "10px 20px" => Shadow::new(
                    Length::px(10.0),
                    Length::px(20.0),
                    None,
                    None,
                    None,
                    false,
                ),
                "10px 20px 30px 40px red inset" => Shadow::new(
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
        Vec<Shadow>, parse_vec_shadow,

        custom {
            success {
                "10px 20px, 10px 20px 30px 40px red inset" => vec![
                    Shadow::new(
                        Length::px(10.0),
                        Length::px(20.0),
                        None,
                        None,
                        None,
                        false,
                    ),
                    Shadow::new(
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
