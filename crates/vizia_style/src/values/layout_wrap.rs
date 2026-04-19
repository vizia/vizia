use crate::{Parse, impl_parse};

pub use morphorm::LayoutWrap;

impl_parse! {
    LayoutWrap,

    tokens {
        ident {
            "no-wrap" => LayoutWrap::NoWrap,
            "wrap" => LayoutWrap::Wrap,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        LayoutWrap, assert_layout_wrap,

        ident {
            "no-wrap" => LayoutWrap::NoWrap,
            "wrap" => LayoutWrap::Wrap,
        }
    }
}
