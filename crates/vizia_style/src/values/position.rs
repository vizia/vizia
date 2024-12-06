use crate::{impl_parse, Parse};
pub use morphorm::Position;

impl_parse! {
    Position,

    tokens {
        ident {
            "absolute" => Position::Absolute,
            "relative" => Position::Relative,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Position, assert_position,

        ident {
            "absolute" => Position::Absolute,
            "relative" => Position::Relative,
        }
    }
}
