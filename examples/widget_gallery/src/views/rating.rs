use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Clone, Lens)]
struct RatingData {
    rating: u32,
}

impl Model for RatingData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            RatingEvent::SetRating(val) => self.rating = *val,
        })
    }
}

enum RatingEvent {
    SetRating(u32),
}

pub fn rating(cx: &mut Context) {
    RatingData { rating: 3 }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Rating").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic rating").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Rating::new(cx, 5, RatingData::rating)
                    .on_change(|ex, rating| ex.emit(RatingEvent::SetRating(rating)));
            },
            r#"Rating::new(cx, 5, RatingData::rating)
    .on_change(|ex, rating| ex.emit(RatingEvent::SetRating(rating)));"#,
        );
    })
    .class("panel");
}
