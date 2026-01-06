use crate::modifiers::ModalEvent;
use crate::{icons::ICON_CHEVRON_RIGHT, prelude::*};

/// A view which represents a horizontal group of menus.
pub struct MenuBar {
    is_open: Signal<bool>,
}

impl MenuBar {
    /// Creates a new [MenuBar] view.
    pub fn new(cx: &mut Context, content: impl Fn(&mut Context)) -> Handle<Self> {
        let is_open = cx.state(false);
        let layout_row = cx.state(LayoutType::Row);
        Self { is_open }
            .build(cx, |cx| {
                cx.add_listener(move |menu_bar: &mut Self, cx, event| {
                    let flag = *menu_bar.is_open.get(cx);
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
            .layout_type(layout_row)
    }
}

impl View for MenuBar {
    fn element(&self) -> Option<&'static str> {
        Some("menubar")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|menu_event, _| match menu_event {
            MenuEvent::MenuIsOpen => {
                self.is_open.set(cx, true);
            }

            MenuEvent::CloseAll => {
                self.is_open.set(cx, false);
                cx.emit_custom(
                    Event::new(MenuEvent::Close).target(cx.current).propagate(Propagation::Subtree),
                );
            }

            _ => {}
        });
    }
}

/// Events used by menus.
pub enum MenuEvent {
    /// Toggle the open state of the menu.
    ToggleOpen,
    /// Sets the menu to an open state.
    Open,
    /// Sets the menu to a closed state.
    Close,
    /// Closes the menu and any submenus.
    CloseAll,
    /// Event emitted when a menu or submenu is opened.
    MenuIsOpen,
}

/// A view which represents a submenu within a menu.
pub struct Submenu {
    is_open: Signal<bool>,
    open_on_hover: Signal<bool>,
}

impl Submenu {
    /// Creates a new [Submenu] view.
    pub fn new<V: View>(
        cx: &mut Context,
        content: impl Fn(&mut Context) -> Handle<V> + 'static,
        menu: impl Fn(&mut Context) + 'static,
    ) -> Handle<Self> {
        // Check if any ancestor is a submenu (not just immediate parent)
        // This handles nested submenus where the immediate parent is a Popup
        let submenu_hash = fxhash::hash32("submenu");
        let is_submenu_value = cx.current().parent_iter(&cx.tree).any(|ancestor| {
            cx.style.element.get(ancestor).map(|element| *element == submenu_hash).unwrap_or(false)
        });
        let is_open = cx.state(false);
        let open_on_hover = cx.state(is_submenu_value);
        let is_submenu = cx.state(is_submenu_value);
        let false_signal = cx.state(false);
        let navigable = cx.state(true);
        let layout_row = cx.state(LayoutType::Row);
        let chevron_icon = cx.state(ICON_CHEVRON_RIGHT);

        let handle = Self { is_open, open_on_hover }
            .build(cx, |cx| {
                cx.add_listener(move |menu_button: &mut Self, cx, event| {
                    let flag = *menu_button.is_open.get(cx);
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
                (content)(cx).hoverable(false_signal);
                Svg::new(cx, chevron_icon).class("arrow").hoverable(false_signal);
                // });
                let placement = cx.derived({
                    let is_submenu_signal = is_submenu;
                    move |store| {
                        if *is_submenu_signal.get(store) {
                            Placement::RightStart
                        } else {
                            Placement::BottomStart
                        }
                    }
                });
                let arrow_size = cx.state(Length::Value(LengthValue::Px(0.0)));
                Binding::new(cx, is_open, move |cx| {
                    if *is_open.get(cx) {
                        Popup::new(cx, |cx| {
                            (menu)(cx);
                        })
                        .placement(placement)
                        .arrow_size(arrow_size)
                        .checked(is_open)
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
            .navigable(navigable)
            .checked(is_open)
            .layout_type(layout_row)
            .on_press(|cx| cx.emit(MenuEvent::ToggleOpen));
        handle
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
                    let open_on_hover = if let Some(menu_bar) = cx.data::<MenuBar>() {
                        *menu_bar.is_open.get(cx)
                    } else {
                        *self.open_on_hover.get(cx)
                    };
                    if open_on_hover {
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
                    if *self.is_open.get(cx) {
                        self.is_open.set(cx, false);
                        cx.focus();
                        meta.consume();
                    }
                    // }
                }

                Code::ArrowRight => {
                    if !*self.is_open.get(cx) {
                        self.is_open.set(cx, true);
                    }
                }

                _ => {}
            },

            _ => {}
        });

        event.map(|menu_event, meta| match menu_event {
            MenuEvent::Open => {
                self.is_open.set(cx, true);
                meta.consume();
            }

            MenuEvent::Close => {
                self.is_open.set(cx, false);
                // meta.consume();
            }

            MenuEvent::ToggleOpen => {
                let is_open = *self.is_open.get(cx);
                self.is_open.set(cx, !is_open);
                if !is_open {
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

/// A view which represents a pressable item within a menu.
pub struct MenuButton {}

impl MenuButton {
    /// Creates a new [MenuButton] view.
    pub fn new<V: View>(
        cx: &mut Context,
        action: impl Fn(&mut EventContext) + Send + Sync + 'static,
        content: impl Fn(&mut Context) -> Handle<V> + 'static,
    ) -> Handle<Self> {
        let false_signal = cx.state(false);
        let true_signal = cx.state(true);
        Self {}
            .build(cx, |cx| {
                (content)(cx).hoverable(false_signal);
            })
            .on_press(move |cx| {
                (action)(cx);
                cx.emit(MenuEvent::CloseAll);
                cx.emit(ModalEvent::HideMenu);
                cx.emit(MenuEvent::Close);
            })
            .role(Role::MenuItem)
            .navigable(true_signal)
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
