use crate::style::PropGet;
use crate::views::checkbox::ICON_CHECK;
use crate::{
    Context, Entity, Event, HStack, Handle, Label, Lens, LensExt, MouseButton, PropSet,
    Propagation, Res, TreeExt, View, WindowEvent,
};
use morphorm::{Hierarchy, Units};

pub const ICON_ARROW: &str = "\u{E315}";

/// A button containing a menu when you click/hover it.
pub struct Menu {}

impl Menu {
    /// Construct a new menu. The first closure is the label/stack/etc that will be displayed
    /// while the menu is closed, and the second closure will be passed to a vertical MenuStack
    /// to be constructed and then displayed when the menu is opened
    pub fn new<F1, F2, Lbl, List>(cx: &mut Context, label: F1, items: F2) -> Handle<'_, Self>
    where
        F1: 'static + FnOnce(&mut Context) -> Handle<'_, Lbl>,
        F2: 'static + FnOnce(&mut Context) -> List,
    {
        Self {}.build2(cx, move |cx| {
            HStack::new(cx, move |cx| {
                label(cx);
                Label::new(cx, ICON_ARROW).class("menu_arrow");
            });
            MenuStack::new_vertical(cx, move |cx| {
                items(cx);
            });
        })
    }
}

impl View for Menu {
    fn element(&self) -> Option<String> {
        Some("menu".to_owned())
    }
}

pub struct MenuStack {}

/// A MenuStack is a container object which holds menu items. It controls the highlighting of its
/// children. It has special behavior when it is nested inside another MenuStack (i.e. a submenu).
impl MenuStack {
    fn new<F>(cx: &mut Context, builder: F) -> Handle<'_, Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        Self {}.build2(cx, move |cx| {
            builder(cx);
        })
    }

    /// Build a MenuStack laid out horizontally, i.e. a menu bar.
    pub fn new_horizontal<F>(cx: &mut Context, builder: F) -> Handle<'_, Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        Self::new(cx, builder).class("horizontal")
    }

    /// Build a MenuStack laid out vertically, i.e. a menu list.
    pub fn new_vertical<F>(cx: &mut Context, builder: F) -> Handle<'_, Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        Self::new(cx, builder).class("vertical")
    }

    // uncheck all menu children, effectively clearing their highlighting and closing them
    fn uncheck_all(&self, cx: &mut Context) {
        let children = cx.current.branch_iter(&cx.tree).collect::<Vec<_>>();
        for child in children {
            if let Some(view) = cx.views.get(&child) {
                if view.element().map_or(false, |s| s.starts_with("menu")) {
                    child.set_checked(cx, false);
                }
            }
        }
    }

    fn is_root(&self, cx: &mut Context) -> bool {
        let parents = cx.current.parent_iter(&cx.tree).collect::<Vec<_>>();
        for parent in parents {
            if let Some(view) = cx.views.get(&parent) {
                if view.element().map_or(false, |s| s == "menustack") {
                    return false;
                }
            }
        }

        true
    }
}

impl View for MenuStack {
    fn element(&self) -> Option<String> {
        Some("menustack".to_owned())
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(msg) = event.message.downcast() {
            match msg {
                WindowEvent::MouseMove(_, _) => {
                    // we capture focus in order to see clicks outside the menus, but we don't want
                    // that behavior on hover. redirect direct-propagated events to their actual
                    // target.
                    if event.propagation == Propagation::Direct {
                        cx.event_queue.push_back(
                            Event::new(msg.clone())
                                .propagate(Propagation::Up)
                                .target(cx.hovered)
                                .origin(event.origin.clone()),
                        );
                        return;
                    }
                    // Don't let any parent menustack see this
                    event.consume();

                    // find the target of the event which is a direct child of us, or return
                    let mut h = cx.hovered;
                    while cx.tree.parent(h) != Some(cx.current) {
                        if let Some(parent) = h.get_parent(cx) {
                            h = parent;
                        } else {
                            return;
                        }
                    }

                    // we only respond to hover events if we're inside a focused menustack
                    // search ancestry for one of those
                    let mut c = cx.current;
                    loop {
                        if let Some(view) = cx.views.get(&c) {
                            if view.element().map_or(false, |x| x == "menustack")
                                && c.is_focused(cx)
                            {
                                break;
                            }
                        } else if c == cx.current {
                            if c.is_focused(cx) {
                                break;
                            }
                        }
                        if let Some(parent) = c.get_parent(cx) {
                            c = parent;
                        } else {
                            return;
                        }
                    }

                    // uncheck all children, in anticipation of checking one of them
                    self.uncheck_all(cx);

                    // if the child of ours the event was directed at is a menu, check it
                    if let Some(view) = cx.views.get(&h) {
                        if view.element().map_or(false, |s| s.starts_with("menu")) {
                            h.set_checked(cx, true);
                        }
                    }
                }
                WindowEvent::MouseDown(MouseButton::Left) => {
                    if cx.captured == cx.current {
                        let current = cx.current;
                        // propagate click events ONLY to descendents
                        if cx.hovered.parent_iter(&cx.tree).any(|c| c == current)
                            && event.propagation == Propagation::Direct
                        {
                            cx.event_queue.push_back(
                                Event::new(msg.clone())
                                    .propagate(Propagation::Up)
                                    .target(cx.hovered)
                                    .origin(event.origin.clone()),
                            );
                        } else {
                            // if we clicked outside the menu, or we clicked inside the menu and
                            // nobody caught the upward-propagating event, close the menu
                            cx.current.set_focus(cx, false);
                            cx.captured = Entity::null();
                            self.uncheck_all(cx);
                        }
                    } else if self.is_root(cx) {
                        // capture focus on click
                        cx.current.set_focus(cx, true);
                        cx.captured = cx.current;
                        cx.emit_to(cx.current, WindowEvent::MouseMove(0.0, 0.0));
                    }
                }
                _ => {}
            }
        }
    }
}

/// A MenuButton is an entry in a menu that can be clicked to perform some action. It has various
/// constructors depending on whether you want to make this button show a check icon conditionally.
pub struct MenuButton {
    action: Option<Box<dyn Fn(&mut Context)>>,
}

impl MenuButton {
    pub fn new<F, A>(cx: &mut Context, contents: F, action: A) -> Handle<'_, Self>
    where
        F: 'static + FnOnce(&mut Context),
        A: 'static + Fn(&mut Context),
    {
        Self { action: Some(Box::new(action)) }.build2(cx, move |cx| {
            contents(cx);
        })
    }

    pub fn new_simple<U: ToString, A>(
        cx: &mut Context,
        text: impl 'static + Res<U>,
        action: A,
    ) -> Handle<'_, Self>
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

    pub fn new_check<F, A, L>(cx: &mut Context, builder: F, action: A, lens: L) -> Handle<'_, Self>
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
    ) -> Handle<'_, Self>
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
                self.action = Some(callback);
            }
        }
    }
}
