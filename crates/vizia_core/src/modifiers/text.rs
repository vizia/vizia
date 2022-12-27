use super::internal;
use crate::{prelude::*, text::Selection};
use cosmic_text::Attrs;

/// Modifiers for changing the text properties of a view.
pub trait TextModifiers: internal::Modifiable {
    /// Sets the text content of the view.
    fn text<U: ToString>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, val| {
            let text_data = val.to_string();

            cx.cosmic_context.with_buffer(entity, |buffer| {
                buffer.set_text(&text_data, Attrs::new());
            });

            cx.need_relayout();
            cx.need_redraw();

            // if let Some(prev_data) = cx.style.text.get(entity) {
            //     if prev_data != &text_data {
            //         cx.style.text.insert(entity, text_data);

            //         cx.cosmic_context.with_buffer(entity, f)

            //         cx.need_relayout();
            //         cx.need_redraw();
            //     }
            // } else {
            //     cx.style.text.insert(entity, text_data);

            //     cx.need_relayout();
            //     cx.need_redraw();
            // }
        });

        self
    }

    modifier!(
        /// Sets the font that should be used by the view.
        ///
        /// The font name refers to the name assigned when the font is added to context.
        font,
        String
    );

    /// Sets the text color of the view.
    fn color<U: Into<Color>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            cx.style.font_color.insert(entity, v.into());
        });
        self
    }

    /// Sets the font size of the view.
    fn font_size(mut self, value: impl Res<f32>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            cx.style.font_size.insert(entity, v.into());
        });
        self
    }

    modifier!(
        /// Sets the text selection of the view.
        text_selection,
        Selection
    );

    modifier!(
        /// Sets the ext caret color of the view.
        caret_color,
        Color
    );

    modifier!(
        /// Sets the color used to highlight selected text within the view.
        selection_color,
        Color
    );

    modifier!(
        /// Sets whether the text of the view should be allowed to wrap.
        text_wrap,
        bool
    );
}

impl<'a, V> TextModifiers for Handle<'a, V> {}
