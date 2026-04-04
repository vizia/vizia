use crate::context::TreeProps;
use crate::prelude::*;
use bitflags::bitflags;

use crate::vg;

/// A model which can be used by views which contain a popup.
#[derive(Debug, Default, Clone)]
pub struct PopupData {
    /// The open state of the popup.
    pub is_open: bool,
}

impl From<PopupData> for bool {
    fn from(value: PopupData) -> Self {
        value.is_open
    }
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

/// Events used by the [Popup] view.
#[derive(Debug)]
pub enum PopupEvent {
    /// Opens the popup.
    Open,
    /// Closes the popup.
    Close,
    /// Switches the state of the popup from closed to open or open to closed.
    Switch,
}

/// A view for displaying popup content.
pub struct Popup {
    placement: Signal<Placement>,
    show_arrow: Signal<bool>,
    arrow_size: Signal<Length>,
    should_reposition: Signal<bool>,
}

impl Popup {
    /// Creates a new [Popup] view.
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        let placement = Signal::new(Placement::Bottom);
        let show_arrow = Signal::new(true);
        let arrow_size = Signal::new(Length::Value(LengthValue::Px(8.0)));
        let should_reposition = Signal::new(true);

        Self { placement, show_arrow, arrow_size, should_reposition }
            .build(cx, |cx| {
                (content)(cx);
                Binding::new(cx, show_arrow, move |cx| {
                    let show_arrow = show_arrow.get();
                    if show_arrow {
                        Arrow::new(cx, placement, arrow_size);
                    }
                });
            })
            .position_type(PositionType::Absolute)
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
                let window_bounds = cx.cache.get_bounds(cx.parent_window());
                let scale = cx.scale_factor();
                let arrow_size = self.arrow_size.get().to_px().unwrap() * cx.scale_factor();

                let shift = if self.should_reposition.get() {
                    let mut available = AvailablePlacement::all();

                    let top_start_bounds = BoundingBox::from_min_max(
                        parent_bounds.left(),
                        parent_bounds.top() - bounds.height() - arrow_size,
                        parent_bounds.left() + bounds.width(),
                        parent_bounds.top(),
                    );

                    available.set(
                        AvailablePlacement::TOP_START,
                        window_bounds.contains(&top_start_bounds),
                    );

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

                    available
                        .set(AvailablePlacement::TOP_END, window_bounds.contains(&top_end_bounds));

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

                    available
                        .set(AvailablePlacement::BOTTOM, window_bounds.contains(&bottom_bounds));

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

                    available.set(
                        AvailablePlacement::LEFT_END,
                        window_bounds.contains(&left_end_bounds),
                    );

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

                    available.set(
                        AvailablePlacement::RIGHT_END,
                        window_bounds.contains(&right_end_bounds),
                    );

                    self.placement.get().place(available)
                } else {
                    if let Some(first_child) = cx.tree.get_layout_first_child(cx.current) {
                        let mut child_bounds = cx.cache.get_bounds(first_child);
                        child_bounds.h = window_bounds.bottom()
                            - parent_bounds.bottom()
                            - arrow_size * scale
                            - 8.0;
                        cx.style.max_height.insert(first_child, Pixels(child_bounds.h / scale));
                    }
                    self.placement.get()
                };

                let arrow_size = self.arrow_size.get().to_px().unwrap();

                let translate = match shift {
                    Placement::Top => (
                        -(bounds.width() - parent_bounds.width()) / (2.0 * scale),
                        -bounds.height() / scale - arrow_size,
                    ),
                    Placement::TopStart => (0.0, -bounds.height() / scale - arrow_size),
                    Placement::TopEnd => (
                        -(bounds.width() - parent_bounds.width()) / scale,
                        -bounds.height() / scale - arrow_size,
                    ),
                    Placement::Bottom => (
                        -(bounds.width() - parent_bounds.width()) / (2.0 * scale),
                        parent_bounds.height() / scale + arrow_size,
                    ),
                    Placement::BottomStart => (0.0, parent_bounds.height() / scale + arrow_size),
                    Placement::BottomEnd => (
                        -(bounds.width() - parent_bounds.width()) / scale,
                        parent_bounds.height() / scale + arrow_size,
                    ),
                    Placement::LeftStart => (-(bounds.width() / scale) - arrow_size, 0.0),
                    Placement::Left => (
                        -(bounds.width() / scale) - arrow_size,
                        -(bounds.height() - parent_bounds.height()) / (2.0 * scale),
                    ),
                    Placement::LeftEnd => (
                        -(bounds.width() / scale) - arrow_size,
                        -(bounds.height() - parent_bounds.height()) / scale,
                    ),
                    Placement::RightStart => ((parent_bounds.width() / scale) + arrow_size, 0.0),
                    Placement::Right => (
                        (parent_bounds.width() / scale) + arrow_size,
                        -(bounds.height() - parent_bounds.height()) / (2.0 * scale),
                    ),
                    Placement::RightEnd => (
                        (parent_bounds.width() / scale) + arrow_size,
                        -(bounds.height() - parent_bounds.height()) / scale,
                    ),

                    _ => (0.0, 0.0),
                };
                cx.set_translate((Pixels(translate.0.round()), Pixels(translate.1.round())));
            }

            _ => {}
        });
    }
}

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

        placement
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

/// Modifiers for configuring [Popup] behavior and positioning.
pub trait PopupModifiers: Sized {
    /// Sets the position where the popup should appear relative to its parent element.
    /// Defaults to `Placement::Bottom`.
    fn placement(self, placement: impl Res<Placement> + 'static) -> Self;

    /// Sets whether the popup should include an arrow. Defaults to true.
    fn show_arrow(self, show_arrow: impl Res<bool> + 'static) -> Self;

    /// Sets the size of the popup arrow, or gap if the arrow is hidden.
    fn arrow_size<U: Into<Length> + Clone + 'static>(
        self,
        size: impl Res<U> + 'static,
    ) -> Self;

    /// Set to whether the popup should reposition to always be visible.
    fn should_reposition(self, should_reposition: impl Res<bool> + 'static) -> Self;

    /// Registers a callback for when the user clicks off of the popup, usually with the intent of
    /// closing it.
    fn on_blur<F>(self, f: F) -> Self
    where
        F: 'static + Fn(&mut EventContext);
}

impl PopupModifiers for Handle<'_, Popup> {
    fn placement(self, placement: impl Res<Placement> + 'static) -> Self {
        let placement = placement.to_signal(self.cx);
        self.bind(placement, move |handle| {
            let placement = placement.get();
            handle.modify(|popup| {
                popup.placement.set(placement);
            });
        })
    }

    fn show_arrow(self, show_arrow: impl Res<bool> + 'static) -> Self {
        let show_arrow = show_arrow.to_signal(self.cx);
        self.bind(show_arrow, move |handle| {
            let show_arrow = show_arrow.get();
            handle.modify(|popup| popup.show_arrow.set(show_arrow));
        })
    }

    fn arrow_size<U: Into<Length> + Clone + 'static>(
        self,
        size: impl Res<U> + 'static,
    ) -> Self {
        let size = size.to_signal(self.cx);
        self.bind(size, move |handle| {
            let size = size.get();
            let size = size.into();
            handle.modify(|popup| popup.arrow_size.set(size));
        })
    }

    fn should_reposition(self, should_reposition: impl Res<bool> + 'static) -> Self {
        let should_reposition = should_reposition.to_signal(self.cx);
        self.bind(should_reposition, move |handle| {
            let should_reposition = should_reposition.get();
            handle.modify(|popup| popup.should_reposition.set(should_reposition));
        })
    }

    fn on_blur<F>(self, f: F) -> Self
    where
        F: 'static + Fn(&mut EventContext),
    {
        let focus_event = Box::new(f);
        self.cx.with_current(self.entity, |cx| {
            cx.add_listener(move |_: &mut Popup, cx, event| {
                event.map(|window_event, meta| match window_event {
                    WindowEvent::MouseDown(_) => {
                        if meta.origin != cx.current() {
                            // Check if the mouse was pressed outside of any descendants
                            if !cx.hovered.is_descendant_of(cx.tree, cx.current) {
                                (focus_event)(cx);
                                meta.consume();
                            }
                        }
                    }

                    WindowEvent::KeyDown(code, _) => {
                        if *code == Code::Escape {
                            (focus_event)(cx);
                        }
                    }

                    _ => {}
                });
            });
        });

        self
    }
}

/// An arrow view used by the Popup view.
pub(crate) struct Arrow {
    placement: Signal<Placement>,
}

impl Arrow {
    pub(crate) fn new(
        cx: &mut Context,
        placement: Signal<Placement>,
        arrow_size: Signal<Length>,
    ) -> Handle<Self> {
        Self { placement }.build(cx, |_| {}).position_type(PositionType::Absolute).bind(
            placement,
            move |mut handle| {
                let placement = placement.get();
                let (t, b) = match placement {
                    Placement::TopStart | Placement::Top | Placement::TopEnd => {
                        (Percentage(100.0), Stretch(1.0))
                    }
                    Placement::BottomStart | Placement::Bottom | Placement::BottomEnd => {
                        (Stretch(1.0), Percentage(100.0))
                    }
                    _ => (Stretch(1.0), Stretch(1.0)),
                };

                let (l, r) = match placement {
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

                handle = handle
                    .top(t)
                    .bottom(b)
                    .left(l)
                    .right(r)
                    .position_type(PositionType::Absolute)
                    .hoverable(false);

                handle.bind(arrow_size, move |handle| {
                    let arrow_size = arrow_size.get();
                    let arrow_size = arrow_size.to_px().unwrap_or(8.0);
                    let (w, h) = match placement {
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
    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let bounds = cx.bounds();
        let mut path = vg::PathBuilder::new();
        match self.placement.get() {
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
                path.move_to(bounds.bottom_left());
                path.line_to(bounds.center_top());
                path.line_to(bounds.bottom_right());
                path.line_to(bounds.bottom_left());
            }

            Placement::Top | Placement::TopStart | Placement::TopEnd => {
                path.move_to(bounds.top_left());
                path.line_to(bounds.center_bottom());
                path.line_to(bounds.top_right());
                path.line_to(bounds.top_left());
            }

            Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
                path.move_to(bounds.top_left());
                path.line_to(bounds.center_right());
                path.line_to(bounds.bottom_left());
                path.line_to(bounds.top_left());
            }

            Placement::Right | Placement::RightStart | Placement::RightEnd => {
                path.move_to(bounds.top_right());
                path.line_to(bounds.center_left());
                path.line_to(bounds.bottom_right());
                path.line_to(bounds.top_right());
            }

            _ => {}
        }
        path.close();

        let bg = cx.background_color();
        let mut paint = vg::Paint::default();
        paint.set_color(bg);
        let path = path.detach();
        canvas.draw_path(&path, &paint);
    }
}
