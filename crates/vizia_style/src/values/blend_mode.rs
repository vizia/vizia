use crate::{define_enum, Parse};

define_enum! {
    #[derive(Default)]
    pub enum BlendMode {
        "plus": Plus,
        #[default]
        "normal": Normal,
        "multiply": Multiply,
        "screen": Screen,
        "overlay": Overlay,
        "darken": Darken,
        "lighten": Lighten,
        "color-dodge": ColorDodge,
        "color-burn": ColorBurn,
        "hard-light": HardLight,
        "soft-light": SoftLight,
        "difference": Difference,
        "exclusion": Exclusion,
        "hue": Hue,
        "saturation": Saturation,
        "color": Color,
        "luminosity": Luminosity,

    }
}

impl From<BlendMode> for skia_safe::BlendMode {
    fn from(value: BlendMode) -> Self {
        match value {
            BlendMode::Plus => Self::Plus,
            BlendMode::Normal => Self::SrcOver,
            BlendMode::Multiply => Self::Multiply,
            BlendMode::Screen => Self::Screen,
            BlendMode::Overlay => Self::Overlay,
            BlendMode::Darken => Self::Darken,
            BlendMode::Lighten => Self::Lighten,
            BlendMode::ColorDodge => Self::ColorDodge,
            BlendMode::ColorBurn => Self::ColorBurn,
            BlendMode::HardLight => Self::HardLight,
            BlendMode::SoftLight => Self::SoftLight,
            BlendMode::Difference => Self::Difference,
            BlendMode::Exclusion => Self::Exclusion,
            BlendMode::Hue => Self::Hue,
            BlendMode::Saturation => Self::Saturation,
            BlendMode::Color => Self::Color,
            BlendMode::Luminosity => Self::Luminosity,
        }
    }
}
