use crate::{context::TreeProps, prelude::*};

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
            let progress = lens.map(|v| Units::Percentage(*v));
            Element::new(cx).width(progress).class("bar");
        })
    }

    /// Creates a new vertical progress bar bound to the value targeted by the lens.
    pub fn vertical<L>(cx: &mut Context, lens: L) -> Handle<Self>
    where
        L: Lens<Target = f32>,
    {
        Self.build(cx, |cx| {
            let progress = lens.map(|v| Units::Percentage(*v));
            Element::new(cx).top(Stretch(1.0)).height(progress).class("bar");
        })
    }
}

impl<'a> Handle<'a, ProgressBar> {
    /// Set the color of the bar inside the ProgressBar
    pub fn bar_color(self, color: impl Res<Color>) -> Self {
        color.set_or_bind(self.cx, self.entity, move |cx, val| {
            let first_child = cx.first_child();
            cx.with_current(first_child, |cx| {
                cx.set_background_color(val);
            })
        });

        self
    }
}
