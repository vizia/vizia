use crate::{icons::ICON_STAR_FILLED, prelude::*};

/// A view which represents a rating as a number of filled stars.
pub struct Rating {
    rating: Signal<u32>,
    max_rating: u32,
    on_change: Option<Box<dyn Fn(&mut EventContext, u32)>>,
}

pub(crate) enum RatingEvent {
    SetRating(u32),
    EmitRating,
    Increment,
    Decrement,
}

impl Rating {
    /// Creates a new [Rating] view.
    pub fn new<L: Res<u32>>(cx: &mut Context, max_rating: u32, value: L) -> Handle<Self> {
        let initial = value.get(cx);
        let rating = cx.state(initial);

        Self { rating, max_rating, on_change: None }
            .build(cx, |cx| {
                for i in 1..max_rating + 1 {
                    Svg::new(cx, ICON_STAR_FILLED)
                        .checkable(true)
                        .numeric_value(1)
                        .role(Role::RadioButton)
                        .bind(rating, move |handle, val| {
                            let v = *val.get(&handle);
                            handle.checked(v >= i);
                        })
                        .on_hover(move |ex| ex.emit(RatingEvent::SetRating(i)))
                        .on_press(|ex| ex.emit(RatingEvent::EmitRating));
                }
            })
            .numeric_value(rating)
            .navigable(true)
            .role(Role::RadioGroup)
            .bind(value, move |handle, v| {
                let val = v.get(&handle);
                handle.modify2(|rating, cx| rating.rating.set(cx, val));
            })
    }
}

impl View for Rating {
    fn element(&self) -> Option<&'static str> {
        Some("rating")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|rating_event, _| match rating_event {
            RatingEvent::SetRating(val) => self.rating.set(cx, *val),
            RatingEvent::EmitRating => {
                if let Some(callback) = &self.on_change {
                    (callback)(cx, *self.rating.get(cx))
                }
            }
            RatingEvent::Increment => {
                let current = *self.rating.get(cx);
                self.rating.set(cx, (current + 1) % (self.max_rating + 1));
                cx.emit(RatingEvent::EmitRating);
            }
            RatingEvent::Decrement => {
                let current = *self.rating.get(cx);
                let new_val = if current == 0 { self.max_rating } else { current.saturating_sub(1) };
                self.rating.set(cx, new_val);
                cx.emit(RatingEvent::EmitRating);
            }
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::KeyDown(code, _) => match code {
                Code::ArrowLeft => {
                    cx.emit(RatingEvent::Decrement);
                }

                Code::ArrowRight => {
                    cx.emit(RatingEvent::Increment);
                }

                _ => {}
            },

            _ => {}
        });
    }
}

impl Handle<'_, Rating> {
    /// Set the callback which is triggered when the rating changes.
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, u32),
    {
        self.modify(|rating| rating.on_change = Some(Box::new(callback)))
    }
}
