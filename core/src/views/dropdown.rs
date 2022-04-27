use crate::{Actions, Context, Handle, Model, Popup, PopupData, PopupEvent, Units::*, View};

pub const ICON_DOWN_OPEN: &str = "\u{e75c}";

pub struct Dropdown {}

impl Dropdown {
    pub fn new<F, L, V>(cx: &mut Context, label: L, content: F) -> Handle<Self>
    where
        L: 'static + Fn(&mut Context) -> Handle<V>,
        F: 'static + Fn(&mut Context),
        V: 'static + View,
    {
        Self {}
            .build(cx, move |cx| {
                PopupData::default().build(cx);

                (label)(cx)
                    .class("title")
                    .width(Stretch(1.0))
                    .on_press(|cx| cx.emit(PopupEvent::Switch));

                Popup::new(cx, PopupData::is_open, move |cx| {
                    (content)(cx);
                })
                .something(|cx| cx.emit(PopupEvent::Close))
                .top(Percentage(100.0))
                .height(Auto);
            })
            .size(Auto)
    }
}

impl View for Dropdown {
    fn element(&self) -> Option<String> {
        Some(String::from("dropdown"))
    }
}
