//! Events
//!
//! Views communicate with each other and model data via events. An [Event] contains a [Message], as well as metadata, [EventMeta], to describe the origin and target
//! of an event, as well as how it should propagate through the tree. By default events will propagate up the tree from the target to the root from ancestor to ancestor.
//!
//! A [Message] can be any static type but is usually an enum. For example:
//! ```
//! enum MyEvent {
//!     ReadDocs,
//!     CloseDocs,    
//! }
//! ```
//! Then, to send an event up the tree from the current view:
//! ```ignore
//! cx.emit(MyEvent::ReadDocs);
//! ```
//! Or, to send an event from the current entity directly to a target:
//! ```ignore
//! cx.emit_to(target, MyEvent::ReadDocs);
//! ```
//!
//! Views and Models receive events through the `event()` method of the View or Model traits.
//! The event message must then be converted to the right type with the `map` method:
//! ```ignore
//! fn on_event(&mut self, cx: &mut EventContext, event: &mut Event) {
//!     event.map(|my_event, _| match my_event {
//!         MyEvent::ReadDocs => {
//!             // Do something
//!         }
//!
//!         MyEvent::CloseDocs => {
//!             // Do something else
//!         }
//!     });
//! }
//! ```

// TODO: Add part to docs about event metadata

mod event_manager;
pub use event_manager::EventManager;

mod event;
pub use event::{Event, EventMeta, Message, Propagation};

mod event_handler;
pub use event_handler::ViewHandler;
