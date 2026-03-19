use crate::{Parse, impl_parse};
pub use morphorm::LayoutType;

impl_parse! {
    LayoutType,

    tokens {
        ident {
            "row" => LayoutType::Row,
            "column" => LayoutType::Column,
            "grid" => LayoutType::Grid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        LayoutType, assert_layout_type,

        ident {
            "row" => LayoutType::Row,
            "column" => LayoutType::Column,
            // "grid" => LayoutType::Grid,
        }
    }
}
