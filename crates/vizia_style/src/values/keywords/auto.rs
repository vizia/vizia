use crate::{impl_parse, Parse};

/// The 'auto' keyword.
///
/// It is used to parse the [`Units::Auto`](crate::Units::Auto) variant.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AutoKeyword;

impl_parse! {
    AutoKeyword,

    tokens {
        ident {
            "auto" => AutoKeyword,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        AutoKeyword, assert_auto_keyword,

        ident {
            "auto" => AutoKeyword,
        }
    }
}
