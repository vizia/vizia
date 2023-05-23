pub use raw_window_handle::RawWindowHandle;

pub mod application;
mod convert;
mod window;

pub trait GetRawWindowHandle {
    fn raw_window_handle(&mut self) -> Option<RawWindowHandle>;
}

use raw_window_handle::HasRawWindowHandle;
use vizia_core::prelude::{Entity, EventContext, GenerationalId};
use window::Window;

impl<'a> GetRawWindowHandle for EventContext<'a> {
    fn raw_window_handle(&mut self) -> Option<RawWindowHandle> {
        self.with_current(Entity::root(), |cx| {
            cx.get_view::<Window>().map(|window| window.window().raw_window_handle())
        })
    }
}
