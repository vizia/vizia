use crate::{impl_parse, Parse};

/// The 'inset' keyword.
///
/// It is used to parse the `inset` property of the [`BoxShadow`](crate::BoxShadow).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InsetKeyword;

impl_parse! {
    InsetKeyword,

    tokens {
        ident {
            "inset" => InsetKeyword,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        InsetKeyword, assert_inset_keyword,

        ident {
            "inset" => InsetKeyword,
        }
    }
}
