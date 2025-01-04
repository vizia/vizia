use crate::prelude::*;

/// A simple progress bar that can be used to show the progress of something.
///
/// The input lens need to be a lens to an [f32] with range of `0.0..1.0`.
///
/// # Example
///
/// ### Vertical ProgressBar bound to the input lens
/// ```
/// # use vizia_core::prelude::*;
/// # use vizia_derive::*;
/// # let mut cx = &mut Context::default();
/// # #[derive(Lens, Default)]
/// # pub struct AppData {
/// #     progress: f32,
/// # }
/// # impl Model for AppData {}
/// # AppData::default().build(cx);
/// ProgressBar::vertical(cx, AppData::progress);
/// ```
///
/// ### Horizontal ProgressBar bound to the input lens
/// ```
/// # use vizia_core::prelude::*;
/// # use vizia_derive::*;
/// # let mut cx = &mut Context::default();
/// # #[derive(Lens, Default)]
/// # pub struct AppData {
/// #     progress: f32,
/// # }
/// # impl Model for AppData {}
/// # AppData::default().build(cx);
/// ProgressBar::horizontal(cx, AppData::progress);
/// ```
///
/// ### A Horizontal ProgressBar with a label beside it to show the progress
/// ```
/// # use vizia_core::prelude::*;
/// # use vizia_derive::*;
/// # let mut cx = &mut Context::default();
/// # #[derive(Lens, Default)]
/// # pub struct AppData {
/// #     progress: f32,
/// # }
/// # impl Model for AppData {}
/// # AppData::default().build(cx);
/// HStack::new(cx, |cx| {
///     ProgressBar::horizontal(cx, AppData::progress);
///     Label::new(cx, AppData::progress.map(|v| format!("{:.0}%", v * 100.0)));
/// });
/// ```
pub struct ProgressBar;

impl View for ProgressBar {
    fn element(&self) -> Option<&'static str> {
        Some("progressbar")
    }
}

impl ProgressBar {
    /// Creates a new progress bar bound to the value targeted by the lens.
    ///
    /// # Example
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_derive::*;
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Lens, Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # AppData::default().build(cx);
    /// ProgressBar::new(cx, AppData::value, Orientation::Horizontal);
    /// ```
    pub fn new<L>(cx: &mut Context, lens: L, orientation: Orientation) -> Handle<Self>
    where
        L: Lens<Target = f32>,
    {
        match orientation {
            Orientation::Horizontal => Self::horizontal(cx, lens),
            Orientation::Vertical => Self::vertical(cx, lens),
        }
    }

    /// Creates a new horizontal progress bar bound to the value targeted by the lens.
    pub fn horizontal<L>(cx: &mut Context, lens: L) -> Handle<Self>
    where
        L: Lens<Target = f32>,
    {
        Self.build(cx, |cx| {
            let progress = lens.map(|v| Units::Percentage(v * 100.0));
            Element::new(cx).width(progress).class("progressbar-bar");
        })
    }

    /// Creates a new vertical progress bar bound to the value targeted by the lens.
    pub fn vertical<L>(cx: &mut Context, lens: L) -> Handle<Self>
    where
        L: Lens<Target = f32>,
    {
        Self.build(cx, |cx| {
            let progress = lens.map(|v| Units::Percentage(v * 100.0));
            Element::new(cx).top(Stretch(1.0)).height(progress).class("progressbar-bar");
        })
    }
}
