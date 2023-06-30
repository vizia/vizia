//! Events for communicating state change to views and models.
//!
//! Views and Models communicate with each other via events. An [Event] contains a message, as well as metadata to describe how events
//! should propagate through the tree. By default events will propagate up the tree from the target, visiting each ancestor as well as
//! any models attached to the ancestors.
//!
//! A message can be any static thread-safe type but is usually an enum. For example:
//! ```
//! enum AppEvent {
//!     Increment,
//!     Decrement,    
//! }
//! ```
//! Then, to send an event up the tree from the current entity, we can use `cx.emit(...)`, for example:
//! ```no_run
//! # use vizia_core::prelude::*;
//! # use vizia_winit::application::Application;
//! # let cx = &mut Context::default();
//! pub struct AppData {
//!     count: i32,
//! }
//!
//! impl Model for AppData {}
//!
//! pub enum AppEvent {
//!     Increment,
//!     Decrement,    
//! }
//!
//! Application::new(|cx|{
//!     AppData {
//!         count: 0,
//!     }.build(cx);
//!
//!     Label::new(cx, "Increment")
//!         .on_press(|cx| cx.emit(AppEvent::Increment));
//! })
//! .run();
//! ```
//!
//! Views and Models receive events through the `event()` method of the View or Model traits.
//! The event message must then be downcast to the right type using the [`map`](Event::map) or [`take`](Event::take) methods on the event:
//! ```no_run
//! # use vizia_core::prelude::*;
//! # let cx = &mut Context::default();
//! # use vizia_winit::application::Application;
//!
//! pub struct AppData {
//!     count: i32,
//! }
//!
//! pub enum AppEvent {
//!     Increment,
//!     Decrement,    
//! }
//!
//! impl Model for AppData {
//!     fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
//!         // `event.map()` will attempt to cast the event message to the desired type and
//!         // pass a reference to the message type to the closure passed to the `map()` method.
//!         event.map(|app_event, _| match app_event {
//!             AppEvent::Increment => {
//!                 self.count += 1;
//!             }
//!
//!             AppEvent::Decrement => {
//!                 self.count -= 1;
//!             }
//!         });
//!     
//!         // Alternatively, `event.take()` will attempt to cast the event message to the
//!         // desired type and return the value of the message (not a reference),
//!         // removing it from the event and thus preventing it from propagating further.
//!         if let Some(app_event) = event.take() {
//!             match app_event {
//!                 AppEvent::Increment => {
//!                     self.count += 1;
//!                 }
//!
//!                 AppEvent::Decrement => {
//!                     self.count -= 1;
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

mod event_manager;
pub(crate) use event_manager::EventManager;

mod event;
pub(crate) use event::TimedEvent;
pub use event::{Event, EventMeta, Propagation, TimedEventHandle};

mod event_handler;
pub(crate) use event_handler::ViewHandler;

mod timer;
pub(crate) use timer::TimerState;
pub use timer::{Timer, TimerAction};

pub use crate::window::WindowEvent;
