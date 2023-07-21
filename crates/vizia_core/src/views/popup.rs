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
        Self { lens }
            .build(cx, |cx| {
                let parent = cx.current;
                Binding::new(cx, lens, move |cx, lens| {
                    if let Some(geo) = cx.cache.geo_changed.get_mut(parent) {
                        geo.set(GeoChanged::WIDTH_CHANGED, true);
                    }

                    if lens.get(cx) {
                        (content)(cx);
                    }
                });
            })
            .bind(lens, move |handle, val| {
                if val.get(&handle) && capture_focus {
                    handle.lock_focus_to_within();
                }
            })
            .role(Role::Dialog)
            .checked(lens)
            .position_type(PositionType::SelfDirected)
            .z_index(100)
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

impl<L> View for Popup<L>
where
    L: Lens,
    L::Target: Into<bool>,
{
    fn element(&self) -> Option<&'static str> {
        Some("popup")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::GeometryChanged(_) => {
                let bounds = cx.bounds();
                let window_bounds = cx.cache.get_bounds(Entity::root());

                let dist_bottom = window_bounds.bottom() - bounds.bottom();
                let dist_top = bounds.top() - window_bounds.top();

                let scale = cx.scale_factor();

                if dist_bottom < 0.0 {
                    if dist_top.abs() < dist_bottom.abs() {
                        cx.set_translate((Pixels(0.0), Pixels(-dist_top.abs() / scale)));
                    } else {
                        cx.set_translate((Pixels(0.0), Pixels(-dist_bottom.abs() / scale)));
                    }
                } else {
                    cx.set_translate((Pixels(0.0), Pixels(4.0)));
                }
            }

            _ => {}
        });
    }
}
