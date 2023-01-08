use crate::{CustomParseError, Parse};

use cssparser::*;

/// A simple ident.
#[derive(Debug, Clone, PartialEq)]
pub struct Ident<'i>(pub CowRcStr<'i>);

impl<'i> Parse<'i> for Ident<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let ident = input.expect_ident()?;
        Ok(Ident(ident.clone()))
    }
}

impl<'i> From<String> for Ident<'i> {
    fn from(string: String) -> Self {
        Ident(string.into())
    }
}

impl<'i> From<Ident<'i>> for String {
    fn from(ident: Ident<'i>) -> Self {
        ident.0.to_string()
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
                "ident" => Ident(String::from("ident").into()),
                "border" => Ident(String::from("border").into()),
                "color" => Ident(String::from("color").into()),
                "yes" => Ident(String::from("yes").into()),
                "no" => Ident(String::from("no").into()),
            }

            failure {
                "123",
                "123ident",
            }
        }
    }
}
