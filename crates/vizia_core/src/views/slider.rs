use std::ops::Range;

use accesskit::ActionData;

use crate::prelude::*;

/// The slider control can be used to select from a continuous set of values.
///
/// The slider control consists of three main parts, a **thumb** element which can be moved between the extremes of a linear **track**,
/// and an **active** element which fills the slider to indicate the current value.
///
/// # Examples
///
/// ## Basic Slider
/// In the following example, a slider is bound to a signal. The `on_change` callback is used to
/// update the signal when the slider thumb is moved, or if the track is clicked on.
/// ```
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::default();
/// # let value = cx.state(0.5f32);
/// Slider::new(cx, value)
///     .on_change(move |cx, val| {
///         value.set(cx, val);
///     });
/// ```
///
/// ## Slider with Label
/// ```
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::default();
/// # let value = cx.state(0.5f32);
/// HStack::new(cx, |cx|{
///     Slider::new(cx, value)
///         .on_change(move |cx, val| {
///             value.set(cx, val);
///         });
///     Label::new(cx, value.map(|val| format!("{:.2}", val)));
/// });
/// ```
pub struct Slider {
    value: Signal<f32>,
    is_dragging: bool,
    orientation: Orientation,
    range: Range<f32>,
    step: f32,
    keyboard_fraction: f32,
    on_change: Option<Box<dyn Fn(&mut EventContext, f32)>>,
}

impl Slider {
    /// Creates a new slider bound to the given signal.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// # let value = cx.state(0.5f32);
    /// Slider::new(cx, value)
    ///     .on_change(move |cx, val| {
    ///         value.set(cx, val);
    ///     });
    /// ```
    pub fn new(cx: &mut Context, value: Signal<f32>) -> Handle<Self> {
        Self {
            value,
            is_dragging: false,
            orientation: Orientation::Horizontal,
            range: 0.0..1.0,
            step: 0.01,
            keyboard_fraction: 0.1,
            on_change: None,
        }
        .build(cx, move |cx| {
            // Track
            HStack::new(cx, move |cx| {
                // Active track
                VStack::new(cx, |cx| {
                    // Thumb
                    Element::new(cx).class("thumb").bind(value, move |handle, val| {
                        let v = *val.get(&handle);
                        // We'll get range from parent via event, for now use 0..1
                        let normal_val = v.clamp(0.0, 1.0);
                        handle.translate((Percentage(100.0 * (1.0 - normal_val)), Pixels(0.0)));
                    });
                })
                .class("active")
                .bind(value, move |handle, val| {
                    let v = *val.get(&handle);
                    let normal_val = v.clamp(0.0, 1.0);
                    handle
                        .height(Stretch(1.0))
                        .width(Percentage(normal_val * 100.0))
                        .layout_type(LayoutType::Row)
                        .alignment(Alignment::Right);
                });
            })
            .class("track");
        })
        .role(Role::Slider)
        .numeric_value(value.map(|val| (*val as f64 * 100.0).round() / 100.0))
        .text_value(value.map(|val| {
            let v = (*val as f64 * 100.0).round() / 100.0;
            format!("{}", v)
        }))
        .navigable(true)
    }
}

impl View for Slider {
    fn element(&self) -> Option<&'static str> {
        Some("slider")
    }

    fn accessibility(&self, _cx: &mut AccessContext, node: &mut AccessNode) {
        node.set_numeric_value_step(self.step as f64);
        node.set_min_numeric_value(self.range.start as f64);
        node.set_max_numeric_value(self.range.end as f64);
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                if !cx.is_disabled() {
                    self.is_dragging = true;
                    cx.capture();
                    cx.focus_with_visibility(false);
                    cx.with_current(Entity::root(), |cx| {
                        cx.set_pointer_events(false);
                    });

                    let thumb = cx.get_entities_by_class("thumb").first().copied().unwrap();
                    let thumb_size = match self.orientation {
                        Orientation::Horizontal => cx.cache.get_width(thumb),
                        Orientation::Vertical => cx.cache.get_height(thumb),
                    };
                    let min = self.range.start;
                    let max = self.range.end;
                    let step = self.step;

                    let current = cx.current();
                    let width = cx.cache.get_width(current);
                    let height = cx.cache.get_height(current);
                    let posx = cx.cache.get_posx(current);
                    let posy = cx.cache.get_posy(current);

                    let mut dx = match self.orientation {
                        Orientation::Horizontal => {
                            (cx.mouse.left.pos_down.0 - posx - thumb_size / 2.0)
                                / (width - thumb_size)
                        }

                        Orientation::Vertical => {
                            (height - (cx.mouse.left.pos_down.1 - posy) - thumb_size / 2.0)
                                / (height - thumb_size)
                        }
                    };

                    dx = dx.clamp(0.0, 1.0);

                    let mut val = min + dx * (max - min);

                    val = step * (val / step).ceil();
                    val = val.clamp(min, max);

                    if let Some(callback) = self.on_change.take() {
                        (callback)(cx, val);

                        self.on_change = Some(callback);
                    }
                }
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                self.is_dragging = false;
                cx.focus_with_visibility(false);
                cx.release();
                cx.with_current(Entity::root(), |cx| {
                    cx.set_pointer_events(true);
                });
            }

            WindowEvent::MouseMove(x, y) => {
                if self.is_dragging {
                    let thumb = cx.get_entities_by_class("thumb").first().copied().unwrap();
                    let thumb_size = match self.orientation {
                        Orientation::Horizontal => cx.cache.get_width(thumb),
                        Orientation::Vertical => cx.cache.get_height(thumb),
                    };

                    let min = self.range.start;
                    let max = self.range.end;
                    let step = self.step;

                    let current = cx.current();
                    let width = cx.cache.get_width(current);
                    let height = cx.cache.get_height(current);
                    let posx = cx.cache.get_posx(current);
                    let posy = cx.cache.get_posy(current);

                    let mut dx = match self.orientation {
                        Orientation::Horizontal => {
                            (*x - posx - thumb_size / 2.0) / (width - thumb_size)
                        }

                        Orientation::Vertical => {
                            (height - (*y - posy) - thumb_size / 2.0) / (height - thumb_size)
                        }
                    };

                    dx = dx.clamp(0.0, 1.0);

                    let mut val = min + dx * (max - min);

                    val = step * (val / step).ceil();
                    val = val.clamp(min, max);

                    if let Some(callback) = &self.on_change {
                        (callback)(cx, val);
                    }
                }
            }

            WindowEvent::KeyDown(Code::ArrowUp | Code::ArrowRight, _) => {
                let min = self.range.start;
                let max = self.range.end;
                let step = self.step;
                let mut val = *self.value.get(cx) + step;
                val = val.clamp(min, max);
                if let Some(callback) = &self.on_change {
                    (callback)(cx, val);
                }
            }

            WindowEvent::KeyDown(Code::ArrowDown | Code::ArrowLeft, _) => {
                let min = self.range.start;
                let max = self.range.end;
                let step = self.step;
                let mut val = *self.value.get(cx) - step;
                val = val.clamp(min, max);
                if let Some(callback) = &self.on_change {
                    (callback)(cx, val);
                }
            }

            WindowEvent::ActionRequest(action) => match action.action {
                Action::Increment => {
                    let min = self.range.start;
                    let max = self.range.end;
                    let step = self.step;
                    let mut val = *self.value.get(cx) + step;
                    val = step * (val / step).ceil();
                    val = val.clamp(min, max);
                    if let Some(callback) = &self.on_change {
                        (callback)(cx, val);
                    }
                }

                Action::Decrement => {
                    let min = self.range.start;
                    let max = self.range.end;
                    let step = self.step;
                    let mut val = *self.value.get(cx) - step;
                    val = step * (val / step).ceil();
                    val = val.clamp(min, max);
                    if let Some(callback) = &self.on_change {
                        (callback)(cx, val);
                    }
                }

                Action::SetValue => {
                    if let Some(ActionData::NumericValue(val)) = action.data {
                        let min = self.range.start;
                        let max = self.range.end;
                        let mut v = val as f32;
                        v = v.clamp(min, max);
                        if let Some(callback) = &self.on_change {
                            (callback)(cx, v);
                        }
                    }
                }

                _ => {}
            },

            _ => {}
        });
    }
}

impl Handle<'_, Slider> {
    /// Sets the callback triggered when the slider value is changed.
    ///
    /// Takes a closure which triggers when the slider value is changed,
    /// either by pressing the track or dragging the thumb along the track.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// # let value = cx.state(0.5f32);
    /// Slider::new(cx, value)
    ///     .on_change(move |cx, val| {
    ///         value.set(cx, val);
    ///     });
    /// ```
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, f32),
    {
        self.modify(|slider| slider.on_change = Some(Box::new(callback)))
    }

    /// Sets the range of the slider.
    ///
    /// If the bound data is outside of the range then the slider will clip to min/max of the range.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// # let value = cx.state(0.5f32);
    /// Slider::new(cx, value)
    ///     .range(-20.0..50.0)
    ///     .on_change(move |cx, val| {
    ///         value.set(cx, val);
    ///     });
    /// ```
    pub fn range(self, range: Range<f32>) -> Self {
        self.modify(|slider| {
            slider.range = range;
        })
    }

    /// Sets the orientation of the slider.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// # let value = cx.state(0.5f32);
    /// Slider::new(cx, value)
    ///     .orientation(Orientation::Vertical)
    ///     .on_change(move |cx, val| {
    ///         value.set(cx, val);
    ///     });
    /// ```
    pub fn orientation(self, orientation: Orientation) -> Self {
        self.modify(|slider| {
            slider.orientation = orientation;
        })
        .toggle_class("vertical", orientation == Orientation::Vertical)
    }

    /// Set the step value for the slider.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// # let value = cx.state(0.5f32);
    /// Slider::new(cx, value)
    ///     .step(0.1)
    ///     .on_change(move |cx, val| {
    ///         value.set(cx, val);
    ///     });
    /// ```
    pub fn step(self, step: f32) -> Self {
        self.modify(|slider| {
            slider.step = step;
        })
    }

    /// Sets the fraction of a slider that a press of an arrow key will change.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// # let value = cx.state(0.5f32);
    /// Slider::new(cx, value)
    ///     .keyboard_fraction(0.05)
    ///     .on_change(move |cx, val| {
    ///         value.set(cx, val);
    ///     });
    /// ```
    pub fn keyboard_fraction(self, keyboard_fraction: f32) -> Self {
        self.modify(|slider| {
            slider.keyboard_fraction = keyboard_fraction;
        })
    }
}
