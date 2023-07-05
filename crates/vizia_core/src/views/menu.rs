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
        let handle = Self { is_open: false, open_on_hover: false }
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
                // cx.emit(MenuEvent::Close);
            })
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
    L::Target: Clone + Into<bool>,
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
