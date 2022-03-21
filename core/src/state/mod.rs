//! # Data Binding
//!
//! Binding provides reactivity to a vizia application. Rather than sending events back and forth between widgets
//! to update local widget data, widgets can instead `bind` to application data.
//!
//! # Example
//! Fist we declare the data for our application. The [Lens] trait has been derived for the data, which allows us to bind to fields of the struct:
//! ```compile_fail
//! #[derive(Default, Lens)]
//! struct AppData {
//!     some_data: bool,
//! }
//!
//! ```
//! Next we'll declare some events which will be sent by widgets to modify the app data. Data binding in vizia is one-way, events are sent up the tree
//! to the app data to mutate it and updated values are sent to observers, such as a [`Binding`] view.
//! ```
//! enum AppEvent {
//!     SetTrue,
//!     SetFalse,
//! }
//! ```
//! Next we implement the [`Model`] trait on our app data, which allows us to modify the data in response to an `Event`:
//! ```compile_fail
//! impl Model for AppData {
//!     fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
//!         if let Some(app_event) = event.message.downcast() {
//!             match app_event {
//!                 AppEvent::SetTrue => {
//!                     self.some_data = true;
//!                 }
//!
//!                 AppEvent::SetFalse => {
//!                     self.some_data = false;
//!                 }
//!             }   
//!         }
//!     }
//! }
//! ```
//! This trait also allows data to be built into the [Tree]:
//! ```compile_fail
//! fn main() {
//!     Application::new(WindowDescription::new(), |cx|{
//!         AppData::default().build(cx);
//!     }).run();  
//! }
//! ```
//! A [`Binding`] view is one way in which data can be used by widgets. A [`Lens`] is used to determine what data the binding should react to:
//! ```compile_fail
//! fn main() {
//!     Application::new(WindowDescription::new(), |cx|{
//!         AppData::default().build(cx);
//!
//!         Binding::new(cx, AppData::some_data, |cx, some_data|{
//!             Label::new(cx, &some_data.get(cx).to_string());
//!         });
//!     }).run();
//! }
//! ```
//! The second parameter to the [Binding] view is a [Lens], allowing us to bind to some field of the application data.
//! The third parameter is a closure which provides the context and the lens, which can be used to retrieve the bound data using the `.get()`
//! method, which takes the [Context] as an argument.
//!
//! Now when the data is modified by another widget, the label will update, for example:
//! ```compile_fail
//! fn main() {
//!     Application::new(WindowDescription::new(), |cx|{
//!         AppData::default().build(cx);
//!
//!         Binding::new(cx, AppData::some_data, |cx, some_data|{
//!             Label::new(cx, &some_data.get(cx).to_string());
//!         });
//!
//!         Checkbox::new(cx, false)
//!             .on_checked(cx, |cx| cx.emit(AppEvent::SetTrue))
//!             .on_unchecked(cx, |cx| cx.emit(AppEvent::SetFalse));
//!     }).run();
//! }
//! ```
//! Note, the checkbox does not need to be bound to the data to send an event to it. By default events will propagate up the tree.
//!
mod lens;
pub use lens::*;

mod model;
pub use model::*;

mod store;
pub(crate) use store::*;

mod data;
pub use data::*;

mod binding;
pub use binding::*;

mod res;
pub(crate) use res::*;
