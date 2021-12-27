use crate::{Canvas, Context, Entity, Event};

use std::any::{Any, TypeId};

pub trait ViewHandler: Any {
    fn debug(&self, entity: Entity) -> String {
        entity.to_string()
    }

    #[allow(unused_variables)]
    fn update(&mut self, new: &dyn ViewHandler) {}

    fn element(&self) -> Option<String> {
        None
    }

    fn body(&mut self, cx: &mut Context);

    // Called when events are flushed
    fn event(&mut self, cx: &mut Context, event: &mut Event);

    fn draw(&self, cx: &Context, canvas: &mut Canvas);
}

impl dyn ViewHandler {
    // Check if a view handler is a certain type
    pub fn is<T: ViewHandler + 'static>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let concrete = self.type_id();

        // Compare both TypeIds on equality
        t == concrete
    }

    // Attempt to cast a view handler to a mutable reference to the specified type
    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: ViewHandler + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn ViewHandler as *mut T)) }
        } else {
            None
        }
    }

    // Attempt to cast a view handler to an immutable reference to the specified type
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: ViewHandler + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn ViewHandler as *const T)) }
        } else {
            None
        }
    }
}
