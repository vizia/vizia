use crate::{Event};

use crate::{Entity, Context};

//use image::Pixels;


use std::any::{Any, TypeId};


/// TODO - Make crate private (provide method to retrieve event handler in state)
pub trait ContainerHandler: Any {
    fn debug(&self, entity: Entity) -> String {
        "".to_string()
    }
    // Called when events are flushed
    fn on_event_(&mut self, cx: &mut Context, event: &mut Event) {}

    // Called when the view is rebuilt
    fn on_build(&mut self, cx: &mut Context) {}

}

pub trait NodeHandler: Any {
    fn debug(&self, entity: Entity) -> String {
        "".to_string()
    }
    // Called when events are flushed
    fn on_event_(&mut self, cx: &mut Context, event: &mut Event) {}
}

// impl dyn EventHandler {
//     // Check if a message is a certain type
//     pub fn is<T: EventHandler + 'static>(&self) -> bool {
//         // Get TypeId of the type this function is instantiated with
//         let t = TypeId::of::<T>();

//         // Get TypeId of the type in the trait object
//         let concrete = self.type_id();

//         // Compare both TypeIds on equality
//         t == concrete
//     }

//     // Casts a message to the specified type if the message is of that type
//     pub fn downcast<T>(&mut self) -> Option<&mut T>
//     where
//         T: EventHandler + 'static,
//     {
//         if self.is::<T>() {
//             unsafe { Some(&mut *(self as *mut dyn EventHandler as *mut T)) }
//         } else {
//             None
//         }
//     }

//     pub fn downcast_ref<T>(&self) -> Option<&T>
//     where
//         T: EventHandler + 'static,
//     {
//         if self.is::<T>() {
//             unsafe { Some(&*(self as *const dyn EventHandler as *const T)) }
//         } else {
//             None
//         }
//     }
// }
