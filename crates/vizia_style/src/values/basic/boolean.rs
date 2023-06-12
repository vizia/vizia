use crate::{macros::impl_parse, Parse};

impl_parse! {
    bool,

    tokens {
        ident {
            "on" => true,
            "off" => false,
            "true" => true,
            "false" => false,
            "yes" => true,
            "no" => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        bool, assert_boolean,

        ident {
            "on" => true,
            "off" => false,
            "true" => true,
            "false" => false,
            "yes" => true,
            "no" => false,
        }
    }
}
