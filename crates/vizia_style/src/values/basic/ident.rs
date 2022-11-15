use crate::{macros::impl_parse, CustomParseError, Parse};

use cssparser::*;

/// A simple ident.
#[derive(Debug, Clone, PartialEq)]
pub struct Ident(pub String);

impl_parse! {
    Ident,

    tokens {
        custom {
            cssparser::Token::Ident(ident) => ident.as_ref().to_owned().into(),
        }
    }
}

impl From<String> for Ident {
    fn from(string: String) -> Self {
        Ident(string)
    }
}

impl From<Ident> for String {
    fn from(ident: Ident) -> Self {
        ident.0
    }
}

/// A CSS [`<dashed-ident>`](https://www.w3.org/TR/css-values-4/#dashed-idents) declaration.
///
/// Dashed idents are used in cases where an identifier can be either author defined _or_ CSS-defined.
/// Author defined idents must start with two dash characters ("--") or parsing will fail.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DashedIdent<'i>(pub CowRcStr<'i>);

impl<'i> Parse<'i> for DashedIdent<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        if !ident.starts_with("--") {
            return Err(location.new_unexpected_token_error(Token::Ident(ident.clone())));
        }

        Ok(DashedIdent(ident.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Ident, assert_ident,

        custom {
            success {
                "ident" => Ident(String::from("ident")),
                "border" => Ident(String::from("border")),
                "color" => Ident(String::from("color")),
                "yes" => Ident(String::from("yes")),
                "no" => Ident(String::from("no")),
            }

            failure {
                "123",
                "123ident",
            }
        }
    }
}
