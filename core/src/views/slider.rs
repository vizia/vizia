use std::marker::PhantomData;

use morphorm::GeometryChanged;

use crate::{
    Actions, Binding, Context, Data, Element, HStack, Handle, Lens, LensExt, Model, MouseButton,
    Overflow, PropSet, Units::*, View, WindowEvent, ZStack,
};

#[derive(Debug)]
pub enum SliderEventInternal {
    SetThumbSize(f32, f32),
}

#[derive(Clone, Debug, Default, Lens, Data)]
pub struct SliderDataInternal {
    pub orientation: Orientation,
    pub size: f32,
    pub thumb_size: f32,
}

impl Model for SliderDataInternal {
    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::GeometryChanged(geo) => match self.orientation {
                    Orientation::Horizontal => {
                        if geo.contains(GeometryChanged::WIDTH_CHANGED) {
                            self.size = cx.cache.get_width(cx.current);
                        }
                    }

                    Orientation::Vertical => {
                        if geo.contains(GeometryChanged::HEIGHT_CHANGED) {
                            self.size = cx.cache.get_height(cx.current);
                        }
                    }
                },

                _ => {}
            }
        }

        if let Some(slider_event_internal) = event.message.downcast() {
            match slider_event_internal {
                SliderEventInternal::SetThumbSize(width, height) => match self.orientation {
                    Orientation::Horizontal => {
                        self.thumb_size = *width;
                    }

                    Orientation::Vertical => {
                        self.thumb_size = *height;
                    }
                },
            }
        }
    }
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

pub struct Slider<L> {
    is_dragging: bool,
    p: PhantomData<L>,

    // Event sent when the slider value has changed
    on_change: Option<Box<dyn Fn(&mut Context, f32)>>,
    // event sent when the slider value is changing
    on_changing: Option<Box<dyn Fn(&mut Context, f32)>>,
    // Event sent when the slider reaches the minimum value
    on_min: Option<Box<dyn Fn(&mut Context)>>,
    // Event sent when the slider reaches the maximum value
    on_max: Option<Box<dyn Fn(&mut Context)>>,
}

impl<L> Slider<L>
where
    L: Lens<Target = f32>,
{
    pub fn new(cx: &mut Context, lens: L, orientation: Orientation) -> Handle<Self> {
        Self {
            p: PhantomData::default(),
            is_dragging: false,
            on_change: None,
            on_changing: None,
            on_min: None,
            on_max: None,
        }
        .build2(cx, move |cx| {
            // Only create this if it doesn't already exist otherwise it resets the thumb_width
            // This causes a very subtle bug:
            //      When the slider is updated the style data doesn't change, which means the size doesn't change
            //      and after layout the GeometryChanged event is never sent.
            //      If this internal data is recreated with thumb_width == 0.0, then the calculation for thumb position
            //      becomes NaN and the thumb size is never updated due to the lack of GeometryChanged event.
            //      The solution is to only create this if it doesn't already exist. This wouldn't be a problem if
            //      it were possible to bind directly to style properties.
            if cx.data::<SliderDataInternal>().is_none() {
                // Create some internal slider data (not exposed to the user)
                SliderDataInternal { size: 0.0, thumb_size: 0.0, orientation }.build(cx);
            }

            // Add the various slider components using bindings to the slider data
            Binding::new(cx, SliderDataInternal::root, move |cx, slider_data_internal| {
                let lens = lens.clone();
                ZStack::new(cx, move |cx| {
                    let thumb_size = slider_data_internal.get(cx).thumb_size;
                    let orientation = slider_data_internal.get(cx).orientation;
                    let size = slider_data_internal.get(cx).size;

                    match orientation {
                        Orientation::Horizontal => {
                            //(Percentage(dx * 100.0), Stretch(1.0))
                            Element::new(cx)
                                .height(Stretch(1.0))
                                .left(Pixels(0.0))
                                .right(Stretch(1.0))
                                .class("active")
                                .bind(lens.clone(), move |handle, value| {
                                    let val = *value.get(handle.cx);
                                    let min = thumb_size / size;
                                    let max = 1.0;
                                    let dx = min + val * (max - min);

                                    handle.width(Percentage(dx * 100.0));
                                });

                            HStack::new(cx, |cx| {
                                Element::new(cx).class("inner").hoverable(false);
                            })
                            .right(Stretch(1.0))
                            .top(Stretch(1.0))
                            .bottom(Stretch(1.0))
                            .overflow(Overflow::Visible)
                            .class("thumb")
                            .on_geo_changed(|cx, geo| {
                                if geo.contains(GeometryChanged::WIDTH_CHANGED) {
                                    cx.emit(SliderEventInternal::SetThumbSize(
                                        cx.cache.get_width(cx.current),
                                        cx.cache.get_height(cx.current),
                                    ));
                                }
                            })
                            .bind(
                                lens.clone(),
                                move |handle, value| {
                                    let val = *value.get(handle.cx);
                                    let px = val * (1.0 - (thumb_size / size));

                                    handle.left(Percentage(100.0 * px));
                                },
                            );
                        }

                        Orientation::Vertical => {
                            //(Stretch(1.0), Percentage(dx * 100.0))
                            Element::new(cx)
                                .width(Stretch(1.0))
                                .top(Stretch(1.0))
                                .bottom(Pixels(0.0))
                                .class("active")
                                .bind(lens.clone(), move |handle, value| {
                                    let val = *value.get(handle.cx);
                                    let min = thumb_size / size;
                                    let max = 1.0;
                                    let dx = min + val * (max - min);

                                    handle.height(Percentage(dx * 100.0));
                                });

                            Element::new(cx)
                                .top(Stretch(1.0))
                                .left(Stretch(1.0))
                                .right(Stretch(1.0))
                                .overflow(Overflow::Visible)
                                .class("thumb")
                                .on_geo_changed(|cx, geo| {
                                    if geo.contains(GeometryChanged::HEIGHT_CHANGED) {
                                        cx.emit(SliderEventInternal::SetThumbSize(
                                            cx.cache.get_width(cx.current),
                                            cx.cache.get_height(cx.current),
                                        ));
                                    }
                                })
                                .bind(lens.clone(), move |handle, value| {
                                    let val = *value.get(handle.cx);
                                    let px = val * (1.0 - (thumb_size / size));

                                    handle.bottom(Percentage(100.0 * px));
                                });
                        }
                    };
                });
            });
        })
    }
}

impl<L: 'static> View for Slider<L> {
    fn element(&self) -> Option<String> {
        Some("slider".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    self.is_dragging = true;
                    cx.capture();
                    cx.current.set_active(cx, true);

                    if let Some(slider_data_internal) = cx.data::<SliderDataInternal>() {
                        let thumb_size = slider_data_internal.thumb_size;

                        let mut dx = match slider_data_internal.orientation {
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
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    self.is_dragging = false;
                    cx.release();
                    cx.current.set_active(cx, false);
                }

                WindowEvent::MouseMove(x, y) => {
                    if self.is_dragging {
                        if let Some(slider_data_internal) = cx.data::<SliderDataInternal>() {
                            let thumb_size = slider_data_internal.thumb_size;

                            let mut dx = match slider_data_internal.orientation {
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
                }

                _ => {}
            }
        }
    }
}

impl<'a, L: 'static> Handle<'a, Slider<L>> {
    /// Set the callback triggered when the slider value has changed.
    ///
    /// Takes a closure which provides the current value and returns an event to be sent when the slider
    /// value has changed after releasing the slider. If the slider thumb is pressed but not moved, and thus
    /// the value is not changed, then the event will not be sent.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// Slider::new(cx, 0.0, Orientation::Horizontal)
    ///     .on_change(cx, |cx, value| {
    ///         cx.emit(WindowEvent::Debug(format!("Slider on_change: {}", value)));
    ///     });
    /// ```
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context, f32),
    {
        if let Some(slider) =
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<Slider<L>>())
        {
            slider.on_change = Some(Box::new(callback));
        }

        self
    }

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

    /// Set the callback triggered when the slider value reaches the minimum.
    ///
    /// Takes a closure which triggers when the slider reaches the minimum value,
    /// either by pressing the track at the start or dragging the thumb to the start
    /// of the track. The event is sent once for each time the value reaches the minimum.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// Slider::new(cx, 0.0, Orientation::Horizontal)
    ///     .on_min(cx, |cx| {
    ///         cx.emit(WindowEvent::Debug(format!("Slider on_min")));
    ///     });
    /// ```
    pub fn on_min<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        if let Some(slider) =
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<Slider<L>>())
        {
            slider.on_min = Some(Box::new(callback));
        }

        self
    }

    /// Set the callback triggered when the slider value reaches the maximum.
    ///
    /// Takes a closure which triggers when the slider reaches the maximum value,
    /// either by pressing the track at the end or dragging the thumb to the end
    /// of the track. The event is sent once for each time the value reaches the maximum.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// Slider::new(cx, 0.0, Orientation::Horizontal)
    ///     .on_max(|cx| {
    ///         cx.emit(WindowEvent::Debug(format!("Slider on_max")));
    ///     });
    /// ```
    pub fn on_max<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        if let Some(slider) =
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<Slider<L>>())
        {
            slider.on_max = Some(Box::new(callback));
        }

        self
    }
}
