pub mod fonts;
pub mod input;
pub mod tree;
pub mod text;
pub mod views;
pub mod state;
pub mod events;
pub mod localization;
pub mod window;
pub mod entity;
pub mod handle;
pub mod context;
pub mod animation;
pub mod resource;
pub mod environment;
pub mod style;
pub mod view;
pub mod modifiers;
pub mod layout;
pub mod cache;

mod id;
mod storage;
mod hover_system;
mod style_system;

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
    pub use super::views::*;
    pub use super::view::{View, Canvas};
    pub use super::context::{Context, ContextProxy, DrawContext, DataContext, ProxyEmitError};
    pub use super::window::{WindowDescription, WindowEvent, WindowSize, WindowModifiers, CursorIcon};
    pub use super::events::{Event, Message, Propagation};
    pub use super::animation::{Animation, AnimationBuilder, AnimExt};
    pub use super::localization::Localized;
    pub use super::entity::Entity;
    pub use super::handle::Handle;
    pub use super::tree::{Tree, TreeExt};
    pub use super::environment::Env;
    pub use super::modifiers::Actions;
    pub use super::input::{MouseButtonState, MouseButton, Keymap, KeyChord, KeymapEvent, Modifiers};
    pub use super::state::{Data, Lens, Res, Binding, LensExt, Model};

    pub use vizia_derive::{Data, Lens};

    pub use super::style::{
        BorderCornerShape, Display, Overflow, PropSet, PseudoClass, Visibility, Abilities, Color,
        GradientDirection, GradientStop, LinearGradient, Opacity, PropGet,

    };

    pub use morphorm::{GeometryChanged, LayoutType, PositionType, Units};
    pub use morphorm::Units::*;
    pub use keyboard_types::Code;
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
