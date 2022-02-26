use std::marker::PhantomData;

use crate::Units::*;
use crate::{Binding, Button, Color, Context, Data, Handle, Label, Lens, View};

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
