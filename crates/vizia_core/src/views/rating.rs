use crate::{icons::ICON_STAR_FILLED, prelude::*};

#[derive(Lens)]
pub struct Rating {
    rating: u32,
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
    pub fn new(cx: &mut Context, max_rating: u32, lens: impl Lens<Target = u32>) -> Handle<Self> {
        Self { rating: lens.get(cx), max_rating, on_change: None }
            .build(cx, |cx| {
                for i in 1..max_rating + 1 {
                    Label::new(cx, ICON_STAR_FILLED)
                        // .navigable(true)
                        .checkable(true)
                        .numeric_value(1)
                        .role(Role::RadioButton)
                        .default_action_verb(DefaultActionVerb::Click)
                        .class("icon")
                        .checked(lens.map(move |val| *val >= i))
                        .toggle_class("foo", Rating::rating.map(move |val| *val >= i))
                        .on_hover(move |ex| ex.emit(RatingEvent::SetRating(i)))
                        .on_press(|ex| ex.emit(RatingEvent::EmitRating));
                }
            })
            .numeric_value(Self::rating)
            .navigable(true)
            .role(Role::RadioGroup)
            .bind(lens, |handle, lens| {
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
            RatingEvent::Increment => {
                self.rating += 1;
                self.rating %= 6;
                cx.emit(RatingEvent::EmitRating);
            }
            RatingEvent::Decrement => {
                self.rating = if self.rating == 0 { 5 } else { self.rating.saturating_sub(1) };
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

impl<'a> Handle<'a, Rating> {
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, u32),
    {
        self.modify(|rating| rating.on_change = Some(Box::new(callback)))
    }
}
