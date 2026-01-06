use crate::{icons::ICON_X, prelude::*};
use std::sync::Arc;

/// A visual indicator such as a tag.
pub struct Chip {
    on_close: Signal<Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>>,
}

impl Chip {
    /// Creates a new [Chip] view with the provided text.
    ///
    /// Accepts either a plain value or a `Signal<T>` for reactive text.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// // Static text
    /// Chip::new(cx, "Tag");
    ///
    /// // Reactive text
    /// Chip::new(cx, cx.state("Chip"));
    /// ```
    pub fn new<T>(cx: &mut Context, text: impl Res<T> + Clone + 'static) -> Handle<Self>
    where
        T: ToStringLocalized + Clone + 'static,
    {
        let on_close: Signal<Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>> = cx.state(None);
        let has_close = cx.derived({
            let on_close = on_close;
            move |store| on_close.get(store).is_some()
        });
        let layout_row = cx.state(LayoutType::Row);
        let stretch_one = cx.state(Stretch(1.0));
        let align_left = cx.state(Alignment::Left);
        let align_center = cx.state(Alignment::Center);

        Self { on_close }
            .build(cx, move |cx| {
                Label::new(cx, text).height(stretch_one).alignment(align_left);
                let close_icon = cx.state(ICON_X);
                let close_size = cx.state(Pixels(16.0));
                Binding::new(cx, on_close, move |cx| {
                    if let Some(callback) = on_close.get(cx).as_ref() {
                        let callback = callback.clone();
                        Button::new(cx, |cx| Svg::new(cx, close_icon))
                            .class("close-icon")
                            .height(close_size)
                            .width(close_size)
                            .alignment(align_center)
                            .on_press(move |cx| (callback)(cx));
                    }
                });
            })
            .toggle_class("close", has_close)
            .layout_type(layout_row)
    }
}

impl View for Chip {
    fn element(&self) -> Option<&'static str> {
        Some("chip")
    }
}

/// Used in conjunction with the `variant` modifier for selecting the style variant of a chip.
#[derive(Debug, Clone, Copy, Data, PartialEq, Eq)]
pub enum ChipVariant {
    /// A filled chip.
    Filled,
    /// A chip with no fill and just a border.
    Outline,
}

impl Handle<'_, Chip> {
    /// Set the callback triggered when the close button of the chip is pressed.
    /// The chip close button is not displayed by default. Setting this callback causes the close button to be displayed.
    pub fn on_close(self, callback: impl 'static + Fn(&mut EventContext) + Send + Sync) -> Self {
        let callback = Arc::new(callback);
        self.modify2(|chip, cx| chip.on_close.set(cx, Some(callback)))
    }

    /// Selects the style variant to be used by the chip. Accepts a `Signal<ChipVariant>`.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Chip::new(cx, "Chip")
    ///     .variant(cx.state(ChipVariant::Filled));
    /// ```
    pub fn variant(mut self, variant: Signal<ChipVariant>) -> Self {
        let is_outline = self.context().derived({
            let variant = variant;
            move |store| *variant.get(store) == ChipVariant::Outline
        });

        self.toggle_class("outline", is_outline)
    }
}
