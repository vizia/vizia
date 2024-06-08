pub mod application;
mod convert;
mod window;

pub mod rwh {
    pub use raw_window_handle::*;
}

pub trait GetRawWindowHandle {
    fn raw_window_handle(&mut self) -> rwh::RawWindowHandle;
    fn mutate_window<T>(&mut self, f: impl FnOnce(&winit::window::Window) -> T) -> Option<T>;
}

use raw_window_handle::HasRawWindowHandle;
use vizia_core::prelude::{Context, Entity, EventContext, GenerationalId};
use window::Window;

impl<'a> GetRawWindowHandle for EventContext<'a> {
    fn raw_window_handle(&mut self) -> rwh::RawWindowHandle {
        self.with_current(Entity::root(), |cx| {
            cx.get_view::<Window>().map(|window| window.window().raw_window_handle())
        })
        .unwrap()
    }

    fn mutate_window<T>(&mut self, f: impl FnOnce(&winit::window::Window) -> T) -> Option<T> {
        self.with_current(Entity::root(), move |cx| {
            cx.get_view::<Window>().map(move |window| (f)(window.window()))
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl GetRawWindowHandle for Context {
    fn raw_window_handle(&mut self) -> rwh::RawWindowHandle {
        let mut cx = EventContext::new(self);
        cx.with_current(Entity::root(), |cx| {
            cx.get_view::<Window>().map(|window| window.window().raw_window_handle())
        })
        .unwrap()
    }

    fn mutate_window<T>(&mut self, f: impl FnOnce(&winit::window::Window) -> T) -> Option<T> {
        let mut cx = EventContext::new(self);

        cx.with_current(Entity::root(), move |cx| {
            cx.get_view::<Window>().map(move |window| (f)(window.window()))
        })
    }
}
