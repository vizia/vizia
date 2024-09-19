use crate::{icons::ICON_X, prelude::*};
use std::sync::Arc;

/// A visual indicator such as a tag.
#[derive(Lens)]
pub struct Chip {
    on_close: Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>,
}

impl Chip {
    /// Creates a new Chip view.
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
                Label::new(cx, text)
                    .height(Stretch(1.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Stretch(1.0))
                    .top(Pixels(0.0))
                    .bottom(Pixels(0.0));
                Binding::new(cx, Self::on_close.map(Option::is_some), |cx, val| {
                    if val.get(cx) {
                        let on_close = Self::on_close.get(cx).unwrap();
                        Button::new(cx, |cx| Svg::new(cx, ICON_X))
                            .class("close-icon")
                            .height(Pixels(16.0))
                            .width(Pixels(16.0))
                            .child_space(Stretch(1.0))
                            .on_press(move |cx| (on_close)(cx));
                    }
                });
            })
            .layout_type(LayoutType::Row)
    }
}

impl View for Chip {
    fn element(&self) -> Option<&'static str> {
        Some("chip")
    }
}

/// Used in conjunction with the `variant`modifier for selecting the style variant of a chip.
#[derive(Debug, Clone, Copy, Data, PartialEq, Eq)]
pub enum ChipVariant {
    Filled,
    Outline,
}

impl_res_simple!(ChipVariant);

impl<'a> Handle<'a, Chip> {
    pub fn on_close(self, callback: impl 'static + Fn(&mut EventContext) + Send + Sync) -> Self {
        self.modify(|chip: &mut Chip| {
            chip.on_close = Some(Arc::new(callback));
        })
    }

    /// Selects the style variant to be used by the chip.
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
