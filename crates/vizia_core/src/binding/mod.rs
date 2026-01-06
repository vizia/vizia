//! Data binding links view content to signals so that views update when state changes.
//!
//! # Example
//! A [`Binding`] rebuilds its contents whenever the observed signal changes.
//! ```no_run
//! # use vizia_core::prelude::*;
//! # use vizia_winit::application::Application;
//! fn main() {
//!     Application::new(|cx| {
//!         let count = cx.state(0i32);
//!
//!         Binding::new(cx, count, move |cx| {
//!             let text = cx.derived({
//!                 let count = count;
//!                 move |s| format!("Count: {}", count.get(s))
//!             });
//!             Label::new(cx, text);
//!         });
//!
//!         Button::new(cx, |cx| Label::static_text(cx, "Increment"))
//!             .on_press(move |cx| count.update(cx, |value| *value += 1));
//!     })
//!     .run();
//! }
//! ```
//!
//! ## Property bindings
//! If you only need to update a property (like label text), you can pass a signal directly.
//! ```no_run
//! # use vizia_core::prelude::*;
//! # use vizia_winit::application::Application;
//! fn main() {
//!     Application::new(|cx| {
//!         let count = cx.state(0i32);
//!         Label::new(cx, count);
//!     })
//!     .run();
//! }
//! ```

mod binding_view;
pub use binding_view::*;

mod data;
pub use data::*;

mod res;
pub use res::*;
