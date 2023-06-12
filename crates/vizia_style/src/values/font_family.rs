use crate::{macros::impl_parse, CustomParseError, Parse};
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
        let location = input.current_source_location();
        if let Ok(generic) = input.try_parse(GenericFontFamily::parse) {
            Ok(FontFamily::Generic(generic))
        } else {
            Ok(FontFamily::Named(input.expect_ident_or_string().cloned().map_err(|_err| {
                cssparser::ParseError {
                    kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidValue),
                    location,
                }
            })?))
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
