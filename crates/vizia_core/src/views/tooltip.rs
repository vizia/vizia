use crate::context::TreeProps;
use crate::vg;
use crate::{modifiers::TooltipModel, prelude::*};

/// A tooltip view.
///
/// Should be used with the [tooltip](crate::modifiers::ActionModifiers::tooltip) modifier.
///
/// # Example
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # enum AppEvent {
/// #     Action,
/// # }
/// #
/// # let cx = &mut Context::default();
/// #
/// Button::new(cx, |cx| Label::new(cx, "Text"))
///     .tooltip(|cx|{
///         Tooltip::new(cx, |cx|{
///             Label::new(cx, "Tooltip Text");
///         })
///     })
/// ```
#[derive(Lens)]
pub struct Tooltip {
    default_placement: Placement,
    placement: Placement,
    show_arrow: bool,
    arrow_size: Length,
}

impl Tooltip {
    /// Creates a new Tooltip view with the given content.
    ///
    /// Should be used with the [tooltip](crate::modifiers::ActionModifiers::tooltip) modifier.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # enum AppEvent {
    /// #     Action,
    /// # }
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Button::new(cx, |cx| Label::new(cx, "Text"))
    ///     .tooltip(|cx|{
    ///         Tooltip::new(cx, |cx|{
    ///             Label::new(cx, "Tooltip Text");
    ///         })
    ///     })
    /// ```
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Self {
            placement: Placement::Bottom,
            default_placement: Placement::Bottom,
            show_arrow: true,
            arrow_size: Length::Value(LengthValue::Px(8.0)),
        }
        .build(cx, |cx| {
            Binding::new(cx, Tooltip::show_arrow, |cx, show_arrow| {
                if show_arrow.get(cx) {
                    Arrow::new(cx);
                }
            });
            (content)(cx);
        })
        .z_index(100)
        .bind(Tooltip::placement, |mut handle, placement| {
            let (t, b) = match placement.get(&handle) {
                Placement::TopStart | Placement::Top | Placement::TopEnd => {
                    (Auto, Percentage(100.0))
                }
                Placement::BottomStart | Placement::Bottom | Placement::BottomEnd => {
                    (Percentage(100.0), Stretch(1.0))
                }
                Placement::LeftStart | Placement::RightStart => (Pixels(0.0), Stretch(1.0)),
                Placement::LeftEnd | Placement::RightEnd => (Stretch(1.0), Pixels(0.0)),
                Placement::Left | Placement::Right | Placement::Over => {
                    (Stretch(1.0), Stretch(1.0))
                }
            };

            let (l, r) = match placement.get(&handle) {
                Placement::TopStart | Placement::BottomStart => (Pixels(0.0), Stretch(1.0)),
                Placement::TopEnd | Placement::BottomEnd => (Stretch(1.0), Pixels(0.0)),
                Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
                    (Stretch(1.0), Percentage(100.0))
                }
                Placement::Right | Placement::RightStart | Placement::RightEnd => {
                    (Percentage(100.0), Stretch(1.0))
                }
                Placement::Top | Placement::Bottom | Placement::Over => {
                    (Stretch(1.0), Stretch(1.0))
                }
            };

            handle = handle.top(t).bottom(b).left(l).right(r);

            handle.bind(Tooltip::arrow_size, move |handle, arrow_size| {
                let arrow_size = arrow_size.get(&handle).to_px().unwrap_or(8.0);
                let translate = match placement.get(&handle) {
                    Placement::Top | Placement::TopStart | Placement::TopEnd => {
                        (Pixels(0.0), Pixels(-arrow_size))
                    }
                    Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
                        (Pixels(0.0), Pixels(arrow_size))
                    }
                    Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
                        (Pixels(-arrow_size), Pixels(0.0))
                    }
                    Placement::Right | Placement::RightStart | Placement::RightEnd => {
                        (Pixels(arrow_size), Pixels(0.0))
                    }
                    _ => (Pixels(0.0), Pixels(0.0)),
                };
                handle.translate(translate);
            });
        })
        .hoverable(false)
        .on_build(|ex| {
            ex.add_listener(move |_: &mut Tooltip, ex, event| {
                let flag = TooltipModel::tooltip_visible.get(ex);
                event.map(|window_event, meta| match window_event {
                    WindowEvent::MouseDown(_) => {
                        if flag && meta.origin != ex.current() {
                            ex.toggle_class("vis", false);
                        }
                    }

                    _ => {}
                });
            });
        })
    }

    fn place(&mut self, dist_top: f32, dist_bottom: f32, dist_left: f32, dist_right: f32) {
        match self.placement {
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd
                if dist_bottom < 0.0 =>
            {
                if dist_top < 0.0 && dist_left < 0.0 && dist_right < 0.0 {
                    self.placement = Placement::Over;
                    return;
                }

                if dist_top < 0.0 {
                    self.placement = Placement::Right;
                } else {
                    self.placement = Placement::Top;
                }
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::Top | Placement::TopStart | Placement::TopEnd if dist_top < 0.0 => {
                if dist_bottom < 0.0 && dist_left < 0.0 && dist_right < 0.0 {
                    self.placement = Placement::Over;
                    return;
                }

                if dist_bottom < 0.0 {
                    self.placement = Placement::Right;
                } else {
                    self.placement = Placement::Bottom;
                }
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::Left | Placement::LeftStart | Placement::LeftEnd if dist_left < 0.0 => {
                if dist_top < 0.0 && dist_bottom < 0.0 && dist_right < 0.0 {
                    self.placement = Placement::Over;
                    return;
                }

                if dist_right < 0.0 {
                    self.placement = Placement::Bottom;
                } else {
                    self.placement = Placement::Right;
                }
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::Right | Placement::RightStart | Placement::RightEnd if dist_right < 0.0 => {
                if dist_top < 0.0 && dist_left < 0.0 && dist_bottom < 0.0 {
                    self.placement = Placement::Over;
                    return;
                }

                if dist_left < 0.0 {
                    self.placement = Placement::Bottom;
                } else {
                    self.placement = Placement::Left;
                }
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            _ => {}
        }
    }
}

impl View for Tooltip {
    fn element(&self) -> Option<&'static str> {
        Some("tooltip")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            // Reposition tooltip if there isn't enough room for it.
            WindowEvent::GeometryChanged(_) => {
                let parent = cx.parent();
                let parent_bounds = cx.cache.get_bounds(parent);
                let bounds = cx.bounds();
                let window_bounds = cx.cache.get_bounds(Entity::root());

                let arrow_size = self.arrow_size.to_px().unwrap();

                let dist_bottom = window_bounds.bottom()
                    - (parent_bounds.bottom() + bounds.height() + arrow_size);
                let dist_top =
                    (parent_bounds.top() - bounds.height() - arrow_size) - window_bounds.top();
                let dist_left =
                    (parent_bounds.left() - bounds.width() - arrow_size) - window_bounds.left();
                let dist_right =
                    window_bounds.right() - (parent_bounds.right() + bounds.width() + arrow_size);

                self.placement = self.default_placement;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            _ => {}
        });
    }
}

/// Describes the placement of a tooltip relative to its parent element.
#[derive(Debug, Clone, Copy, Data, PartialEq, Eq)]
pub enum Placement {
    TopStart,
    Top,
    TopEnd,
    LeftStart,
    Left,
    LeftEnd,
    BottomStart,
    Bottom,
    BottomEnd,
    RightStart,
    Right,
    RightEnd,
    Over,
}

impl<'a> Handle<'a, Tooltip> {
    // TODO: Change this to use Res when lens value PR is merged
    /// Sets the position where the tooltip should appear relative to its parent element.
    /// Defaults to `Placement::Bottom`.
    pub fn placement(self, placement: Placement) -> Self {
        self.modify(|tooltip| {
            tooltip.placement = placement;
            tooltip.default_placement = placement;
        })
    }

    // TODO: Change this to use Res when lens value PR is merged
    /// Sets whether the tooltip should include an arrow. Defaults to true.
    pub fn arrow(self, show_arrow: bool) -> Self {
        self.modify(|tooltip| tooltip.show_arrow = show_arrow)
    }

    pub fn arrow_size(self, size: impl Into<Length>) -> Self {
        self.modify(|tooltip| tooltip.arrow_size = size.into())
    }
}

/// An arrow view used by the Tooltip view.
pub(crate) struct Arrow {}

impl Arrow {
    pub(crate) fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {}).bind(Tooltip::placement, |mut handle, placement| {
            let (t, b) = match placement.get(&handle) {
                Placement::TopStart | Placement::Top | Placement::TopEnd => {
                    (Percentage(100.0), Stretch(1.0))
                }
                Placement::BottomStart | Placement::Bottom | Placement::BottomEnd => {
                    (Stretch(1.0), Percentage(100.0))
                }
                _ => (Stretch(1.0), Stretch(1.0)),
            };

            let (l, r) = match placement.get(&handle) {
                Placement::LeftStart | Placement::Left | Placement::LeftEnd => {
                    (Percentage(100.0), Stretch(1.0))
                }
                Placement::RightStart | Placement::Right | Placement::RightEnd => {
                    (Stretch(1.0), Percentage(100.0))
                }
                Placement::TopStart | Placement::BottomStart => {
                    // TODO: Use border radius
                    (Pixels(8.0), Stretch(1.0))
                }
                Placement::TopEnd | Placement::BottomEnd => {
                    // TODO: Use border radius
                    (Stretch(1.0), Pixels(8.0))
                }
                _ => (Stretch(1.0), Stretch(1.0)),
            };

            handle = handle.top(t).bottom(b).left(l).right(r);

            handle.bind(Tooltip::arrow_size, move |handle, arrow_size| {
                let arrow_size = arrow_size.get(&handle).to_px().unwrap_or(8.0);
                let (w, h) = match placement.get(&handle) {
                    Placement::Top
                    | Placement::Bottom
                    | Placement::TopStart
                    | Placement::BottomStart
                    | Placement::TopEnd
                    | Placement::BottomEnd => (Pixels(arrow_size * 2.0), Pixels(arrow_size)),

                    _ => (Pixels(arrow_size), Pixels(arrow_size * 2.0)),
                };

                handle.width(w).height(h);
            });
        })
    }
}

impl View for Arrow {
    fn element(&self) -> Option<&'static str> {
        Some("arrow")
    }
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let mut path = vg::Path::new();
        match Tooltip::placement.get(cx) {
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
                path.move_to(bounds.bottom_left().1, bounds.bottom_left().0);
                path.line_to(bounds.center_top().0, bounds.center_top().1);
                path.line_to(bounds.bottom_right().1, bounds.bottom_right().0);
                path.line_to(bounds.bottom_left().1, bounds.bottom_left().0);
            }

            Placement::Top | Placement::TopStart | Placement::TopEnd => {
                path.move_to(bounds.top_left().1, bounds.top_left().0);
                path.line_to(bounds.center_bottom().0, bounds.center_bottom().1);
                path.line_to(bounds.top_right().1, bounds.top_right().0);
                path.line_to(bounds.top_left().1, bounds.top_left().0);
            }

            Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
                path.move_to(bounds.top_left().1, bounds.top_left().0);
                path.line_to(bounds.center_right().0, bounds.center_right().1);
                path.line_to(bounds.bottom_left().1, bounds.bottom_left().0);
                path.line_to(bounds.top_left().1, bounds.top_left().0);
            }

            Placement::Right | Placement::RightStart | Placement::RightEnd => {
                path.move_to(bounds.top_right().1, bounds.top_right().0);
                path.line_to(bounds.center_left().0, bounds.center_left().1);
                path.line_to(bounds.bottom_right().1, bounds.bottom_right().0);
                path.line_to(bounds.top_right().1, bounds.top_right().0);
            }

            Placement::Over => {}
        }
        path.close();

        let bg = cx.background_color();

        canvas.fill_path(&path, &vg::Paint::color(bg.into()));
    }
}
