use crate::{impl_parse, OverflowKeyword, Parse, Rect};

/// Determines how to deal with content that overflows the bounding box of the element on the x and y axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Overflow {
    /// Determines how to deal with content that overflows the bounding box of the element on the x axis.
    pub x: OverflowKeyword,
    /// Determines how to deal with content that overflows the bounding box of the element on the y axis.
    pub y: OverflowKeyword,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow {
            x: OverflowKeyword::default(),
            y: OverflowKeyword::default(),
        }
    }
}

impl Overflow {
    pub fn new(x: OverflowKeyword, y: OverflowKeyword) -> Self {
        Self { x, y }
    }
}

impl_parse! {
    Overflow,

    try_parse {
        Rect<OverflowKeyword>,
    }
}

impl From<Rect<OverflowKeyword>> for Overflow {
    fn from(rect: Rect<OverflowKeyword>) -> Self {
        Overflow {
            x: rect.0,
            y: rect.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::assert_parse, OverflowKeyword::*};

    assert_parse! {
        Overflow, parse_value,

        custom {
            success {
                "visible" => Overflow::new(Visible, Visible),
                "hidden" => Overflow::new(Hidden, Hidden),
                "clip" => Overflow::new(Clip, Clip),
                "scroll" => Overflow::new(Scroll, Scroll),
                "visible visible" => Overflow::new(Visible, Visible),
                "hidden hidden" => Overflow::new(Hidden, Hidden),
                "clip clip" => Overflow::new(Clip, Clip),
                "scroll scroll" => Overflow::new(Scroll, Scroll),
                "visible hidden" => Overflow::new(Visible, Hidden),
                "hidden clip" => Overflow::new(Hidden, Clip),
                "clip scroll" => Overflow::new(Clip, Scroll),
                "scroll visible" => Overflow::new(Scroll, Visible),
            }

            failure {
                "test",
                "123",
                "test visible",
            }
        }
    }
}
