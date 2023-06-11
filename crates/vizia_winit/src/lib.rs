pub mod application;
mod convert;
mod window;

#[cfg(not(target_arch = "wasm32"))]
pub mod rwh {
    pub use raw_window_handle::*;
}

#[cfg(not(target_arch = "wasm32"))]
pub trait GetRawWindowHandle {
    fn raw_window_handle(&mut self) -> rwh::RawWindowHandle;
    fn mutate_window(&mut self, f: impl FnOnce(&winit::window::Window));
}

#[cfg(not(target_arch = "wasm32"))]
use raw_window_handle::HasRawWindowHandle;
#[cfg(not(target_arch = "wasm32"))]
use vizia_core::prelude::{Context, Entity, EventContext, GenerationalId};
#[cfg(not(target_arch = "wasm32"))]
use window::Window;

#[cfg(not(target_arch = "wasm32"))]
impl<'a> GetRawWindowHandle for EventContext<'a> {
    fn raw_window_handle(&mut self) -> rwh::RawWindowHandle {
        self.with_current(Entity::root(), |cx| {
            cx.get_view::<Window>().map(|window| window.window().raw_window_handle())
        })
        .unwrap()
    }

    fn mutate_window(&mut self, f: impl FnOnce(&winit::window::Window)) {
        self.with_current(Entity::root(), move |cx| {
            cx.get_view::<Window>().map(move |window| (f)(window.window()))
        });
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

    fn mutate_window(&mut self, f: impl FnOnce(&winit::window::Window)) {
        let mut cx = EventContext::new(self);

        cx.with_current(Entity::root(), move |cx| {
            cx.get_view::<Window>().map(move |window| (f)(window.window()))
        });
    }
}
