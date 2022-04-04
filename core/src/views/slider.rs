use std::marker::PhantomData;

use crate::{
    Actions, Binding, Context, Data, Element, GeometryChanged, Handle, Lens, LensExt, MouseButton,
    Overflow, PropSet, Units::*, View, WindowEvent, ZStack,
};

#[derive(Debug)]
enum SliderEventInternal {
    SetThumbSize(f32, f32),
}

#[derive(Clone, Debug, Default, Data)]
pub struct SliderDataInternal {
    pub orientation: Orientation,
    pub size: f32,
    pub thumb_size: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Horizontal
    }
}

#[derive(Lens)]
pub struct Slider<L: Lens> {
    p: PhantomData<L>,
    is_dragging: bool,
    internal: SliderDataInternal,
    on_changing: Option<Box<dyn Fn(&mut Context, f32)>>,
}

impl<L> Slider<L>
where
    L: Lens<Target = f32>,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self {
            p: PhantomData::default(),
            is_dragging: false,

            internal: SliderDataInternal {
                orientation: Orientation::Horizontal,
                thumb_size: 0.0,
                size: 0.0,
            },

            on_changing: None,
        }
        .build2(cx, move |cx| {
            Binding::new(cx, Slider::<L>::internal, move |cx, slider_data| {
                let lens = lens.clone();
                ZStack::new(cx, move |cx| {
                    let thumb_size = slider_data.get(cx).thumb_size;
                    let orientation = slider_data.get(cx).orientation;
                    let size = slider_data.get(cx).size;

                    // Active track
                    Element::new(cx).class("active").bind(lens.clone(), move |handle, value| {
                        let val = value.get(handle.cx);
                        let min = thumb_size / size;
                        let max = 1.0;
                        let dx = min + val * (max - min);

                        if orientation == Orientation::Horizontal {
                            handle
                                .height(Stretch(1.0))
                                .left(Pixels(0.0))
                                .right(Stretch(1.0))
                                .width(Percentage(dx * 100.0));
                        } else {
                            handle
                                .width(Stretch(1.0))
                                .top(Stretch(1.0))
                                .bottom(Pixels(0.0))
                                .height(Percentage(dx * 100.0));
                        }
                    });

                    // Thumb
                    Element::new(cx)
                        .class("thumb")
                        .on_geo_changed(|cx, geo| {
                            if geo.contains(GeometryChanged::WIDTH_CHANGED) {
                                cx.emit(SliderEventInternal::SetThumbSize(
                                    cx.cache.get_width(cx.current),
                                    cx.cache.get_height(cx.current),
                                ));
                            }
                        })
                        .bind(lens.clone(), move |handle, value| {
                            let val = value.get(handle.cx);
                            let px = val * (1.0 - (thumb_size / size));
                            if orientation == Orientation::Horizontal {
                                handle
                                    .right(Stretch(1.0))
                                    .top(Stretch(1.0))
                                    .bottom(Stretch(1.0))
                                    .left(Percentage(100.0 * px));
                            } else {
                                handle
                                    .top(Stretch(1.0))
                                    .left(Stretch(1.0))
                                    .right(Stretch(1.0))
                                    .bottom(Percentage(100.0 * px));
                            }
                        });
                })
                .overflow(Overflow::Visible);
            });
        })
        .overflow(Overflow::Visible)
    }
}

impl<L: Lens> View for Slider<L> {
    fn element(&self) -> Option<String> {
        Some("slider".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(slider_event_internal) = event.message.downcast() {
            match slider_event_internal {
                SliderEventInternal::SetThumbSize(width, height) => match self.internal.orientation
                {
                    Orientation::Horizontal => {
                        self.internal.thumb_size = *width;
                    }

                    Orientation::Vertical => {
                        self.internal.thumb_size = *height;
                    }
                },
            }
        }

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::GeometryChanged(_) => {
                    let width = cx.cache.get_width(cx.current);
                    let height = cx.cache.get_height(cx.current);

                    if width >= height {
                        self.internal.orientation = Orientation::Horizontal;
                        self.internal.size = width;
                    } else {
                        self.internal.orientation = Orientation::Vertical;
                        self.internal.size = height;
                    }
                }

                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    self.is_dragging = true;
                    cx.capture();
                    cx.current.set_active(cx, true);

                    let thumb_size = self.internal.thumb_size;

                    let mut dx = match self.internal.orientation {
                        Orientation::Horizontal => {
                            (cx.mouse.left.pos_down.0
                                - cx.cache.get_posx(cx.current)
                                - thumb_size / 2.0)
                                / (cx.cache.get_width(cx.current) - thumb_size)
                        }

                        Orientation::Vertical => {
                            (cx.cache.get_height(cx.current)
                                - (cx.mouse.left.pos_down.1 - cx.cache.get_posy(cx.current))
                                - thumb_size / 2.0)
                                / (cx.cache.get_height(cx.current) - thumb_size)
                        }
                    };

                    dx = dx.clamp(0.0, 1.0);

                    if let Some(callback) = self.on_changing.take() {
                        (callback)(cx, dx);

                        self.on_changing = Some(callback);
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    self.is_dragging = false;
                    cx.release();
                    cx.current.set_active(cx, false);
                }

                WindowEvent::MouseMove(x, y) => {
                    if self.is_dragging {
                        let thumb_size = self.internal.thumb_size;

                        let mut dx = match self.internal.orientation {
                            Orientation::Horizontal => {
                                (*x - cx.cache.get_posx(cx.current) - thumb_size / 2.0)
                                    / (cx.cache.get_width(cx.current) - thumb_size)
                            }

                            Orientation::Vertical => {
                                (cx.cache.get_height(cx.current)
                                    - (*y - cx.cache.get_posy(cx.current))
                                    - thumb_size / 2.0)
                                    / (cx.cache.get_height(cx.current) - thumb_size)
                            }
                        };

                        dx = dx.clamp(0.0, 1.0);

                        if let Some(callback) = self.on_changing.take() {
                            (callback)(cx, dx);

                            self.on_changing = Some(callback);
                        }
                    }
                }

                _ => {}
            }
        }
    }
}

impl<'a, L: Lens> Handle<'a, Slider<L>> {
    /// Set the callback triggered when the slider value is changing (dragging).
    ///
    /// Takes a closure which triggers when the slider value is changing,
    /// either by pressing the track or dragging the thumb along the track.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// Slider::new(cx, 0.0, Orientation::Horizontal)
    ///     .on_changing(|cx, value| {
    ///         cx.emit(WindowEvent::Debug(format!("Slider on_changing: {}", value)));
    ///     });
    /// ```
    pub fn on_changing<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context, f32),
    {
        if let Some(slider) =
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<Slider<L>>())
        {
            slider.on_changing = Some(Box::new(callback));
        }

        self
    }
}