use crate::{define_enum, CustomParseError, LengthOrPercentage, Parse};
use bitflags::bitflags;
use cssparser::*;
use cssparser_color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct TextDecoration {
    pub line: TextDecorationLine,
    pub style: TextDecorationStyle,
    pub thickness: LengthOrPercentage,
    pub color: Color,
}

impl<'i> Parse<'i> for TextDecoration {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut line = None;
        let mut style = None;
        let mut thickness = None;
        let mut color = None;

        loop {
            if line.is_none() {
                if let Ok(value) = input.try_parse(TextDecorationLine::parse) {
                    line = Some(value);
                    continue;
                }
            }

            if style.is_none() {
                if let Ok(value) = input.try_parse(TextDecorationStyle::parse) {
                    style = Some(value);
                    continue;
                }
            }

            if thickness.is_none() {
                if let Ok(value) = input.try_parse(LengthOrPercentage::parse) {
                    thickness = Some(value);
                    continue;
                }
            }

            if color.is_none() {
                if let Ok(value) = input.try_parse(Color::parse) {
                    color = Some(value);
                    continue;
                }
            }

            break;
        }

        Ok(TextDecoration {
            line: line.unwrap_or_default(),
            style: style.unwrap_or_default(),
            thickness: thickness.unwrap_or_default(),
            color: color.unwrap_or(Color::CurrentColor),
        })
    }
}

bitflags! {
  #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
  pub struct TextDecorationLine: u8 {
    /// Each line of text is underlined.
    const Underline     = 0b00000001;
    /// Each line of text has a line over it.
    const Overline      = 0b00000010;
    /// Each line of text has a line through the middle.
    const Strikethrough = 0b00000100;
  }
}

impl Default for TextDecorationLine {
    fn default() -> TextDecorationLine {
        TextDecorationLine::empty()
    }
}

impl<'i> Parse<'i> for TextDecorationLine {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut value = TextDecorationLine::empty();
        let mut any = false;

        loop {
            let flag: Result<_, ParseError<'i, CustomParseError<'i>>> = input.try_parse(|input| {
                let location = input.current_source_location();
                let ident = input.expect_ident()?;
                Ok(match_ignore_ascii_case! { &ident,
                  "none" if value.is_empty() => TextDecorationLine::empty(),
                  "underline" => TextDecorationLine::Underline,
                  "overline" => TextDecorationLine::Overline,
                  "strikethrough" | "line-through" => TextDecorationLine::Strikethrough,
                  _ => return Err(location.new_unexpected_token_error(
                    cssparser::Token::Ident(ident.clone())
                  ))
                })
            });

            if let Ok(flag) = flag {
                value |= flag;
                any = true;
            } else {
                break;
            }
        }

        if !any {
            return Err(input.new_custom_error(CustomParseError::InvalidDeclaration));
        }

        Ok(value)
    }
}

impl From<TextDecorationLine> for vizia_render::text::TextDecoration {
    fn from(value: TextDecorationLine) -> Self {
        let mut decoration = vizia_render::text::TextDecoration::empty();

        decoration.set(
            vizia_render::text::TextDecoration::UNDERLINE,
            value.contains(TextDecorationLine::Underline),
        );
        decoration.set(
            vizia_render::text::TextDecoration::OVERLINE,
            value.contains(TextDecorationLine::Overline),
        );
        decoration.set(
            vizia_render::text::TextDecoration::LINE_THROUGH,
            value.contains(TextDecorationLine::Strikethrough),
        );

        decoration
    }
}

define_enum! {
    #[derive(Default)]
    pub enum TextDecorationStyle {
        /// A single line segment.
        #[default]
        "solid": Solid,
        /// Two parallel solid lines with some space between them.
        "double": Double,
        /// A series of round dots.
        "dotted": Dotted,
        /// A series of square-ended dashes.
        "dashed": Dashed,
        /// A wavy line.
        "wavy": Wavy,
    }
}

impl From<TextDecorationStyle> for vizia_render::text::TextDecorationStyle {
    fn from(value: TextDecorationStyle) -> Self {
        match value {
            TextDecorationStyle::Solid => vizia_render::text::TextDecorationStyle::Solid,
            TextDecorationStyle::Dashed => vizia_render::text::TextDecorationStyle::Dashed,
            TextDecorationStyle::Dotted => vizia_render::text::TextDecorationStyle::Dotted,
            TextDecorationStyle::Double => vizia_render::text::TextDecorationStyle::Double,
            TextDecorationStyle::Wavy => vizia_render::text::TextDecorationStyle::Wavy,
        }
    }
}
