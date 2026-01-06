use crate::prelude::*;

/// A simple progress bar that can be used to show the progress of something.
///
/// The input value should be in the range of `0.0..1.0`.
///
/// # Example
///
/// ### Vertical ProgressBar bound to a signal
/// ```
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::default();
/// # let progress = cx.state(0.5f32);
/// ProgressBar::vertical(cx, progress);
/// ```
///
/// ### Horizontal ProgressBar bound to a signal
/// ```
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::default();
/// # let progress = cx.state(0.5f32);
/// ProgressBar::horizontal(cx, progress);
/// ```
///
/// ### A Horizontal ProgressBar with a label beside it to show the progress
/// ```
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::default();
/// # let progress = cx.state(0.5f32);
/// let progress_text = cx.derived({
///     let progress = progress;
///     move |store| format!("{:.0}%", *progress.get(store) * 100.0)
/// });
/// HStack::new(cx, |cx| {
///     ProgressBar::horizontal(cx, progress);
///     Label::new(cx, progress_text);
/// });
/// ```
pub struct ProgressBar;

impl View for ProgressBar {
    fn element(&self) -> Option<&'static str> {
        Some("progressbar")
    }
}

impl ProgressBar {
    /// Creates a new progress bar bound to the given value.
    ///
    /// Accepts either a plain f32 value or a `Signal<f32>` for reactive state.
    ///
    /// # Example
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// // Static value
    /// ProgressBar::new(cx, 0.5f32, Orientation::Horizontal);
    ///
    /// // Reactive
    /// # let progress = cx.state(0.5f32);
    /// ProgressBar::new(cx, progress, Orientation::Horizontal);
    /// ```
    pub fn new(
        cx: &mut Context,
        value: impl Res<f32> + 'static,
        orientation: Orientation,
    ) -> Handle<Self> {
        match orientation {
            Orientation::Horizontal => Self::horizontal(cx, value),
            Orientation::Vertical => Self::vertical(cx, value),
        }
    }

    /// Creates a new horizontal progress bar bound to the given value.
    pub fn horizontal(cx: &mut Context, value: impl Res<f32> + 'static) -> Handle<Self> {
        let value = value.into_signal(cx);
        Self.build(cx, |cx| {
            let bar_width = cx.state(Units::Percentage(0.0));
            Element::new(cx)
                .width(bar_width)
                .bind(value, move |handle, val| {
                    let v = *val.get(&handle);
                    let mut event_cx = EventContext::new(handle.cx);
                    bar_width.set(&mut event_cx, Units::Percentage(v * 100.0));
                })
                .class("progressbar-bar");
        })
    }

    /// Creates a new vertical progress bar bound to the given value.
    pub fn vertical(cx: &mut Context, value: impl Res<f32> + 'static) -> Handle<Self> {
        let value = value.into_signal(cx);
        Self.build(cx, |cx| {
            let bar_top = cx.state(Stretch(1.0));
            let bar_height = cx.state(Units::Percentage(0.0));
            Element::new(cx)
                .top(bar_top)
                .height(bar_height)
                .bind(value, move |handle, val| {
                    let v = *val.get(&handle);
                    let mut event_cx = EventContext::new(handle.cx);
                    bar_height.set(&mut event_cx, Units::Percentage(v * 100.0));
                })
                .class("progressbar-bar");
        })
    }
}
