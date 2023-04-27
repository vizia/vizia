use crate::{icons::ICON_STAR_FILLED, prelude::*};

#[derive(Lens)]
pub struct Rating {
    rating: u32,
    on_change: Option<Box<dyn Fn(&mut EventContext, u32)>>,
}

pub(crate) enum RatingEvent {
    SetRating(u32),
    EmitRating,
}

impl Rating {
    pub fn new(cx: &mut Context, lens: impl Lens<Target = u32>) -> Handle<Self> {
        Self { rating: lens.get(cx), on_change: None }
            .build(cx, |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, ICON_STAR_FILLED)
                        .class("icon")
                        .checked(lens.clone().map(|val| *val >= 1))
                        .toggle_class("foo", Rating::rating.map(|val| *val >= 1))
                        .on_hover(|ex| ex.emit(RatingEvent::SetRating(1)))
                        .on_press(|ex| ex.emit(RatingEvent::EmitRating));
                    Label::new(cx, ICON_STAR_FILLED)
                        .class("icon")
                        .checked(lens.clone().map(|val| *val >= 2))
                        .toggle_class("foo", Rating::rating.map(|val| *val >= 2))
                        .on_hover(|ex| ex.emit(RatingEvent::SetRating(2)))
                        .on_press(|ex| ex.emit(RatingEvent::EmitRating));
                    Label::new(cx, ICON_STAR_FILLED)
                        .class("icon")
                        .checked(lens.clone().map(|val| *val >= 3))
                        .toggle_class("foo", Rating::rating.map(|val| *val >= 3))
                        .on_hover(|ex| ex.emit(RatingEvent::SetRating(3)))
                        .on_press(|ex| ex.emit(RatingEvent::EmitRating));
                    Label::new(cx, ICON_STAR_FILLED)
                        .class("icon")
                        .checked(lens.clone().map(|val| *val >= 4))
                        .toggle_class("foo", Rating::rating.map(|val| *val >= 4))
                        .on_hover(|ex| ex.emit(RatingEvent::SetRating(4)))
                        .on_press(|ex: &mut EventContext| ex.emit(RatingEvent::EmitRating));
                    Label::new(cx, ICON_STAR_FILLED)
                        .class("icon")
                        .checked(lens.clone().map(|val| *val >= 5))
                        .toggle_class("foo", Rating::rating.map(|val| *val >= 5))
                        .on_hover(|ex| ex.emit(RatingEvent::SetRating(5)))
                        .on_press(|ex| ex.emit(RatingEvent::EmitRating));
                })
                .class("rating-container")
                .size(Auto);
            })
            .bind(lens.clone(), |handle, lens| {
                let val = lens.get(handle.cx);
                handle.modify(|rating| rating.rating = val);
            })
    }
}

impl View for Rating {
    fn element(&self) -> Option<&'static str> {
        Some("rating")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|rating_event, _| match rating_event {
            RatingEvent::SetRating(val) => self.rating = *val,
            RatingEvent::EmitRating => {
                if let Some(callback) = &self.on_change {
                    (callback)(cx, self.rating)
                }
            }
        })
    }
}

impl<'a> Handle<'a, Rating> {
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, u32),
    {
        self.modify(|rating| rating.on_change = Some(Box::new(callback)))
    }
}
