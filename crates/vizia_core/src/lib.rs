#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub mod accessibility;
pub mod animation;
pub mod cache;
pub mod context;
pub mod entity;
pub mod environment;
pub mod events;
pub mod fonts;
pub mod handle;
pub mod input;
pub mod layout;
pub mod localization;
pub mod modifiers;
pub mod resource;
pub mod state;
pub mod style;
pub mod systems;
pub mod text;
pub mod tree;
pub mod view;
pub mod views;
pub mod window;

mod storage;

/// This is a re-export of [femtovg](https://docs.rs/femtovg/latest/femtovg/).
pub mod vg {
    pub use femtovg::*;
}

/// This is a re-export of [image](https://docs.rs/image/latest/image/).
pub mod image {
    pub use image::*;
}

/// Members which we recommend you wildcard-import.
pub mod prelude {
    pub use super::animation::{Animation, AnimationBuilder, Interpolator};
    pub use super::context::{
        AccessContext, AccessNode, Context, ContextProxy, DataContext, DrawContext, EmitContext,
        EventContext, ProxyEmitError,
    };
    pub use super::entity::Entity;
    pub use super::environment::{Environment, EnvironmentEvent};
    pub use super::events::{Event, Propagation};
    pub use super::handle::Handle;
    pub use super::input::{Keymap, KeymapEntry, KeymapEvent};
    pub use super::localization::Localized;
    pub use super::modifiers::{
        AbilityModifiers, AccessibilityModifiers, ActionModifiers, LayoutModifiers, StyleModifiers,
        TextModifiers,
    };
    pub use super::state::{Binding, Data, Lens, LensExt, Model, OrLens, Res, Setter, Wrapper};
    pub use super::view::{Canvas, View};
    pub use super::views::*;
    pub use super::window::WindowModifiers;
    pub use accesskit::{Action, DefaultActionVerb, Live, Role};
    pub use vizia_derive::{Data, Lens, Model, Setter};
    pub use vizia_id::GenerationalId;
    pub use vizia_input::{Code, Key, KeyChord, Modifiers, MouseButton, MouseButtonState};
    pub use vizia_storage::{Tree, TreeExt};
    pub use vizia_window::{WindowDescription, WindowEvent, WindowSize};

    pub use super::style::{
        Abilities, Angle, BackgroundImage, BorderCornerShape, Color, CursorIcon, Display,
        FontStyle, FontWeight, Length, LengthOrPercentage, LengthValue, LineDirection,
        LinearGradient, Matrix, Opacity, Overflow, PseudoClassFlags, Transform, Visibility, RGBA,
    };

    pub use cosmic_text::FamilyOwned;
    pub use morphorm::Units::*;
    pub use morphorm::{LayoutType, PositionType, Units};
    pub use unic_langid::LanguageIdentifier;
}

/// One very small function for abstracting debugging between web and desktop programming.
/// On the desktop, it will print to stdout, and on the web, it will print to the console log.
#[cfg(not(target_arch = "wasm32"))]
pub fn log(text: &str) {
    println!("{}", text);
}
#[cfg(target_arch = "wasm32")]
pub fn log(text: &str) {
    web_sys::console::log_1(&text.into());
}
