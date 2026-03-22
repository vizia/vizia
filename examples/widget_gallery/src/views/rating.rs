use vizia::prelude::*;

use crate::DemoRegion;

struct RatingData {
    rating: Signal<u32>,
}

impl Model for RatingData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            RatingEvent::SetRating(val) => self.rating.set(*val),
        })
    }
}

enum RatingEvent {
    SetRating(u32),
}

pub fn rating(cx: &mut Context) {
    let rating = Signal::new(3u32);
    RatingData { rating }.build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Rating");

        Divider::new(cx);

        Markdown::new(cx, "### Basic rating");

        DemoRegion::new(
            cx,
            move |cx| {
                Rating::new(cx, 5, rating)
                    .on_change(|ex, rating| ex.emit(RatingEvent::SetRating(rating)));
            },
            r#"Rating::new(cx, 5, RatingData::rating)
    .on_change(|ex, rating| ex.emit(RatingEvent::SetRating(rating)));"#,
        );
    })
    .class("panel");
}
