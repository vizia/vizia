use morphorm::{LayoutType, Units};

use crate::{Event, Handle, MouseButton, Units::*, View, ViewModifers, WindowEvent};

use crate::{Context, Element, Entity};


#[derive(Debug, Clone, PartialEq)]
pub enum SliderEvent {
    // TODO - Remove this
    ValueChanged(f32),
    SetValue(f32),
    SetMin(f32),
    SetMax(f32),
}

pub struct Slider {
    // The track that the thumb slides along
    track: Handle<Element>,
    // An overlay on the track to indicate the value
    active: Handle<Element>,
    // A marker used to indicate the value by its position along the track
    thumb: Handle<Element>,

    // Event sent when the slider value has changed
    on_change: Option<Box<dyn Fn(&mut Self, &mut Context)>>,
    // event sent when the slider value is changing
    on_changing: Option<Box<dyn Fn(&mut Self, &mut Context)>>,
    // Event sent when the slider reaches the minimum value
    on_min: Option<Box<dyn Fn(&mut Self, &mut Context)>>,
    // Event sent when the slider reaches the maximum value
    on_max: Option<Box<dyn Fn(&mut Self, &mut Context)>>,
    // Event sent when the slider is pressed
    on_press: Option<Box<dyn Fn(&mut Self, &mut Context)>>,
    // Event sent when the slider is released
    on_release: Option<Box<dyn Fn(&mut Self, &mut Context)>>,
    // Event sent when the mouse cursor enters the slider
    on_over: Option<Box<dyn Fn(&mut Self, &mut Context)>>,
    // Event sent when the mouse cusor leaves the slider
    on_out: Option<Box<dyn Fn(&mut Self, &mut Context)>>,

    pub value: f32,
    prev: f32,
    min: f32,
    max: f32,

    is_min: bool,
    is_max: bool,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            track: Handle::null(),
            active: Handle::null(),
            thumb: Handle::null(),

            on_change: None,
            on_changing: None,
            on_min: None,
            on_max: None,
            on_press: None,
            on_release: None,
            on_over: None,
            on_out: None,

            value: 0.0,
            prev: 0.0,
            min: 0.0,
            max: 1.0,

            is_min: true,
            is_max: false,
        }
    }
}

impl Slider {
    /// Create a new slider widget with default values (min: 0.0, max: 1.0, val: 0.0).
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new().build(context, parent, |builder| builder);
    /// ```
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self::default()
            .build(cx)            
            .layout_type(LayoutType::Row)
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))

    }

    /// Set the initial value of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///    .with_init(0.5)
    ///    .build(context, parent, |builder| builder)
    /// ```
    pub fn with_init(mut self, val: f32) -> Self {
        self.value = val;

        self
    }

    /// Set the range of the slider. Min and Max values are extracted from the range.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .with_range(0.0..5.0)
    ///     .build(context, parent, |builder| builder)
    /// ```
    pub fn with_range(mut self, range: std::ops::Range<f32>) -> Self {
        self.min = range.start;
        self.max = range.end;

        self
    }

    /// Set the minimum value of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .with_min(0.2)
    ///     .build(context, parent, |builder| builder)
    /// ```
    pub fn with_min(mut self, val: f32) -> Self {
        self.min = val;
        self
    }

    /// Set the maximum value of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .with_max()
    ///     .build(context, parent, |builder| builder)
    /// ```
    pub fn with_max(mut self, val: f32) -> Self {
        self.max = val;
        self
    }

    /// Set the callback triggered when the slider value has changed.
    ///
    /// Takes a closure which provides the current value and returns an event to be sent when the slider
    /// value has changed after releasing the slider. If the slider thumb is pressed but not moved, and thus
    /// the value is not changed, then the event will not be sent.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_change(|slider, context, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_change: {}", slider.value)));
    ///     })
    ///     .build(context, parent, |builder| builder);
    /// ```
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut Context),
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Set the callback triggered when the slider value is changing (dragging).
    ///
    /// Takes a closure which triggers when the slider value is changing, 
    /// either by pressing the track or dragging the thumb along the track.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_changing(|slider, context, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_changing: {}", slider.value)));
    ///     })
    ///     .build(context, parent, |builder| builder);
    /// ```
    pub fn on_changing<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut Context),
    {
        self.on_changing = Some(Box::new(callback));
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
    /// ```
    /// Slider::new()
    ///     .on_min(|slider, context, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(context, parent, |builder| builder);
    /// ```
    pub fn on_min<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut Context),
    {
        self.on_min = Some(Box::new(callback));
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
    /// ```
    /// Slider::new()
    ///     .on_max(|slider, context, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(context, parent, |builder| builder);
    /// ```
    pub fn on_max<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut Context),
    {
        self.on_max = Some(Box::new(callback));
        self
    }

    /// Set the event sent when the slider is pressed.
    ///
    /// The event is sent when the left mouse button is pressed on any part of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_max(|slider, context, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(context, parent, |builder| builder);
    /// ```
    // pub fn on_press<F>(mut self, callback: F) -> Self 
    // where
    //     F: 'static + Fn(&mut Self, &mut Context, Entity),
    // {
    //     self.on_press = Some(Box::new(callback));
    //     self
    // }

    /// Set the event sent when the slider is released.
    ///
    /// The event is sent when the left mouse button is released after being pressed on any part of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_max(|slider, context, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(context, parent, |builder| builder);
    /// ```
    pub fn on_release<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut Context),
    {
        self.on_release = Some(Box::new(callback));
        self
    }

    /// Set the event sent when the mouse cursor enters the slider.
    ///
    /// The event is sent when the mouse cursor enters the bounding box of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_max(|slider, context, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(context, parent, |builder| builder);
    /// ```
    pub fn on_over<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut Context),
    {
        self.on_over = Some(Box::new(callback));
        self
    }

    /// Set the event sent when the mouse cursor leaves the slider
    ///
    /// The event is sent when the mouse cursor leaves the bounding box of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_max(|slider, context, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(context, parent, |builder| builder);
    /// ```
    pub fn on_out<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut Context),
    {
        self.on_out = Some(Box::new(callback));
        self
    }

    // Private helper functions

    // Update the active size and thumb position
    fn update_value(&mut self, cx: &mut Context, mut dx: f32) {
        let width = cx.cache.get_width(cx.current);
        let thumb_width = cx.cache.get_width(self.thumb.entity());

        if dx <= thumb_width / 2.0 {
            dx = thumb_width / 2.0;
        }
        if dx >= width - thumb_width / 2.0 {
            dx = width - thumb_width / 2.0;
        }

        let nx = (dx - thumb_width / 2.0) / (width - thumb_width);

        // self.thumb
        //     .set_left(cx, Units::Percentage(100.0 * (dx - thumb_width / 2.0) / width));

        //cx.style.borrow_mut().left.insert(self.thumb, Percentage(100.0 * (dx - thumb_width / 2.0) / width));

        self.thumb.left(Percentage(100.0 * (dx - thumb_width / 2.0) / width));

        self.active.width(Units::Percentage(nx * 100.0));


        self.value = self.min + nx * (self.max - self.min);

        if self.value == self.min {
            if !self.is_min {
                self.is_min = true;
                //self.send_value_event(context, entity, &self.on_min);
                if let Some(callback) = self.on_min.take() {
                    (callback)(self, cx);
                    self.on_min = Some(callback);
                }
            }
        } else {
            self.is_min = false;
        }

        if self.value == self.max {
            if !self.is_max {
                self.is_max = true;
                if let Some(callback) = self.on_max.take() {
                    (callback)(self, cx);
                    self.on_max = Some(callback);
                }
            }
        } else {
            self.is_max = false;
        }
    }

    fn update_visuals(&mut self, cx: &mut Context) {
        let normalised_value = (self.value - self.min) / (self.max - self.min);

        let width = cx.cache.get_width(cx.current);
        let thumb_width = cx.cache.get_width(self.thumb.entity());

        let dx = normalised_value * (width - thumb_width) + thumb_width / 2.0;

        self.update_value(cx, dx);
    }

    fn clamp_value(&mut self) {
        self.value = self.value.clamp(self.min, self.max);
    }
}

impl View for Slider {

    fn element(&self) -> Option<String> {
        Some("slider".to_string())
    }

    fn body(&mut self, cx: &mut Context) 
    {
        if self.min > self.max {
            panic!("minimum value must be less than maximum value")
        }

        self.clamp_value();

        self.is_min = self.value == self.min;
        self.is_max = self.value == self.max;

        // entity
        //     .set_layout_type(context, LayoutType::Row)
        //     .set_child_top(context, Stretch(1.0))
        //     .set_child_bottom(context, Stretch(1.0));

        // Track
        self.track = Element::new(cx)
            .width(Stretch(1.0))
            // .set_height(Pixels(4.0))
            .bottom(Auto)
            //.hoverable(false)
            .class("track")
            .overlay(cx, |cx|{
                self.active = Element::new(cx);
            });

        // Active
        // self.active = Element::new(cx, self.track, |builder| {
        //     builder
        //         .set_width(Percentage(0.0))
        //         .set_height(Stretch(1.0))
        //         .set_hoverable(false)
        //         .class("active")
        // });

        // Thumb
        // self.thumb = Element::new(cx, entity, |builder| {
        //     builder
        //         .set_position_type(PositionType::SelfDirected)
        //         .set_hoverable(false)
        //         .class("thumb")
        // });
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        // Handle window events
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                
                //TODO
                // WindowEvent::GeometryChanged(_) if event.target == entity => {
                //     self.update_visuals(cx, entity);
                // }

                WindowEvent::MouseOver if event.target == cx.current => {
                    if let Some(callback) = self.on_over.take() {
                        (callback)(self, cx);
                        self.on_over = Some(callback);
                    }
                }

                WindowEvent::MouseOut if event.target == cx.current => {
                    if let Some(callback) = self.on_out.take() {
                        (callback)(self, cx);
                        self.on_out = Some(callback);
                    }
                }

                WindowEvent::MouseDown(button) if event.target == cx.current => {
                    if *button == MouseButton::Left {
                        //cx.capture(cx.current);

                        self.prev = self.value;

                        //entity.set_active(cx, true);

                        if let Some(callback) = self.on_press.take() {
                            (callback)(self, cx);
                            self.on_press = Some(callback);
                        }

                        let dx = cx.mouse.left.pos_down.0 - cx.cache.get_posx(cx.current);

                        self.update_value(cx, dx);

                        if let Some(callback) = self.on_changing.take() {
                            (callback)(self, cx);
                            self.on_changing = Some(callback);
                        }

                        cx.emit(SliderEvent::ValueChanged(self.value));
                    }
                }

                WindowEvent::MouseUp(button) if event.target == cx.current => {
                    if *button == MouseButton::Left {
                        //cx.release(cx.current);

                        //entity.set_active(cx, false);

                        if self.prev != self.value {
                            //self.send_value_event(context, entity, &self.on_change);
                            if let Some(callback) = self.on_change.take() {
                                (callback)(self, cx);
                                self.on_change = Some(callback);
                            }

                        }

                        if let Some(callback) = self.on_release.take() {
                            (callback)(self, cx);
                            self.on_release = Some(callback);
                        }
                    }
                }

                WindowEvent::MouseMove(x, _) if event.target == cx.current => {
                    //if entity.is_active(cx) {
                        let dx = *x - cx.cache.get_posx(cx.current);

                        self.update_value(cx, dx);
                        
                        if let Some(callback) = self.on_changing.take() {
                            (callback)(self, cx);
                            self.on_changing = Some(callback);
                        }
                    //}
                }

                // TODO - Add keyboard control
                _ => {}
            }
        }

        // Handle slider events
        if let Some(slider_event) = event.message.downcast() {
            match slider_event {
                SliderEvent::SetMin(val) => {
                    self.min = *val;
                    self.min = self.min.min(self.max);
                    self.clamp_value();

                    self.update_visuals(cx);
                }

                SliderEvent::SetMax(val) => {
                    self.max = *val;
                    self.max = self.max.max(self.min);
                    self.clamp_value();

                    self.update_visuals(cx);
                }

                SliderEvent::SetValue(val) => {
                    self.value = *val;
                    self.clamp_value();

                    self.update_visuals(cx);
                }

                _ => {}
            }
        }
    }

    // fn on_update(&mut self, context: &mut Context, entity: Entity, data: &Self::Data) {
    //     self.value = *data;
    //     self.update_visuals(context, entity);
    // }
}
