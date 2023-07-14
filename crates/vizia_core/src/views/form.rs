use crate::prelude::*;

pub struct FormControl {}

impl FormControl {
    pub fn new<T: ToString, V: View>(
        cx: &mut Context,
        control: impl Fn(&mut Context) -> Handle<V> + 'static,
        label: impl Res<T> + Clone,
    ) -> Handle<Self> {
        Self {}
            .build(cx, |cx| {
                let id = cx.current().to_string();
                (control)(cx).id(&id);
                Label::new(cx, label).describing(&id);
            })
            .layout_type(LayoutType::Row)
            .size(Auto)
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(5.0))
    }
}

impl View for FormControl {}

pub struct FormGroup {}

impl FormGroup {
    pub fn new<T: ToString>(
        cx: &mut Context,
        label: impl Res<T> + Clone,
        content: impl FnOnce(&mut Context),
    ) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            let has_text = !label.get_val(cx).to_string().is_empty();
            Label::new(cx, label.clone()).class("legend").display(has_text);

            (content)(cx);
        })
    }
}

impl View for FormGroup {
    fn element(&self) -> Option<&'static str> {
        Some("form-group")
    }
}
