use crate::prelude::*;
use crate::views::Orientation;

pub struct Scrollbar<L1> {
    value: L1,
    orientation: Orientation,

    reference_points: Option<(f32, f32)>,

    on_changing: Option<Box<dyn Fn(&mut EventContext, f32)>>,
}

impl<L1: Lens<Target = f32>> Scrollbar<L1> {
    pub fn new<F, L2: Lens<Target = f32>>(
        cx: &mut Context,
        value: L1,
        ratio: L2,
        orientation: Orientation,
        callback: F,
    ) -> Handle<Self>
    where
        F: 'static + Fn(&mut EventContext, f32),
    {
        Self {
            value: value.clone(),
            orientation,
            reference_points: None,
            on_changing: Some(Box::new(callback)),
        }
        .build(cx, move |cx| {
            Element::new(cx)
                .class("thumb")
                .focusable(true)
                .bind(value, move |handle, value| {
                    let value = value.get(handle.cx);
                    match orientation {
                        Orientation::Horizontal => {
                            handle.left(Units::Stretch(value)).right(Units::Stretch(1.0 - value))
                        }
                        Orientation::Vertical => {
                            handle.top(Units::Stretch(value)).bottom(Units::Stretch(1.0 - value))
                        }
                    };
                })
                .bind(ratio, move |handle, ratio| {
                    let ratio = ratio.get(handle.cx);
                    match orientation {
                        Orientation::Horizontal => handle.width(Units::Percentage(ratio * 100.0)),
                        Orientation::Vertical => handle.height(Units::Percentage(ratio * 100.0)),
                    };
                });
        })
        .class(match orientation {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        })
    }

    fn container_and_thumb_size(&self, cx: &mut EventContext) -> (f32, f32) {
        let current = cx.current();
        let child = cx.tree.get_child(current, 0).unwrap();
        match &self.orientation {
            Orientation::Horizontal => (cx.cache.get_width(current), cx.cache.get_width(child)),
            Orientation::Vertical => (cx.cache.get_height(current), cx.cache.get_height(child)),
        }
    }

    fn thumb_bounds(&self, cx: &mut EventContext) -> (f32, f32, f32, f32) {
        let current = cx.current();
        let child = cx.tree.get_child(current, 0).unwrap();
        (
            cx.cache.get_posx(child),
            cx.cache.get_posy(child),
            cx.cache.get_width(child),
            cx.cache.get_height(child),
        )
    }

    fn compute_new_value(&self, cx: &mut EventContext, physical_delta: f32, value_ref: f32) -> f32 {
        // delta is moving within the negative space of the thumb: (1 - ratio) * container
        let (size, thumb_size) = self.container_and_thumb_size(cx);
        let negative_space = size - thumb_size;
        if negative_space == 0.0 {
            value_ref
        } else {
            // what percentage of negative space have we crossed?
            let logical_delta = physical_delta / negative_space;
            value_ref + logical_delta
        }
    }

    fn change(&mut self, cx: &mut EventContext, new_val: f32) {
        if let Some(callback) = &self.on_changing {
            callback(cx, new_val.clamp(0.0, 1.0));
        }
    }
}

impl<L1: 'static + Lens<Target = f32>> View for Scrollbar<L1> {
    fn element(&self) -> Option<&'static str> {
        Some("scrollbar")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| {
            let pos = match &self.orientation {
                Orientation::Horizontal => cx.mouse.cursorx,
                Orientation::Vertical => cx.mouse.cursory,
            };
            match window_event {
                WindowEvent::MouseDown(MouseButton::Left) => {
                    if meta.target != cx.current() {
                        self.reference_points = Some((pos, self.value.get(cx)));
                        cx.capture();
                    } else {
                        let (_, jump) = self.container_and_thumb_size(cx);
                        let (tx, ty, tw, th) = self.thumb_bounds(cx);
                        let physical_delta = match &self.orientation {
                            Orientation::Horizontal => {
                                if cx.mouse.cursorx < tx {
                                    -jump
                                } else if cx.mouse.cursorx >= tx + tw {
                                    jump
                                } else {
                                    return;
                                }
                            }
                            Orientation::Vertical => {
                                if cx.mouse.cursory < ty {
                                    -jump
                                } else if cx.mouse.cursory >= ty + th {
                                    jump
                                } else {
                                    return;
                                }
                            }
                        };
                        let changed =
                            self.compute_new_value(cx, physical_delta, self.value.get(cx));
                        self.change(cx, changed);
                    }
                }

                WindowEvent::MouseUp(MouseButton::Left) => {
                    self.reference_points = None;
                    cx.focus_with_visibility(false);
                    cx.release();
                }

                WindowEvent::MouseMove(_, _) => {
                    if let Some((mouse_ref, value_ref)) = self.reference_points {
                        let physical_delta = pos - mouse_ref;
                        let changed = self.compute_new_value(cx, physical_delta, value_ref);
                        self.change(cx, changed);
                    }
                }

                _ => {}
            }
        });
    }
}
