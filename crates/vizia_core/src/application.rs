use crate::{
    context::{Context, EventContext},
    events::Event,
    model::Model,
};

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
///     fn view(self, cx: &mut Context) -> Self {
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
    fn view(self, cx: &mut Context) -> Self;

    /// Handle application-level events.
    /// This method can be overridden to respond to events at the application level.
    fn event(&mut self, _cx: &mut EventContext, _event: &mut Event) {
        // Default implementation does nothing
    }
}

impl<T: App> Model for T {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        App::event(self, cx, event);
    }
}
