use crate::{impl_parse, BorderWidthKeyword, Length, LengthOrPercentage, Parse};

/// A border width value either being a [`BorderWidthKeyword`] or a [`LengthOrPercentage`].
#[derive(Debug, Clone, PartialEq)]
pub struct BorderWidthValue(pub LengthOrPercentage);

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
            BorderWidthKeyword::Thin => {
                BorderWidthValue(LengthOrPercentage::Length(Length::px(1.0)))
            }
            BorderWidthKeyword::Medium => {
                BorderWidthValue(LengthOrPercentage::Length(Length::px(3.0)))
            }
            BorderWidthKeyword::Thick => {
                BorderWidthValue(LengthOrPercentage::Length(Length::px(5.0)))
            }
        }
    }
}

impl From<Length> for BorderWidthValue {
    fn from(length: Length) -> Self {
        BorderWidthValue(LengthOrPercentage::Length(length))
    }
}

impl From<BorderWidthValue> for LengthOrPercentage {
    fn from(border_width_value: BorderWidthValue) -> Self {
        border_width_value.0
    }
}

impl Default for BorderWidthValue {
    fn default() -> Self {
        Self(LengthOrPercentage::Length(Length::px(3.0)))
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
                "thin" => BorderWidthValue(Length::px(1.0).into()),
                "medium" => BorderWidthValue(Length::px(3.0).into()),
                "thick" => BorderWidthValue(Length::px(5.0).into()),
            }

            failure {
                "test",
                "123",
                "thinpx",
            }
        }
    }
}
