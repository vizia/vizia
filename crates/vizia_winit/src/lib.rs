#![allow(clippy::uninlined_format_args)]
#![allow(mismatched_lifetime_syntaxes)]

pub mod application;
mod convert;
pub mod window;
pub mod window_modifiers;

/// Re-export of the [`tray-icon`](https://docs.rs/tray-icon) crate, used with
/// [`Application::with_tray_icon`](crate::application::Application::with_tray_icon).
#[cfg(feature = "tray")]
pub use tray_icon;

pub trait ModifyWindow {
    /// Takes a closure which mutates the parent window of the current view.
    fn modify_window<T>(&mut self, f: impl FnOnce(&winit::window::Window) -> T) -> Option<T>;
    /// Returns a read-only pointer to the parent window of the current view.
    fn window(&self) -> Option<Arc<winit::window::Window>>;
}

use std::sync::Arc;

use vizia_core::{context::TreeProps, prelude::EventContext};
use window::Window;

impl ModifyWindow for EventContext<'_> {
    fn modify_window<T>(&mut self, f: impl FnOnce(&winit::window::Window) -> T) -> Option<T> {
        self.with_current(self.parent_window(), move |cx| {
            cx.get_view::<Window>()
                .and_then(|window| window.window.clone())
                .map(|window| (f)(window.as_ref()))
        })
    }

    fn window(&self) -> Option<Arc<winit::window::Window>> {
        self.get_view_with::<Window>(self.parent_window()).and_then(|window| window.window.clone())
    }
}
