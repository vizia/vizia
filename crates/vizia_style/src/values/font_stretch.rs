use crate::{CustomParseError, Ident, Parse, Percentage};

/// A font stretch value.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl<'i> Parse<'i> for FontStretch {
    fn parse<'t>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, crate::CustomParseError<'i>>> {
        let location = input.current_source_location();
        match input.try_parse(Ident::parse) {
            Ok(ident) => match ident.0.as_ref() {
                "ultra-condensed" => Ok(FontStretch::UltraCondensed),
                "extra-condensed" => Ok(FontStretch::ExtraCondensed),
                "condensed" => Ok(FontStretch::Condensed),
                "semi-condensed" => Ok(FontStretch::SemiCondensed),
                "normal" => Ok(FontStretch::Normal),
                "semi-expanded" => Ok(FontStretch::SemiExpanded),
                "expanded" => Ok(FontStretch::Expanded),
                "extra-expanded" => Ok(FontStretch::ExtraExpanded),
                "ultra-expanded" => Ok(FontStretch::UltraExpanded),
                _ => Err(cssparser::ParseError {
                    kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidValue),
                    location,
                }),
            },

            Err(_) => input.try_parse(Percentage::parse).map(|val| val.into()),
        }
    }
}

impl From<Percentage> for FontStretch {
    fn from(p: Percentage) -> Self {
        if p.0 >= 0.0 && p.0 <= 0.5625 {
            FontStretch::UltraCondensed
        } else if p.0 > 0.5625 && p.0 <= 0.6875 {
            FontStretch::ExtraCondensed
        } else if p.0 > 0.6875 && p.0 <= 0.7625 {
            FontStretch::Condensed
        } else if p.0 > 0.7625 && p.0 <= 0.8875 {
            FontStretch::SemiCondensed
        } else if p.0 > 0.8875 && p.0 <= 1.0125 {
            FontStretch::Normal
        } else if p.0 > 1.0125 && p.0 <= 1.1375 {
            FontStretch::SemiExpanded
        } else if p.0 > 1.1375 && p.0 <= 1.375 {
            FontStretch::Expanded
        } else if p.0 > 1.375 && p.0 <= 1.75 {
            FontStretch::ExtraExpanded
        } else {
            FontStretch::UltraExpanded
        }
    }
}
