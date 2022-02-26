use morphorm::PositionType;

use crate::{style::PropGet, Code, Context, Data, Handle, Lens, LensExt, Model, View, WindowEvent};

#[derive(Debug, Default, Data, Lens, Clone)]
pub struct PopupData {
    is_open: bool,
}

impl Model for PopupData {
    fn event(&mut self, _: &mut Context, event: &mut crate::Event) {
        if let Some(popup_event) = event.message.downcast() {
            match popup_event {
                PopupEvent::Open => {
                    self.is_open = true;
                    event.consume();
                }

                PopupEvent::Close => {
                    self.is_open = false;
                    event.consume();
                }

                PopupEvent::Switch => {
                    self.is_open ^= true;
                    event.consume();
                }
            }
        }
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
    pub fn new<F>(cx: &mut Context, lens: L, builder: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        Self { lens: lens.clone() }
            .build2(cx, |cx| {
                (builder)(cx);
            })
            .checked(lens)
            .position_type(PositionType::SelfDirected)
            .z_order(100)
    }
}

impl<'a, L> Handle<'a, Popup<L>>
where
    L: Lens,
    L::Target: Clone + Into<bool>,
{
    pub fn something<F>(self, f: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        let focus_event = Box::new(f);
        let prev = self.cx.current;
        self.cx.current = self.entity;
        self.cx.add_listener(move |popup: &mut Popup<L>, cx, event| {
            let flag: bool = popup.lens.get(cx).clone().into();
            if let Some(window_event) = event.message.downcast() {
                match window_event {
                    WindowEvent::MouseDown(_) => {
                        if flag {
                            if event.origin != cx.current {
                                if !cx.current.is_over(cx) {
                                    (focus_event)(cx);
                                    event.consume();
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
                }
            }
        });
        self.cx.current = prev;

        self
    }
}

impl<L> View for Popup<L>
where
    L: Lens,
    L::Target: Into<bool>,
{
    fn element(&self) -> Option<String> {
        Some("popup".to_string())
    }
}
