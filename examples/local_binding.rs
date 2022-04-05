use vizia::*;

#[derive(Lens)]
pub struct CustomView {
    some_data: String,
}

impl CustomView {
    pub fn new<'a>(cx: &'a mut Context, txt: &str) -> Handle<'a, Self> {
        Self { some_data: txt.to_owned() }.build(cx, |cx| {
            Label::new(cx, CustomView::some_data).on_press(|cx| cx.emit(CustomEvent::ChangeData));
        })
    }
}

pub enum CustomEvent {
    ChangeData,
}

impl View for CustomView {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::ChangeData => {
                    self.some_data = "Something".to_owned();
                }
            }
        }
    }
}

fn main() {
    Application::new(WindowDescription::new(), |cx| {
        CustomView::new(cx, "Test");
    })
    .run();
}
