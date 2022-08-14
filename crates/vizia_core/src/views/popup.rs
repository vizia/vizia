use morphorm::PositionType;

use crate::prelude::*;

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

pub struct Popup<L> {
    lens: L,
}

impl<L> Popup<L>
where
    L: Lens<Target = bool>,
{
    pub fn new<F>(cx: &mut Context, lens: L, capture_focus: bool, content: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        let mut handle = Self { lens: lens.clone() }
            .build(cx, |cx| {
                (content)(cx);
            })
            .checked(lens.clone())
            .position_type(PositionType::SelfDirected)
            .z_order(100);

        if capture_focus {
            use std::cell::Cell;

            let old_focus = Cell::new(Entity::root());
            let old_lock = Cell::new(Entity::root());
            handle = handle.bind(lens, move |handle, enabled| {
                if enabled.get(handle.cx) {
                    old_focus.set(handle.cx.focused);
                    old_lock.set(handle.cx.lock_focus_to);
                    handle.cx.lock_focus_to(handle.entity);
                } else {
                    handle.cx.lock_focus_to(old_lock.get());
                    handle.cx.with_current(old_focus.get(), |cx| cx.focus());
                }
            });
        }

        handle
    }
}

impl<'a, L> Handle<'a, Popup<L>>
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
            cx.add_listener(move |popup: &mut Popup<L>, cx, event| {
                let flag: bool = popup.lens.get(cx).clone().into();
                event.map(|window_event, meta| match window_event {
                    WindowEvent::MouseDown(_) => {
                        if flag {
                            if meta.origin != cx.current() {
                                if !cx.is_over() {
                                    (focus_event)(cx);
                                    meta.consume();
                                }
                            }
                        }
                    }

                    WindowEvent::KeyDown(code, _) => {
                        if flag {
                            if *code == Code::Escape {
                                (focus_event)(cx);
                            }
                        }
                    }

                    _ => {}
                });
            });
        });

        self
    }
}

impl<L> View for Popup<L>
where
    L: Lens,
    L::Target: Into<bool>,
{
    fn element(&self) -> Option<&'static str> {
        Some("popup")
    }
}