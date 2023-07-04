#![allow(clippy::type_complexity)]
mod application;
mod parent_window;
pub(crate) mod proxy;
mod window;

pub use parent_window::ParentWindow;

pub use application::Application;

pub use baseview::WindowScalePolicy;
use femtovg::renderer::OpenGl as Renderer;
