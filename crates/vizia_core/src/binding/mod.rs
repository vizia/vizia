//! Data binding provides a way to link views to model data so that view properties update when data changes.
//!
//! # Example
//! First we declare some data for our application:
//! ```
//! # use vizia_core::prelude::*;
//!
//!
//! struct AppData {
//!     count: i32,
//! }
//!
//! ```
//! Next we'll declare some events which will be sent by views to modify the data. Data binding in vizia is one-way, events are sent up the tree
//! to the app data to mutate it and updated values are sent to observers, such as a [`Binding`] view.
//! ```
//! enum AppEvent {
//!     Increment,
//!     Decrement,
//! }
//! ```
//! Then we implement the [`Model`](crate::model::Model) trait on our data, which allows us to modify the it in response to an [`Event`](crate::events::Event):
//! ```
//! # use vizia_core::prelude::*;
//!
//!
//! struct AppData {
//!     count: i32,
//! }
//!
//! enum AppEvent {
//!     Increment,
//!     Decrement,
//! }
//!
//! impl Model for AppData {
//!     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
//!         event.map(|app_event, _| match app_event {
//!             AppEvent::Increment => {
//!                 self.count += 1;
//!             }
//!
//!             AppEvent::Decrement => {
//!                 self.count -= 1;
//!             }
//!         });
//!     }
//! }
//! ```
//! This trait also allows data to be built into the application [Tree](crate::prelude::Tree):
//! ```ignore
//! # use vizia_core::prelude::*;
//!
//! # use vizia_winit::application::Application;
//!
//! struct AppData {
//!     count: i32,
//! }
//!
//! impl Model for AppData {}
//!
//! fn main() {
//!     Application::new(|cx|{
//!         AppData {
//!             count: 0,
//!         }.build(cx);
//!     }).run();  
//! }
//! ```
//! A [`Binding`] view is one way in which data can be used by views. It observes a signal and rebuilds whenever the signal changes:
//! ```ignore
//! # use vizia_core::prelude::*;
//!
//! # use vizia_winit::application::Application;
//!
//! struct AppData {
//!     count: Signal<i32>,
//! }
//!
//! impl Model for AppData {
//!     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
//!         event.map(|app_event, _| match app_event {
//!             AppEvent::Increment => {
//!                 self.count.update(|count| *count += 1);
//!             }
//!
//!             AppEvent::Decrement => {
//!                 self.count.update(|count| *count -= 1);
//!             }
//!         });
//!     }
//! }
//!
//! enum AppEvent {
//!     Increment,
//!     Decrement,
//! }
//!
//! fn main() {
//!     Application::new(|cx|{
//!         let count = Signal::new(0);
//!         AppData {
//!             count,
//!         }.build(cx);
//!
//!         Binding::new(cx, count, |cx, value|{
//!             Label::new(cx, value.to_string());
//!         });
//!
//!         Button::new(cx, |cx|{
//!             Label::new(cx, "Increment")
//!         })
//!         .on_press(|cx| cx.emit(AppEvent::Increment));
//!
//!         Button::new(cx, |cx|{
//!             Label::new(cx, "Decrement")
//!         })
//!         .on_press(|cx| cx.emit(AppEvent::Decrement));
//!     }).run();
//! }
//! ```
//! Note, the button does not need to be bound to the data to send an event to it. By default events will propagate up the tree.
//!
//! Completely rebuilding the `Label` when the data changes is unnecessary in this case. Instead we can pass a signal directly to
//! the view constructor so only the relevant property updates.
//! ```ignore
//! # use vizia_core::prelude::*;
//! # use vizia_winit::application::Application;
//!
//! fn main() {
//!     Application::new(|cx|{
//!         let count = Signal::new(0);
//!
//!         Label::new(cx, count);
//!
//!         Button::new(cx, |cx|{
//!             Label::new(cx, "Increment")
//!         })
//!         .on_press(move |_| count.update(|v| *v += 1));
//!
//!         Button::new(cx, |cx|{
//!             Label::new(cx, "Decrement")
//!         })
//!         .on_press(move |_| count.update(|v| *v -= 1));
//!     }).run();
//! }
//! ```
//!
//! Note that even though the `count` value is `i32`, the label accepts it because it implements `ToString` and is converted internally.
//! If the data is the wrong type and cannot be converted internally, use a mapped signal or a custom formatter.
mod handler;
pub(crate) use handler::BindingHandler;

mod res;
pub use res::*;

#[allow(clippy::module_inception)]
mod binding;
pub use binding::Binding;
