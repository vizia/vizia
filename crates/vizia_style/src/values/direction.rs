use crate::{Parse, impl_parse};

pub use morphorm::Direction;

impl_parse! {
    Direction,

    tokens {
        ident {
            "ltr" => Direction::LeftToRight,
            "rtl" => Direction::RightToLeft,
        }
    }
}
