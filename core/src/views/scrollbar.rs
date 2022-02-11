use morphorm::GeometryChanged;

use crate::{
    Actions, Binding, Context, Data, Element, Entity, Handle, Lens, Model, MouseButton, Overflow,
    Units::*, View, WindowEvent, ZStack, SliderData, Orientation, LensExt,
};

use super::slider::{SliderDataInternal, SliderEventInternal};


pub struct Scrollbar {
    is_dragging: bool,

    // Event sent when the slider value has changed
    on_change: Option<Box<dyn Fn(&mut Context, f32)>>,
    // event sent when the slider value is changing
    on_changing: Option<Box<dyn Fn(&mut Context, f32)>>,
    // Event sent when the slider reaches the minimum value
    on_min: Option<Box<dyn Fn(&mut Context)>>,
    // Event sent when the slider reaches the maximum value
    on_max: Option<Box<dyn Fn(&mut Context)>>,
}

impl Scrollbar {
    pub fn new(cx: &mut Context, pos: f32, thumb_size: f32, orientation: Orientation) -> Handle<Self> {
        Self { is_dragging: false, on_change: None, on_changing: None, on_min: None, on_max: None }
            .build2(cx, move |cx| {
                // Create some slider data
                SliderData { value: pos.clamp(0.0, 1.0) }.build(cx);

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
                    SliderDataInternal { size: 0.0, thumb_size: thumb_size, orientation }.build(cx);
                }

                // Add the various slider components using bindings to the slider data
                Binding::new(cx, SliderData::value, move |cx, value| {
                    Binding::new(cx, SliderDataInternal::root, move |cx, slider_data_internal| {
                        let value = value.clone();
                        ZStack::new(cx, move |cx| {
                            //let thumb_size = slider_data_internal.get(cx).thumb_size;

                            //println!("min: {} {}", thumb_size, slider_data_internal.get(cx).size);

                            println!("{}", value.get(cx));

                            let val = value.get(cx);
                            let size = slider_data_internal.get(cx).size;
                            let min = thumb_size;
                            let max = 1.0;
                            let dx = min + val * (max - min);
                            let px = val * (1.0 - thumb_size);

                            let orientation = slider_data_internal.get(cx).orientation;

                            match orientation {
                                Orientation::Horizontal => {
                                    //(Percentage(dx * 100.0), Stretch(1.0))
                                    Element::new(cx)
                                        .width(Percentage(dx * 100.0))
                                        .height(Stretch(1.0))
                                        .left(Pixels(0.0))
                                        .right(Stretch(1.0))
                                        .class("active");

                                    Element::new(cx)
                                        .left(Percentage(100.0 * (1.0 - px)))
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
                                        });
                                }

                                Orientation::Vertical => {
                                    //println!("scrollbar height ratio: {}", thumb_size * 100.0);
                                    //(Stretch(1.0), Percentage(dx * 100.0))
                                    // Element::new(cx)
                                    //     .height(Percentage(dx * 100.0))
                                    //     .width(Stretch(1.0))
                                    //     .top(Stretch(1.0))
                                    //     .bottom(Pixels(0.0))
                                    //     .class("active");

                                    Element::new(cx)
                                        .top(Percentage(100.0 * px))
                                        .bottom(Stretch(1.0))
                                        .left(Stretch(1.0))
                                        .right(Stretch(1.0))
                                        .overflow(Overflow::Visible)
                                        .height(Percentage(thumb_size * 100.0))
                                        //.height(Pixels(thumb_size))
                                        .class("thumb");
                                        // .on_geo_changed(|cx, geo| {
                                        //     if geo.contains(GeometryChanged::HEIGHT_CHANGED) {
                                        //         cx.emit(SliderEventInternal::SetThumbSize(
                                        //             cx.cache.get_width(cx.current),
                                        //             cx.cache.get_height(cx.current),
                                        //         ));
                                        //     }
                                        // });
                                }
                            };
                        });
                    });
                });
            })
    }
}

impl View for Scrollbar {
    fn element(&self) -> Option<String> {
        Some("slider".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    self.is_dragging = true;
                    cx.captured = cx.current;

                    if let Some(slider_data_internal) = cx.data::<SliderDataInternal>() {
                        let thumb_size = slider_data_internal.thumb_size;

                        //let t = thumb_size;

                        let mut dx = match slider_data_internal.orientation {
                            Orientation::Horizontal => {
                                let t = thumb_size * cx.cache.get_width(cx.current);
                                (cx.mouse.left.pos_down.0
                                    - cx.cache.get_posx(cx.current)
                                    - t / 2.0)
                                    / (cx.cache.get_width(cx.current) - t)
                            }

                            Orientation::Vertical => {
                                let t = thumb_size * cx.cache.get_height(cx.current);
                                //println!("t: {}", t);
                                (cx.cache.get_height(cx.current)
                                    - (cx.mouse.left.pos_down.1 - cx.cache.get_posy(cx.current))
                                    - t / 2.0)
                                    / (cx.cache.get_height(cx.current) - t)
                            }
                        };

                        dx = dx.clamp(0.0, 1.0);
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    self.is_dragging = false;
                    cx.captured = Entity::null();
                }

                WindowEvent::MouseMove(x, y) => {
                    if self.is_dragging {
                        if let Some(slider_data_internal) = cx.data::<SliderDataInternal>() {
                            let thumb_size = slider_data_internal.thumb_size;

                            //let t = thumb_size;

                            let mut dx = match slider_data_internal.orientation {
                                Orientation::Horizontal => {
                                    let t = thumb_size * cx.cache.get_width(cx.current);
                                    (*x - cx.cache.get_posx(cx.current) - t / 2.0)
                                        / (cx.cache.get_width(cx.current) - t)
                                }

                                Orientation::Vertical => {
                                    let t = thumb_size * cx.cache.get_height(cx.current);
                                    1.0 - ((cx.cache.get_height(cx.current)
                                        - (*y - cx.cache.get_posy(cx.current))
                                        - t / 2.0)
                                        / (cx.cache.get_height(cx.current) - t))
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

impl<'a> Handle<'a, Scrollbar> {
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
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<Scrollbar>())
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
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<Scrollbar>())
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
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<Scrollbar>())
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
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<Scrollbar>())
        {
            slider.on_max = Some(Box::new(callback));
        }

        self
    }
}
