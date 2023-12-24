use crate::prelude::*;

use crate::context::TreeProps;
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

/// A Menu view.
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
pub struct Popup {
    default_placement: Placement,
    placement: Placement,
    show_arrow: bool,
    arrow_size: Length,
}

impl Popup {
    /// Creates a new Menu view with the given content.
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
            Binding::new(cx, Popup::show_arrow, |cx, show_arrow| {
                if show_arrow.get(cx) {
                    Arrow::new(cx);
                }
            });
            (content)(cx);
        })
        .z_index(100)
        .bind(Popup::placement, |mut handle, placement| {
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
                _ => (Stretch(1.0), Stretch(1.0)),
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
                _ => (Stretch(1.0), Stretch(1.0)),
            };
            handle = handle.top(t).bottom(b).left(l).right(r);
            handle.bind(Popup::arrow_size, move |handle, arrow_size| {
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
        .on_build(|ex| {
            // ex.add_listener(move |menu: &mut Menu, cx, event| {
            //     let flag = ModalModel::menu_visible.get(cx);
            //     event.map(|window_event, meta| match window_event {
            //         WindowEvent::MouseDown(_) => {
            //             if flag && meta.origin != cx.current() {
            //                 // Check if the mouse was pressed outside of any descendants
            //                 if !cx.hovered.is_descendant_of(cx.tree, cx.current) {
            //                     // cx.toggle_class("vis", false);
            //                     cx.emit(ModalEvent::HideMenu);
            //                     meta.consume();
            //                 }
            //             }
            //         }

            //         WindowEvent::KeyDown(code, _) => {
            //             if flag && *code == Code::Escape {
            //                 cx.emit(ModalEvent::HideMenu);
            //                 // cx.toggle_class("vis", false);
            //             }
            //         }

            //         _ => {}
            //     });
            // });

            // ex.add_listener(move |menu: &mut Menu, ex, event| {
            //     let flag = ModalModel::tooltip_visible.get(ex);
            //     event.map(|window_event, meta| match window_event {
            //         WindowEvent::MouseDown(_) => {
            //             if flag && meta.origin != ex.current() {
            //                 ex.toggle_class("vis", false);
            //             }
            //         }

            //         WindowEvent::MouseMove(x, y) => {
            //             if menu.placement == Placement::Cursor {
            //                 if !x.is_nan() && !y.is_nan() {
            //                     let scale = ex.scale_factor();
            //                     let parent = ex.parent();
            //                     let parent_bounds = ex.cache.get_bounds(parent);
            //                     if parent_bounds.contains_point(*x, *y) {
            //                         ex.set_left(Pixels(
            //                             ((*x - parent_bounds.x) - ex.bounds().width() / 2.0)
            //                                 / scale,
            //                         ));
            //                         ex.set_top(Pixels((*y - parent_bounds.y) / scale));
            //                     }
            //                 }
            //             }
            //         }

            //         _ => {}
            //     });
            // });
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

            Placement::Bottom if dist_left < 0.0 => {
                if dist_left < 0.0 && dist_right < 0.0 {
                    return;
                }

                self.placement = Placement::BottomStart;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::Bottom if dist_right < 0.0 => {
                if dist_left < 0.0 && dist_right < 0.0 {
                    return;
                }

                self.placement = Placement::BottomEnd;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::BottomEnd if dist_left < 0.0 => {
                self.placement = Placement::Bottom;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::BottomStart if dist_right < 0.0 => {
                self.placement = Placement::Bottom;
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

            Placement::Top if dist_left < 0.0 => {
                if dist_left < 0.0 && dist_right < 0.0 {
                    return;
                }

                self.placement = Placement::TopStart;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::Top if dist_right < 0.0 => {
                if dist_left < 0.0 && dist_right < 0.0 {
                    return;
                }

                self.placement = Placement::TopEnd;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::TopEnd if dist_left < 0.0 => {
                self.placement = Placement::Top;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::TopStart if dist_right < 0.0 => {
                self.placement = Placement::Top;
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

            Placement::Left if dist_top < 0.0 => {
                if dist_top < 0.0 && dist_bottom < 0.0 {
                    return;
                }

                self.placement = Placement::LeftStart;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::Left if dist_bottom < 0.0 => {
                if dist_top < 0.0 && dist_bottom < 0.0 {
                    return;
                }

                self.placement = Placement::LeftEnd;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::LeftEnd if dist_top < 0.0 => {
                self.placement = Placement::Left;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::LeftStart if dist_bottom < 0.0 => {
                self.placement = Placement::Left;
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

            Placement::Right if dist_top < 0.0 => {
                if dist_top < 0.0 && dist_bottom < 0.0 {
                    return;
                }

                self.placement = Placement::RightStart;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::Right if dist_bottom < 0.0 => {
                if dist_top < 0.0 && dist_bottom < 0.0 {
                    return;
                }

                self.placement = Placement::RightEnd;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::RightEnd if dist_top < 0.0 => {
                self.placement = Placement::Right;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            Placement::RightStart if dist_bottom < 0.0 => {
                self.placement = Placement::Right;
                self.place(dist_top, dist_bottom, dist_left, dist_right);
            }

            _ => {}
        }
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
                println!("popup geo changed");
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

/// Describes the placement of a popup relative to its parent element.
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
    Cursor,
}

impl<'a> Handle<'a, Popup> {
    // TODO: Change this to use Res when lens value PR is merged
    /// Sets the position where the tooltip should appear relative to its parent element.
    /// Defaults to `Placement::Bottom`.
    pub fn placement(self, placement: Placement) -> Self {
        self.modify(|popup| {
            popup.placement = placement;
            popup.default_placement = placement;
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

    /// Registers a callback for when the user clicks off of the popup, usually with the intent of
    /// closing it.
    pub fn on_blur<F>(self, f: F) -> Self
    where
        F: 'static + Fn(&mut EventContext),
    {
        let focus_event = Box::new(f);
        self.cx.with_current(self.entity, |cx| {
            cx.add_listener(move |popup: &mut Popup, cx, event| {
                // let flag: bool = popup.lens.get(cx).into();
                event.map(|window_event, meta| match window_event {
                    WindowEvent::PressDown { mouse: _ } => {
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
pub(crate) struct Arrow {}

impl Arrow {
    pub(crate) fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {}).bind(Popup::placement, |mut handle, placement| {
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

// pub struct Popup<L> {
//     lens: L,
// }

// impl<L> Popup<L>
// where
//     L: Lens<Target = bool>,
// {
//     pub fn new<F>(cx: &mut Context, lens: L, capture_focus: bool, content: F) -> Handle<Self>
//     where
//         F: 'static + Fn(&mut Context),
//     {
//         Self { lens }
//             .build(cx, |cx| {
//                 let parent = cx.current;
//                 Binding::new(cx, lens, move |cx, lens| {
//                     if let Some(geo) = cx.cache.geo_changed.get_mut(parent) {
//                         geo.set(GeoChanged::WIDTH_CHANGED, true);
//                     }

//                     if lens.get(cx) {
//                         (content)(cx);
//                     }
//                 });
//             })
//             .bind(lens, move |handle, val| {
//                 if val.get(&handle) && capture_focus {
//                     handle.lock_focus_to_within();
//                 }
//             })
//             .role(Role::Dialog)
//             .checked(lens)
//             .position_type(PositionType::SelfDirected)
//             .z_index(100)
//     }
// }

// impl<'a, L> Handle<'a, Popup<L>>
// where
//     L: Lens,
//     L::Target: Clone + Data + Into<bool>,
// {
//     /// Registers a callback for when the user clicks off of the popup, usually with the intent of
//     /// closing it.
//     pub fn on_blur<F>(self, f: F) -> Self
//     where
//         F: 'static + Fn(&mut EventContext),
//     {
//         let focus_event = Box::new(f);
//         self.cx.with_current(self.entity, |cx| {
//             cx.add_listener(move |popup: &mut Popup<L>, cx, event| {
//                 let flag: bool = popup.lens.get(cx).into();
//                 event.map(|window_event, meta| match window_event {
//                     WindowEvent::PressDown { mouse: _ } => {
//                         if flag && meta.origin != cx.current() {
//                             // Check if the mouse was pressed outside of any descendants
//                             if !cx.hovered.is_descendant_of(cx.tree, cx.current) {
//                                 (focus_event)(cx);
//                                 meta.consume();
//                             }
//                         }
//                     }

//                     WindowEvent::KeyDown(code, _) => {
//                         if flag && *code == Code::Escape {
//                             (focus_event)(cx);
//                         }
//                     }

//                     _ => {}
//                 });
//             });
//         });

//         self
//     }
// }

// impl<L> View for Popup<L>
// where
//     L: Lens,
//     L::Target: Into<bool>,
// {
//     fn element(&self) -> Option<&'static str> {
//         Some("popup")
//     }

//     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
//         event.map(|window_event, _| match window_event {
//             WindowEvent::GeometryChanged(_) => {
//                 let bounds = cx.bounds();
//                 let window_bounds = cx.cache.get_bounds(Entity::root());

//                 let dist_bottom = window_bounds.bottom() - bounds.bottom();
//                 let dist_top = bounds.top() - window_bounds.top();

//                 let scale = cx.scale_factor();

//                 if dist_bottom < 0.0 {
//                     if dist_top.abs() < dist_bottom.abs() {
//                         cx.set_translate((Pixels(0.0), Pixels(-dist_top.abs() / scale)));
//                     } else {
//                         cx.set_translate((Pixels(0.0), Pixels(-dist_bottom.abs() / scale)));
//                     }
//                 } else {
//                     cx.set_translate((Pixels(0.0), Pixels(4.0)));
//                 }
//             }

//             WindowEvent::FocusOut => {
//                 if !cx.focused.is_descendant_of(cx.tree, cx.current) {
//                     cx.emit(PopupEvent::Close);
//                 }
//             }

//             _ => {}
//         });
//     }
// }
