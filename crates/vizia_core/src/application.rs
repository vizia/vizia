use crate::{
    context::{Context, EventContext},
    events::Event,
    model::Model,
};
use std::any::Any;

/// Opaque window configuration returned by [`App::window_config`].
///
/// Create instances using the `window()` helper from the prelude, which takes
/// a closure that receives and returns the Application with window modifiers applied.
pub struct WindowConfig(pub Box<dyn Any>);

impl WindowConfig {
    /// Creates a no-op window configuration.
    pub fn none() -> Self {
        Self(Box::new(()))
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self::none()
    }
}

/// Trait for defining application-level state and UI structure.
///
/// This provides a more structured approach to application setup compared to
/// the closure-based `Application::new()` method, while enabling application-level
/// state management with signals.
///
/// # Basic Example
///
/// ```ignore
/// use vizia::prelude::*;
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
///             Label::new(cx, self.count);
///             Button::new(cx, |cx| Label::new(cx, "+"))
///                 .on_press(|cx| cx.emit(CounterEvent::Increment));
///         });
///         self
///     }
///
///     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
///         event.map(|e, _| match e {
///             CounterEvent::Increment => self.count.upd(cx, |c| *c += 1),
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
///
/// # Reactive Window Properties
///
/// Override `window()` to configure window modifiers with access to signals:
///
/// ```ignore
/// struct MyApp {
///     title: Signal<String>,
/// }
///
/// impl App for MyApp {
///     fn new(cx: &mut Context) -> Self {
///         Self { title: cx.state(String::from("My App")) }
///     }
///
///     fn on_build(self, cx: &mut Context) -> Self {
///         // UI that might update self.title...
///         self
///     }
///
///     fn window(&self, app: Application) -> Application {
///         app.title(self.title)
///            .inner_size((800, 600))
///     }
/// }
///
/// fn main() -> Result<(), ApplicationError> {
///     MyApp::create().run()  // Window config auto-applied!
/// }
/// ```
pub trait App: Sized + 'static {
    /// Returns the application name used for persistence storage path.
    ///
    /// Override this to set a custom app name. Defaults to "vizia_app".
    /// This is called before `new()` to configure persistence.
    ///
    /// ```ignore
    /// fn app_name() -> &'static str {
    ///     "my_awesome_app"
    /// }
    /// ```
    fn app_name() -> &'static str {
        "vizia_app"
    }

    /// Initialize application-level state.
    fn new(cx: &mut Context) -> Self;

    /// Build the application's UI structure.
    ///
    /// This method receives the application instance and should construct
    /// the main UI hierarchy. The application instance is returned to
    /// maintain ownership.
    fn on_build(self, cx: &mut Context) -> Self;

    /// Handle application-level events.
    /// This method can be overridden to respond to events at the application level.
    fn event(&mut self, _cx: &mut EventContext, _event: &mut Event) {
        // Default implementation does nothing
    }

    /// Configure window properties with access to signals.
    ///
    /// Override this method to set window title, size, position, etc.
    /// Use the `window()` helper from the prelude to create the config:
    ///
    /// ```ignore
    /// fn window_config(&self) -> WindowConfig {
    ///     window(|app| {
    ///         app.title(self.title_signal)
    ///            .inner_size((800, 600))
    ///     })
    /// }
    /// ```
    fn window_config(&self) -> WindowConfig {
        WindowConfig::default()
    }
}

impl<T: App> Model for T {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        App::event(self, cx, event);
    }
}
