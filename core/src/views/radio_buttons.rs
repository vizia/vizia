use crate::{Context, Entity, Event, Handle, View, WindowEvent, MouseButton, PseudoClass, Label, LayoutType};
use crate::style::PropSet;
use crate::Units::*;

pub struct RadioButtons {
    on_changed: Option<Box<dyn Fn(&mut Context, usize)>>,
}

const ICON_CHECK: &str = "\u{00B7}";

impl RadioButtons {
    pub fn with_icons<F>(
        cx: &mut Context,
        idx: usize,
        idx_count: usize,
        builder: F,
        icon_checked: String,
        icon_unchecked: String,
    ) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, usize) + Clone
    {
        Self {
            on_changed: None,
        }
            .build2(cx, move |cx| {
                for idx_available in 0..idx_count {
                    let checked = idx_available == idx;
                    let text = if checked { icon_checked.clone() } else { icon_unchecked.clone() };
                    let one_builder = builder.clone();
                    RadioButton::new(cx, idx_available, text, move |cx| {
                        (one_builder)(cx, idx_available);
                    });
                }
            })
    }

    pub fn new<F>(
        cx: &mut Context,
        idx: usize,
        idx_count: usize,
        builder: F
    ) -> Handle<Self>
        where
            F: 'static + Fn(&mut Context, usize) + Clone
    {
        Self::with_icons(cx, idx, idx_count, builder, ICON_CHECK.to_owned(), "".to_owned())
    }
}

impl Handle<RadioButtons> {
    pub fn on_changed<F>(self, cx: &mut Context, callback: F) -> Self
        where
            F: 'static + Fn(&mut Context, usize),
    {
        if let Some(view) = cx.views.get_mut(&self.entity) {
            if let Some(view) = view.downcast_mut::<RadioButtons>() {
                view.on_changed = Some(Box::new(callback));
            }
        }
        self
    }

}

impl View for RadioButtons {
    fn element(&self) -> Option<String> {
        Some("radiobuttons".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(RadioButtonSelected(idx)) = event.message.downcast() {
            if let Some(func) = self.on_changed.take() {
                (func)(cx, *idx);
                self.on_changed = Some(func);
            }
        }
    }
}

struct RadioButton {
    idx: usize,
}

impl View for RadioButton {
    fn element(&self) -> Option<String> {
        Some("radiobutton".to_owned())
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(WindowEvent::MouseDown(btn)) = event.message.downcast() {
            if *btn == MouseButton::Left {
                cx.emit(RadioButtonSelected(self.idx));
            }
        }
    }
}

impl RadioButton {
    fn new<F>(cx: &mut Context, idx: usize, icon_text: String, builder: F) -> Handle<RadioButton>
    where
        F: 'static + FnOnce(&mut Context)
    {
        RadioButton {
            idx
        }.build2(cx, move |cx| {
            let entity = cx.current;
            entity.set_layout_type(cx, LayoutType::Row);
            entity.set_height(cx, Stretch(0.0));
            Label::new(cx, &icon_text)
                .class("radioknob");
            (builder)(cx);
        })
    }
}

#[derive(Debug)]
struct RadioButtonSelected(usize);
