use std::ops::Range;

use accesskit::ActionData;

use crate::prelude::*;

/// Internal data used by the slider.
#[derive(Clone, Debug, Default, Data)]
pub struct SliderDataInternal {
    /// The orientation of the slider.
    pub orientation: Orientation,
    /// The range of the slider.
    pub range: Range<f32>,
    /// The step of the slider.
    pub step: f32,
    /// How much the slider should change in response to keyboard events.
    pub keyboard_fraction: f32,
}

/// The slider control can be used to select from a continuous set of values.
///
/// The slider control consists of three main parts, a **thumb** element which can be moved between the extremes of a linear **track**,
/// and an **active** element which fills the slider to indicate the current value.
///
/// # Examples
///
/// ## Basic Slider
/// In the following example, a slider is bound to a value. The `on_change` callback is used to send an event to mutate the
/// bound value when the slider thumb is moved, or if the track is clicked on.
/// ```
/// # use vizia_core::prelude::*;
/// # use vizia_derive::*;
/// # let mut cx = &mut Context::default();
/// # #[derive(Lens, Default)]
/// # pub struct AppData {
/// #     value: f32,
/// # }
/// # impl Model for AppData {}
/// # AppData::default().build(cx);
/// Slider::new(cx, AppData::value)
///     .on_change(|cx, value| {
///         debug!("Slider on_change: {}", value);
///     });
/// ```
///
/// ## Slider with Label
/// ```
/// # use vizia_core::prelude::*;
/// # use vizia_derive::*;
/// # let mut cx = &mut Context::default();
/// # #[derive(Lens, Default)]
/// # pub struct AppData {
/// #     value: f32,
/// # }
/// # impl Model for AppData {}
/// # AppData::default().build(cx);
/// HStack::new(cx, |cx|{
///     Slider::new(cx, AppData::value)
///         .on_change(|cx, value| {
///             debug!("Slider on_change: {}", value);
///         });
///     Label::new(cx, AppData::value.map(|val| format!("{:.2}", val)));
/// });
/// ```
#[derive(Lens)]
pub struct Slider<L: Lens> {
    lens: L,
    is_dragging: bool,
    internal: SliderDataInternal,
    on_change: Option<Box<dyn Fn(&mut EventContext, f32)>>,
}

impl<L> Slider<L>
where
    L: Lens<Target = f32>,
{
    /// Creates a new slider bound to the value targeted by the lens.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_derive::*;
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Lens, Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # AppData::default().build(cx);
    /// Slider::new(cx, AppData::value)
    ///     .on_change(|cx, value| {
    ///         debug!("Slider on_change: {}", value);
    ///     });
    /// ```
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self {
            lens,
            is_dragging: false,

            internal: SliderDataInternal {
                orientation: Orientation::Horizontal,
                range: 0.0..1.0,
                step: 0.01,
                keyboard_fraction: 0.1,
            },

            on_change: None,
        }
        .build(cx, move |cx| {
            Binding::new(cx, Slider::<L>::internal, move |cx, slider_data| {
                // Track
                HStack::new(cx, move |cx| {
                    let slider_data = slider_data.get(cx);
                    let orientation = slider_data.orientation;
                    let range = slider_data.range;

                    // Active track
                    VStack::new(cx, |cx| {
                        // Thumb
                        Element::new(cx).class("thumb").bind(lens, move |handle, value| {
                            let val = value.get(&handle).clamp(range.start, range.end);
                            let normal_val = (val - range.start) / (range.end - range.start);
                            if orientation == Orientation::Horizontal {
                                handle.translate((
                                    Percentage(100.0 * (1.0 - normal_val)),
                                    Pixels(0.0),
                                ));
                            } else {
                                handle.translate((
                                    Pixels(0.0),
                                    Percentage(-100.0 * (1.0 - normal_val)),
                                ));
                            }
                        });
                    })
                    .class("active")
                    .bind(lens, move |handle, value| {
                        let val = value.get(&handle).clamp(range.start, range.end);
                        let normal_val = (val - range.start) / (range.end - range.start);

                        if orientation == Orientation::Horizontal {
                            handle
                                .height(Stretch(1.0))
                                .width(Percentage(normal_val * 100.0))
                                .layout_type(LayoutType::Row)
                                .alignment(Alignment::Right);
                        } else {
                            handle
                                .width(Stretch(1.0))
                                .height(Percentage(normal_val * 100.0))
                                .layout_type(LayoutType::Column)
                                .alignment(Alignment::TopCenter);
                        }
                    });
                })
                .class("track");
            });
        })
        .toggle_class(
            "vertical",
            Self::internal.map(|slider_data| slider_data.orientation == Orientation::Vertical),
        )
        .role(Role::Slider)
        .numeric_value(lens.map(|val| (*val as f64 * 100.0).round() / 100.0))
        .text_value(lens.map(|val| {
            let v = (*val as f64 * 100.0).round() / 100.0;
            format!("{}", v)
        }))
        .navigable(true)
    }
}

impl<L: Lens<Target = f32>> View for Slider<L> {
    fn element(&self) -> Option<&'static str> {
        Some("slider")
    }

    fn accessibility(&self, _cx: &mut AccessContext, node: &mut AccessNode) {
        node.set_numeric_value_step(self.internal.step as f64);
        node.set_min_numeric_value(self.internal.range.start as f64);
        node.set_max_numeric_value(self.internal.range.end as f64);
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
                    let thumb_size = match self.internal.orientation {
                        Orientation::Horizontal => cx.cache.get_width(thumb),
                        Orientation::Vertical => cx.cache.get_height(thumb),
                    };
                    let min = self.internal.range.start;
                    let max = self.internal.range.end;
                    let step = self.internal.step;

                    let current = cx.current();
                    let width = cx.cache.get_width(current);
                    let height = cx.cache.get_height(current);
                    let posx = cx.cache.get_posx(current);
                    let posy = cx.cache.get_posy(current);

                    let mut dx = match self.internal.orientation {
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
                    let thumb_size = match self.internal.orientation {
                        Orientation::Horizontal => cx.cache.get_width(thumb),
                        Orientation::Vertical => cx.cache.get_height(thumb),
                    };

                    let min = self.internal.range.start;
                    let max = self.internal.range.end;
                    let step = self.internal.step;

                    let current = cx.current();
                    let width = cx.cache.get_width(current);
                    let height = cx.cache.get_height(current);
                    let posx = cx.cache.get_posx(current);
                    let posy = cx.cache.get_posy(current);

                    let mut dx = match self.internal.orientation {
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
                let min = self.internal.range.start;
                let max = self.internal.range.end;
                let step = self.internal.step;
                let mut val = self.lens.get(cx) + step;
                val = val.clamp(min, max);
                if let Some(callback) = &self.on_change {
                    (callback)(cx, val);
                }
            }

            WindowEvent::KeyDown(Code::ArrowDown | Code::ArrowLeft, _) => {
                let min = self.internal.range.start;
                let max = self.internal.range.end;
                let step = self.internal.step;
                let mut val = self.lens.get(cx) - step;
                val = val.clamp(min, max);
                if let Some(callback) = &self.on_change {
                    (callback)(cx, val);
                }
            }

            WindowEvent::ActionRequest(action) => match action.action {
                Action::Increment => {
                    let min = self.internal.range.start;
                    let max = self.internal.range.end;
                    let step = self.internal.step;
                    let mut val = self.lens.get(cx) + step;
                    val = step * (val / step).ceil();
                    val = val.clamp(min, max);
                    if let Some(callback) = &self.on_change {
                        (callback)(cx, val);
                    }
                }

                Action::Decrement => {
                    let min = self.internal.range.start;
                    let max = self.internal.range.end;
                    let step = self.internal.step;
                    let mut val = self.lens.get(cx) - step;
                    val = step * (val / step).ceil();
                    val = val.clamp(min, max);
                    if let Some(callback) = &self.on_change {
                        (callback)(cx, val);
                    }
                }

                Action::SetValue => {
                    if let Some(ActionData::NumericValue(val)) = action.data {
                        let min = self.internal.range.start;
                        let max = self.internal.range.end;
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

impl<L: Lens> Handle<'_, Slider<L>> {
    /// Sets the callback triggered when the slider value is changed.
    ///
    /// Takes a closure which triggers when the slider value is changed,
    /// either by pressing the track or dragging the thumb along the track.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_derive::*;
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Lens, Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # AppData::default().build(cx);
    /// Slider::new(cx, AppData::value)
    ///     .on_change(|cx, value| {
    ///         debug!("Slider on_change: {}", value);
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
    /// # use vizia_derive::*;
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Lens, Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # AppData::default().build(cx);
    /// Slider::new(cx, AppData::value)
    ///     .range(-20.0..50.0)
    ///     .on_change(|cx, value| {
    ///         debug!("Slider on_change: {}", value);
    ///     });
    /// ```
    pub fn range<U: Into<Range<f32>>>(self, range: impl Res<U>) -> Self {
        self.bind(range, |handle, range| {
            let range = range.get(&handle).into();
            handle.modify(|slider| {
                slider.internal.range = range;
            });
        })
    }

    /// Sets the orientation of the slider.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_derive::*;
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Lens, Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # AppData::default().build(cx);
    /// Slider::new(cx, AppData::value)
    ///     .orientation(Orientation::Vertical)
    ///     .on_change(|cx, value| {
    ///         debug!("Slider on_change: {}", value);
    ///     });
    /// ```
    pub fn orientation<U: Into<Orientation>>(self, orientation: impl Res<U>) -> Self {
        self.bind(orientation, |handle, orientation| {
            let orientation = orientation.get(&handle).into();
            handle.modify(|slider: &mut Slider<L>| {
                slider.internal.orientation = orientation;
            });
        })
    }

    /// Set the step value for the slider.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_derive::*;
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Lens, Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # AppData::default().build(cx);
    /// Slider::new(cx, AppData::value)
    ///     .step(0.1)
    ///     .on_change(|cx, value| {
    ///         debug!("Slider on_change: {}", value);
    ///     });
    /// ```
    pub fn step<U: Into<f32>>(self, step: impl Res<U>) -> Self {
        self.bind(step, |handle, step| {
            let step = step.get(&handle).into();
            handle.modify(|slider| {
                slider.internal.step = step;
            });
        })
    }

    /// Sets the fraction of a slider that a press of an arrow key will change.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_derive::*;
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Lens, Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # AppData::default().build(cx);
    /// Slider::new(cx, AppData::value)
    ///     .keyboard_fraction(0.05)
    ///     .on_change(|cx, value| {
    ///         debug!("Slider on_change: {}", value);
    ///     });
    /// ```
    pub fn keyboard_fraction<U: Into<f32>>(self, keyboard_fraction: impl Res<U>) -> Self {
        self.bind(keyboard_fraction, |handle, keyboard_fraction| {
            let keyboard_fraction = keyboard_fraction.get(&handle).into();
            handle.modify(|slider| {
                slider.internal.keyboard_fraction = keyboard_fraction;
            });
        })
    }
}
