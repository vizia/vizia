use vizia_core::context::Context;

use crate::application::{Application, ApplicationError};

/// Trait for defining application-level state and UI structure.
///
/// This provides a more structured approach to application setup compared to
/// the closure-based `Application::new()` method, while enabling application-level
/// state management with signals.
///
/// # Examples
///
/// ```
/// use vizia::prelude::*;
///
/// struct MyApp {
///     user_settings: Signal<UserSettings>,
///     theme: Signal<Theme>,
/// }
///
/// impl App for MyApp {
///     fn new(cx: &mut Context) -> Self {
///         Self {
///             user_settings: cx.state(UserSettings::default()),
///             theme: cx.state(Theme::default()),
///         }
///     }
///
///     fn on_build(self, cx: &mut Context) -> Self {
///         VStack::new(cx, |cx| {
///             HeaderView::new(cx, self.theme);
///             MainView::new(cx, self.user_settings);
///         });
///
///         self
///     }
/// }
///
/// fn main() -> Result<(), ApplicationError> {
///     MyApp::run()
///         .title("My Application")
///         .inner_size((800, 600))
///         .run()
/// }
/// ```
pub trait App: Sized + 'static {
    /// Initialize application-level state.
    fn new(cx: &mut Context) -> Self;

    /// Build the application's UI structure.
    ///
    /// This method receives the application instance and should construct
    /// the main UI hierarchy. The application instance is returned to
    /// maintain ownership.
    fn on_build(self, cx: &mut Context) -> Self;

    /// Optional cleanup method called when the application is shutting down.
    ///
    /// Override this to perform any necessary cleanup of application-level
    /// state or resources.
    fn on_exit(&mut self, _cx: &mut Context) {}

    fn build() -> Application {
        Application::new(|cx| {
            let app = Self::new(cx);
            app.on_build(cx);
        })
    }
}
