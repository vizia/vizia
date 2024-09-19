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
                "ultra-condensed" => Ok(Self::UltraCondensed),
                "extra-condensed" => Ok(Self::ExtraCondensed),
                "condensed" => Ok(Self::Condensed),
                "semi-condensed" => Ok(Self::SemiCondensed),
                "normal" => Ok(Self::Normal),
                "semi-expanded" => Ok(Self::SemiExpanded),
                "expanded" => Ok(Self::Expanded),
                "extra-expanded" => Ok(Self::ExtraExpanded),
                "ultra-expanded" => Ok(Self::UltraExpanded),
                _ => Err(cssparser::ParseError {
                    kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidValue),
                    location,
                }),
            },

            Err(_) => input.try_parse(Percentage::parse).map(std::convert::Into::into),
        }
    }
}

impl From<Percentage> for FontWidth {
    fn from(p: Percentage) -> Self {
        if p.0 >= 0.0 && p.0 <= 0.5625 {
            Self::UltraCondensed
        } else if p.0 > 0.5625 && p.0 <= 0.6875 {
            Self::ExtraCondensed
        } else if p.0 > 0.6875 && p.0 <= 0.7625 {
            Self::Condensed
        } else if p.0 > 0.7625 && p.0 <= 0.8875 {
            Self::SemiCondensed
        } else if p.0 > 0.8875 && p.0 <= 1.0125 {
            Self::Normal
        } else if p.0 > 1.0125 && p.0 <= 1.1375 {
            Self::SemiExpanded
        } else if p.0 > 1.1375 && p.0 <= 1.375 {
            Self::Expanded
        } else if p.0 > 1.375 && p.0 <= 1.75 {
            Self::ExtraExpanded
        } else {
            Self::UltraExpanded
        }
    }
}

impl From<FontWidth> for Width {
    fn from(value: FontWidth) -> Self {
        match value {
            FontWidth::UltraCondensed => Self::ULTRA_CONDENSED,
            FontWidth::ExtraCondensed => Self::EXTRA_CONDENSED,
            FontWidth::Condensed => Self::CONDENSED,
            FontWidth::SemiCondensed => Self::SEMI_CONDENSED,
            FontWidth::Normal => Self::NORMAL,
            FontWidth::SemiExpanded => Self::SEMI_EXPANDED,
            FontWidth::Expanded => Self::EXPANDED,
            FontWidth::ExtraExpanded => Self::EXTRA_EXPANDED,
            FontWidth::UltraExpanded => Self::ULTRA_EXPANDED,
        }
    }
}
