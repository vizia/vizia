use vizia_core::{
    application::{App, WindowConfig},
    context::Context,
    model::Model,
};

use crate::application::{Application, ApplicationError};
use crate::window_modifiers::WindowModifiers;

/// Type alias for the window configuration closure.
type WindowConfigFn = Box<dyn FnOnce(Application) -> Application>;

/// Type alias for the idle callback closure.
type IdleCallbackFn = Box<dyn Fn(&mut Context) + 'static>;

/// Combined window and idle configuration.
pub(crate) struct WinitConfig {
    pub window: Option<WindowConfigFn>,
    pub on_idle: Option<IdleCallbackFn>,
}

/// Creates a [`WindowConfig`] from a closure that configures window properties.
///
/// Use this in your [`App::window_config`] implementation to set window title,
/// size, position, and other properties with access to signals.
///
/// # Example
///
/// ```ignore
/// impl App for MyApp {
///     fn window_config(&self) -> WindowConfig {
///         let title = self.title;  // Copy signal handle
///         window(move |app| {
///             app.title(title)
///                .inner_size((800, 600))
///                .min_inner_size(Some((400, 300)))
///         })
///     }
/// }
/// ```
///
/// # With on_idle callback
///
/// Chain `.on_idle()` for idle callbacks (e.g., native menu handling):
///
/// ```ignore
/// window(|app| app.title("My App"))
///     .on_idle(move |cx| { /* poll events */ })
/// ```
pub fn window<F>(f: F) -> WindowConfig
where
    F: FnOnce(Application) -> Application + 'static,
{
    WindowConfig(Box::new(WinitConfig {
        window: Some(Box::new(f) as WindowConfigFn),
        on_idle: None,
    }))
}

/// Extension trait for adding idle callbacks to [`WindowConfig`].
pub trait WindowConfigExt {
    /// Adds an idle callback to the window configuration.
    ///
    /// The callback is called on each event loop iteration and is useful for
    /// polling external event sources like native OS menus.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn window_config(&self) -> WindowConfig {
    ///     let menu_receiver = MenuEvent::receiver();
    ///     window(|app| app.title("My App").inner_size((800, 600)))
    ///         .on_idle(move |cx| {
    ///             while let Ok(event) = menu_receiver.try_recv() {
    ///                 handle_menu_event(cx, &event);
    ///             }
    ///         })
    /// }
    /// ```
    fn on_idle<I: Fn(&mut Context) + 'static>(self, callback: I) -> WindowConfig;
}

impl WindowConfigExt for WindowConfig {
    fn on_idle<I: Fn(&mut Context) + 'static>(self, callback: I) -> WindowConfig {
        // Try to downcast and modify the existing WinitConfig
        if let Ok(mut winit_config) = self.0.downcast::<WinitConfig>() {
            winit_config.on_idle = Some(Box::new(callback) as IdleCallbackFn);
            WindowConfig(winit_config)
        } else {
            // If not a WinitConfig, create a new one with just the idle callback
            WindowConfig(Box::new(WinitConfig {
                window: None,
                on_idle: Some(Box::new(callback) as IdleCallbackFn),
            }))
        }
    }
}

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
/// 3. Override `window_config()` to set window title, size, etc.
/// 4. Call `YourApp::run()` to start the application
///
/// # Example
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
///     fn window_config(&self) -> WindowConfig {
///         window(|app| app.title("Counter").inner_size((300, 200)))
///     }
/// }
///
/// fn main() -> Result<(), ApplicationError> {
///     CounterApp::run()
/// }
/// ```
///
/// The trait is auto-implemented for any `App + Model` type, providing a simple
/// `run()` method that handles initialization and event loop setup.
pub trait WinitApp: Sized + 'static {
    /// Runs the application.
    ///
    /// Creates the application, applies window configuration from `window_config()`,
    /// and starts the event loop.
    fn run() -> Result<(), ApplicationError>;
}

impl<T: App + Model> WinitApp for T {
    fn run() -> Result<(), ApplicationError> {
        let app_name = T::app_name();

        let (application, config) = Application::new_with_state(move |cx| {
            // Configure persistence with app name BEFORE creating state
            cx.configure_persistence(app_name);

            let mut app = T::new(cx);
            app = app.on_build(cx);
            let config = app.window_config();
            app.build(cx);
            config
        });

        // Set default window title from app_name (can be overridden by window_config)
        let application = application.title(app_name);

        // Apply window and idle configuration if provided
        let application = if let Ok(winit_config) = config.0.downcast::<WinitConfig>() {
            let mut app = application;

            // Apply window configuration (may override default title)
            if let Some(configure) = winit_config.window {
                app = configure(app);
            }

            // Apply idle callback
            if let Some(idle_callback) = winit_config.on_idle {
                app = app.on_idle(move |cx| idle_callback(cx));
            }

            app
        } else {
            application
        };

        application.run()
    }
}
