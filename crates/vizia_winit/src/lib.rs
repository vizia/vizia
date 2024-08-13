#![allow(clippy::type_complexity)] // TODO: Fix these

pub mod application;
mod convert;
pub mod window;
pub mod window_modifiers;

pub trait ModifyWindow {
    fn modify_window<T>(&mut self, f: impl FnOnce(&winit::window::Window) -> T) -> Option<T>;
}

use vizia_core::{
    context::TreeProps,
    prelude::{Entity, EventContext, GenerationalId},
};
use window::Window;

impl<'a> ModifyWindow for EventContext<'a> {
    fn modify_window<T>(&mut self, f: impl FnOnce(&winit::window::Window) -> T) -> Option<T> {
        self.with_current(self.parent_window().unwrap_or(Entity::root()), move |cx| {
            cx.get_view::<Window>()
                .and_then(|window| window.window.clone())
                .map(|window| (f)(window.as_ref()))
        })
    }
}
