use crate::{impl_parse, BorderWidthKeyword, Length, Parse};

/// A border width value either being a [`BorderWidthKeyword`] or a [`Length`].
#[derive(Debug, Clone, PartialEq)]
pub struct BorderWidthValue(pub Length);

impl_parse! {
    BorderWidthValue,

    try_parse {
        BorderWidthKeyword,
        Length,
    }
}

impl From<BorderWidthKeyword> for BorderWidthValue {
    fn from(border_width_keyword: BorderWidthKeyword) -> Self {
        match border_width_keyword {
            BorderWidthKeyword::Thin => BorderWidthValue(Length::px(1.0)),
            BorderWidthKeyword::Medium => BorderWidthValue(Length::px(3.0)),
            BorderWidthKeyword::Thick => BorderWidthValue(Length::px(5.0)),
        }
    }
}

impl From<Length> for BorderWidthValue {
    fn from(length: Length) -> Self {
        BorderWidthValue(length)
    }
}

impl From<BorderWidthValue> for Length {
    fn from(border_width_value: BorderWidthValue) -> Self {
        border_width_value.0
    }
}

impl Default for BorderWidthValue {
    fn default() -> Self {
        Self(Length::px(3.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        BorderWidthValue, assert_border_width_value,

        length {
            BorderWidthValue,
        }

        custom {
            success {
                "thin" => BorderWidthValue(Length::px(1.0)),
                "medium" => BorderWidthValue(Length::px(3.0)),
                "thick" => BorderWidthValue(Length::px(5.0)),
            }

            failure {
                "test",
                "123",
                "thinpx",
            }
        }
    }
}
