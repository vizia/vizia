use std::cell::RefCell;

use crate::views::checkbox::ICON_CHECK;
use crate::{Units, Context, Entity, Event, HStack, Handle, Label, Lens, LensExt, MouseButton, PropSet, Propagation, Res, TreeExt, View, WindowEvent, VStack, Model, Actions, Popup, PopupData, PopupEvent, Move, Canvas};

pub const ICON_ARROW: &str = "\u{E315}";

pub fn setup_menu_entry<T, F1, F2>(handle: Handle<'_, T>, on_select: F1, on_deselect: F2) -> Handle<'_, Move<T>>
where
    T: View,
    F1: 'static + Fn(&mut Context),
    F2: 'static + Fn(&mut Context),
{
    if let Some(data) = handle.cx.data::<MenuData>() {
        let i = *data.counter.borrow();
        *data.counter.borrow_mut() += 1;
        handle.bind(MenuData::selected, move |handle, selected| {
            let selected = *selected.get(handle.cx) == Some(i);
            handle.entity.set_selected(handle.cx, selected);
            if selected {
                on_select(handle.cx);
            } else {
                on_deselect(handle.cx);
            }
        }).on_move(move |cx, _, _| {
            if cx.data::<MenuControllerData>().unwrap().active {
                cx.emit(MenuEvent::SetSelected(Some(i)));
            }
        })
    } else {
        panic!("Using a menu entry outside of a menu")
    }
}

#[derive(Lens)]
pub struct MenuData {
    pub selected: Option<usize>,
    pub counter: RefCell<usize>,
}

pub struct MenuControllerData {
    pub active: bool,
}

pub enum MenuEvent {
    SetSelected(Option<usize>),
    Close,
    Activate,
}

impl Model for MenuData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        if let Some(msg) = event.message.downcast() {
            match msg {
                MenuEvent::SetSelected(sel) => {
                    self.selected = *sel;
                    event.consume();
                },
                MenuEvent::Close => self.selected = None,
                MenuEvent::Activate => {}
            }
        }
    }
}

impl Model for MenuControllerData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(msg) = event.message.downcast() {
            match msg {
                MenuEvent::Close => {
                    self.active = false;
                    cx.captured = Entity::null();
                },
                MenuEvent::Activate => self.active = true,
                _ => {}
            }
        }
    }
}

pub struct MenuController {}

/// A MenuController is a container object which holds a menu. It is responsible for managing
/// the focus of the menu, i.e. grabbing click events until the menu is closed.
impl MenuController {
    pub fn new<F: FnOnce(&mut Context)>(cx: &mut Context, active: bool, builder: F) -> Handle<'_, Self> {
        if cx.data::<MenuControllerData>().is_some() {
            panic!("Building a MenuController inside a MenuController. This is illegal.")
        }

        Self {}.build2(cx, move |cx| {
            MenuControllerData { active }.build(cx);
            if active {
                cx.captured = cx.current;
            }
            builder(cx);
        })
    }
}

impl View for MenuController {
    fn element(&self) -> Option<String> {
        Some("menucontroller".to_owned())
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        let active = cx.data::<MenuControllerData>().unwrap().active;

        if let Some(msg) = event.message.downcast() {
            if active {
                let is_child = cx.hovered.is_descendant_of(&cx.tree, cx.current);
                // we capture focus in order to see clicks outside the menus, but we don't want
                // to deprive our children of their events.
                // We also want mouse scroll events to be seen as normal
                if event.propagation == Propagation::Direct {
                    if (is_child && matches!(msg, WindowEvent::MouseMove(_, _) | WindowEvent::MouseDown(_) | WindowEvent::MouseUp(_) | WindowEvent::MouseScroll(_, _) | WindowEvent::MouseDoubleClick(_))) ||
                        (!is_child && matches!(msg, WindowEvent::MouseScroll(_, _))) {
                        cx.event_queue.push_back(
                            Event::new(msg.clone())
                                .propagate(Propagation::Up)
                                .target(cx.hovered)
                                .origin(event.origin.clone()),
                        );
                    }
                    // if we click outside the menu, close everything
                    if matches!(msg, WindowEvent::MouseDown(_)) && !is_child {
                        cx.event_queue.push_back(
                            Event::new(MenuEvent::Close)
                                .propagate(Propagation::Subtree)
                                .target(cx.current)
                                .origin(cx.current)
                        );
                    }
                }
            } else {
                if let WindowEvent::MouseDown(_) = msg {
                    // capture focus on click
                    cx.captured = cx.current;
                    cx.emit(MenuEvent::Activate);
                    // send a hover event to highlight whatever we're hovered on
                    cx.event_queue.push_back(
                        Event::new(WindowEvent::MouseMove(0.0, 0.0))
                            .propagate(Propagation::Up)
                            .target(cx.hovered)
                            .origin(cx.current)
                    );
                }
            }
        }
    }
}

pub struct MenuStack {}

impl MenuStack {
    fn new<F: FnOnce(&mut Context)>(cx: &mut Context, builder: F) -> Handle<'_, Self> {
        MenuData { selected: None, counter: RefCell::new(0) }.build(cx);
        Self {}.build2(cx, builder)
    }

    pub fn new_vertical<F: FnOnce(&mut Context)>(cx: &mut Context, builder: F) -> Handle<'_, Self> {
        Self::new(cx, builder).class("vertical")
    }

    pub fn new_horizontal<F: FnOnce(&mut Context)>(cx: &mut Context, builder: F) -> Handle<'_, Self> {
        Self::new(cx, builder).class("horizontal")
    }
}

impl View for MenuStack {
    fn element(&self) -> Option<String> {
        Some("menustack".to_owned())
    }
}

/// A button containing a menu when you click/hover it.
pub struct Menu {}

impl Menu {
    /// Construct a new menu. The first closure is the label/stack/etc that will be displayed
    /// while the menu is closed, and the second closure will be passed to a vertical MenuStack
    /// to be constructed and then displayed when the menu is opened
    pub fn new<F1, F2, Lbl>(cx: &mut Context, label: F1, items: F2) -> Handle<'_, Move<Self>>
        where
            F1: 'static + FnOnce(&mut Context) -> Handle<'_, Lbl>,
            F2: 'static + FnOnce(&mut Context),
    {
        setup_menu_entry(Self {}.build2(cx, move |cx| {
            HStack::new(cx, move |cx| {
                label(cx);
                Label::new(cx, ICON_ARROW).class("menu_arrow");
            });
            MenuStack::new_vertical(cx, items);
        }), |_| {}, |_| {})
    }
}

impl View for Menu {
    fn element(&self) -> Option<String> {
        Some("menu".to_owned())
    }
}


/// A MenuButton is an entry in a menu that can be clicked to perform some action. It has various
/// constructors depending on whether you want to make this button show a check icon conditionally.
pub struct MenuButton {
    action: Option<Box<dyn Fn(&mut Context)>>,
}

impl MenuButton {
    pub fn new<F, A>(cx: &mut Context, contents: F, action: A) -> Handle<'_, Move<Self>>
    where
        F: 'static + FnOnce(&mut Context),
        A: 'static + Fn(&mut Context),
    {
        setup_menu_entry(Self { action: Some(Box::new(action)) }.build2(cx, move |cx| {
            contents(cx);
        }), |_| {}, |_| {})
    }

    pub fn new_simple<U: ToString, A>(
        cx: &mut Context,
        text: impl 'static + Res<U>,
        action: A,
    ) -> Handle<'_, Move<Self>>
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

    pub fn new_check<F, A, L>(cx: &mut Context, builder: F, action: A, lens: L) -> Handle<'_, Move<Self>>
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
                        handle.text(if val.as_deref() == Some(&true) { ICON_CHECK } else { "" });
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
    ) -> Handle<'_, Move<Self>>
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
    fn element(&self) -> Option<String> {
        Some("menubutton".to_owned())
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(WindowEvent::MouseDown(MouseButton::Left)) = event.message.downcast() {
            if let Some(callback) = self.action.take() {
                callback(cx);
                //cx.emit(MenuEvent::Close);
                self.action = Some(callback);
            }
        }
    }
}
