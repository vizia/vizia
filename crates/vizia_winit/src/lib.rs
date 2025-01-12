pub mod application;
mod convert;
pub mod window;
pub mod window_modifiers;

pub trait ModifyWindow {
    /// Takes a closure which mutates the parent window of the current view.
    fn modify_window<T>(&mut self, f: impl FnOnce(&winit::window::Window) -> T) -> Option<T>;
    /// Returns a read-only pointer to the parent window of the current view.
    fn window(&self) -> Option<Arc<winit::window::Window>>;
}

use std::sync::Arc;

use vizia_core::{
    context::TreeProps,
    prelude::{Entity, EventContext, GenerationalId},
};
use window::Window;

impl ModifyWindow for EventContext<'_> {
    fn modify_window<T>(&mut self, f: impl FnOnce(&winit::window::Window) -> T) -> Option<T> {
        self.with_current(self.parent_window().unwrap_or(Entity::root()), move |cx| {
            cx.get_view::<Window>()
                .and_then(|window| window.window.clone())
                .map(|window| (f)(window.as_ref()))
        })
    }

    fn window(&self) -> Option<Arc<winit::window::Window>> {
        self.get_view_with::<Window>(self.parent_window().unwrap_or(Entity::root()))
            .and_then(|window| window.window.clone())
    }
}
