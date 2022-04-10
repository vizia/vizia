mod application;
mod parent_window;
mod window;
pub(crate) mod proxy;

pub use parent_window::ParentWindow;

pub use application::Application;

pub use baseview::WindowScalePolicy;
use femtovg::renderer::OpenGl as Renderer;
