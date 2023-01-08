use crate::{macros::impl_parse, Ident, Parse};
use cssparser::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericFontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
}

impl_parse! {
    GenericFontFamily,

    tokens {
        ident {
            "serif" => GenericFontFamily::Serif,
            "sans-serif" => GenericFontFamily::SansSerif,
            "cursive" => GenericFontFamily::Cursive,
            "fantasy" => GenericFontFamily::Fantasy,
            "monospace" => GenericFontFamily::Monospace,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FontFamily<'i> {
    Generic(GenericFontFamily),
    Named(CowRcStr<'i>),
}

impl<'i> Parse<'i> for FontFamily<'i> {
    fn parse<'t>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, crate::CustomParseError<'i>>> {
        if let Ok(generic) = GenericFontFamily::parse(input) {
            return Ok(FontFamily::Generic(generic));
        } else {
            return Ident::parse(input).map(|ident| FontFamily::Named(ident.0));
        }
    }
}

impl<'i> Parse<'i> for Vec<FontFamily<'i>> {
    fn parse<'t>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, crate::CustomParseError<'i>>> {
        input.parse_comma_separated(FontFamily::parse)
    }
}
