use crate::prelude::*;

/// A simple progress bar that can be used to show the progress of something.
///
/// The input source should resolve to an [f32] in the range `0.0..1.0`.
///
/// # Example
///
/// ### Vertical progress bar bound to a value source
/// ```ignore
/// # use vizia_core::prelude::*;
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
/// ### Horizontal progress bar bound to a value source
/// ```ignore
/// # use vizia_core::prelude::*;
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
/// ### A horizontal progress bar with a label beside it
/// ```ignore
/// # use vizia_core::prelude::*;
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

    fn accessibility(&self, _cx: &mut AccessContext, node: &mut AccessNode) {
        node.set_min_numeric_value(0.0);
        node.set_max_numeric_value(1.0);
    }
}

impl ProgressBar {
    /// Creates a new progress bar bound to the provided value source.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Lens, Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # AppData::default().build(cx);
    /// ProgressBar::new(cx, AppData::value, Orientation::Horizontal);
    /// ```
    pub fn new<L>(cx: &mut Context, signal: L, orientation: Orientation) -> Handle<Self>
    where
        L: SignalGet<f32> + SignalMap<f32>,
    {
        match orientation {
            Orientation::Horizontal => Self::horizontal(cx, signal),
            Orientation::Vertical => Self::vertical(cx, signal),
        }
    }

    /// Creates a new horizontal progress bar bound to the provided value source.
    pub fn horizontal<L>(cx: &mut Context, signal: L) -> Handle<Self>
    where
        L: SignalGet<f32> + SignalMap<f32>,
    {
        Self.build(cx, |cx| {
            let progress = signal.map(|v| Units::Percentage(v * 100.0));
            Element::new(cx).width(progress).class("progressbar-bar");
        })
        .role(Role::ProgressIndicator)
        .numeric_value(lens.map(|val| *val as f64))
    }

    /// Creates a new vertical progress bar bound to the provided value source.
    pub fn vertical<L>(cx: &mut Context, signal: L) -> Handle<Self>
    where
        L: SignalGet<f32> + SignalMap<f32>,
    {
        Self.build(cx, |cx| {
            let progress = signal.map(|v| Units::Percentage(v * 100.0));
            Element::new(cx).top(Stretch(1.0)).height(progress).class("progressbar-bar");
        })
        .role(Role::ProgressIndicator)
        .numeric_value(lens.map(|val| *val as f64))
    }
}
