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
/// HStack::new(cx, |cx| {
///     ProgressBar::horizontal(cx, progress);
///     Label::new(cx, progress.map(|v| format!("{:.0}%", v * 100.0)));
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
    /// # Example
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// # let progress = cx.state(0.5f32);
    /// ProgressBar::new(cx, progress, Orientation::Horizontal);
    /// ```
    pub fn new<L: Res<f32>>(cx: &mut Context, value: L, orientation: Orientation) -> Handle<Self> {
        match orientation {
            Orientation::Horizontal => Self::horizontal(cx, value),
            Orientation::Vertical => Self::vertical(cx, value),
        }
    }

    /// Creates a new horizontal progress bar bound to the given value.
    pub fn horizontal<L: Res<f32>>(cx: &mut Context, value: L) -> Handle<Self> {
        Self.build(cx, |cx| {
            Element::new(cx)
                .bind(value, |handle, val| {
                    let v = val.get(&handle);
                    handle.width(Units::Percentage(v * 100.0));
                })
                .class("progressbar-bar");
        })
    }

    /// Creates a new vertical progress bar bound to the given value.
    pub fn vertical<L: Res<f32>>(cx: &mut Context, value: L) -> Handle<Self> {
        Self.build(cx, |cx| {
            Element::new(cx)
                .bind(value, |handle, val| {
                    let v = val.get(&handle);
                    handle.top(Stretch(1.0)).height(Units::Percentage(v * 100.0));
                })
                .class("progressbar-bar");
        })
    }
}
