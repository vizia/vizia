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

impl_res_simple!(BadgePlacement);

/// A Badge view for showing notifications, counts, or status information.
#[derive(Lens)]
pub struct Badge {
    placement: Option<BadgePlacement>,
}

impl Badge {
    fn common<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        Self { placement: None }.build(cx, content).bind(
            Badge::placement,
            |mut handle, placement| {
                if let Some(placement) = placement.get(&handle) {
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
                }
            },
        )
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
    /// Sets the placement of a badge relative to its parent. Accepts a value of, or lens to, a [BadgePlacement].
    pub fn placement<U: Into<BadgePlacement>>(self, placement: impl Res<U>) -> Self {
        self.bind(placement, |handle, val| {
            let placement = val.get(&handle).into();
            handle.modify(|badge| badge.placement = Some(placement));
        })
    }
}
