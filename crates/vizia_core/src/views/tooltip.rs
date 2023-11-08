use crate::context::TreeProps;
use crate::vg;
use crate::{modifiers::TooltipModel, prelude::*};

#[derive(Lens)]
pub struct Tooltip {
    default_placement: Placement,
    placement: Placement,
}

impl Tooltip {
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Self { placement: Placement::Bottom, default_placement: Placement::Bottom }
            .build(cx, |cx| {
                Arrow::new(cx);
                (content)(cx);
            })
            .position_type(PositionType::SelfDirected)
            .z_index(100)
            .size(Auto)
            .bind(Tooltip::placement, |handle, placement| {
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

                let translate = match placement.get(&handle) {
                    Placement::Top | Placement::TopStart | Placement::TopEnd => {
                        (Pixels(0.0), Pixels(-8.0))
                    }
                    Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
                        (Pixels(0.0), Pixels(8.0))
                    }
                    Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
                        (Pixels(-8.0), Pixels(0.0))
                    }
                    Placement::Right | Placement::RightStart | Placement::RightEnd => {
                        (Pixels(8.0), Pixels(0.0))
                    }
                    _ => (Pixels(0.0), Pixels(0.0)),
                };

                handle.top(t).bottom(b).left(l).right(r).translate(translate);
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
            WindowEvent::GeometryChanged(_) => {
                let parent = cx.parent();
                let parent_bounds = cx.cache.get_bounds(parent);
                let bounds = cx.bounds();
                let window_bounds = cx.cache.get_bounds(Entity::root());

                let dist_bottom =
                    window_bounds.bottom() - (parent_bounds.bottom() + bounds.height());
                let dist_top = (parent_bounds.top() - bounds.height()) - window_bounds.top();
                let dist_left = (parent_bounds.left() - bounds.width()) - window_bounds.left();
                let dist_right = window_bounds.right() - (parent_bounds.right() + bounds.width());

                self.placement = self.default_placement;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            _ => {}
        });
    }
}

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

impl Placement {}

impl<'a> Handle<'a, Tooltip> {
    pub fn placement(self, placement: Placement) -> Self {
        self.modify(|tooltip| {
            tooltip.placement = placement;
            tooltip.default_placement = placement;
        })
    }
}

pub struct Arrow {}

impl Arrow {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}
            .build(cx, |_| {})
            .background_color(Color::from("#181818"))
            .position_type(PositionType::SelfDirected)
            .bind(Tooltip::placement, |handle, placement| {
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

                let (w, h) = match placement.get(&handle) {
                    Placement::Top
                    | Placement::Bottom
                    | Placement::TopStart
                    | Placement::BottomStart
                    | Placement::TopEnd
                    | Placement::BottomEnd => (Pixels(16.0), Pixels(8.0)),

                    _ => (Pixels(8.0), Pixels(16.0)),
                };

                handle.top(t).bottom(b).left(l).right(r).width(w).height(h);
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

            _ => {
                path.move_to(bounds.bottom_left().1, bounds.bottom_left().0);
                path.line_to(bounds.center_top().0, bounds.center_top().1);
                path.line_to(bounds.bottom_right().1, bounds.bottom_right().0);
                path.line_to(bounds.bottom_left().1, bounds.bottom_left().0);
            }
        }
        path.close();

        let bg = cx.background_color();

        canvas.fill_path(&path, &vg::Paint::color(bg.into()));
    }
}
