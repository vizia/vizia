use std::marker::PhantomData;

use crate::Units::*;
use crate::{
    Actions, Binding, Button, Color, Context, Data, Handle, Label, Lens, Model, Overflow, Popup,
    PopupData, PopupEvent, View,
};

pub struct Picker<L>
where
    L: Lens,
{
    lens: PhantomData<L>,
}

impl<L> Picker<L>
where
    L: 'static + Lens,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
{
    pub fn new<F>(cx: &mut Context, lens: L, builder: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, L),
    {
        Self { lens: PhantomData::default() }
            .build2(cx, move |cx| {
                Binding::new(cx, lens, move |cx, option| {
                    (builder)(cx, option);
                });
            })
            .size(Auto)
    }
}

impl<L: Lens> View for Picker<L> {
    fn element(&self) -> Option<String> {
        Some("picker".to_string())
    }
}

#[derive(Debug)]
pub enum PickerEvent<T: std::fmt::Debug> {
    SetOption(T),
}

pub struct PickerItem {}

impl PickerItem {
    pub fn new<'a, T: 'static + std::fmt::Debug + Clone + PartialEq + Send>(
        cx: &'a mut Context,
        text: &'static str,
        option: T,
        value: T,
    ) -> Handle<'a, Self> {
        Self {}
            .build2(cx, move |cx| {
                let opt = option.clone();
                Button::new(
                    cx,
                    move |cx| cx.emit(PickerEvent::SetOption(option.clone())),
                    move |cx| Label::new(cx, text),
                )
                .background_color(if value == opt {
                    Color::red()
                } else {
                    Color::blue()
                });
            })
            .height(Auto)
    }
}

impl View for PickerItem {}

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
                //if cx.data::<PopupData>().is_none() {
                    PopupData::default().build(cx);
                //}

                (label)(cx).class("title").on_press(|cx| cx.emit(PopupEvent::Switch));

                Popup::new(cx, move |cx| {
                    (builder)(cx);
                })
                .top(Percentage(100.0))
                .width(Stretch(1.0))
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
