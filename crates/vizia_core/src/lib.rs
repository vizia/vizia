#![allow(clippy::type_complexity)] // TODO: Fix these
// To allow a match syntax in event handlers with one event variant
#![allow(clippy::single_match)]
// To allow enum names with the same prefix
#![allow(clippy::enum_variant_names)]

#[doc(hidden)]
mod accessibility;
pub(crate) mod animation;
pub mod binding;
#[doc(hidden)]
pub mod cache;
pub mod context;
#[doc(hidden)]
pub mod entity;
pub mod environment;
pub mod events;
pub mod input;
pub mod layout;
pub mod localization;
pub mod model;
pub mod modifiers;
pub mod resource;
pub mod style;
mod systems;
pub(crate) mod text;
#[doc(hidden)]
pub mod tree;
pub mod util;
pub mod view;
pub mod views;
pub mod window;

mod storage;

/// Contains types and functions used for custom drawing within views. This is a re-export of [femtovg](https://docs.rs/femtovg/latest/femtovg/).
pub mod vg {
    pub use femtovg::*;
}

/// Contains types and functions used for loading and manipulating images. This is a re-export of [image](https://docs.rs/image/latest/image/).
pub mod image {
    pub use image::*;
}

/// A collection of codepoints for built-in icons.
pub mod icons;

pub mod fonts;
pub use fonts::*;

#[doc(hidden)]
pub mod backend {
    #[cfg(not(target_arch = "wasm32"))]
    pub use super::accessibility::IntoNode;
    pub use super::context::backend::BackendContext;
    pub use super::text::cosmic::TextConfig;
    pub use vizia_window::WindowDescription;
}

/// Members which we recommend you wildcard-import.
#[doc(hidden)]
pub mod prelude {
    pub use super::binding::{
        Binding, Data, Index, Lens, LensExt, Res, Setter, StaticLens, Then, UnwrapLens, Wrapper,
    };

    pub use crate::model::Model;

    pub use super::animation::{Animation, AnimationBuilder, KeyframeBuilder};
    pub use super::context::{
        AccessContext, AccessNode, Context, ContextProxy, DataContext, DrawContext, EmitContext,
        EventContext, ProxyEmitError,
    };
    pub use super::entity::Entity;
    pub use super::environment::{AppTheme, Environment, EnvironmentEvent, ThemeMode};
    pub use super::events::{Event, Propagation, Timer, TimerAction};
    pub use super::include_style;
    pub use super::input::{Keymap, KeymapEntry, KeymapEvent};
    pub use super::layout::{BoundingBox, GeoChanged};
    pub use super::localization::{Localized, ToStringLocalized};
    pub use super::modifiers::{
        AbilityModifiers, AccessibilityModifiers, ActionModifiers, BoxShadowBuilder,
        LayoutModifiers, LinearGradientBuilder, StyleModifiers, TextModifiers,
    };
    pub use super::resource::ImageRetentionPolicy;
    pub use super::util::{IntoCssStr, CSS};
    pub use super::view::{Canvas, Handle, View};
    pub use super::views::*;
    pub use super::window::{DropData, WindowEvent, WindowModifiers};
    pub use accesskit::{Action, DefaultActionVerb, Live, Role};
    pub use vizia_derive::{Data, Lens, Model, Setter};
    pub use vizia_id::GenerationalId;
    pub use vizia_input::{Code, Key, KeyChord, Modifiers, MouseButton, MouseButtonState};
    pub use vizia_storage::{Tree, TreeExt};
    pub use vizia_window::WindowSize;

    pub use super::style::*;

    pub use cosmic_text::FamilyOwned;
    pub use instant::{Duration, Instant};
    pub use morphorm::Units::*;
    pub use morphorm::{LayoutType, PositionType, Units};
    pub use unic_langid::{langid, LanguageIdentifier};
}
