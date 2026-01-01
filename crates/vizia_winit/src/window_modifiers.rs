use vizia_core::{context::EventContext, prelude::Signal};

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
    /// Sets the title of the window to the given value. Accepts a `Signal<T>` where `T: ToString`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// let (app, title) = Application::new_with_state(|cx|{
    ///     let title = cx.state("Vizia Application".to_string());
    ///     // Content here
    ///     title
    /// });
    /// app.title(title).run();
    /// ```
    fn title<T: ToString>(self, title: Signal<T>) -> Self;
    /// Sets the inner size of the window to the given value. Accepts a signal which can be converted to a [`WindowSize`].
    ///
    /// The inner size is the window area excluding the window borders.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// let (app, size) = Application::new_with_state(|cx|{
    ///     let size = cx.state((300, 300));
    ///     // Content here
    ///     size
    /// });
    /// app.inner_size(size).run();
    /// ```
    fn inner_size<S: Into<WindowSize> + Clone>(self, size: Signal<S>) -> Self;
    /// Sets the minimum inner size of the window to the given value. Accepts a signal of an optional value that can be converted to a [`WindowSize`].
    ///
    /// Setting the minimum inner size to `None` removes the minimum inner size constraint from the window.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// let (app, min_size) = Application::new_with_state(|cx|{
    ///     let min_size = cx.state(Some((300, 300)));
    ///     // Content here
    ///     min_size
    /// });
    /// app.min_inner_size(min_size).run();
    /// ```
    fn min_inner_size<S: Into<WindowSize> + Clone>(self, size: Signal<Option<S>>) -> Self;
    /// Sets the maximum inner size of the window to the given value. Accepts a signal of an optional value that can be converted to a [`WindowSize`].
    ///
    /// Setting the maximum inner size to `None` removes the maximum inner size constraint from the window.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// let (app, max_size) = Application::new_with_state(|cx|{
    ///     let max_size = cx.state(Some((1000, 1000)));
    ///     // Content here
    ///     max_size
    /// });
    /// app.max_inner_size(max_size).run();
    /// ```
    fn max_inner_size<S: Into<WindowSize> + Clone>(self, size: Signal<Option<S>>) -> Self;
    /// Sets the position of the window to the given value. Accepts a signal which can be converted to a [`Position`].
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// let (app, position) = Application::new_with_state(|cx|{
    ///     let position = cx.state((100, 200));
    ///     // Content here
    ///     position
    /// });
    /// app.position(position).run();
    /// ```
    fn position<P: Into<WindowPosition> + Clone>(self, position: Signal<P>) -> Self;

    fn offset<P: Into<WindowPosition> + Clone>(self, offset: Signal<P>) -> Self;

    fn anchor<P: Into<Anchor> + Clone>(self, anchor: Signal<P>) -> Self;

    fn anchor_target<P: Into<AnchorTarget> + Clone>(self, anchor_target: Signal<P>) -> Self;

    fn parent_anchor<P: Into<Anchor> + Clone>(self, anchor: Signal<P>) -> Self;

    /// Sets whether the window can be resized. Accepts a `Signal<bool>`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// let (app, resizable) = Application::new_with_state(|cx|{
    ///     let resizable = cx.state(false);
    ///     // Content here
    ///     resizable
    /// });
    /// app.resizable(resizable).run();
    /// ```
    fn resizable(self, flag: Signal<bool>) -> Self;
    /// Sets whether the window is minimized. Accepts a `Signal<bool>`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// let (app, minimized) = Application::new_with_state(|cx|{
    ///     let minimized = cx.state(true);
    ///     // Content here
    ///     minimized
    /// });
    /// app.minimized(minimized).run();
    /// ```
    fn minimized(self, flag: Signal<bool>) -> Self;
    /// Sets whether the window is maximized. Accepts a `Signal<bool>`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// let (app, maximized) = Application::new_with_state(|cx|{
    ///     let maximized = cx.state(true);
    ///     // Content here
    ///     maximized
    /// });
    /// app.maximized(maximized).run();
    /// ```
    fn maximized(self, flag: Signal<bool>) -> Self;
    /// Sets whether the window is visible. Accepts a `Signal<bool>`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// let (app, visible) = Application::new_with_state(|cx|{
    ///     let visible = cx.state(false);
    ///     // Content here
    ///     visible
    /// });
    /// app.visible(visible).run();
    /// ```
    fn visible(self, flag: Signal<bool>) -> Self;
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
