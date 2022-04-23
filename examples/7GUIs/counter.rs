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
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => {
                self.count += 1;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        
        AppData { count: 0 }.build(cx);

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| cx.emit(AppEvent::Increment), |cx| Label::new(cx, "Increment"));

            Label::new(cx, AppData::count).width(Pixels(50.0));
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(50.0));
    })
    .title(AppData::count.map(|val| format!("Counter: {}", val)))
    .inner_size(400, 100)
    .run();
}
