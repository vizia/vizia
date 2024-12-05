use crate::{impl_parse, Parse};
pub use morphorm::Alignment;

impl_parse! {
    Alignment,

    tokens {
        ident {
            "top-left" => Alignment::TopLeft,
            "top-center" => Alignment::TopCenter,
            "top-right" => Alignment::TopRight,
            "left" => Alignment::Left,
            "center" => Alignment::Center,
            "right" => Alignment::Right,
            "bottom-left" => Alignment::BottomLeft,
            "bottom-center" => Alignment::BottomCenter,
            "bottom-right" => Alignment::BottomRight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Alignment, assert_alignment,

        ident {
            "top-left" => Alignment::TopLeft,
            "top-center" => Alignment::TopCenter,
            "top-right" => Alignment::TopRight,
            "left" => Alignment::Left,
            "center" => Alignment::Center,
            "right" => Alignment::Right,
            "bottom-left" => Alignment::BottomLeft,
            "bottom-center" => Alignment::BottomCenter,
            "bottom-right" => Alignment::BottomRight,
        }
    }
}
