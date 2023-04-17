//! # Vizia

#[cfg(all(not(feature = "baseview"), feature = "winit"))]
pub use vizia_winit::application::Application;

#[cfg(all(not(feature = "winit"), feature = "baseview"))]
pub use vizia_baseview::{Application, ParentWindow, WindowScalePolicy};

pub use vizia_core::*;

#[doc(hidden)]
pub mod prelude {
    pub use vizia_core::prelude::*;

    #[cfg(all(not(feature = "baseview"), feature = "winit"))]
    pub use vizia_winit::application::Application;

    #[cfg(all(not(feature = "winit"), feature = "baseview"))]
    pub use vizia_baseview::Application;
}
