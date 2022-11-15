use crate::{impl_parse, Parse};

/// A factor of the remaining free space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Stretch(pub f32);

impl_parse! {
    Stretch,

    tokens {
        dimension {
            "st" => Stretch,
        }
    }
}

impl From<f32> for Stretch {
    fn from(number: f32) -> Self {
        Stretch(number)
    }
}

impl From<Stretch> for f32 {
    fn from(stretch: Stretch) -> Self {
        stretch.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Stretch, assert_stretch,

        dimension {
            "st" => Stretch,
        }
    }
}
