mod helpers;
use helpers::*;
use vizia::prelude::*;

struct AppData {
    rating1: Signal<u32>,
    rating2: Signal<u32>,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetRating1(val) => self.rating1.set(*val),
            AppEvent::SetRating2(val) => self.rating2.set(*val),
        })
    }
}

enum AppEvent {
    SetRating1(u32),
    SetRating2(u32),
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let rating1 = Signal::new(3);
        let rating2 = Signal::new(7);

        AppData { rating1, rating2 }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Rating::new(cx, 5, rating1)
                .on_change(|ex, rating| ex.emit(AppEvent::SetRating1(rating)));
            Rating::new(cx, 10, rating2)
                .on_change(|ex, rating| ex.emit(AppEvent::SetRating2(rating)));
        });
    })
    .title("Rating")
    .inner_size((400, 200))
    .run()
}
