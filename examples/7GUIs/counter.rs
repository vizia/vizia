use vizia::*;

#[derive(Lens)]
pub struct AppData {
    count: i32,
}

pub enum AppEvent {
    Increment,
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::Increment => {
                    self.count += 1;
                }
            }
        }
    }
}

fn main() {
    let window_description =
        WindowDescription::new().with_title("Counter").with_inner_size(400, 100);
    Application::new(window_description, |cx| {
        AppData { count: 0 }.build(cx);

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| cx.emit(AppEvent::Increment), |cx| Label::new(cx, "Increment"));

            Binding::new(cx, AppData::count, |cx, count| {
                Label::new(cx, count).width(Pixels(50.0));
            });
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(50.0));
    })
    .run();
}
