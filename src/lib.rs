//! # Vizia

#![allow(clippy::uninlined_format_args)]

extern crate self as vizia;

#[cfg(feature = "winit")]
pub use vizia_winit::application::{Application, ApplicationError};

pub use vizia_core::*;

pub mod prelude {
    pub use vizia_core::prelude::*;

    #[cfg(feature = "winit")]
    pub use vizia_winit::{
        ModifyWindow,
        application::{Application, ApplicationError},
        window::Window,
        window_modifiers::WindowModifiers,
    };
}
