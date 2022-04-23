use morphorm::PositionType;

use crate::{style::PropGet, Code, Context, Data, Handle, Lens, LensExt, Model, View, WindowEvent};

#[derive(Debug, Default, Data, Lens, Clone)]
pub struct PopupData {
    pub is_open: bool,
}

impl Model for PopupData {
    fn event(&mut self, _: &mut Context, event: &mut crate::Event) {
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
    pub fn new<F>(cx: &mut Context, lens: L, content: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        Self { lens: lens.clone() }
            .build(cx, |cx| {
                (content)(cx);
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
            event.map(|window_event, meta| match window_event {
                WindowEvent::MouseDown(_) => {
                    if flag {
                        if meta.origin != cx.current {
                            if !cx.current.is_over(cx) {
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
        Some(String::from("popup"))
    }
}
