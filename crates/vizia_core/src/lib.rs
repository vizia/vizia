#![feature(trait_alias)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

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
    pub use super::animation::{AnimExt, Animation, AnimationBuilder};
    pub use super::context::{
        Context, ContextProxy, DataContext, DrawContext, EmitContext, EventContext, ProxyEmitError,
    };
    pub use super::entity::Entity;
    pub use super::environment::{Environment, EnvironmentEvent};
    pub use super::events::{Event, Propagation};
    pub use super::handle::Handle;
    pub use super::input::{Keymap, KeymapEntry, KeymapEvent};
    pub use super::localization::Localized;
    pub use super::modifiers::{
        AbilityModifiers, ActionModifiers, LayoutModifiers, StyleModifiers, TextModifiers,
    };
    pub use super::state::{
        Binding, Data, Lens, LensSimple, LensExt, LensValue, Model, Res, Setter, StatelessLens, Wrapper,
    };
    pub use super::view::{Canvas, View};
    pub use super::views::*;
    pub use super::window::WindowModifiers;
    pub use vizia_derive::{Data, Lens, Model, Setter};
    pub use vizia_id::GenerationalId;
    pub use vizia_input::{Code, Key, KeyChord, Modifiers, MouseButton, MouseButtonState};
    pub use vizia_storage::{Tree, TreeExt};
    pub use vizia_window::{CursorIcon, WindowDescription, WindowEvent, WindowSize};

    pub use super::style::{
        Abilities, BorderCornerShape, Color, Display, GradientDirection, GradientStop,
        LinearGradient, Opacity, Overflow, PseudoClass, Visibility,
    };

    pub use cosmic_text::{FamilyOwned, Style as FontStyle, Weight};
    pub use morphorm::Units::*;
    pub use morphorm::{GeometryChanged, LayoutType, PositionType, Units};
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
