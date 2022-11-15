use crate::{impl_parse, Parse};
pub use morphorm::PositionType;

impl_parse! {
    PositionType,

    tokens {
        ident {
            "self-directed" => PositionType::SelfDirected,
            "parent-directed" => PositionType::ParentDirected,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        PositionType, assert_position_type,

        ident {
            "self-directed" => PositionType::SelfDirected,
            "parent-directed" => PositionType::ParentDirected,
        }
    }
}
