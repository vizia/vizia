use crate::modifiers::{ModalEvent, ModalModel};
use crate::{icons::ICON_CHEVRON_RIGHT, prelude::*};

#[derive(Lens)]
pub struct MenuBar {
    is_open: bool,
}

impl MenuBar {
    pub fn new(cx: &mut Context, content: impl Fn(&mut Context)) -> Handle<Self> {
        Self { is_open: false }
            .build(cx, |cx| {
                cx.add_listener(move |menu_bar: &mut Self, cx, event| {
                    let flag: bool = menu_bar.is_open;
                    event.map(
                        |window_event, meta: &mut crate::events::EventMeta| match window_event {
                            WindowEvent::MouseDown(_) => {
                                if flag && meta.origin != cx.current() {
                                    // Check if the mouse was pressed outside of any descendants
                                    if !cx.hovered.is_descendant_of(cx.tree, cx.current) {
                                        cx.emit(MenuEvent::CloseAll);
                                        // TODO: This might be needed
                                        // meta.consume();
                                    }
                                }
                            }

                            _ => {}
                        },
                    );
                });

                // MenuController { open_menu: None }.build(cx);
                (content)(cx);
            })
            .layout_type(LayoutType::Row)
    }
}

impl View for MenuBar {
    fn element(&self) -> Option<&'static str> {
        Some("menubar")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|menu_event, _| match menu_event {
            MenuEvent::MenuIsOpen => {
                self.is_open = true;
            }

            MenuEvent::CloseAll => {
                self.is_open = false;
                cx.emit_custom(
                    Event::new(MenuEvent::Close).target(cx.current).propagate(Propagation::Subtree),
                );
            }

            _ => {}
        });
    }
}

pub enum MenuEvent {
    ToggleOpen,
    HoverMenu,
    Open,
    Close,
    CloseAll,
    MenuIsOpen,
}

#[derive(Lens)]
pub struct Submenu {
    is_open: bool,
    open_on_hover: bool,
}

impl Submenu {
    pub fn new<V: View>(
        cx: &mut Context,
        content: impl Fn(&mut Context) -> Handle<V> + 'static,
        menu: impl Fn(&mut Context) + 'static,
    ) -> Handle<Self> {
        let handle = Self { is_open: false, open_on_hover: true }
            .build(cx, |cx| {
                cx.add_listener(move |menu_button: &mut Self, cx, event| {
                    let flag: bool = menu_button.is_open;
                    event.map(
                        |window_event, meta: &mut crate::events::EventMeta| match window_event {
                            WindowEvent::MouseDown(_) => {
                                if flag && meta.origin != cx.current() {
                                    // Check if the mouse was pressed outside of any descendants
                                    if !cx.hovered.is_descendant_of(cx.tree, cx.current) {
                                        cx.emit(MenuEvent::CloseAll);
                                        cx.emit(MenuEvent::Close);
                                        // TODO: This might be needed
                                        // meta.consume();
                                    }
                                }
                            }

                            _ => {}
                        },
                    );
                });
                // HStack::new(cx, |cx| {
                (content)(cx).hoverable(false);
                Label::new(cx, ICON_CHEVRON_RIGHT).class("icon").class("arrow").hoverable(false);
                // });
                MenuPopup::new(cx, Submenu::is_open, false, move |cx| {
                    (menu)(cx);
                });
                // .on_press_down(|cx| cx.emit(MenuEvent::CloseAll));
                // .on_blur(|cx| cx.emit(MenuEvent::CloseAll));
            })
            // .navigable(true)
            .checked(Submenu::is_open)
            .layout_type(LayoutType::Row)
            .on_press(|cx| cx.emit(MenuEvent::ToggleOpen));

        if handle.data::<MenuBar>().is_some() {
            handle.bind(MenuBar::is_open, |handle, is_open| {
                let is_open = is_open.get(&handle);
                handle.modify(|menu_button| menu_button.open_on_hover = is_open);
            })
        } else {
            handle
        }
    }
}

impl View for Submenu {
    fn element(&self) -> Option<&'static str> {
        Some("submenu")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseEnter => {
                if meta.target == cx.current {
                    // if self.open_on_hover {
                    //     cx.focus();
                    // }
                    if self.open_on_hover {
                        let parent = cx.tree.get_parent(cx.current).unwrap();
                        cx.emit_custom(
                            Event::new(MenuEvent::Close)
                                .target(parent)
                                .propagate(Propagation::Subtree),
                        );
                        cx.emit(MenuEvent::Open);
                    }
                }
            }

            WindowEvent::KeyDown(code, _) => match code {
                Code::ArrowLeft => {
                    // if cx.is_focused() {
                    if self.is_open {
                        self.is_open = false;
                        cx.focus();
                        meta.consume();
                    }
                    // }
                }

                Code::ArrowRight => {
                    if !self.is_open {
                        self.is_open = true;
                    }
                }

                _ => {}
            },

            _ => {}
        });

        event.map(|menu_event, meta| match menu_event {
            MenuEvent::Open => {
                self.is_open = true;
                meta.consume();
            }

            MenuEvent::Close => {
                self.is_open = false;
                // meta.consume();
            }

            MenuEvent::ToggleOpen => {
                self.is_open ^= true;
                if self.is_open {
                    cx.emit(MenuEvent::MenuIsOpen);
                }
                meta.consume();
            }

            _ => {}
        });
    }
}

#[derive(Lens)]
pub struct MenuButton {}

impl MenuButton {
    pub fn new<V: View>(
        cx: &mut Context,
        action: impl Fn(&mut EventContext) + Send + Sync + 'static,
        content: impl Fn(&mut Context) -> Handle<V> + 'static,
    ) -> Handle<Self> {
        Self {}
            .build(cx, |cx| {
                (content)(cx).hoverable(false);
            })
            .on_press(move |cx| {
                (action)(cx);
                cx.emit(MenuEvent::CloseAll);
                cx.emit(ModalEvent::HideMenu);
                // cx.emit(MenuEvent::Close);
            })
            .role(Role::MenuItem)
        // .navigable(true)
    }
}

impl View for MenuButton {
    fn element(&self) -> Option<&'static str> {
        Some("menubutton")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseEnter => {
                if meta.target == cx.current {
                    let parent = cx.tree.get_parent(cx.current).unwrap();
                    cx.emit_custom(
                        Event::new(MenuEvent::Close).target(parent).propagate(Propagation::Subtree),
                    );
                }
            }

            _ => {}
        });
    }
}

pub struct MenuPopup<L> {
    lens: L,
}

impl<L> MenuPopup<L>
where
    L: Lens<Target = bool>,
{
    pub fn new<F>(cx: &mut Context, lens: L, _capture_focus: bool, content: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        Self { lens }
            .build(cx, |cx| {
                let parent = cx.current;

                (content)(cx);

                Binding::new(cx, lens, move |cx, _| {
                    if let Some(geo) = cx.cache.geo_changed.get_mut(parent) {
                        geo.set(GeoChanged::WIDTH_CHANGED, true);
                    }
                });
            })
            .role(Role::Dialog)
            .checked(lens)
            .position_type(PositionType::SelfDirected)
            .z_index(100)
    }
}

impl<'a, L> Handle<'a, MenuPopup<L>>
where
    L: Lens,
    L::Target: Clone + Data + Into<bool>,
{
    /// Registers a callback for when the user clicks off of the popup, usually with the intent of
    /// closing it.
    pub fn on_blur<F>(self, f: F) -> Self
    where
        F: 'static + Fn(&mut EventContext),
    {
        let focus_event = Box::new(f);
        self.cx.with_current(self.entity, |cx| {
            cx.add_listener(move |popup: &mut MenuPopup<L>, cx, event| {
                let flag: bool = popup.lens.get(cx).into();
                event.map(|window_event, meta| match window_event {
                    WindowEvent::MouseDown(_) => {
                        if flag && meta.origin != cx.current() {
                            // Check if the mouse was pressed outside of any descendants
                            if !cx.hovered.is_descendant_of(cx.tree, cx.current) {
                                (focus_event)(cx);
                                meta.consume();
                            }
                        }
                    }

                    WindowEvent::KeyDown(code, _) => {
                        if flag && *code == Code::Escape {
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

impl<L> View for MenuPopup<L>
where
    L: Lens,
    L::Target: Into<bool>,
{
    fn element(&self) -> Option<&'static str> {
        Some("popup")
    }
}

pub struct MenuDivider {}

impl MenuDivider {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            Element::new(cx).class("line");
        })
    }
}

impl View for MenuDivider {
    fn element(&self) -> Option<&'static str> {
        Some("menu-divider")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseEnter => {
                if meta.target == cx.current {
                    let parent = cx.tree.get_parent(cx.current).unwrap();
                    cx.emit_custom(
                        Event::new(MenuEvent::Close).target(parent).propagate(Propagation::Subtree),
                    );
                }
            }

            _ => {}
        });
    }
}

use crate::context::TreeProps;
use crate::vg;

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
pub struct Menu {
    default_placement: Placement,
    placement: Placement,
    show_arrow: bool,
    arrow_size: Length,
}

impl Menu {
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
            Binding::new(cx, Menu::show_arrow, |cx, show_arrow| {
                if show_arrow.get(cx) {
                    Arrow::new(cx);
                }
            });
            (content)(cx);
        })
        .z_index(100)
        .bind(Menu::placement, |mut handle, placement| {
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

            handle.bind(Menu::arrow_size, move |handle, arrow_size| {
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
            ex.add_listener(move |menu: &mut Menu, cx, event| {
                let flag = ModalModel::menu_visible.get(cx);
                event.map(|window_event, meta| match window_event {
                    WindowEvent::MouseDown(_) => {
                        if flag && meta.origin != cx.current() {
                            // Check if the mouse was pressed outside of any descendants
                            if !cx.hovered.is_descendant_of(cx.tree, cx.current) {
                                // cx.toggle_class("vis", false);
                                cx.emit(ModalEvent::HideMenu);
                                meta.consume();
                            }
                        }
                    }

                    WindowEvent::KeyDown(code, _) => {
                        if flag && *code == Code::Escape {
                            cx.emit(ModalEvent::HideMenu);
                            // cx.toggle_class("vis", false);
                        }
                    }

                    _ => {}
                });
            });

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

impl View for Menu {
    fn element(&self) -> Option<&'static str> {
        Some("menu")
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
    Cursor,
}

impl<'a> Handle<'a, Menu> {
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
        Self {}.build(cx, |_| {}).bind(Menu::placement, |mut handle, placement| {
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

            handle.bind(Menu::arrow_size, move |handle, arrow_size| {
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
        match Menu::placement.get(cx) {
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
