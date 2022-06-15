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
pub mod text;
pub mod tree;
pub mod view;
pub mod views;
pub mod window;

mod hover_system;
mod id;
mod storage;
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
    pub use super::animation::{AnimExt, Animation, AnimationBuilder};
    pub use super::context::{Context, ContextProxy, DataContext, DrawContext, ProxyEmitError};
    pub use super::entity::Entity;
    pub use super::environment::Env;
    pub use super::events::{Event, Message, Propagation};
    pub use super::handle::Handle;
    pub use super::input::{
        KeyChord, Keymap, KeymapEvent, Modifiers, MouseButton, MouseButtonState,
    };
    pub use super::localization::Localized;
    pub use super::modifiers::Actions;
    pub use super::state::{Binding, Data, Lens, LensExt, Model, Res};
    pub use super::tree::{Tree, TreeExt};
    pub use super::view::{Canvas, View};
    pub use super::views::*;
    pub use super::window::{
        CursorIcon, WindowDescription, WindowEvent, WindowModifiers, WindowSize,
    };

    pub use vizia_derive::{Data, Lens};

    pub use super::style::{
        Abilities, BorderCornerShape, Color, Display, GradientDirection, GradientStop,
        LinearGradient, Opacity, Overflow, PseudoClass, Visibility,
    };

    pub use keyboard_types::{Code, Key};
    pub use morphorm::Units::*;
    pub use morphorm::{GeometryChanged, LayoutType, PositionType, Units};
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
