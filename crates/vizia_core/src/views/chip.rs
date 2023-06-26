use crate::{icons::ICON_X, prelude::*};
use std::sync::Arc;

/// A visual indicator such as a tag.
#[derive(Lens)]
pub struct Chip {
    on_close: Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>,
}

impl Chip {
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
                Binding::new(cx, Chip::on_close.map(|on_close| on_close.is_some()), |cx, val| {
                    if val.get(cx) {
                        let on_close = Chip::on_close.get(cx).unwrap();
                        Label::new(cx, ICON_X)
                            .class("icon")
                            .height(Pixels(16.0))
                            .width(Pixels(16.0))
                            .right(Pixels(2.0))
                            .child_space(Stretch(1.0))
                            .on_press(move |cx| (on_close)(cx));
                    }
                });
            })
            .col_between(Pixels(4.0))
            .layout_type(LayoutType::Row)
    }
}

impl View for Chip {
    fn element(&self) -> Option<&'static str> {
        Some("chip")
    }
}

impl<'a> Handle<'a, Chip> {
    pub fn on_close(self, callback: impl 'static + Fn(&mut EventContext) + Send + Sync) -> Self {
        self.modify(|chip: &mut Chip| {
            chip.on_close = Some(Arc::new(callback));
        })
    }
}
