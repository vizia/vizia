use vizia_core::{application::App, model::Model};

use crate::application::{Application, ApplicationError};

/// A trait for creating Winit-based applications from `App` implementations.
///
/// This trait is automatically implemented for any type that implements both
/// [`App`] and [`Model`], providing a bridge between your application logic
/// and the Winit window system.
///
/// # Usage
///
/// 1. Create a struct with `Signal<T>` fields for reactive state
/// 2. Implement `App` trait with `new()`, `on_build()`, and optionally `event()`
/// 3. Call `YourApp::create()` to get an `Application` that can be configured and run
///
/// # Example
///
/// ```rust
/// use vizia::prelude::*;
///
/// #[derive(Debug, Clone)]
/// enum CounterEvent { Increment, Decrement }
///
/// struct CounterApp {
///     count: Signal<i32>,
/// }
///
/// impl App for CounterApp {
///     fn new(cx: &mut Context) -> Self {
///         Self { count: cx.state(0) }
///     }
///
///     fn on_build(self, cx: &mut Context) -> Self {
///         VStack::new(cx, |cx| {
///             Label::new(cx, self.count.map(|c| format!("Count: {}", c)));
///             
///             HStack::new(cx, |cx| {
///                 Button::new(cx, |cx| Label::new(cx, "-"))
///                     .on_press(|cx| cx.emit(CounterEvent::Decrement));
///                 Button::new(cx, |cx| Label::new(cx, "+"))
///                     .on_press(|cx| cx.emit(CounterEvent::Increment));
///             });
///         });
///         self
///     }
///
///     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
///         event.map(|counter_event, _| match counter_event {
///             CounterEvent::Increment => self.count.update(cx, |c| *c += 1),
///             CounterEvent::Decrement => self.count.update(cx, |c| *c -= 1),
///         });
///     }
/// }
///
/// fn main() -> Result<(), ApplicationError> {
///     CounterApp::create()
///         .title("Counter")
///         .inner_size((300, 200))
///         .run()
/// }
/// ```
pub trait WinitApp: Sized + 'static {
    /// Creates a new Winit application from an `App` implementation.
    ///
    /// Returns an `Application` that can be configured with window settings
    /// like title, size, etc., then run with `.run()`.
    fn create() -> Application;
}

impl<T: App + Model> WinitApp for T {
    fn create() -> Application {
        Application::new(|cx| {
            let mut app = T::new(cx);
            app = app.view(cx);
            app.build(cx);
        })
    }
}
