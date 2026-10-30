#![allow(clippy::type_complexity)]
mod application;
mod parent_window;
pub(crate) mod proxy;
mod window;
mod window_settings;

pub use parent_window::ParentWindow;

pub use application::{Application, ApplicationError};
pub use baseview::Window;
pub use window_settings::{FramePolicy, WindowScalePolicy};
