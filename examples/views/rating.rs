mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppData {
    rating1: u32,
    rating2: u32,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetRating1(val) => self.rating1 = *val,
            AppEvent::SetRating2(val) => self.rating2 = *val,
        })
    }
}

enum AppEvent {
    SetRating1(u32),
    SetRating2(u32),
}

fn main() {
    Application::new(|cx| {
        AppData { rating1: 3, rating2: 7 }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Rating::new(cx, 5, AppData::rating1)
                .on_change(|ex, rating| ex.emit(AppEvent::SetRating1(rating)));
            Rating::new(cx, 10, AppData::rating2)
                .on_change(|ex, rating| ex.emit(AppEvent::SetRating2(rating)));
        });
    })
    .title("Rating")
    .inner_size((400, 200))
    .run();
}
