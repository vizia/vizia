use cssparser::*;

use crate::{macros::impl_parse, FontSizeKeyword, Parse};

use super::Length;

/// A font size value.
#[derive(Debug, Clone, PartialEq)]
pub struct FontSize(pub Length);

impl_parse! {
    FontSize,

    try_parse {
        FontSizeKeyword,
        Length,
    }
}

impl From<FontSizeKeyword> for FontSize {
    fn from(font_size_keyword: FontSizeKeyword) -> Self {
        match font_size_keyword {
            FontSizeKeyword::XXSmall => FontSize(Length::px(8.4)),
            FontSizeKeyword::XSmall => FontSize(Length::px(10.5)),
            FontSizeKeyword::Small => FontSize(Length::px(12.5)),
            FontSizeKeyword::Medium => FontSize(Length::px(14.0)),
            FontSizeKeyword::Large => FontSize(Length::px(16.8)),
            FontSizeKeyword::XLarge => FontSize(Length::px(21.0)),
            FontSizeKeyword::XXLarge => FontSize(Length::px(28.0)),
            FontSizeKeyword::XXXLarge => FontSize(Length::px(42.0)),
        }
    }
}

impl Default for FontSize {
    fn default() -> Self {
        FontSize(Length::px(14.0))
    }
}

impl From<Length> for FontSize {
    fn from(length: Length) -> Self {
        FontSize(length)
    }
}

impl From<u32> for FontSize {
    fn from(number: u32) -> Self {
        FontSize(Length::px(number as f32))
    }
}

impl From<i32> for FontSize {
    fn from(number: i32) -> Self {
        FontSize(Length::px(number as f32))
    }
}

impl From<f32> for FontSize {
    fn from(number: f32) -> Self {
        FontSize(Length::px(number))
    }
}

impl From<f64> for FontSize {
    fn from(number: f64) -> Self {
        FontSize(Length::px(number as f32))
    }
}

impl From<&str> for FontSize {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        FontSize::parse(&mut parser).unwrap_or_default()
    }
}

impl From<FontSize> for f32 {
    fn from(font_size: FontSize) -> Self {
        font_size.0.to_px().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        FontSize, font_size,

        ident {
            "xx-small" => FontSize(Length::px(8.4)),
            "x-small" => FontSize(Length::px(10.5)),
            "small" => FontSize(Length::px(12.5)),
            "medium" => FontSize(Length::px(14.0)),
            "large" => FontSize(Length::px(16.8)),
            "x-large" => FontSize(Length::px(21.0)),
            "xx-large" => FontSize(Length::px(28.0)),
            "xxx-large" => FontSize(Length::px(42.0)),
        }

    }
}
