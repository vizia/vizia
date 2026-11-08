//! # Vizia

#![allow(clippy::uninlined_format_args)]

extern crate self as vizia;

#[cfg(all(not(feature = "baseview"), feature = "winit"))]
pub use vizia_winit::application::{Application, ApplicationError};

/// Re-export of the [`tray-icon`](https://docs.rs/tray-icon) crate, used with
/// `Application::with_tray_icon`.
#[cfg(all(not(feature = "baseview"), feature = "winit", feature = "tray"))]
pub use vizia_winit::tray_icon;

#[cfg(all(not(feature = "winit"), feature = "baseview"))]
pub use vizia_baseview::{
    Application, ApplicationError, FramePolicy, ParentWindow, Window, WindowScalePolicy,
};

pub use vizia_core::*;

pub mod prelude {
    pub use vizia_core::prelude::*;

    #[cfg(all(not(feature = "baseview"), feature = "winit"))]
    pub use vizia_winit::{
        ModifyWindow,
        application::{Application, ApplicationError},
        window::Window,
        window_modifiers::WindowModifiers,
    };

    #[cfg(all(not(feature = "baseview"), feature = "winit", feature = "tray"))]
    pub use vizia_winit::tray_icon;

    #[cfg(all(not(feature = "winit"), feature = "baseview"))]
    pub use vizia_baseview::{
        Application, ApplicationError, FramePolicy, Window, WindowScalePolicy,
    };
}
