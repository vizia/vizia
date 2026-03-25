use std::ops::Range;

use crate::prelude::*;
use accesskit::ActionData;

/// Internal data used by the slider.
#[derive(Clone, Debug, Default)]
pub struct SliderDataInternal {}

/// The slider control can be used to select from a continuous set of values.
///
/// The slider control consists of three main parts, a **thumb** element which can be moved between the extremes of a linear **track**,
/// and an **active** element which fills the slider to indicate the current value.
///
/// # Examples
///
/// ## Basic Slider
/// In the following example, a slider reads from a value source. The `on_change` callback is used
/// to update that value when the slider thumb is moved, or if the track is clicked on.
/// ```
/// # use vizia_core::prelude::*;
///
/// # let mut cx = &mut Context::default();
/// # #[derive(Default)]
/// # pub struct AppData {
/// #     value: f32,
/// # }
/// # impl Model for AppData {}
/// # let value = Signal::new(0.5);
/// Slider::new(cx, value)
///     .on_change(|cx, value| {
///         let _ = (cx, value);
///     });
/// ```
///
/// ## Slider with Label
/// ```
/// # use vizia_core::prelude::*;
///
/// # let mut cx = &mut Context::default();
/// # #[derive(Default)]
/// # pub struct AppData {
/// #     value: f32,
/// # }
/// # impl Model for AppData {}
/// # let value = Signal::new(0.5);
/// HStack::new(cx, |cx|{
///     Slider::new(cx, value)
///         .on_change(|cx, value| {
///             let _ = (cx, value);
///         });
///     Label::new(cx, value.map(|val| format!("{:.2}", val)));
/// });
/// ```
pub struct Slider<S> {
    value: S,
    is_dragging: bool,
    /// The orientation of the slider.
    pub orientation: Signal<Orientation>,
    /// The range of the slider.
    pub range: Signal<Range<f32>>,
    /// The step of the slider.
    pub step: Signal<f32>,
    /// How much the slider should change in response to keyboard events.
    pub keyboard_fraction: Signal<f32>,
    on_change: Option<Box<dyn Fn(&mut EventContext, f32)>>,
}

impl<S> Slider<S>
where
    S: SignalGet<f32> + SignalMap<f32> + Copy + 'static,
{
    /// Creates a new slider from the provided value source.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    ///
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # let value = Signal::new(0.5);
    /// Slider::new(cx, value)
    ///     .on_change(|cx, value| {
    ///         let _ = (cx, value);
    ///     });
    /// ```
    pub fn new(cx: &mut Context, value: S) -> Handle<Self> {
        let range = Signal::new(0.0..1.0);
        let orientation = Signal::new(Orientation::Horizontal);
        let step = Signal::new(0.01);

        Self {
            value,
            is_dragging: false,
            orientation,
            range,
            step,
            keyboard_fraction: Signal::new(0.1),
            on_change: None,
        }
        .build(cx, move |cx| {
            // Track
            HStack::new(cx, move |cx| {
                let active_normalized = Memo::new(move |_| {
                    let active_range = range.get();
                    let val = value.get().clamp(active_range.start, active_range.end);
                    (val - active_range.start) / (active_range.end - active_range.start)
                });

                let active_width = Memo::new(move |_| {
                    let normal_val = active_normalized.get();
                    if orientation.get() == Orientation::Horizontal {
                        Percentage(normal_val * 100.0)
                    } else {
                        Stretch(1.0)
                    }
                });

                let active_height = Memo::new(move |_| {
                    let normal_val = active_normalized.get();
                    if orientation.get() == Orientation::Horizontal {
                        Stretch(1.0)
                    } else {
                        Percentage(normal_val * 100.0)
                    }
                });

                // Active track
                VStack::new(cx, move |cx| {
                    let thumb_translate: Memo<Translate> = Memo::new(move |_| {
                        let thumb_range = range.get();
                        let val = value.get().clamp(thumb_range.start, thumb_range.end);
                        let normal_val =
                            (val - thumb_range.start) / (thumb_range.end - thumb_range.start);

                        if orientation.get() == Orientation::Horizontal {
                            (Percentage(100.0 * (1.0 - normal_val)), Pixels(0.0)).into()
                        } else {
                            (Pixels(0.0), Percentage(-100.0 * (1.0 - normal_val))).into()
                        }
                    });

                    // Thumb
                    Element::new(cx).class("thumb").translate(thumb_translate);
                })
                .class("active")
                .width(active_width)
                .height(active_height)
                .layout_type(orientation.map(|o| {
                    if *o == Orientation::Horizontal { LayoutType::Row } else { LayoutType::Column }
                }))
                .alignment(orientation.map(|o| {
                    if *o == Orientation::Horizontal {
                        Alignment::Right
                    } else {
                        Alignment::TopCenter
                    }
                }));
            })
            .class("track");
        })
        .toggle_class("vertical", orientation.map(|o| *o == Orientation::Vertical))
        .role(Role::Slider)
        .numeric_value(value.map(|v| (*v as f64 * 100.0).round() / 100.0))
        .text_value(value.map(|v| format!("{}", (*v as f64 * 100.0).round() / 100.0)))
        .navigable(true)
    }
}

impl<S> View for Slider<S>
where
    S: SignalGet<f32> + 'static,
{
    fn element(&self) -> Option<&'static str> {
        Some("slider")
    }

    fn accessibility(&self, _cx: &mut AccessContext, node: &mut AccessNode) {
        node.set_numeric_value_step(self.step.get() as f64);
        node.set_min_numeric_value(self.range.get().start as f64);
        node.set_max_numeric_value(self.range.get().end as f64);
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
                    let thumb_size = match self.orientation.get() {
                        Orientation::Horizontal => cx.cache.get_width(thumb),
                        Orientation::Vertical => cx.cache.get_height(thumb),
                    };
                    let min = self.range.get().start;
                    let max = self.range.get().end;
                    let step = self.step.get();

                    let current = cx.current();
                    let width = cx.cache.get_width(current);
                    let height = cx.cache.get_height(current);
                    let posx = cx.cache.get_posx(current);
                    let posy = cx.cache.get_posy(current);

                    let mut dx = match self.orientation.get() {
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
                    let thumb_size = match self.orientation.get() {
                        Orientation::Horizontal => cx.cache.get_width(thumb),
                        Orientation::Vertical => cx.cache.get_height(thumb),
                    };

                    let min = self.range.get().start;
                    let max = self.range.get().end;
                    let step = self.step.get();

                    let current = cx.current();
                    let width = cx.cache.get_width(current);
                    let height = cx.cache.get_height(current);
                    let posx = cx.cache.get_posx(current);
                    let posy = cx.cache.get_posy(current);

                    let mut dx = match self.orientation.get() {
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
                let min = self.range.get().start;
                let max = self.range.get().end;
                let step = self.step.get();
                let mut val = self.value.get() + step;
                val = val.clamp(min, max);
                if let Some(callback) = &self.on_change {
                    (callback)(cx, val);
                }
            }

            WindowEvent::KeyDown(Code::ArrowDown | Code::ArrowLeft, _) => {
                let min = self.range.get().start;
                let max = self.range.get().end;
                let step = self.step.get();
                let mut val = self.value.get() - step;
                val = val.clamp(min, max);
                if let Some(callback) = &self.on_change {
                    (callback)(cx, val);
                }
            }

            WindowEvent::ActionRequest(action) => match action.action {
                Action::Increment => {
                    let min = self.range.get().start;
                    let max = self.range.get().end;
                    let step = self.step.get();
                    let mut val = self.value.get() + step;
                    val = step * (val / step).ceil();
                    val = val.clamp(min, max);
                    if let Some(callback) = &self.on_change {
                        (callback)(cx, val);
                    }
                }

                Action::Decrement => {
                    let min = self.range.get().start;
                    let max = self.range.get().end;
                    let step = self.step.get();
                    let mut val = self.value.get() - step;
                    val = step * (val / step).ceil();
                    val = val.clamp(min, max);
                    if let Some(callback) = &self.on_change {
                        (callback)(cx, val);
                    }
                }

                Action::SetValue => {
                    if let Some(ActionData::NumericValue(val)) = action.data {
                        let min = self.range.get().start;
                        let max = self.range.get().end;
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

impl<S> Handle<'_, Slider<S>>
where
    S: SignalGet<f32> + 'static,
{
    /// Sets the callback triggered when the slider value is changed.
    ///
    /// Takes a closure which triggers when the slider value is changed,
    /// either by pressing the track or dragging the thumb along the track.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    ///
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # let value = Signal::new(0.5);
    /// Slider::new(cx, value)
    ///     .on_change(|cx, value| {
    ///         let _ = (cx, value);
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
    /// If the source value is outside of the range then the slider will clip to min/max of the range.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    ///
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # let value = Signal::new(0.5);
    /// Slider::new(cx, value)
    ///     .range(-20.0..50.0)
    ///     .on_change(|cx, value| {
    ///         let _ = (cx, value);
    ///     });
    /// ```
    pub fn range<U: Into<Range<f32>> + Clone + 'static>(
        self,
        range: impl Res<U> + 'static,
    ) -> Self {
        let range = range.to_signal(self.cx);
        self.bind(range, move |handle| {
            let range = range.get();
            let range = range.into();
            handle.modify(|slider| {
                slider.range.set(range);
            });
        })
    }

    /// Sets the orientation of the slider.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    ///
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # let value = Signal::new(0.5);
    /// Slider::new(cx, value)
    ///     .orientation(Orientation::Vertical)
    ///     .on_change(|cx, value| {
    ///         let _ = (cx, value);
    ///     });
    /// ```
    pub fn orientation<U: Into<Orientation> + Clone + 'static>(
        self,
        orientation: impl Res<U> + 'static,
    ) -> Self {
        let orientation = orientation.to_signal(self.cx);
        self.bind(orientation, move |handle| {
            let orientation = orientation.get();
            let orientation = orientation.into();
            handle.modify(|slider| {
                slider.orientation.set(orientation);
            });
        })
    }

    /// Set the step value for the slider.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    ///
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # let value = Signal::new(0.5);
    /// Slider::new(cx, value)
    ///     .step(0.1_f32)
    ///     .on_change(|cx, value| {
    ///         let _ = (cx, value);
    ///     });
    /// ```
    pub fn step<U: Into<f32> + Clone + 'static>(self, step: impl Res<U> + 'static) -> Self {
        let step = step.to_signal(self.cx);
        self.bind(step, move |handle| {
            let step = step.get();
            let step = step.into();
            handle.modify(|slider| {
                slider.step.set(step);
            });
        })
    }

    /// Sets the fraction of a slider that a press of an arrow key will change.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    ///
    /// # let mut cx = &mut Context::default();
    /// # #[derive(Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # let value = Signal::new(0.5);
    /// Slider::new(cx, value)
    ///     .keyboard_fraction(0.05_f32)
    ///     .on_change(|cx, value| {
    ///         let _ = (cx, value);
    ///     });
    /// ```
    pub fn keyboard_fraction<U: Into<f32> + Clone + 'static>(
        self,
        keyboard_fraction: impl Res<U> + 'static,
    ) -> Self {
        let keyboard_fraction = keyboard_fraction.to_signal(self.cx);
        self.bind(keyboard_fraction, move |handle| {
            let keyboard_fraction = keyboard_fraction.get();
            let keyboard_fraction = keyboard_fraction.into();
            handle.modify(|slider| {
                slider.keyboard_fraction.set(keyboard_fraction);
            });
        })
    }
}
