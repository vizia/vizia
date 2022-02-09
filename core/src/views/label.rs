use crate::{Context, Handle, Res, View};

pub struct Label;

impl Label {
    // pub fn new<'a>(cx: &mut Context, text: impl LocalizedStringKey<'a>) -> Handle<Self> {
    //     Self {}.build2(cx, |_| {}).text(text.key())
    // }

    pub fn new<'a, T>(cx: &'a mut Context, text: impl Res<T>) -> Handle<'a, Self>
    where
        T: ToString,
    {
        let txt = text.get_ref(cx).to_string();
        Self {}.build2(cx, |_| {}).text(&txt)
    }
}

impl View for Label {
    fn element(&self) -> Option<String> {
        Some("label".to_string())
    }
}
