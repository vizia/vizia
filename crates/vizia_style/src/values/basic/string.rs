use crate::{macros::impl_parse, Parse};
use cssparser::Token;

impl_parse! {
    String,

    tokens {
        custom {
            Token::QuotedString(ref value) => value.as_ref().to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        String, assert_string,

        custom {
            success {
                r#""test""# => String::from("test"),
                r#"'test'"# => String::from("test"),
                r#""abc"def"ghi""# => String::from("abc"),
                r#""a b c d e f g""# => String::from("a b c d e f g"),
            }

            failure {
                "test",
                "123",
            }
        }
    }
}
