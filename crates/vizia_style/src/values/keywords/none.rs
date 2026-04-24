use crate::{Parse, impl_parse};

/// The 'none' keyword.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NoneKeyword;

impl_parse! {
    NoneKeyword,

    tokens {
        ident {
            "none" => NoneKeyword,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        NoneKeyword, assert_none_keyword,

        ident {
            "none" => NoneKeyword,
        }
    }
}
