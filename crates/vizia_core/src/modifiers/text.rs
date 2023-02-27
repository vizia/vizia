use super::internal;
use crate::{prelude::*, style::SystemFlags};
use cosmic_text::{FamilyOwned, Weight};

/// Modifiers for changing the text properties of a view.
pub trait TextModifiers: internal::Modifiable {
    /// Sets the text content of the view.
    fn text<U: ToString>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, val| {
            let text_data = val.to_string();
            cx.text_context.set_text(entity, &text_data);

            cx.style.needs_text_layout.insert(entity, true).unwrap();
        });

        self
    }

    modifier!(
        /// Sets the font that should be used by the view.
        ///
        /// The font name refers to the name assigned when the font is added to context.
        font_family,
        Vec<FamilyOwned>,
        SystemFlags::REFLOW
    );

    modifier!(
        /// Sets the font weight that should be used by the view.
        font_weight,
        Weight,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the font style that should be used by the view.
        font_style,
        FontStyle,
        SystemFlags::REDRAW
    );

    /// Sets the text color of the view.
    fn color<U: Into<Color>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            cx.style.font_color.insert(entity, v.into());
            cx.style.needs_redraw();
        });
        self
    }

    /// Sets the font size of the view.
    fn font_size(mut self, value: impl Res<f32>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            cx.style.font_size.insert(entity, v);
            cx.style.needs_text_layout.insert(entity, true).unwrap();
        });
        self
    }

    modifier!(
        /// Sets the ext caret color of the view.
        caret_color,
        Color,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the color used to highlight selected text within the view.
        selection_color,
        Color,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets whether the text of the view should be allowed to wrap.
        text_wrap,
        bool,
        SystemFlags::REFLOW
    );
}

impl<'a, V> TextModifiers for Handle<'a, V> {}
