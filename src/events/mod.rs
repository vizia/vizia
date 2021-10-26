//! Events
//!
//! Widgets communicate with each other via events. An [Event] contains a [Message], as well as metadata to describe how events
//! should propagate through the tree. By default events will propagate up the tree from the target.
//!
//! A [Message] is any static type and is usually an enum. For example:
//! ```
//! enum MyEvent {
//!     ReadDocs,
//!     CloseDocs,    
//! }
//! ```
//! Then, to send an event up the tree from an entity:
//! ```
//! entity.emit(state, MyEvent::ReadDocs);
//! ```
//! Or, to send the event directly to a target widget:
//! ```
//! entity.emit_to(state, target, MyEvent::ReadDocs);
//! ```
//! 
//! Widgets receive events through the `on_event` method of the [Widget] trait. The event message must then be downcast to the right type:
//! fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
//!     if let Some(my_event) = event.message.downcast() {
//!         match my_event {
//!             MyEvent::ReadDocs => {
//!                 // Do something
//!             }
//!
//!             MyEvent::CloseDocs => {
//!                 // Do something else
//!             }
//!         }
//!     }
//! }
//! ```

mod event_manager;
pub use event_manager::EventManager;

mod event;
pub use event::{Event, Message, Propagation};

mod event_handler;
pub(crate) use event_handler::{ContainerHandler, NodeHandler};

