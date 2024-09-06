use crate::{define_enum, CustomParseError, LengthOrPercentage, Parse};
use bitflags::bitflags;
use cssparser::*;

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

        Ok(Self {
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
    fn default() -> Self {
        Self::empty()
    }
}

impl<'i> Parse<'i> for TextDecorationLine {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut value = Self::empty();
        let mut any = false;

        loop {
            let flag: Result<_, ParseError<'i, CustomParseError<'i>>> = input.try_parse(|input| {
                let location = input.current_source_location();
                let ident = input.expect_ident()?;
                Ok(match_ignore_ascii_case! { &ident,
                  "none" if value.is_empty() => Self::empty(),
                  "underline" => Self::Underline,
                  "overline" => Self::Overline,
                  "strikethrough" | "line-through" => Self::Strikethrough,
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

impl From<TextDecorationLine> for skia_safe::textlayout::TextDecoration {
    fn from(value: TextDecorationLine) -> Self {
        let mut decoration = Self::empty();

        decoration.set(
            Self::UNDERLINE,
            value.contains(TextDecorationLine::Underline),
        );
        decoration.set(
            Self::OVERLINE,
            value.contains(TextDecorationLine::Overline),
        );
        decoration.set(
            Self::LINE_THROUGH,
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

impl From<TextDecorationStyle> for skia_safe::textlayout::TextDecorationStyle {
    fn from(value: TextDecorationStyle) -> Self {
        match value {
            TextDecorationStyle::Solid => Self::Solid,
            TextDecorationStyle::Dashed => Self::Dashed,
            TextDecorationStyle::Dotted => Self::Dotted,
            TextDecorationStyle::Double => Self::Double,
            TextDecorationStyle::Wavy => Self::Wavy,
        }
    }
}
