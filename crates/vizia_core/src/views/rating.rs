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
    pub fn new(
        cx: &mut Context,
        max_rating: u32,
        rating: impl Res<u32> + SignalGet<u32> + SignalMap<u32>,
    ) -> Handle<Self> {
        let local_rating = Signal::new(rating.get());
        Self { rating: local_rating, max_rating, on_change: None }
            .build(cx, |cx| {
                for i in 1..max_rating + 1 {
                    Svg::new(cx, ICON_STAR_FILLED)
                        // .navigable(true)
                        .checkable(true)
                        .numeric_value(1)
                        .role(Role::RadioButton)
                        .checked(rating.map(move |r| *r >= i))
                        .toggle_class("foo", local_rating.map(move |r| *r >= i))
                        .on_hover(move |ex| ex.emit(RatingEvent::SetRating(i)))
                        .on_press(|ex| ex.emit(RatingEvent::EmitRating));
                }
            })
            .numeric_value(rating)
            .navigable(true)
            .role(Role::RadioGroup)
            .bind(rating, move |handle| {
                let val = rating.get();
                handle.modify(|rating| rating.rating.set(val));
            })
    }
}

impl View for Rating {
    fn element(&self) -> Option<&'static str> {
        Some("rating")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|rating_event, _| match rating_event {
            RatingEvent::SetRating(val) => self.rating.set(*val),
            RatingEvent::EmitRating => {
                if let Some(callback) = &self.on_change {
                    (callback)(cx, self.rating.get());
                }
            }
            RatingEvent::Increment => {
                self.rating.set((self.rating.get() + 1) % (self.max_rating + 1));
                cx.emit(RatingEvent::EmitRating);
            }
            RatingEvent::Decrement => {
                self.rating.set(if self.rating.get() == 0 {
                    self.max_rating
                } else {
                    self.rating.get().saturating_sub(1)
                });
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
