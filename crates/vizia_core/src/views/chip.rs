use crate::{icons::ICON_X, prelude::*};
use std::sync::Arc;

/// A visual indicator such as a tag.
#[derive(Lens)]
pub struct Chip {
    on_close: Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>,
}

impl Chip {
    /// Creates a new [Chip] view with the provided text.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Chip::new(cx, "Chip");
    /// ```
    pub fn new<T>(cx: &mut Context, text: impl Res<T> + Clone) -> Handle<Self>
    where
        T: ToStringLocalized,
    {
        Self { on_close: None }
            .build(cx, move |cx| {
                Label::new(cx, text).height(Stretch(1.0)).alignment(Alignment::Left);
                Binding::new(cx, Chip::on_close.map(|on_close| on_close.is_some()), |cx, val| {
                    if val.get(cx) {
                        let on_close = Chip::on_close.get(cx).unwrap();
                        Button::new(cx, |cx| Svg::new(cx, ICON_X))
                            .class("close-icon")
                            .height(Pixels(16.0))
                            .width(Pixels(16.0))
                            .alignment(Alignment::Center)
                            .on_press(move |cx| (on_close)(cx));
                    }
                });
            })
            .toggle_class("close", Chip::on_close.map(|on_close| on_close.is_some()))
            .layout_type(LayoutType::Row)
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

impl_res_simple!(ChipVariant);

impl Handle<'_, Chip> {
    /// Set the callback triggered when the close button of the chip is pressed.
    /// The chip close button is not displayed by default. Setting this callback causes the close button to be displayed.
    pub fn on_close(self, callback: impl 'static + Fn(&mut EventContext) + Send + Sync) -> Self {
        self.modify(|chip: &mut Chip| {
            chip.on_close = Some(Arc::new(callback));
        })
    }

    /// Selects the style variant to be used by the chip. Accepts a value of, or lens to, a [ChipVariant].
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Chip::new(cx, "Chip")
    ///     .variant(ChipVariant::Filled);
    /// ```
    pub fn variant<U: Into<ChipVariant>>(self, variant: impl Res<U>) -> Self {
        self.bind(variant, |handle, variant| {
            let variant = variant.get(&handle).into();

            match variant {
                ChipVariant::Filled => {
                    handle.toggle_class("outline", false);
                }

                ChipVariant::Outline => {
                    handle.toggle_class("outline", true);
                }
            }
        })
    }
}
