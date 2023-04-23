use cssparser::*;

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
            FontWeightKeyword::Thin => FontWeight(100),
            FontWeightKeyword::Hairline => FontWeight(100),
            FontWeightKeyword::ExtraLight => FontWeight(200),
            FontWeightKeyword::UltraLight => FontWeight(200),
            FontWeightKeyword::Light => FontWeight(300),
            FontWeightKeyword::Normal => FontWeight(400),
            FontWeightKeyword::Regular => FontWeight(400),
            FontWeightKeyword::Medium => FontWeight(500),
            FontWeightKeyword::SemiBold => FontWeight(600),
            FontWeightKeyword::DemiBold => FontWeight(600),
            FontWeightKeyword::Bold => FontWeight(700),
            FontWeightKeyword::ExtraBold => FontWeight(800),
            FontWeightKeyword::UltraBold => FontWeight(800),
            FontWeightKeyword::Black => FontWeight(900),
            FontWeightKeyword::Heavy => FontWeight(900),
            FontWeightKeyword::ExtraBlack => FontWeight(950),
            FontWeightKeyword::UltraBlack => FontWeight(950),
        }
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight(400)
    }
}

impl From<u16> for FontWeight {
    fn from(number: u16) -> Self {
        FontWeight(number)
    }
}

impl From<u32> for FontWeight {
    fn from(number: u32) -> Self {
        FontWeight(number as u16)
    }
}

impl From<&str> for FontWeight {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        FontWeight::parse(&mut parser).unwrap_or_default()
    }
}

impl From<FontWeight> for u16 {
    fn from(font_weight: FontWeight) -> Self {
        font_weight.0
    }
}
