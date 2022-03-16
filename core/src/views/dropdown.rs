use crate::{
    Button, Context, Handle, Model, Overflow, Popup, PopupData, PopupEvent, Units::*, View,
};

pub const ICON_DOWN_OPEN: &str = "\u{e75c}";

pub struct Dropdown {}

impl Dropdown {
    pub fn new<F, L, Label>(cx: &mut Context, label: L, builder: F) -> Handle<Self>
    where
        L: 'static + Fn(&mut Context) -> Handle<Label>,
        F: 'static + Fn(&mut Context),
        Label: 'static + View,
    {
        Self {}
            .build2(cx, move |cx| {
                if cx.data::<PopupData>().is_none() {
                    PopupData::default().build(cx);
                }

                Button::new(
                    cx,
                    |cx| cx.emit(PopupEvent::Switch),
                    move |cx| {
                        (label)(cx).class("title")
                        //.on_press(|cx| cx.emit(PopupEvent::Switch));
                    },
                )
                .width(Stretch(1.0));

                Popup::new(cx, PopupData::is_open, move |cx| {
                    (builder)(cx);
                })
                .something(|cx| cx.emit(PopupEvent::Close))
                .top(Percentage(100.0))
                .height(Auto)
                .overflow(Overflow::Visible);
            })
            .size(Auto)
    }
}

impl View for Dropdown {
    fn element(&self) -> Option<String> {
        Some("dropdown".to_string())
    }
}
