use crate::modifiers::ModalEvent;
use crate::style::{Abilities, Display};
use crate::{icons::ICON_CHEVRON_RIGHT, prelude::*};

fn first_focusable_descendant(tree: &Tree<Entity>, style: &Style, root: Entity) -> Option<Entity> {
    vizia_storage::TreeIterator::subtree(tree, root).skip(1).find(|node| {
        if style.display.get(*node).copied().unwrap_or_default() == Display::None {
            return false;
        }
        if style.disabled.get(*node).copied().unwrap_or_default() {
            return false;
        }
        style
            .abilities
            .get(*node)
            .map(|abilities| abilities.contains(Abilities::FOCUSABLE))
            .unwrap_or(false)
    })
}

fn is_focusable_item(cx: &EventContext, entity: Entity) -> bool {
    if cx.style.display.get(entity).copied().unwrap_or_default() == Display::None {
        return false;
    }

    if cx.style.disabled.get(entity).copied().unwrap_or_default() {
        return false;
    }

    cx.style
        .abilities
        .get(entity)
        .map(|abilities| abilities.contains(Abilities::FOCUSABLE))
        .unwrap_or(false)
}

fn first_focusable_child(cx: &EventContext, root: Entity) -> Option<Entity> {
    let mut child = cx.tree.get_first_child(root);
    while let Some(entity) = child {
        if is_focusable_item(cx, entity) {
            return Some(entity);
        }
        child = cx.tree.get_next_sibling(entity);
    }

    None
}

fn first_menu_bar_item(cx: &EventContext, root: Entity) -> Option<Entity> {
    first_focusable_child(cx, root)
}

/// A view which represents a horizontal group of menus.
pub struct MenuBar {
    is_open: Signal<bool>,
    focused_item: Signal<Option<Entity>>,
}

impl MenuBar {
    /// Creates a new [MenuBar] view.
    pub fn new(cx: &mut Context, content: impl Fn(&mut Context)) -> Handle<Self> {
        let is_open = Signal::new(false);
        let focused_item = Signal::new(None);

        Self { is_open, focused_item }
            .build(cx, |cx| {
                cx.add_listener(move |menu_bar: &mut Self, cx, event| {
                    let flag = menu_bar.is_open.get();
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
            .role(Role::MenuBar)
            .orientation(Orientation::Horizontal)
            .navigable(true)
    }
}

impl View for MenuBar {
    fn element(&self) -> Option<&'static str> {
        Some("menubar")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::FocusIn => {
                if meta.target == cx.current() {
                    if let Some(first_item) = first_menu_bar_item(cx, cx.current()) {
                        focus_entity(cx, first_item);
                        self.focused_item.set(Some(first_item));
                        meta.consume();
                    }
                }
            }

            WindowEvent::KeyDown(Code::ArrowLeft, _) => {
                if !self.is_open.get() {
                    cx.emit(MenuEvent::FocusPrevMenuBarItem);

                    meta.consume();
                }
            }

            WindowEvent::KeyDown(Code::ArrowRight, _) => {
                if !self.is_open.get() {
                    cx.emit(MenuEvent::FocusNextMenuBarItem);
                    meta.consume();
                }
            }
            _ => {}
        });

        event.map(|menu_event, _| match menu_event {
            MenuEvent::MenuIsOpen => {
                self.is_open.set_if_changed(true);
            }

            MenuEvent::CloseAll => {
                self.is_open.set_if_changed(false);
                cx.emit_custom(
                    Event::new(MenuEvent::Close).target(cx.current).propagate(Propagation::Subtree),
                );
            }

            MenuEvent::FocusPrevMenuBarItem => {
                if let Some(current) = self.focused_item.get() {
                    if let Some(next) = prev_sibling_wrapped(cx, current) {
                        focus_entity(cx, next);
                        if self.is_open.get() {
                            cx.emit_custom(
                                Event::new(MenuEvent::Close)
                                    .target(current)
                                    .propagate(Propagation::Subtree),
                            );
                            cx.emit_custom(
                                Event::new(MenuEvent::TriggerArrowDown)
                                    .target(next)
                                    .propagate(Propagation::Direct),
                            );
                        }
                        self.focused_item.set(Some(next));
                    }
                }
            }

            MenuEvent::FocusNextMenuBarItem => {
                if let Some(current) = self.focused_item.get() {
                    if let Some(next) = next_sibling_wrapped(cx, current) {
                        focus_entity(cx, next);
                        if self.is_open.get() {
                            cx.emit_custom(
                                Event::new(MenuEvent::Close)
                                    .target(current)
                                    .propagate(Propagation::Subtree),
                            );
                            cx.emit_custom(
                                Event::new(MenuEvent::TriggerArrowDown)
                                    .target(next)
                                    .propagate(Propagation::Direct),
                            );
                        }
                        self.focused_item.set(Some(next));
                    }
                }
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
    /// Closes the active menu and restores focus to the trigger that opened it.
    CloseAndFocusTrigger,
    /// Closes the menu and any submenus.
    CloseAll,
    /// Event emitted when a menu or submenu is opened.
    MenuIsOpen,
    /// Move focus to the next item within the open popup.
    FocusNext,
    /// Move focus to the previous item within the open popup.
    FocusPrev,
    /// Move focus to the first item within the open popup.
    FocusFirst,
    /// Move focus to the last item within the open popup.
    FocusLast,
    /// ArrowDown pressed on a submenu trigger (open popup or within popup).
    TriggerArrowDown,
    /// ArrowRight pressed on a submenu trigger (navigate or open submenu).
    TriggerArrowRight,
    /// ArrowLeft pressed on a submenu trigger (navigate or close submenu).
    TriggerArrowLeft,
    /// Focus the next top-level menu bar item (used when pressing arrow keys on menu bar items).
    FocusNextMenuBarItem,
    /// Focus the previous top-level menu bar item (used when pressing arrow keys on menu bar items).
    FocusPrevMenuBarItem,
}

fn focus_entity(cx: &mut EventContext, entity: Entity) {
    cx.with_current(entity, |cx| cx.focus());
}

fn next_sibling_wrapped(cx: &EventContext, entity: Entity) -> Option<Entity> {
    let parent = cx.tree.get_parent(entity)?;
    let mut next = cx.tree.get_next_sibling(entity).or_else(|| cx.tree.get_first_child(parent));

    while let Some(candidate) = next {
        if candidate == entity {
            break;
        }

        if is_focusable_item(cx, candidate) {
            return Some(candidate);
        }

        next = cx.tree.get_next_sibling(candidate).or_else(|| cx.tree.get_first_child(parent));
    }

    None
}

fn prev_sibling_wrapped(cx: &EventContext, entity: Entity) -> Option<Entity> {
    let parent = cx.tree.get_parent(entity)?;
    let mut prev =
        cx.tree.get_prev_sibling(entity).or_else(|| cx.tree.get_last_child(parent).copied());

    while let Some(candidate) = prev {
        if candidate == entity {
            break;
        }

        if is_focusable_item(cx, candidate) {
            return Some(candidate);
        }

        prev =
            cx.tree.get_prev_sibling(candidate).or_else(|| cx.tree.get_last_child(parent).copied());
    }

    None
}

fn focus_next_sibling_wrapped(cx: &mut EventContext) -> bool {
    let current = cx.current();
    if let Some(next) = next_sibling_wrapped(cx, current) {
        focus_entity(cx, next);
        return true;
    }

    false
}

fn focus_prev_sibling_wrapped(cx: &mut EventContext) -> bool {
    let current = cx.current();
    if let Some(prev) = prev_sibling_wrapped(cx, current) {
        focus_entity(cx, prev);
        return true;
    }

    false
}

fn focus_first_sibling(cx: &mut EventContext) -> bool {
    let current = cx.current();
    let Some(parent) = cx.tree.get_parent(current) else {
        return false;
    };

    if let Some(first) = cx.tree.get_first_child(parent) {
        focus_entity(cx, first);
        return true;
    }

    false
}

fn focus_last_sibling(cx: &mut EventContext) -> bool {
    let current = cx.current();
    let Some(parent) = cx.tree.get_parent(current) else {
        return false;
    };

    if let Some(last) = cx.tree.get_last_child(parent).copied() {
        focus_entity(cx, last);
        return true;
    }

    false
}

/// A popup menu view used by submenus and context menus.
pub struct Menu {}

impl Menu {
    /// Creates a new [Menu] popup.
    pub fn new(
        cx: &mut Context,
        placement: impl Res<Placement> + 'static,
        focus_on_open: impl Res<bool> + 'static,
        content: impl Fn(&mut Context),
    ) -> Handle<'_, Popover> {
        let focus_on_open = focus_on_open.to_signal(cx);

        Popover::new(cx, move |cx| {
            let popup = cx.current();

            // Keymap scoped to this popup: handles in-menu keyboard navigation.
            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                    KeymapEntry::new("Focus Next", |cx| cx.emit(MenuEvent::FocusNext)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                    KeymapEntry::new("Focus Prev", |cx| cx.emit(MenuEvent::FocusPrev)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Home),
                    KeymapEntry::new("Focus First", |cx| cx.emit(MenuEvent::FocusFirst)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::End),
                    KeymapEntry::new("Focus Last", |cx| cx.emit(MenuEvent::FocusLast)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Escape),
                    KeymapEntry::new("Close Active Menu", |cx| {
                        cx.emit(MenuEvent::CloseAndFocusTrigger)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowLeft),
                    KeymapEntry::new("Close", |cx| cx.emit(MenuEvent::TriggerArrowLeft)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Tab),
                    KeymapEntry::new("Close All", |cx| cx.emit(MenuEvent::CloseAll)),
                ),
            ])
            .build(cx);

            (content)(cx);

            if focus_on_open.get() {
                if let Some(first_item) = first_focusable_descendant(&cx.tree, &cx.style, popup) {
                    cx.with_current(first_item, |cx| cx.focus());
                }
            }
        })
        .role(Role::Menu)
        .lock_focus_to_within()
        .placement(placement)
        .arrow_size(Pixels(0.0))
    }
}

impl View for Menu {
    fn element(&self) -> Option<&'static str> {
        Some("menu")
    }
}

/// A view which represents a submenu within a menu.
pub struct Submenu {
    is_open: Signal<bool>,
    focus_on_open: Signal<bool>,
    open_on_hover: bool,
    is_submenu: bool,
}

impl Submenu {
    /// Creates a new [Submenu] view.
    pub fn new<V: View>(
        cx: &mut Context,
        content: impl Fn(&mut Context) -> Handle<V> + 'static,
        menu: impl Fn(&mut Context) + 'static,
    ) -> Handle<Self> {
        let is_submenu = cx.try_data::<Submenu>().is_some();
        let is_menu_bar_item = cx.try_data::<MenuBar>().is_some();

        let is_open = Signal::new(false);
        let focus_on_open = Signal::new(false);
        let submenu_popup_placement =
            if is_submenu { Placement::RightStart } else { Placement::BottomStart };

        let handle = Self { is_open, focus_on_open, open_on_hover: is_submenu, is_submenu }
            .build(cx, |cx| {
                cx.add_listener(move |menu_button: &mut Self, cx, event| {
                    let flag = menu_button.is_open.get();
                    event.map(
                        |window_event, meta: &mut crate::events::EventMeta| match window_event {
                            WindowEvent::MouseDown(_) => {
                                if flag && meta.origin != cx.current() {
                                    // Check if the mouse was pressed outside of any descendants
                                    if !cx.hovered.is_descendant_of(cx.tree, cx.current) {
                                        cx.emit(MenuEvent::CloseAll);
                                        cx.emit(MenuEvent::Close);
                                    }
                                }
                            }

                            _ => {}
                        },
                    );
                });

                // Keymap for the submenu trigger itself: handles arrow keys and tab.
                Keymap::from(vec![
                    (
                        KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                        KeymapEntry::new("Open Submenu", |cx| cx.emit(MenuEvent::TriggerArrowDown)),
                    ),
                    (
                        KeyChord::new(Modifiers::empty(), Code::Space),
                        KeymapEntry::new("Open Submenu", |cx| cx.emit(MenuEvent::TriggerArrowDown)),
                    ),
                    (
                        KeyChord::new(Modifiers::empty(), Code::Enter),
                        KeymapEntry::new("Open Submenu", |cx| cx.emit(MenuEvent::TriggerArrowDown)),
                    ),
                    (
                        KeyChord::new(Modifiers::empty(), Code::ArrowRight),
                        KeymapEntry::new("Navigate Right", |cx| {
                            cx.emit(MenuEvent::TriggerArrowRight)
                        }),
                    ),
                ])
                .build(cx);

                (content)(cx).hoverable(false);
                Svg::new(cx, ICON_CHEVRON_RIGHT).class("arrow").hoverable(false);

                Binding::new(cx, is_open, move |cx| {
                    let open = is_open.get();
                    if open {
                        Menu::new(cx, submenu_popup_placement, focus_on_open, |cx| (menu)(cx))
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
            })
            .focusable(true)
            .navigable(!is_menu_bar_item && !is_submenu)
            .role(Role::MenuItem)
            .checked(is_open)
            .expanded(is_open)
            .layout_type(LayoutType::Row)
            .on_press(|cx| cx.emit(MenuEvent::ToggleOpen));

        if handle.try_data::<MenuBar>().is_some() {
            let menu_bar_open = handle.data::<MenuBar>().is_open;
            handle.bind(menu_bar_open, move |handle| {
                let is_open = menu_bar_open.get();
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
                if meta.target == cx.current && self.open_on_hover {
                    // Close any open submenus of the parent.
                    let parent = cx.tree.get_parent(cx.current).unwrap();
                    cx.emit_custom(
                        Event::new(MenuEvent::Close).target(parent).propagate(Propagation::Subtree),
                    );
                    // cx.focus();
                    self.focus_on_open.set(false);
                    cx.emit(MenuEvent::Open);
                }
            }

            _ => {}
        });

        event.map(|menu_event, meta| match menu_event {
            MenuEvent::TriggerArrowDown => {
                let popup_open = self.is_open.get();
                if popup_open {
                    // Popup keymap will handle the navigation; just consume here to stop propagation.
                    meta.consume();
                } else if !self.is_submenu {
                    // Top-level menubar item: open the popup.
                    self.focus_on_open.set(true);
                    self.is_open.set(true);
                    cx.emit(MenuEvent::MenuIsOpen);
                    meta.consume();
                }
            }

            MenuEvent::TriggerArrowRight => {
                if self.is_submenu {
                    let popup_open = self.is_open.get();
                    if !popup_open {
                        self.focus_on_open.set(true);
                        self.is_open.set(true);
                        cx.emit(MenuEvent::MenuIsOpen);
                    }
                }
                meta.consume();
            }

            MenuEvent::TriggerArrowLeft => {
                // Check if that Submenu is a direct child of MenuBar.
                let Some(parent_of_trigger) = cx.tree.get_parent(cx.current()) else {
                    return;
                };

                let is_direct_submenu_of_menubar =
                    cx.get_view_with::<MenuBar>(parent_of_trigger).is_some();

                if is_direct_submenu_of_menubar {
                    cx.emit(MenuEvent::FocusPrevMenuBarItem);
                } else {
                    cx.emit(MenuEvent::CloseAndFocusTrigger);
                }

                meta.consume();
            }
            MenuEvent::Open => {
                if !self.is_open.get() {
                    self.focus_on_open.set(false);
                    self.is_open.set(true);
                    cx.emit(MenuEvent::MenuIsOpen);
                }
                meta.consume();
            }

            MenuEvent::CloseAll => {
                self.is_open.set_if_changed(false);
                cx.emit_custom(
                    Event::new(MenuEvent::Close).target(cx.current).propagate(Propagation::Subtree),
                );
            }

            MenuEvent::Close => {
                self.is_open.set_if_changed(false);
            }

            MenuEvent::CloseAndFocusTrigger => {
                self.is_open.set_if_changed(false);
                cx.focus();
                if !self.is_submenu {
                    cx.emit(MenuEvent::CloseAll);
                }
                meta.consume();
            }

            // Focus navigation — dispatched by the popup-scoped Keymap.
            // We run the helper in the context of the currently focused entity so
            // sibling-relative movement is correct regardless of nesting depth.
            MenuEvent::FocusNext => {
                let focused = cx.focused();
                cx.with_current(focused, |cx| {
                    focus_next_sibling_wrapped(cx);
                });
                meta.consume();
            }

            MenuEvent::FocusPrev => {
                let focused = cx.focused();
                cx.with_current(focused, |cx| {
                    focus_prev_sibling_wrapped(cx);
                });
                meta.consume();
            }

            MenuEvent::FocusFirst => {
                let focused = cx.focused();
                cx.with_current(focused, |cx| {
                    focus_first_sibling(cx);
                });
                meta.consume();
            }

            MenuEvent::FocusLast => {
                let focused = cx.focused();
                cx.with_current(focused, |cx| {
                    focus_last_sibling(cx);
                });
                meta.consume();
            }

            MenuEvent::ToggleOpen => {
                let is_open = !self.is_open.get();
                self.is_open.set(is_open);
                if is_open {
                    self.focus_on_open.set(false);
                    cx.emit(MenuEvent::MenuIsOpen);
                } else {
                    // If the parent is a MenuBar then this will reset the is_open state.
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
            .focusable(true)
            .role(Role::MenuItem)
            .navigable(false)
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

            WindowEvent::KeyDown(Code::ArrowRight, _) => {
                cx.emit(MenuEvent::FocusNextMenuBarItem);
            }

            _ => {}
        });
    }
}
