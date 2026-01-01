use crate::prelude::*;

/// The Divider view provides a thin, unobtrusive line for visually separating views.
pub struct Divider {}

impl Divider {
    /// Creates a dividing line. Orientation is determined by context.
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            Element::new(cx).class("divider-line");
        })
    }

    /// Creates a horizontal dividing line.
    pub fn horizontal(cx: &mut Context) -> Handle<Self> {
        Self::new(cx).class("horizontal")
    }

    /// Creates a vertical dividing line.
    pub fn vertical(cx: &mut Context) -> Handle<Self> {
        Self::new(cx).class("vertical")
    }
}

impl View for Divider {
    fn element(&self) -> Option<&'static str> {
        Some("divider")
    }
}

impl Handle<'_, Divider> {
    /// Set the orientation of the divider. Accepts a signal to an [Orientation].
    pub fn orientation(mut self, orientation: Signal<Orientation>) -> Self {
        let is_horizontal = self.context().derived({
            let orientation = orientation;
            move |store| *orientation.get(store) == Orientation::Horizontal
        });
        let is_vertical = self.context().derived({
            let orientation = orientation;
            move |store| *orientation.get(store) == Orientation::Vertical
        });

        self.toggle_class("horizontal", is_horizontal)
            .toggle_class("vertical", is_vertical)
    }
}
