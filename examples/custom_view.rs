use vizia::prelude::*;

pub struct CustomView;

impl CustomView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            Label::new(cx, "This is a custom view!");
        })
    }
}

impl View for CustomView {
    fn element(&self) -> Option<&'static str> {
        Some("custom-view")
    }
}

fn main() {
    Application::new(|cx| {
        CustomView::new(cx);
    })
    .run();
}
