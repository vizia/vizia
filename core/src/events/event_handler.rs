use crate::{Canvas, Context, Event};

use std::any::{Any, TypeId};

pub trait ViewHandler: Any {
    fn element(&self) -> Option<String> {
        None
    }

    fn body(&mut self, cx: &mut Context);

    fn event(&mut self, cx: &mut Context, event: &mut Event);

    fn draw(&self, cx: &mut Context, canvas: &mut Canvas);
}

impl dyn ViewHandler {
    /// Check if a view handler is a certain type.
    pub fn is<T: Any + 'static>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let concrete = self.type_id();

        // Compare both TypeIds on equality
        t == concrete
    }

    /// Attempt to cast a view handler to a mutable reference to the specified type.
    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn ViewHandler as *mut T)) }
        } else {
            None
        }
    }

    /// Attempt to cast a view handler to an immutable reference to the specified type.
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Any + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn ViewHandler as *const T)) }
        } else {
            None
        }
    }
}
