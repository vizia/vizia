use crate::context::TreeProps;
use crate::prelude::*;
use bitflags::bitflags;

use crate::vg;

#[derive(Debug, Default, Data, Lens, Clone)]
pub struct PopupData {
    pub is_open: bool,
}

impl Model for PopupData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|popup_event, meta| match popup_event {
            PopupEvent::Open => {
                self.is_open = true;
                meta.consume();
            }

            PopupEvent::Close => {
                self.is_open = false;
                meta.consume();
            }

            PopupEvent::Switch => {
                self.is_open ^= true;
                meta.consume();
            }
        });
    }
}

#[derive(Debug)]
pub enum PopupEvent {
    Open,
    Close,
    Switch,
}

#[derive(Lens)]
pub struct Popup {
    placement: Placement,
    show_arrow: bool,
    arrow_size: Length,
}

impl Popup {
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Self {
            placement: Placement::Top,
            show_arrow: true,
            arrow_size: Length::Value(LengthValue::Px(0.0)),
        }
        .build(cx, |cx| {
            Binding::new(cx, Popup::show_arrow, |cx, show_arrow| {
                if show_arrow.get(cx) {
                    Arrow::new(cx);
                }
            });
            (content)(cx);
        })
        .position_type(PositionType::SelfDirected)
        .space(Pixels(0.0))
    }
}

impl View for Popup {
    fn element(&self) -> Option<&'static str> {
        Some("popup")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            // Reposition popup if there isn't enough room for it.
            WindowEvent::GeometryChanged(_) => {
                let parent = cx.parent();
                let parent_bounds = cx.cache.get_bounds(parent);
                let bounds = cx.bounds();
                let window_bounds = cx.cache.get_bounds(Entity::root());

                let arrow_size = self.arrow_size.to_px().unwrap() * cx.scale_factor();

                let mut available = AvailablePlacement::all();

                let top_start_bounds = BoundingBox::from_min_max(
                    parent_bounds.left(),
                    parent_bounds.top() - bounds.height() - arrow_size,
                    parent_bounds.left() + bounds.width(),
                    parent_bounds.top(),
                );

                available
                    .set(AvailablePlacement::TOP_START, window_bounds.contains(&top_start_bounds));

                let top_bounds = BoundingBox::from_min_max(
                    parent_bounds.center().0 - bounds.width() / 2.0,
                    parent_bounds.top() - bounds.height() - arrow_size,
                    parent_bounds.center().0 + bounds.width() / 2.0,
                    parent_bounds.top(),
                );

                available.set(AvailablePlacement::TOP, window_bounds.contains(&top_bounds));

                let top_end_bounds = BoundingBox::from_min_max(
                    parent_bounds.right() - bounds.width(),
                    parent_bounds.top() - bounds.height() - arrow_size,
                    parent_bounds.right(),
                    parent_bounds.top(),
                );

                available.set(AvailablePlacement::TOP_END, window_bounds.contains(&top_end_bounds));

                let bottom_start_bounds = BoundingBox::from_min_max(
                    parent_bounds.left(),
                    parent_bounds.bottom(),
                    parent_bounds.left() + bounds.width(),
                    parent_bounds.bottom() + bounds.height() + arrow_size,
                );

                available.set(
                    AvailablePlacement::BOTTOM_START,
                    window_bounds.contains(&bottom_start_bounds),
                );

                let bottom_bounds = BoundingBox::from_min_max(
                    parent_bounds.center().0 - bounds.width() / 2.0,
                    parent_bounds.bottom(),
                    parent_bounds.center().0 + bounds.width() / 2.0,
                    parent_bounds.bottom() + bounds.height() + arrow_size,
                );

                available.set(AvailablePlacement::BOTTOM, window_bounds.contains(&bottom_bounds));

                let bottom_end_bounds = BoundingBox::from_min_max(
                    parent_bounds.right() - bounds.width(),
                    parent_bounds.bottom(),
                    parent_bounds.right(),
                    parent_bounds.bottom() + bounds.height() + arrow_size,
                );

                available.set(
                    AvailablePlacement::BOTTOM_END,
                    window_bounds.contains(&bottom_end_bounds),
                );

                let left_start_bounds = BoundingBox::from_min_max(
                    parent_bounds.left() - bounds.width() - arrow_size,
                    parent_bounds.top(),
                    parent_bounds.left(),
                    parent_bounds.top() + bounds.height(),
                );

                available.set(
                    AvailablePlacement::LEFT_START,
                    window_bounds.contains(&left_start_bounds),
                );

                let left_bounds = BoundingBox::from_min_max(
                    parent_bounds.left() - bounds.width() - arrow_size,
                    parent_bounds.center().1 - bounds.height() / 2.0,
                    parent_bounds.left(),
                    parent_bounds.center().1 + bounds.height() / 2.0,
                );

                available.set(AvailablePlacement::LEFT, window_bounds.contains(&left_bounds));

                let left_end_bounds = BoundingBox::from_min_max(
                    parent_bounds.left() - bounds.width() - arrow_size,
                    parent_bounds.bottom() - bounds.height(),
                    parent_bounds.left(),
                    parent_bounds.bottom(),
                );

                available
                    .set(AvailablePlacement::LEFT_END, window_bounds.contains(&left_end_bounds));

                let right_start_bounds = BoundingBox::from_min_max(
                    parent_bounds.right(),
                    parent_bounds.top(),
                    parent_bounds.right() + bounds.width() + arrow_size,
                    parent_bounds.top() + bounds.height(),
                );

                available.set(
                    AvailablePlacement::RIGHT_START,
                    window_bounds.contains(&right_start_bounds),
                );

                let right_bounds = BoundingBox::from_min_max(
                    parent_bounds.right(),
                    parent_bounds.center().1 - bounds.height() / 2.0,
                    parent_bounds.right() + bounds.width() + arrow_size,
                    parent_bounds.center().1 + bounds.height() / 2.0,
                );

                available.set(AvailablePlacement::RIGHT, window_bounds.contains(&right_bounds));

                let right_end_bounds = BoundingBox::from_min_max(
                    parent_bounds.right(),
                    parent_bounds.bottom() - bounds.height(),
                    parent_bounds.right() + bounds.width() + arrow_size,
                    parent_bounds.bottom(),
                );

                available
                    .set(AvailablePlacement::RIGHT_END, window_bounds.contains(&right_end_bounds));

                let scale = cx.scale_factor();

                let shift = self.placement.place(available);

                let arrow_size = self.arrow_size.to_px().unwrap();

                let translate = match shift {
                    Placement::Top => (
                        Pixels(-(bounds.width() - parent_bounds.width()) / (2.0 * scale)),
                        Pixels(-bounds.height() / scale - arrow_size),
                    ),
                    Placement::TopStart => {
                        (Pixels(0.0), Pixels(-bounds.height() / scale - arrow_size))
                    }
                    Placement::TopEnd => (
                        Pixels(-(bounds.width() - parent_bounds.width()) / scale),
                        Pixels(-bounds.height() / scale - arrow_size),
                    ),
                    Placement::Bottom => (
                        Pixels(-(bounds.width() - parent_bounds.width()) / (2.0 * scale)),
                        Pixels(parent_bounds.height() / scale + arrow_size),
                    ),
                    Placement::BottomStart => {
                        (Pixels(0.0), Pixels(parent_bounds.height() / scale + arrow_size))
                    }
                    Placement::BottomEnd => (
                        Pixels(-(bounds.width() - parent_bounds.width()) / scale),
                        Pixels(parent_bounds.height() / scale + arrow_size),
                    ),
                    Placement::LeftStart => {
                        (Pixels(-(bounds.width() / scale) - arrow_size), Pixels(0.0))
                    }
                    Placement::Left => (
                        Pixels(-(bounds.width() / scale) - arrow_size),
                        Pixels(-(bounds.height() - parent_bounds.height()) / (2.0 * scale)),
                    ),
                    Placement::LeftEnd => (
                        Pixels(-(bounds.width() / scale) - arrow_size),
                        Pixels(-(bounds.height() - parent_bounds.height()) / scale),
                    ),
                    Placement::RightStart => {
                        (Pixels((parent_bounds.width() / scale) + arrow_size), Pixels(0.0))
                    }
                    Placement::Right => (
                        Pixels((parent_bounds.width() / scale) + arrow_size),
                        Pixels(-(bounds.height() - parent_bounds.height()) / (2.0 * scale)),
                    ),
                    Placement::RightEnd => (
                        Pixels((parent_bounds.width() / scale) + arrow_size),
                        Pixels(-(bounds.height() - parent_bounds.height()) / scale),
                    ),

                    _ => (Pixels(0.0), Pixels(0.0)),
                };

                cx.set_translate(translate);
            }

            _ => {}
        });
    }
}

/// Describes the placement of a popup relative to its parent element.
#[derive(Debug, Clone, Copy, Data, PartialEq, Eq)]
pub enum Placement {
    TopStart,
    Top,
    TopEnd,
    BottomStart,
    Bottom,
    BottomEnd,
    RightStart,
    Right,
    RightEnd,
    LeftStart,
    Left,
    LeftEnd,
    Over,
    Cursor,
}

impl_res_simple!(Placement);

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct AvailablePlacement: u16 {
        const TOP_START = 1 << 0;
        const TOP = 1 << 1;
        const TOP_END = 1 << 2;
        const LEFT_START = 1 << 3;
        const LEFT = 1 << 4;
        const LEFT_END = 1 << 5;
        const BOTTOM_START = 1 << 6;
        const BOTTOM = 1 << 7;
        const BOTTOM_END = 1 << 8;
        const RIGHT_START = 1 << 9;
        const RIGHT = 1 << 10;
        const RIGHT_END = 1 << 11;
    }
}

impl AvailablePlacement {
    fn can_place(&self, placement: Placement) -> bool {
        match placement {
            Placement::Bottom => self.contains(AvailablePlacement::BOTTOM),
            Placement::BottomStart => self.contains(AvailablePlacement::BOTTOM_START),
            Placement::BottomEnd => self.contains(AvailablePlacement::BOTTOM_END),
            Placement::Top => self.contains(AvailablePlacement::TOP),
            Placement::TopStart => self.contains(AvailablePlacement::TOP_START),
            Placement::TopEnd => self.contains(AvailablePlacement::TOP_END),
            Placement::Left => self.contains(AvailablePlacement::LEFT),
            Placement::LeftStart => self.contains(AvailablePlacement::LEFT_START),
            Placement::LeftEnd => self.contains(AvailablePlacement::LEFT_END),
            Placement::Right => self.contains(AvailablePlacement::RIGHT),
            Placement::RightStart => self.contains(AvailablePlacement::RIGHT_START),
            Placement::RightEnd => self.contains(AvailablePlacement::RIGHT_END),
            _ => false,
        }
    }
}

impl Placement {
    fn from_int(int: u16) -> Placement {
        match int {
            0 => Placement::TopStart,
            1 => Placement::Top,
            2 => Placement::TopEnd,
            3 => Placement::BottomStart,
            4 => Placement::Bottom,
            5 => Placement::BottomEnd,
            6 => Placement::RightStart,
            7 => Placement::Right,
            8 => Placement::RightEnd,
            9 => Placement::LeftStart,
            10 => Placement::Left,
            11 => Placement::LeftEnd,
            12 => Placement::Over,
            _ => Placement::Cursor,
        }
    }

    pub(crate) fn place(&self, available: AvailablePlacement) -> Placement {
        if *self == Placement::Over || *self == Placement::Cursor {
            return *self;
        }

        if available.is_empty() {
            return Placement::Over;
        }

        let mut placement = *self;

        while !available.can_place(placement) {
            placement = placement.next(*self);
        }

        return placement;
    }

    fn next(&self, original: Self) -> Self {
        const TOP_START: [u16; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        const TOP: [u16; 12] = [2, 0, 4, 5, 3, 7, 8, 6, 10, 11, 9, 12];
        const TOP_END: [u16; 12] = [5, 0, 1, 8, 3, 4, 11, 6, 7, 12, 9, 10];
        const BOTTOM_START: [u16; 12] = [1, 2, 6, 4, 5, 0, 7, 8, 9, 10, 11, 12];
        const BOTTOM: [u16; 12] = [2, 0, 7, 5, 3, 1, 8, 6, 10, 11, 9, 12];
        const BOTTOM_END: [u16; 12] = [8, 0, 1, 2, 3, 4, 11, 6, 7, 12, 9, 10];
        const LEFT_START: [u16; 12] = [1, 2, 12, 4, 5, 0, 7, 8, 3, 10, 11, 6];
        const LEFT: [u16; 12] = [2, 0, 12, 5, 3, 1, 8, 6, 4, 11, 9, 7];
        const LEFT_END: [u16; 12] = [12, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        const RIGHT_START: [u16; 12] = [1, 2, 12, 4, 5, 0, 7, 8, 9, 10, 11, 3];
        const RIGHT: [u16; 12] = [2, 0, 12, 5, 3, 1, 8, 6, 10, 11, 9, 4];
        const RIGHT_END: [u16; 12] = [12, 0, 1, 2, 3, 4, 11, 6, 7, 5, 9, 10];

        let states = match original {
            Placement::TopStart => TOP_START,
            Placement::Top => TOP,
            Placement::TopEnd => TOP_END,
            Placement::BottomStart => BOTTOM_START,
            Placement::Bottom => BOTTOM,
            Placement::BottomEnd => BOTTOM_END,
            Placement::RightStart => RIGHT_START,
            Placement::Right => RIGHT,
            Placement::RightEnd => RIGHT_END,
            Placement::LeftStart => LEFT_START,
            Placement::Left => LEFT,
            Placement::LeftEnd => LEFT_END,
            _ => unreachable!(),
        };

        Placement::from_int(states[*self as usize])
    }
}

impl<'a> Handle<'a, Popup> {
    // TODO: Change this to use Res when lens value PR is merged
    /// Sets the position where the tooltip should appear relative to its parent element.
    /// Defaults to `Placement::Bottom`.
    pub fn placement(self, placement: impl Res<Placement>) -> Self {
        self.bind(placement, |handle, placement| {
            let placement = placement.get(&handle);
            handle.modify(|popup| {
                popup.placement = placement;
            });
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

/// An arrow view used by the Popup view.
pub(crate) struct Arrow {}

impl Arrow {
    pub(crate) fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {}).position_type(PositionType::SelfDirected).bind(
            Popup::placement,
            |mut handle, placement| {
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

                handle.bind(Popup::arrow_size, move |handle, arrow_size| {
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
            },
        )
    }
}

impl View for Arrow {
    fn element(&self) -> Option<&'static str> {
        Some("arrow")
    }
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let mut path = vg::Path::new();
        match Popup::placement.get(cx) {
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

            _ => {}
        }
        path.close();

        let bg = cx.background_color();

        canvas.fill_path(&path, &vg::Paint::color(bg.into()));
    }
}
