use crate::{macros::impl_parse, FontSizeKeyword, Parse};

/// A font size value.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FontSize(pub f32);

impl_parse! {
    FontSize,

    try_parse {
        FontSizeKeyword,
        f32,
    }
}

impl From<FontSizeKeyword> for FontSize {
    fn from(font_size_keyword: FontSizeKeyword) -> Self {
        match font_size_keyword {
            FontSizeKeyword::XXSmall => FontSize(8.0),
            FontSizeKeyword::XSmall => FontSize(10.0),
            FontSizeKeyword::Small => FontSize(12.0),
            FontSizeKeyword::Medium => FontSize(14.0),
            FontSizeKeyword::Large => FontSize(16.0),
            FontSizeKeyword::XLarge => FontSize(18.0),
            FontSizeKeyword::XXLarge => FontSize(20.0),
        }
    }
}

impl From<f32> for FontSize {
    fn from(number: f32) -> Self {
        FontSize(number)
    }
}

impl From<FontSize> for f32 {
    fn from(font_size: FontSize) -> Self {
        font_size.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        FontSize, font_size,

        ident {
            "xx-small" => FontSize(8.0),
            "x-small" => FontSize(10.0),
            "small" => FontSize(12.0),
            "medium" => FontSize(14.0),
            "large" => FontSize(16.0),
            "x-large" => FontSize(18.0),
            "xx-large" => FontSize(20.0),
        }

        number {
            FontSize,
        }
    }
}
