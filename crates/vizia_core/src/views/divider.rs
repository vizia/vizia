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
    /// Set the orientation of the divider. Accepts a value or a lens to an [Orientation].
    pub fn orientation(self, orientation: impl Res<Orientation>) -> Self {
        self.bind(orientation, move |handle, orientation| {
            let orientation = orientation.get(&handle);
            if orientation == Orientation::Horizontal {
                handle.toggle_class("horizontal", true).toggle_class("vertical", false);
            } else {
                handle.toggle_class("horizontal", false).toggle_class("vertical", true);
            }
        })
    }
}
