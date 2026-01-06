use crate::{icons::ICON_STAR_FILLED, prelude::*};

/// A view which represents a rating as a number of filled stars.
pub struct Rating {
    value: Signal<u32>,
    preview: Signal<u32>,
    max_rating: u32,
    on_change: Option<Box<dyn Fn(&mut EventContext, u32)>>,
}

pub(crate) enum RatingEvent {
    SetPreview(u32),
    ClearPreview,
    SetValue(u32),
    Increment,
    Decrement,
}

impl Rating {
    /// Creates a new [Rating] view.
    ///
    /// Accepts either a plain u32 value or a `Signal<u32>` for reactive state.
    /// Use `.two_way()` for automatic signal synchronization.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// // Static (read-only display)
    /// Rating::new(cx, 5, 3u32);
    ///
    /// // Reactive with two-way binding
    /// let rating = cx.state(3u32);
    /// Rating::new(cx, 5, rating).two_way();
    /// ```
    pub fn new(cx: &mut Context, max_rating: u32, value: impl Res<u32> + 'static) -> Handle<Self> {
        let value = value.into_signal(cx);
        // Preview is used for hover state.
        let preview = cx.state(0u32);

        let star_icon = cx.state(ICON_STAR_FILLED);
        let true_signal = cx.state(true);
        let numeric_one = cx.state(1);

        Self { value, preview, max_rating, on_change: None }
            .build(cx, |cx| {
                for i in 1..max_rating + 1 {
                    let is_checked = cx.derived({
                        let value = value;
                        move |store| *value.get(store) >= i
                    });
                    let is_previewed = cx.derived({
                        let preview = preview;
                        move |store| *preview.get(store) >= i
                    });
                    Svg::new(cx, star_icon)
                        .hoverable(true_signal)
                        .checkable(true_signal)
                        .numeric_value(numeric_one)
                        .role(Role::RadioButton)
                        .checked(is_checked)
                        .toggle_class("foo", is_previewed)
                        .on_hover(move |ex| ex.emit(RatingEvent::SetPreview(i)))
                        .on_press_down(move |ex| ex.emit(RatingEvent::SetValue(i)));
                }
            })
            .on_hover_out(|ex| ex.emit(RatingEvent::ClearPreview))
            .numeric_value(value)
            .navigable(true_signal)
            .role(Role::RadioGroup)
    }
}

impl View for Rating {
    fn element(&self) -> Option<&'static str> {
        Some("rating")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|rating_event, _| match rating_event {
            RatingEvent::SetPreview(val) => self.preview.set(cx, *val),
            RatingEvent::ClearPreview => self.preview.set(cx, 0),
            RatingEvent::SetValue(val) => {
                let val = (*val).min(self.max_rating);
                self.value.set(cx, val);
                if let Some(callback) = &self.on_change {
                    (callback)(cx, *self.value.get(cx))
                }
            }
            RatingEvent::Increment => {
                let current = *self.value.get(cx);
                self.value.set(cx, (current + 1).min(self.max_rating));
                if let Some(callback) = &self.on_change {
                    (callback)(cx, *self.value.get(cx))
                }
            }
            RatingEvent::Decrement => {
                let current = *self.value.get(cx);
                self.value.set(cx, current.saturating_sub(1));
                if let Some(callback) = &self.on_change {
                    (callback)(cx, *self.value.get(cx))
                }
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

    /// Enables two-way binding between the rating and its bound signal.
    ///
    /// Equivalent to:
    /// ```ignore
    /// .on_change(move |cx, val| signal.set(cx, val))
    /// ```
    pub fn two_way(self) -> Self {
        self.modify(|rating| {
            let signal = rating.value;
            rating.on_change = Some(Box::new(move |cx, val| signal.set(cx, val)));
        })
    }
}
