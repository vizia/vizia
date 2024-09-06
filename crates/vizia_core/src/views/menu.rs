use crate::modifiers::ModalEvent;
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
                                    }
                                }
                            }

                            _ => {}
                        },
                    );
                });

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
    is_submenu: bool,
}

impl Submenu {
    pub fn new<V: View>(
        cx: &mut Context,
        content: impl Fn(&mut Context) -> Handle<V> + 'static,
        menu: impl Fn(&mut Context) + 'static,
    ) -> Handle<Self> {
        let is_submenu = cx.data::<Self>().is_some();

        let handle = Self { is_open: false, open_on_hover: is_submenu, is_submenu }
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
                Svg::new(cx, ICON_CHEVRON_RIGHT).class("arrow").hoverable(false);
                // });
                Binding::new(cx, Self::is_open, move |cx, is_open| {
                    if is_open.get(cx) {
                        Popup::new(cx, |cx| {
                            (menu)(cx);
                        })
                        .placement(Self::is_submenu.map(|is_submenu| {
                            if *is_submenu {
                                Placement::RightStart
                            } else {
                                Placement::BottomStart
                            }
                        }))
                        .arrow_size(Pixels(0.0))
                        .checked(Self::is_open)
                        .on_hover(|cx| {
                            cx.emit_custom(
                                Event::new(MenuEvent::Close)
                                    .target(cx.current)
                                    .propagate(Propagation::Subtree),
                            )
                        });
                    }
                });
                // .on_press_down(|cx| cx.emit(MenuEvent::CloseAll));
                // .on_blur(|cx| cx.emit(MenuEvent::CloseAll));
            })
            .navigable(true)
            .checked(Self::is_open)
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
                        // Close any open submenus of the parent
                        let parent = cx.tree.get_parent(cx.current).unwrap();
                        cx.emit_custom(
                            Event::new(MenuEvent::Close)
                                .target(parent)
                                .propagate(Propagation::Subtree),
                        );
                        // Open this submenu
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
                } else {
                    // If the parent is a MenuBar then this will reset the is_open state
                    let parent = cx.tree.get_parent(cx.current).unwrap();
                    cx.emit_custom(
                        Event::new(MenuEvent::CloseAll)
                            .target(parent)
                            .propagate(Propagation::Direct),
                    );
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
                cx.emit(MenuEvent::Close);
            })
            .role(Role::MenuItem)
            .navigable(true)
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
