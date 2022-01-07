use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Local Data"), |cx| {
        CustomView::new().set_value(3.14).build(cx);
    })
    .run();
}

pub struct CustomView {
    value: f32,
}

impl CustomView {
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    pub fn set_value(mut self, value: f32) -> Self {
        self.value = value;

        self
    }
}

impl View for CustomView {
    fn body(&mut self, cx: &mut Context) {
        Label::new(cx, &format!("{}", self.value));
    }
}

pub trait CustomTrait: Sized {
    fn set_value(mut self, cx: &mut Context, value: f32) -> Self {
        self
    }
}

impl<'a> CustomTrait for Handle<'a, CustomView> {
    fn set_value(mut self, cx: &mut Context, value: f32) -> Self {
        if let Some(custom_view) =
            cx.views.get_mut(&self.entity).and_then(|view| view.downcast_mut::<CustomView>())
        {
            custom_view.value = value;
        }

        self
    }
}
