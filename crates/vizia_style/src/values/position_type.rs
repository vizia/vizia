use crate::{impl_parse, Parse};
pub use morphorm::PositionType;

impl_parse! {
    PositionType,

    tokens {
        ident {
            "absolute" => PositionType::Absolute,
            "relative" => PositionType::Relative,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        PositionType, assert_position,

        ident {
            "absolute" => PositionType::Absolute,
            "relative" => PositionType::Relative,
        }
    }
}
