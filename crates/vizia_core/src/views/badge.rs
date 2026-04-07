use crate::prelude::*;

/// Enum which represents the placement of a badge on its parent.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
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

impl_res_simple!(BadgePlacement);

/// A Badge view for showing notifications, counts, or status information.
pub struct Badge {
    placement: Signal<BadgePlacement>,
}

impl Badge {
    fn common<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        let placement = Signal::new(BadgePlacement::TopRight);
        Self { placement }.build(cx, content).bind(placement, move |mut handle| {
            let placement = placement.get();
            let (t, b) = match placement {
                BadgePlacement::TopLeft | BadgePlacement::TopRight => {
                    (Stretch(1.0), Percentage(85.35))
                }
                BadgePlacement::Top => (Stretch(1.0), Percentage(100.0)),
                BadgePlacement::Bottom => (Percentage(100.0), Stretch(1.0)),
                BadgePlacement::BottomLeft | BadgePlacement::BottomRight => {
                    (Percentage(85.35), Stretch(1.0))
                }

                BadgePlacement::Left | BadgePlacement::Right => (Stretch(1.0), Stretch(1.0)),
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
                BadgePlacement::Top | BadgePlacement::Bottom => (Stretch(1.0), Stretch(1.0)),
            };

            handle = handle.top(t).bottom(b).left(l).right(r);

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
            handle.translate(translate);
        })
    }

    /// Creates an empty badge.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_core::icons::ICON_USER;
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
    /// # use vizia_core::icons::ICON_USER;
    /// # let cx = &mut Context::default();
    /// Avatar::new(cx, |cx|{
    ///     Svg::new(cx, ICON_USER);
    /// })
    /// .badge(|cx| Badge::new(cx, |cx| Label::new(cx, "2")));
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
    /// Sets the placement of a badge relative to its parent. Accepts a value or signal of type [BadgePlacement].
    pub fn placement<U: Into<BadgePlacement> + Clone + 'static>(
        self,
        placement: impl Res<U> + 'static,
    ) -> Self {
        let placement = placement.to_signal(self.cx);
        self.bind(placement, move |handle| {
            let value = placement.get();
            let converted: BadgePlacement = value.into();
            handle.modify(|badge| {
                badge.placement.set(converted);
            });
        })
    }
}
