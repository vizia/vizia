use femtovg::{Paint, Path};

use crate::{Units, Context, Entity, Handle, Lens, MouseButton, View, WindowEvent, Orientation, LensExt, Canvas};


pub struct Scrollbar<L1, L2> {
    value: L1,
    ratio: L2,
    orientation: Orientation,

    reference_points: Option<(f32, f32)>,

    on_changing: Option<Box<dyn Fn(&mut Context, f32)>>,
}

impl<L1: Lens<Target = f32>, L2: Lens<Target = f32>> Scrollbar<L1, L2> {
    pub fn new<F>(
        cx: &mut Context,
        value: L1,
        ratio: L2,
        orientation: Orientation,
        callback: F
    ) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, f32),
    {
        let result = Self {
            value, ratio,
            orientation,
            reference_points: None,
            on_changing: Some(Box::new(callback)),
        }
            .build2(cx, move |_| {});

        match orientation {
            Orientation::Horizontal => result.class("horizontal"),
            Orientation::Vertical => result.class("vertical"),
        }
    }

    fn container_and_thumb_size(&self, cx: &mut Context) -> (f32, f32) {
        let (size, min_size) = match &self.orientation {
            Orientation::Horizontal => (cx.cache.get_width(cx.current), cx.style.min_width.get(cx.current)),
            Orientation::Vertical => (cx.cache.get_height(cx.current), cx.style.min_height.get(cx.current)),
        };
        let min_size = if let Units::Pixels(p) = min_size.copied().unwrap_or_else(|| Units::Auto) { p } else { 14.0 };

        let thumb_size = size * *self.ratio.get(cx);
        (size, thumb_size.clamp(min_size, f32::MAX))
    }

    fn thumb_bounds(&self, cx: &mut Context) -> (f32, f32, f32, f32) {
        let start = match &self.orientation {
            Orientation::Horizontal => cx.cache.get_posx(cx.current),
            Orientation::Vertical => cx.cache.get_posy(cx.current),
        };
        let (size, thumb_size) = self.container_and_thumb_size(cx);

        let progress = *self.value.get(cx);
        let center = start + size * progress;
        let thumb_start = center - thumb_size * progress;

        let (tx, tw) = match self.orientation {
            Orientation::Horizontal => (thumb_start, thumb_size),
            Orientation::Vertical => (cx.cache.get_posx(cx.current), cx.cache.get_width(cx.current)),
        };
        let (ty, th) = match self.orientation {
            Orientation::Horizontal => (cx.cache.get_posy(cx.current), cx.cache.get_height(cx.current)),
            Orientation::Vertical => (thumb_start, thumb_size),
        };

        (tx, ty, tw, th)
    }

    fn compute_new_value(&self, cx: &mut Context, physical_delta: f32, value_ref: f32) -> f32 {
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

    fn change(&mut self, cx: &mut Context, new_val: f32) {
        if let Some(callback) = self.on_changing.take() {
            callback(cx, new_val.clamp(0.0, 1.0));
            cx.style.needs_redraw = true;
            self.on_changing = Some(callback);
        }
    }
}

impl<L1: 'static + Lens<Target = f32>, L2: 'static + Lens<Target = f32>> View for Scrollbar<L1, L2> {
    fn element(&self) -> Option<String> {
        Some("scrollbar".to_string())
    }

    fn draw(&self, cx: &mut Context, canvas: &mut Canvas) {
        let x = cx.cache.get_posx(cx.current);
        let y = cx.cache.get_posy(cx.current);
        let w = cx.cache.get_width(cx.current);
        let h = cx.cache.get_height(cx.current);
        let (tx, ty, tw, th) = self.thumb_bounds(cx);

        let mut path = Path::new();
        path.rect(x, y, w, h);
        canvas.fill_path(&mut path, Paint::color(cx.style.background_color.get(cx.current).unwrap().clone().into()));

        let mut path = Path::new();
        path.rect(tx, ty, tw, th);
        canvas.fill_path(&mut path, Paint::color(cx.style.font_color.get(cx.current).unwrap().clone().into()));
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            let pos = match &self.orientation {
                Orientation::Horizontal => cx.mouse.cursorx,
                Orientation::Vertical => cx.mouse.cursory,
            };
            match window_event {
                WindowEvent::MouseDown(MouseButton::Left) => {
                    let (tx, ty, tw, th) = self.thumb_bounds(cx);
                    if tx <= cx.mouse.cursorx && cx.mouse.cursorx < tx + tw && ty <= cx.mouse.cursory && cx.mouse.cursory < ty + th {
                        self.reference_points = Some((pos, *self.value.get(cx)));
                        cx.captured = cx.current;
                    } else {
                        let (_, jump) = self.container_and_thumb_size(cx);
                        match &self.orientation {
                            Orientation::Horizontal => {
                                if cx.mouse.cursorx < tx {
                                    self.change(cx, *self.value.get(cx) - jump);
                                } else if cx.mouse.cursorx >= tx + tw {
                                    self.change(cx, *self.value.get(cx) + jump);
                                }
                            }
                            Orientation::Vertical => {
                                if cx.mouse.cursory < ty {
                                    self.change(cx, *self.value.get(cx) - jump);
                                } else if cx.mouse.cursory >= ty + th {
                                    self.change(cx, *self.value.get(cx) + jump);
                                }
                            }
                        }
                    }
                }

                WindowEvent::MouseUp(MouseButton::Left) => {
                    self.reference_points = None;
                    cx.captured = Entity::null();
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
        }
    }
}
