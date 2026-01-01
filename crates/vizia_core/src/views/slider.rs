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
/// let value_label = cx.derived({
///     let value = value;
///     move |store| format!("{:.2}", *value.get(store))
/// });
/// HStack::new(cx, |cx|{
///     Slider::new(cx, value)
///         .on_change(move |cx, val| {
///             value.set(cx, val);
///         });
///     Label::new(cx, value_label);
/// });
/// ```
pub struct Slider {
    value: Signal<f32>,
    is_dragging: bool,
    orientation: Signal<Orientation>,
    range: Signal<Range<f32>>,
    range_cache: Range<f32>, // Cached for accessibility (AccessContext doesn't impl DataContext)
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
        let range = cx.state(0.0f32..1.0f32);
        let orientation = cx.state(Orientation::Horizontal);

        // Create a derived signal that normalizes the value to 0..1 based on range
        let normalized = cx.derived(move |s| {
            let v = *value.get(s);
            let r = range.get(s);
            let min = r.start;
            let max = r.end;
            if (max - min).abs() < f32::EPSILON {
                0.0
            } else {
                ((v - min) / (max - min)).clamp(0.0, 1.0)
            }
        });
        let thumb_translate = cx.derived({
            let normalized = normalized;
            let orientation = orientation;
            move |store| {
                let v = *normalized.get(store);
                match *orientation.get(store) {
                    Orientation::Horizontal => {
                        Translate::new(Percentage(100.0 * (1.0 - v)), Pixels(0.0))
                    }
                    Orientation::Vertical => {
                        Translate::new(Pixels(0.0), Percentage(-100.0 * (1.0 - v)))
                    }
                }
            }
        });
        let active_height = cx.derived({
            let normalized = normalized;
            let orientation = orientation;
            move |store| {
                let v = *normalized.get(store);
                match *orientation.get(store) {
                    Orientation::Horizontal => Units::Stretch(1.0),
                    Orientation::Vertical => Units::Percentage(v * 100.0),
                }
            }
        });
        let active_width = cx.derived({
            let normalized = normalized;
            let orientation = orientation;
            move |store| {
                let v = *normalized.get(store);
                match *orientation.get(store) {
                    Orientation::Horizontal => Units::Percentage(v * 100.0),
                    Orientation::Vertical => Units::Stretch(1.0),
                }
            }
        });
        let active_layout = cx.derived({
            let orientation = orientation;
            move |store| match *orientation.get(store) {
                Orientation::Horizontal => LayoutType::Row,
                Orientation::Vertical => LayoutType::Column,
            }
        });
        let active_alignment = cx.derived({
            let orientation = orientation;
            move |store| match *orientation.get(store) {
                Orientation::Horizontal => Alignment::Right,
                Orientation::Vertical => Alignment::TopCenter,
            }
        });
        let numeric_value = cx.derived({
            let value = value;
            move |store| (*value.get(store) as f64 * 100.0).round() / 100.0
        });
        let text_value = cx.derived({
            let value = value;
            move |store| {
                let v = (*value.get(store) as f64 * 100.0).round() / 100.0;
                format!("{}", v)
            }
        });
        let navigable = cx.state(true);

        Self {
            value,
            is_dragging: false,
            orientation,
            range,
            range_cache: 0.0..1.0,
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
                    Element::new(cx).class("thumb").translate(thumb_translate);
                })
                .class("active")
                .height(active_height)
                .width(active_width)
                .layout_type(active_layout)
                .alignment(active_alignment);
            })
            .class("track");
        })
        .role(Role::Slider)
        .numeric_value(numeric_value)
        .text_value(text_value)
        .navigable(navigable)
    }
}

impl View for Slider {
    fn element(&self) -> Option<&'static str> {
        Some("slider")
    }

    fn accessibility(&self, _cx: &mut AccessContext, node: &mut AccessNode) {
        node.set_numeric_value_step(self.step as f64);
        node.set_min_numeric_value(self.range_cache.start as f64);
        node.set_max_numeric_value(self.range_cache.end as f64);
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

                    let range = self.range.get(cx);
                    let min = range.start;
                    let max = range.end;
                    let step = self.step;

                    let current = cx.current();
                    let width = cx.cache.get_width(current).max(1.0);
                    let height = cx.cache.get_height(current).max(1.0);
                    let posx = cx.cache.get_posx(current);
                    let posy = cx.cache.get_posy(current);

                    let mut dx = match *self.orientation.get(cx) {
                        Orientation::Horizontal => {
                            (cx.mouse.left.pos_down.0 - posx) / width
                        }

                        Orientation::Vertical => {
                            (height - (cx.mouse.left.pos_down.1 - posy)) / height
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
                    let range = self.range.get(cx);
                    let min = range.start;
                    let max = range.end;
                    let step = self.step;

                    let current = cx.current();
                    let width = cx.cache.get_width(current).max(1.0);
                    let height = cx.cache.get_height(current).max(1.0);
                    let posx = cx.cache.get_posx(current);
                    let posy = cx.cache.get_posy(current);

                    let mut dx = match *self.orientation.get(cx) {
                        Orientation::Horizontal => {
                            (*x - posx) / width
                        }

                        Orientation::Vertical => {
                            (height - (*y - posy)) / height
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
                let range = self.range.get(cx);
                let min = range.start;
                let max = range.end;
                let step = self.step;
                let mut val = *self.value.get(cx) + step;
                val = val.clamp(min, max);
                if let Some(callback) = &self.on_change {
                    (callback)(cx, val);
                }
            }

            WindowEvent::KeyDown(Code::ArrowDown | Code::ArrowLeft, _) => {
                let range = self.range.get(cx);
                let min = range.start;
                let max = range.end;
                let step = self.step;
                let mut val = *self.value.get(cx) - step;
                val = val.clamp(min, max);
                if let Some(callback) = &self.on_change {
                    (callback)(cx, val);
                }
            }

            WindowEvent::ActionRequest(action) => match action.action {
                Action::Increment => {
                    let range = self.range.get(cx);
                    let min = range.start;
                    let max = range.end;
                    let step = self.step;
                    let mut val = *self.value.get(cx) + step;
                    val = step * (val / step).ceil();
                    val = val.clamp(min, max);
                    if let Some(callback) = &self.on_change {
                        (callback)(cx, val);
                    }
                }

                Action::Decrement => {
                    let range = self.range.get(cx);
                    let min = range.start;
                    let max = range.end;
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
                        let range = self.range.get(cx);
                        let min = range.start;
                        let max = range.end;
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

    /// Enables two-way binding: slider changes automatically update the bound signal.
    ///
    /// This is a convenience method equivalent to:
    /// ```ignore
    /// .on_change(move |cx, val| signal.set(cx, val))
    /// ```
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// let value = cx.state(0.5f32);
    /// Slider::new(cx, value).two_way();
    /// ```
    pub fn two_way(self) -> Self {
        self.modify(|slider| {
            let signal = slider.value;
            slider.on_change = Some(Box::new(move |cx, val| signal.set(cx, val)));
        })
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
        self.modify2(|slider, cx| {
            slider.range.set(cx, range.clone());
            slider.range_cache = range;
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
    pub fn orientation(mut self, orientation: Signal<Orientation>) -> Self {
        let is_vertical = self.context().derived({
            let orientation = orientation;
            move |store| *orientation.get(store) == Orientation::Vertical
        });
        self.bind(orientation, |handle, orientation| {
            let orientation = *orientation.get(&handle);
            handle.modify2(|slider, cx| {
                slider.orientation.set(cx, orientation);
            });
        })
        .toggle_class("vertical", is_vertical)
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
