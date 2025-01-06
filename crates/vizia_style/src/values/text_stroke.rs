use crate::{define_enum, impl_parse, Length, Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct TextStroke {
    /// The width of the text stroke
    pub width: Length,
    /// The paint style for the text with a stroke.
    pub style: TextStrokeStyle,
}

define_enum! {
    #[derive(Default)]
    pub enum TextStrokeStyle {
        /// The text will be painted with a fill and a stroke
        #[default]
        "stroke-and-fill": StrokeAndFill,
        /// Only the stroke will be painted
        "stroke": Stroke,
    }
}

impl TextStroke {
    pub fn new(width: impl Into<Length>, style: Option<TextStrokeStyle>) -> Self {
        Self { width: width.into(), style: style.unwrap_or_default() }
    }
}

impl_parse! {
    TextStroke,

    custom {
        |input| {
            let width = Length::parse(input)?;
            let style = input.try_parse(TextStrokeStyle::parse).ok();
            Ok(TextStroke::new(width, style))
        }
    }
}

impl From<TextStrokeStyle> for skia_safe::PaintStyle {
    fn from(value: TextStrokeStyle) -> Self {
        match value {
            TextStrokeStyle::Stroke => skia_safe::PaintStyle::Stroke,
            TextStrokeStyle::StrokeAndFill => skia_safe::PaintStyle::StrokeAndFill,
        }
    }
}

#[cfg(test)]
mod tests_combined {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        TextStroke, parse_text_stroke,

        custom {
            success {
                "2px" => TextStroke::new(Length::px(2.0), None),
                "2.4px stroke" => TextStroke::new(Length::px(2.4), Some(TextStrokeStyle::Stroke)),
                "1.3px stroke-and-fill" => TextStroke::new(Length::px(1.3), Some(TextStrokeStyle::StrokeAndFill)),
                // TODO: Is this what we want?
                "6.7px stronk" => TextStroke::new(Length::px(6.7), None),
            }
            failure {
                "stroke 4.5px",
                "6.7 stronk",
            }
        }
    }
}
