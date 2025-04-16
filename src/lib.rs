//! # Vizia

extern crate self as vizia;

#[cfg(feature = "winit")]
pub use vizia_winit::application::{Application, ApplicationError};

pub use vizia_core::*;

#[doc(hidden)]
pub mod prelude {
    pub use vizia_core::prelude::*;

    #[cfg(feature = "winit")]
    pub use vizia_winit::{
        application::{Application, ApplicationError},
        window::Window,
        window_modifiers::WindowModifiers,
        ModifyWindow,
    };
}
