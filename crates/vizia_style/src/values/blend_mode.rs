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

impl From<BlendMode> for vizia_render::BlendMode {
    fn from(value: BlendMode) -> Self {
        match value {
            BlendMode::Plus => vizia_render::BlendMode::Plus,
            BlendMode::Normal => vizia_render::BlendMode::SrcOver,
            BlendMode::Multiply => vizia_render::BlendMode::Multiply,
            BlendMode::Screen => vizia_render::BlendMode::Screen,
            BlendMode::Overlay => vizia_render::BlendMode::Overlay,
            BlendMode::Darken => vizia_render::BlendMode::Darken,
            BlendMode::Lighten => vizia_render::BlendMode::Lighten,
            BlendMode::ColorDodge => vizia_render::BlendMode::ColorDodge,
            BlendMode::ColorBurn => vizia_render::BlendMode::ColorBurn,
            BlendMode::HardLight => vizia_render::BlendMode::HardLight,
            BlendMode::SoftLight => vizia_render::BlendMode::SoftLight,
            BlendMode::Difference => vizia_render::BlendMode::Difference,
            BlendMode::Exclusion => vizia_render::BlendMode::Exclusion,
            BlendMode::Hue => vizia_render::BlendMode::Hue,
            BlendMode::Saturation => vizia_render::BlendMode::Saturation,
            BlendMode::Color => vizia_render::BlendMode::Color,
            BlendMode::Luminosity => vizia_render::BlendMode::Luminosity,
        }
    }
}
