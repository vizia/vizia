use vizia_core::{context::EventContext, prelude::Res};

pub(crate) fn apply_title_affixes(title: &str) -> String {
    let prefix = std::env::var("VIZIA_TITLE_PREFIX").ok().filter(|s| !s.is_empty());
    let suffix = std::env::var("VIZIA_TITLE_SUFFIX").ok().filter(|s| !s.is_empty());

    if prefix.is_none() && suffix.is_none() {
        return title.to_string();
    }

    let mut out = title.to_string();
    if let Some(prefix) = prefix {
        if !out.starts_with(&prefix) {
            out = format!("{prefix}{out}");
        }
    }

    if let Some(suffix) = suffix {
        if !out.ends_with(&suffix) {
            out.push_str(&suffix);
        }
    }

    out
}
use vizia_window::{Anchor, AnchorTarget, WindowButtons, WindowPosition, WindowSize};

/// Modifiers for setting the properties of a window.
pub trait WindowModifiers {
    fn on_close(self, callback: impl Fn(&mut EventContext) + 'static) -> Self;
    fn on_create(self, callback: impl Fn(&mut EventContext) + 'static) -> Self;
    /// Sets the title of the window to the given value. Accepts a value or `Signal<T>` where `T: ToString`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx| {
    ///     // Content here
    /// })
    /// .title("My Application")
    /// .run();
    /// ```
    ///
    /// For reactive titles, use the App trait:
    /// ```no_run,ignore
    /// impl App for MyApp {
    ///     fn window_config(&self) -> WindowConfig {
    ///         window(|app| app.title(self.title_signal))
    ///     }
    /// }
    /// ```
    fn title<T: ToString + 'static>(self, title: impl Res<T> + 'static) -> Self;
    /// Sets the inner size of the window to the given value. Accepts a value or signal which can be converted to a [`WindowSize`].
    ///
    /// The inner size is the window area excluding the window borders.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx| {
    ///     // Content here
    /// })
    /// .inner_size((800, 600))
    /// .run();
    /// ```
    fn inner_size<S: Into<WindowSize> + Clone>(self, size: impl Res<S>) -> Self;
    /// Sets the minimum inner size of the window to the given value. Accepts a value or signal of an optional value that can be converted to a [`WindowSize`].
    ///
    /// Setting the minimum inner size to `None` removes the minimum inner size constraint from the window.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx| {
    ///     // Content here
    /// })
    /// .min_inner_size(Some((300, 200)))
    /// .run();
    /// ```
    fn min_inner_size<S: Into<WindowSize> + Clone>(self, size: impl Res<Option<S>>) -> Self;
    /// Sets the maximum inner size of the window to the given value. Accepts a value or signal of an optional value that can be converted to a [`WindowSize`].
    ///
    /// Setting the maximum inner size to `None` removes the maximum inner size constraint from the window.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx| {
    ///     // Content here
    /// })
    /// .max_inner_size(Some((1920, 1080)))
    /// .run();
    /// ```
    fn max_inner_size<S: Into<WindowSize> + Clone>(self, size: impl Res<Option<S>>) -> Self;
    /// Sets the position of the window to the given value. Accepts a value or signal which can be converted to a [`WindowPosition`].
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx| {
    ///     // Content here
    /// })
    /// .position((100, 200))
    /// .run();
    /// ```
    fn position<P: Into<WindowPosition> + Clone>(self, position: impl Res<P>) -> Self;

    fn offset<P: Into<WindowPosition> + Clone>(self, offset: impl Res<P>) -> Self;

    fn anchor<P: Into<Anchor> + Clone>(self, anchor: impl Res<P>) -> Self;

    fn anchor_target<P: Into<AnchorTarget> + Clone>(self, anchor_target: impl Res<P>) -> Self;

    fn parent_anchor<P: Into<Anchor> + Clone>(self, anchor: impl Res<P>) -> Self;

    /// Sets whether the window can be resized. Accepts a value or `Signal<bool>`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx| {
    ///     // Content here
    /// })
    /// .resizable(false)
    /// .run();
    /// ```
    fn resizable(self, flag: impl Res<bool>) -> Self;
    /// Sets whether the window is minimized. Accepts a value or `Signal<bool>`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx| {
    ///     // Content here
    /// })
    /// .minimized(true)
    /// .run();
    /// ```
    fn minimized(self, flag: impl Res<bool>) -> Self;
    /// Sets whether the window is maximized. Accepts a value or `Signal<bool>`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx| {
    ///     // Content here
    /// })
    /// .maximized(true)
    /// .run();
    /// ```
    fn maximized(self, flag: impl Res<bool>) -> Self;
    /// Sets whether the window is visible. Accepts a value or `Signal<bool>`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx| {
    ///     // Content here
    /// })
    /// .visible(false)
    /// .run();
    /// ```
    fn visible(self, flag: impl Res<bool>) -> Self;
    /// Sets whether the window is transparent. Accepts a boolean value.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .transparent(true)
    /// .run();
    /// ```
    fn transparent(self, flag: bool) -> Self;
    /// Sets whether the window has decorations. Accepts a boolean value.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .decorations(false)
    /// .run();
    /// ```
    fn decorations(self, flag: bool) -> Self;
    /// Sets whether the window should be on top of other windows. Accepts a boolean value.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .always_on_top(true)
    /// .run();
    /// ```
    fn always_on_top(self, flag: bool) -> Self;
    /// Sets whether the window has vsync enabled.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .vsync(true)
    /// .run();
    /// ```
    fn vsync(self, flag: bool) -> Self;
    /// Sets the icon used for the window.
    ///
    /// # Example
    /// ```no_run, ignore
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    ///
    /// let icon = vizia::image::load_from_memory(include_bytes!("../icon.png"))
    ///     .expect("Failed to load icon");
    ///
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .icon(icon.width(), icon.height(), icon.into_bytes())
    /// .run();
    /// ```
    fn icon(self, width: u32, height: u32, image: Vec<u8>) -> Self;

    fn enabled_window_buttons(self, window_buttons: WindowButtons) -> Self;
}
