use crate::prelude::*;

/// Enum which represents the placement of a badge on its parent.
#[derive(Default, Debug, Clone, Copy, Data, PartialEq)]
pub enum BadgePlacement {
    /// The badge should be placed at the top-left of the view.
    TopLeft,
    /// The badge should be placed at the top of the view.
    Top,
    /// The badge should be placed at the top-right of the view.
    #[default]
    TopRight,
    /// The badge should be placed at the left of the view.
    Left,
    /// The badge should be placed at the right of the view.
    Right,
    /// The badge should be placed at the bottom-left of the view.
    BottomLeft,
    /// The badge should be placed at the bottom of the view.
    Bottom,
    /// The badge should be placed at the bottom-right of the view.
    BottomRight,
}

crate::impl_res_simple!(BadgePlacement);

/// A Badge view for showing notifications, counts, or status information.
pub struct Badge {
    placement: Signal<Option<BadgePlacement>>,
}

impl Badge {
    fn common<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        let placement: Signal<Option<BadgePlacement>> = cx.state(None);
        let top = cx.state(Units::Auto);
        let bottom = cx.state(Units::Auto);
        let left = cx.state(Units::Auto);
        let right = cx.state(Units::Auto);
        let translate = cx.state(Translate::default());
        Self { placement }
            .build(cx, content)
            .top(top)
            .bottom(bottom)
            .left(left)
            .right(right)
            .translate(translate)
            .bind(placement, move |handle, placement| {
                let placement = *placement.get(&handle);
                let (t, b, l, r, translate_value) = if let Some(placement) = placement {
                    let (t, b) = match placement {
                        BadgePlacement::TopLeft | BadgePlacement::TopRight => {
                            (Stretch(1.0), Percentage(85.35))
                        }
                        BadgePlacement::Top => (Stretch(1.0), Percentage(100.0)),
                        BadgePlacement::Bottom => (Percentage(100.0), Stretch(1.0)),
                        BadgePlacement::BottomLeft | BadgePlacement::BottomRight => {
                            (Percentage(85.35), Stretch(1.0))
                        }

                        BadgePlacement::Left | BadgePlacement::Right => {
                            (Stretch(1.0), Stretch(1.0))
                        }
                    };

                    let (l, r) = match placement {
                        BadgePlacement::TopLeft | BadgePlacement::BottomLeft => {
                            (Stretch(1.0), Percentage(85.35))
                        }
                        BadgePlacement::TopRight | BadgePlacement::BottomRight => {
                            (Percentage(85.35), Stretch(1.0))
                        }
                        BadgePlacement::Left => (Stretch(1.0), Percentage(100.0)),
                        BadgePlacement::Right => (Percentage(100.0), Stretch(1.0)),
                        BadgePlacement::Top | BadgePlacement::Bottom => {
                            (Stretch(1.0), Stretch(1.0))
                        }
                    };

                    let translate = match placement {
                        BadgePlacement::TopLeft => (Percentage(50.0), Percentage(50.0)),
                        BadgePlacement::Top => (Percentage(0.0), Percentage(50.0)),
                        BadgePlacement::TopRight => (Percentage(-50.0), Percentage(50.0)),
                        BadgePlacement::BottomLeft => (Percentage(50.0), Percentage(-50.0)),
                        BadgePlacement::Bottom => (Percentage(0.0), Percentage(-50.0)),
                        BadgePlacement::BottomRight => (Percentage(-50.0), Percentage(-50.0)),
                        BadgePlacement::Left => (Percentage(50.0), Percentage(0.0)),
                        BadgePlacement::Right => (Percentage(-50.0), Percentage(0.0)),
                    };

                    (t, b, l, r, translate.into())
                } else {
                    (Units::Auto, Units::Auto, Units::Auto, Units::Auto, Translate::default())
                };

                let mut event_cx = EventContext::new(handle.cx);
                top.set(&mut event_cx, t);
                bottom.set(&mut event_cx, b);
                left.set(&mut event_cx, l);
                right.set(&mut event_cx, r);
                translate.set(&mut event_cx, translate_value);
            })
    }

    /// Creates an empty badge.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// Avatar::new(cx, |cx|{
    ///     Svg::new(cx, ICON_USER);
    /// })
    /// .badge(|cx| Badge::empty(cx).class("error"));
    /// ```
    pub fn empty(cx: &mut Context) -> Handle<Self> {
        Self::common(cx, |_| {})
    }

    /// Creates a new badge with the provided content.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// Avatar::new(cx, |cx|{
    ///     Svg::new(cx, ICON_USER);
    /// })
    /// .badge(|cx| Badge::new(|cx| Label::new("2")));
    /// ```
    pub fn new<F, V>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context) -> Handle<V>,
        V: View,
    {
        Self::common(cx, |cx| {
            (content)(cx);
        })
    }
}

impl View for Badge {
    fn element(&self) -> Option<&'static str> {
        Some("badge")
    }
}

impl Handle<'_, Badge> {
    /// Sets the placement of a badge relative to its parent.
    /// Accepts a `BadgePlacement` or `Signal<BadgePlacement>`.
    pub fn placement(mut self, placement: impl Res<BadgePlacement> + 'static) -> Self {
        let placement = placement.into_signal(self.context());
        self.bind(placement, |handle, val| {
            let placement = *val.get(&handle);
            handle.modify2(|badge, cx| badge.placement.set(cx, Some(placement)));
        })
    }
}
