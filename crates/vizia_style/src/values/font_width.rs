use skia_safe::font_style::Width;

use crate::{CustomParseError, Ident, Parse, Percentage};

/// A font stretch value.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum FontWidth {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    #[default]
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl<'i> Parse<'i> for FontWidth {
    fn parse<'t>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, crate::CustomParseError<'i>>> {
        let location = input.current_source_location();
        match input.try_parse(Ident::parse) {
            Ok(ident) => match ident.0.as_ref() {
                "ultra-condensed" => Ok(FontWidth::UltraCondensed),
                "extra-condensed" => Ok(FontWidth::ExtraCondensed),
                "condensed" => Ok(FontWidth::Condensed),
                "semi-condensed" => Ok(FontWidth::SemiCondensed),
                "normal" => Ok(FontWidth::Normal),
                "semi-expanded" => Ok(FontWidth::SemiExpanded),
                "expanded" => Ok(FontWidth::Expanded),
                "extra-expanded" => Ok(FontWidth::ExtraExpanded),
                "ultra-expanded" => Ok(FontWidth::UltraExpanded),
                _ => Err(cssparser::ParseError {
                    kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidValue),
                    location,
                }),
            },

            Err(_) => input.try_parse(Percentage::parse).map(|val| val.into()),
        }
    }
}

impl From<Percentage> for FontWidth {
    fn from(p: Percentage) -> Self {
        if p.0 >= 0.0 && p.0 <= 0.5625 {
            FontWidth::UltraCondensed
        } else if p.0 > 0.5625 && p.0 <= 0.6875 {
            FontWidth::ExtraCondensed
        } else if p.0 > 0.6875 && p.0 <= 0.7625 {
            FontWidth::Condensed
        } else if p.0 > 0.7625 && p.0 <= 0.8875 {
            FontWidth::SemiCondensed
        } else if p.0 > 0.8875 && p.0 <= 1.0125 {
            FontWidth::Normal
        } else if p.0 > 1.0125 && p.0 <= 1.1375 {
            FontWidth::SemiExpanded
        } else if p.0 > 1.1375 && p.0 <= 1.375 {
            FontWidth::Expanded
        } else if p.0 > 1.375 && p.0 <= 1.75 {
            FontWidth::ExtraExpanded
        } else {
            FontWidth::UltraExpanded
        }
    }
}

impl From<FontWidth> for Width {
    fn from(value: FontWidth) -> Self {
        match value {
            FontWidth::UltraCondensed => Width::ULTRA_CONDENSED,
            FontWidth::ExtraCondensed => Width::EXTRA_CONDENSED,
            FontWidth::Condensed => Width::CONDENSED,
            FontWidth::SemiCondensed => Width::SEMI_CONDENSED,
            FontWidth::Normal => Width::NORMAL,
            FontWidth::SemiExpanded => Width::SEMI_EXPANDED,
            FontWidth::Expanded => Width::EXPANDED,
            FontWidth::ExtraExpanded => Width::EXTRA_EXPANDED,
            FontWidth::UltraExpanded => Width::ULTRA_EXPANDED,
        }
    }
}
