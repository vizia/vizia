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
            BlendMode::Plus => skia_safe::BlendMode::Plus,
            BlendMode::Normal => skia_safe::BlendMode::SrcOver,
            BlendMode::Multiply => skia_safe::BlendMode::Multiply,
            BlendMode::Screen => skia_safe::BlendMode::Screen,
            BlendMode::Overlay => skia_safe::BlendMode::Overlay,
            BlendMode::Darken => skia_safe::BlendMode::Darken,
            BlendMode::Lighten => skia_safe::BlendMode::Lighten,
            BlendMode::ColorDodge => skia_safe::BlendMode::ColorDodge,
            BlendMode::ColorBurn => skia_safe::BlendMode::ColorBurn,
            BlendMode::HardLight => skia_safe::BlendMode::HardLight,
            BlendMode::SoftLight => skia_safe::BlendMode::SoftLight,
            BlendMode::Difference => skia_safe::BlendMode::Difference,
            BlendMode::Exclusion => skia_safe::BlendMode::Exclusion,
            BlendMode::Hue => skia_safe::BlendMode::Hue,
            BlendMode::Saturation => skia_safe::BlendMode::Saturation,
            BlendMode::Color => skia_safe::BlendMode::Color,
            BlendMode::Luminosity => skia_safe::BlendMode::Luminosity,
        }
    }
}
