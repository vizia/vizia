use std::cell::RefCell;

use crate::fonts::{material_names::RIGHT, unicode_names::CHECK};
use crate::modifiers::Over;
use crate::prelude::*;
use crate::tree::TreeExt;

/// A helper function which sets up the necessary attributes on a view to be a menu entry.
/// Call this with a handle to an object you would like to be considered a menu entry.
/// It adds an on_over event handler updating the current selected menu entry and binds to
/// said selection, updating the `selected` pseudo-class accordingly and calling the `on_select`
/// and `on_deselect` callbacks appropriately.
pub fn setup_menu_entry<T, F1, F2>(
    handle: Handle<'_, T>,
    on_select: F1,
    on_deselect: F2,
) -> Handle<'_, Over<T>>
where
    T: View,
    F1: 'static + Fn(&mut Context),
    F2: 'static + Fn(&mut Context),
{
    if let Some(data) = handle.cx.data::<MenuData>() {
        let i = *data.counter.borrow();
        *data.counter.borrow_mut() += 1;
        handle
            .bind(MenuData::selected, move |handle, selected| {
                let selected = selected.get(handle.cx) == Some(i);
                handle.cx.set_selected(selected);
                if selected {
                    on_select(handle.cx);
                } else {
                    on_deselect(handle.cx);
                }
            })
            .on_over(move |cx| {
                if cx.data::<MenuControllerData>().unwrap().active {
                    cx.emit(MenuEvent::SetSelected(Some(i)));
                }
            })
    } else {
        panic!("Using a menu entry outside of a menu")
    }
}

/// The data storage for the current selected index of a menu
/// This is automatically created when you construct a MenuStack.
#[derive(Lens, Default)]
struct MenuData {
    selected: Option<usize>,
    counter: RefCell<usize>,
}

struct MenuControllerData {
    active: bool,
}

/// Menu control events.
pub enum MenuEvent {
    SetSelected(Option<usize>),
    Close,
    Activate,
}

impl Model for MenuData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        event.map(|menu_event, meta| match menu_event {
            MenuEvent::SetSelected(sel) => {
                self.selected = *sel;
                meta.consume();
            }
            MenuEvent::Close => self.selected = None,
            MenuEvent::Activate => {}
        });
    }
}

impl Model for MenuControllerData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|menu_event, _| match menu_event {
            MenuEvent::Close => {
                self.active = false;
                cx.release();
            }
            MenuEvent::Activate => self.active = true,
            _ => {}
        });
    }
}

/// A MenuController is a container object which holds a menu. It is responsible for managing
/// the focus of the menu, i.e. grabbing click events until the menu is closed.
pub struct MenuController {}

impl MenuController {
    pub fn new<F: FnOnce(&mut Context)>(
        cx: &mut Context,
        active: bool,
        builder: F,
    ) -> Handle<'_, Self> {
        if cx.data::<MenuControllerData>().is_some() {
            panic!("Building a MenuController inside a MenuController. This is illegal.")
        }

        Self {}.build(cx, move |cx| {
            MenuControllerData { active }.build(cx);
            if active {
                cx.capture();
            }
            builder(cx);
        })
    }
}

impl View for MenuController {
    fn element(&self) -> Option<&'static str> {
        Some("menucontroller")
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        let active = cx.data::<MenuControllerData>().unwrap().active;

        event.map(|window_event, meta| {
            if active {
                let current = cx.current();
                let is_child = cx.hovered().is_descendant_of(cx.tree(), current);
                // we capture focus in order to see clicks outside the menus, but we don't want
                // to deprive our children of their events.
                // We also want mouse scroll events to be seen by everyone
                if meta.propagation == Propagation::Direct {
                    if (is_child
                        && matches!(
                            window_event,
                            WindowEvent::MouseMove(_, _)
                                | WindowEvent::MouseDown(_)
                                | WindowEvent::MouseUp(_)
                                | WindowEvent::MouseScroll(_, _)
                                | WindowEvent::MouseDoubleClick(_)
                        ))
                        || (!is_child && matches!(window_event, WindowEvent::MouseScroll(_, _)))
                    {
                        cx.event_queue.push_back(
                            Event::new(window_event.clone())
                                .propagate(Propagation::Up)
                                .target(cx.hovered())
                                .origin(meta.origin),
                        );
                    }
                    // if we click outside the menu, close everything
                    if matches!(window_event, WindowEvent::MouseDown(_)) && !is_child {
                        cx.event_queue.push_back(
                            Event::new(MenuEvent::Close)
                                .propagate(Propagation::Subtree)
                                .target(cx.current())
                                .origin(cx.current()),
                        );
                    }
                }
            } else if let WindowEvent::MouseDown(_) = window_event {
                // capture focus on click
                cx.capture();
                cx.emit(MenuEvent::Activate);
                // send an over event to highlight whatever we're hovered on
                cx.event_queue.push_back(
                    Event::new(WindowEvent::MouseOver)
                        .propagate(Propagation::Up)
                        .target(cx.hovered())
                        .origin(cx.current()),
                );
            }
        });
    }
}

/// A MenuStack is a stack of views which can be menu entries. The only interesting thing about it
/// is that it builds a MenuData into itself.
pub struct MenuStack {}

impl MenuStack {
    fn new<F: FnOnce(&mut Context)>(cx: &mut Context, builder: F) -> Handle<'_, Self> {
        if cx.data::<MenuControllerData>().is_none() {
            panic!("MenuStacks must be built inside a MenuController");
        }
        Self {}.build(cx, move |cx| {
            MenuData::default().build(cx);
            builder(cx);
        })
    }

    pub fn new_vertical<F: FnOnce(&mut Context)>(cx: &mut Context, builder: F) -> Handle<'_, Self> {
        Self::new(cx, builder).class("vertical")
    }

    pub fn new_horizontal<F: FnOnce(&mut Context)>(
        cx: &mut Context,
        builder: F,
    ) -> Handle<'_, Self> {
        Self::new(cx, builder).class("horizontal")
    }
}

impl View for MenuStack {
    fn element(&self) -> Option<&'static str> {
        Some("menustack")
    }
}

/// A button containing a menu when you click/hover it.
pub struct Menu {}

impl Menu {
    /// Construct a new menu. The first closure is the label/stack/etc that will be displayed
    /// while the menu is closed, and the second closure will be passed to a vertical MenuStack
    /// to be constructed and then displayed when the menu is opened
    pub fn new<F1, F2, Lbl>(cx: &mut Context, label: F1, items: F2) -> Handle<'_, Over<Self>>
    where
        F1: 'static + FnOnce(&mut Context) -> Handle<'_, Lbl>,
        F2: 'static + FnOnce(&mut Context),
    {
        let result = Self {}.build(cx, move |cx| {
            HStack::new(cx, move |cx| {
                label(cx);
                Label::new(cx, RIGHT).class("menu_arrow");
            });
            MenuStack::new_vertical(cx, items);
        });
        let entity = result.entity;
        setup_menu_entry(
            result,
            move |_| {},
            move |cx| {
                cx.event_queue.push_back(
                    Event::new(MenuEvent::Close)
                        .target(entity)
                        .propagate(Propagation::Subtree)
                        .origin(cx.current()),
                );
            },
        )
    }
}

impl View for Menu {
    fn element(&self) -> Option<&'static str> {
        Some("menu")
    }
}

/// A MenuButton is an entry in a menu that can be clicked to perform some action. It has various
/// constructors depending on whether you want to make this button show a check icon conditionally.
pub struct MenuButton {
    action: Option<Box<dyn Fn(&mut Context)>>,
}

impl MenuButton {
    pub fn new<F, A>(cx: &mut Context, contents: F, action: A) -> Handle<'_, Over<Self>>
    where
        F: 'static + FnOnce(&mut Context),
        A: 'static + Fn(&mut Context),
    {
        setup_menu_entry(
            Self { action: Some(Box::new(action)) }.build(cx, move |cx| {
                contents(cx);
            }),
            |_| {},
            |_| {},
        )
    }

    pub fn new_simple<U: ToString, A>(
        cx: &mut Context,
        text: impl 'static + Res<U>,
        action: A,
    ) -> Handle<'_, Over<Self>>
    where
        A: 'static + Fn(&mut Context),
    {
        Self::new(
            cx,
            move |cx| {
                Label::new(cx, text);
            },
            action,
        )
    }

    pub fn new_check<F, A, L>(
        cx: &mut Context,
        builder: F,
        action: A,
        lens: L,
    ) -> Handle<'_, Over<Self>>
    where
        F: 'static + FnOnce(&mut Context),
        A: 'static + Fn(&mut Context),
        L: Lens<Target = bool>,
    {
        Self::new(
            cx,
            move |cx| {
                HStack::new(cx, move |cx| {
                    builder(cx);
                    Label::new(cx, "").left(Units::Stretch(1.0)).bind(lens, move |handle, lens| {
                        let val = lens.get_fallible(handle.cx);
                        handle.text(if val == Some(true) { CHECK } else { "" });
                    });
                });
            },
            action,
        )
    }

    pub fn new_check_simple<U: ToString, A, L>(
        cx: &mut Context,
        text: impl 'static + Res<U>,
        action: A,
        lens: L,
    ) -> Handle<'_, Over<Self>>
    where
        A: 'static + Fn(&mut Context),
        L: 'static + Lens<Target = bool>,
    {
        Self::new_check(
            cx,
            move |cx| {
                Label::new(cx, text);
            },
            action,
            lens,
        )
    }
}

impl View for MenuButton {
    fn element(&self) -> Option<&'static str> {
        Some("menubutton")
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|window_event, meta| {
            if let WindowEvent::MouseDown(MouseButton::Left) = window_event {
                if let Some(callback) = &self.action {
                    callback(cx);
                    cx.emit(MenuEvent::Close);
                    meta.consume();
                }
            }
        });
    }
}
