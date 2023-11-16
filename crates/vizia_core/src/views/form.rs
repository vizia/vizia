use crate::prelude::*;

#[derive(Lens)]
pub struct FormControl {
    label_placement: FormPlacement,
}

impl FormControl {
    pub fn new<T: ToString, V: View>(
        cx: &mut Context,
        control: impl Fn(&mut Context) -> Handle<V> + 'static,
        label: impl Res<T> + Clone,
    ) -> Handle<Self> {
        Self { label_placement: FormPlacement::Start }
            .build(cx, |cx| {
                let id = cx.current().to_string();
                Label::new(cx, label.clone()).describing(&id);
                (control)(cx).id(&id);
                Label::new(cx, label).describing(&id);
            })
            .toggle_class(
                "pos-start",
                FormControl::label_placement.map(|placement| {
                    *placement == FormPlacement::Start || *placement == FormPlacement::Top
                }),
            )
            .toggle_class(
                "vertical",
                FormControl::label_placement.map(|placement| {
                    *placement == FormPlacement::Top || *placement == FormPlacement::Bottom
                }),
            )
    }
}

impl View for FormControl {
    fn element(&self) -> Option<&'static str> {
        Some("form-control")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormPlacement {
    Top,
    Start,
    Bottom,
    End,
}

impl<'a> Handle<'a, FormControl> {
    pub fn label_placement(self, placement: FormPlacement) -> Self {
        self.modify(|form_control| form_control.label_placement = placement)
    }
}

pub struct FormGroup {}

impl FormGroup {
    pub fn new<T: ToString>(
        cx: &mut Context,
        label: impl Res<T> + Clone,
        content: impl FnOnce(&mut Context),
    ) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            let has_text = !label.get(cx).to_string().is_empty();
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
