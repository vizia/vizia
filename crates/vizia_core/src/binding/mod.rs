//! Data binding provides a way to link views to model data so that view properties update when data changes.
//!
//! # Example
//! Fist we declare some data for our application. The [Lens] trait has been derived for the data, which allows us to bind to fields of the struct:
//! ```
//! # use vizia_core::prelude::*;
//! # use vizia_derive::*;
//! #[derive(Lens)]
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
//! # use vizia_derive::*;
//! #[derive(Lens)]
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
//! ```no_run
//! # use vizia_core::prelude::*;
//! # use vizia_derive::*;
//! # use vizia_winit::application::Application;
//! #[derive(Lens)]
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
//! A [`Binding`] view is one way in which data can be used by views. A [`Lens`] is used to determine what data the binding should react to:
//! ```no_run
//! # use vizia_core::prelude::*;
//! # use vizia_derive::*;
//! # use vizia_winit::application::Application;
//! #[derive(Lens)]
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
//!
//!         Binding::new(cx, AppData::count, |cx, count|{
//!             Label::new(cx, &count.get(cx).to_string());
//!         });
//!     }).run();
//! }
//! ```
//! The second parameter to the [Binding] view is a [Lens], allowing us to bind to some field of the application data.
//! The third parameter is a closure which provides the context and the lens, which can be used to retrieve the bound data using the `.get()`
//! method, which takes the [Context](crate::prelude::Context) as an argument.
//!
//! Now when the data is modified, the binding will rebuild its contents and the label will update, for example:
//! ```no_run
//! # use vizia_core::prelude::*;
//! # use vizia_derive::*;
//! # use vizia_winit::application::Application;
//! #[derive(Lens)]
//! struct AppData {
//!     count: i32,
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
//!
//! enum AppEvent {
//!     Increment,
//!     Decrement,
//! }
//!
//! fn main() {
//!     Application::new(|cx|{
//!         AppData {
//!             count: 0,
//!         }.build(cx);
//!
//!         Binding::new(cx, AppData::count, |cx, count|{
//!             Label::new(cx, &count.get(cx).to_string());
//!         });
//!
//!         Button::new(cx, |cx| cx.emit(AppEvent::Increment), |cx|{
//!             Label::new(cx, "Increment")
//!         });
//!
//!         Button::new(cx, |cx| cx.emit(AppEvent::Increment), |cx|{
//!             Label::new(cx, "Decrement")
//!         });
//!     }).run();
//! }
//! ```
//! Note, the checkbox does not need to be bound to the data to send an event to it. By default events will propagate up the tree.
//!
//! Completely rebuilding the `Label` when the data changes is unnecessary in this case. Instead we can update just the text of the label
//! by binding the `text()` property modifier to the application data. This is called a property binding.
//! ```no_run
//! # use vizia_core::prelude::*;
//! # use vizia_winit::application::Application;
//!
//! # #[derive(Lens)]
//! # struct AppData {
//! #     count: i32,
//! # }
//! # impl Model for AppData {
//! #     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
//! #         event.map(|app_event, _| match app_event {
//! #             AppEvent::Increment => {
//! #                 self.count += 1;
//! #             }
//! #             AppEvent::Decrement => {
//! #                 self.count -= 1;
//! #             }
//! #         });
//! #     }
//! # }
//! # enum AppEvent {
//! #     Increment,
//! #     Decrement,
//! # }
//! fn main() {
//!     Application::new(|cx|{
//!         AppData {
//!             count: 0,
//!         }.build(cx);
//!
//!         Label::new(cx, AppData::count);
//!
//!         Button::new(cx, |cx| cx.emit(AppEvent::Increment), |cx|{
//!             Label::new(cx, "Increment")
//!         });
//!
//!         Button::new(cx, |cx| cx.emit(AppEvent::Increment), |cx|{
//!             Label::new(cx, "Decrement")
//!         });
//!     }).run();
//! }
//! ```
//!
//! Note that even though the `count` value is `i32`, the label accepts a lens to this data because it implements `ToString` and is converted internally.
//! If the data is the wrong type and cannot be converted internally, use the [`map()`](crate::binding::LensExt::map) method on the lens.
mod lens;
pub use lens::*;

mod store;
pub(crate) use store::*;

mod binding_view;
pub use binding_view::*;

mod data;
pub use data::*;

mod res;
pub use res::*;

mod ray;
pub use ray::*;

mod map;
pub(crate) use map::MapId;
