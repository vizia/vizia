use crate::{Parse, impl_parse};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default]
    LeftToRight,
    RightToLeft,
    Auto,
}

impl_parse! {
    Direction,

    tokens {
        ident {
            "ltr" => Direction::LeftToRight,
            "rtl" => Direction::RightToLeft,
            "auto" => Direction::Auto,
        }
    }
}
