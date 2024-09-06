use cssparser::*;
use skia_safe::font_style::Weight;

use crate::{macros::impl_parse, FontWeightKeyword, Parse};

/// A font weight value.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FontWeight(pub u16);

impl_parse! {
    FontWeight,

    try_parse {
        FontWeightKeyword,
        u16,
    }
}

impl From<FontWeightKeyword> for FontWeight {
    fn from(font_weight_keyword: FontWeightKeyword) -> Self {
        match font_weight_keyword {
            FontWeightKeyword::Thin => Self(100),
            FontWeightKeyword::Hairline => Self(100),
            FontWeightKeyword::ExtraLight => Self(200),
            FontWeightKeyword::UltraLight => Self(200),
            FontWeightKeyword::Light => Self(300),
            FontWeightKeyword::Normal => Self(400),
            FontWeightKeyword::Regular => Self(400),
            FontWeightKeyword::Medium => Self(500),
            FontWeightKeyword::SemiBold => Self(600),
            FontWeightKeyword::DemiBold => Self(600),
            FontWeightKeyword::Bold => Self(700),
            FontWeightKeyword::ExtraBold => Self(800),
            FontWeightKeyword::UltraBold => Self(800),
            FontWeightKeyword::Black => Self(900),
            FontWeightKeyword::Heavy => Self(900),
            FontWeightKeyword::ExtraBlack => Self(950),
            FontWeightKeyword::UltraBlack => Self(950),
        }
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        Self(400)
    }
}

impl From<u16> for FontWeight {
    fn from(number: u16) -> Self {
        Self(number)
    }
}

impl From<u32> for FontWeight {
    fn from(number: u32) -> Self {
        Self(number as u16)
    }
}

impl From<i32> for FontWeight {
    fn from(number: i32) -> Self {
        Self(number as u16)
    }
}

impl From<&str> for FontWeight {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        Self::parse(&mut parser).unwrap_or_default()
    }
}

impl From<FontWeight> for u16 {
    fn from(font_weight: FontWeight) -> Self {
        font_weight.0
    }
}

impl From<FontWeight> for Weight {
    fn from(value: FontWeight) -> Self {
        Self::from(value.0 as i32)
    }
}
