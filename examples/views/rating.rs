mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppData {
    rating: u32,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetRating(val) => self.rating = *val,
        })
    }
}

enum AppEvent {
    SetRating(u32),
}

fn main() {
    Application::new(|cx| {
        AppData { rating: 3 }.build(cx);

        ExamplePage::new(cx, |cx| {
            Rating::new(cx, AppData::rating)
                .on_change(|ex, rating| ex.emit(AppEvent::SetRating(rating)));
        });
    })
    .title("Rating")
    .inner_size((400, 200))
    .run();
}
