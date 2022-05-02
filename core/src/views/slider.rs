use std::{marker::PhantomData, ops::Range};

use crate::prelude::*;
use crate::views::Orientation;

#[derive(Debug)]
enum SliderEventInternal {
    SetThumbSize(f32, f32),
    SetRange(Range<f32>),
}

#[derive(Clone, Debug, Default, Data)]
pub struct SliderDataInternal {
    pub orientation: Orientation,
    pub size: f32,
    pub thumb_size: f32,
    pub range: Range<f32>,
}

/// The slider control can be used to select from a continuous set of values.
///
/// The slider control consists of three main parts, a **thumb** element which can be moved between the extremes of a linear **track**,
/// and an **active** element which fills the slider to indicate the current value.
///
/// The slider orientation is determined by its dimensions. If the slider width is greater than the height then the thumb
/// moves horizontally, whereas if the slider height is greater than the width the thumb moves vertically.
///
/// # Examples
///
/// ## Basic Slider
/// In the following example, a slider is bound to a value. The `on_changing` callback is used to send an event to mutate the
/// bound value when the slider thumb is moved, or if the track is clicked on.
/// ```
/// # use vizia_core::prelude::*;
/// # use vizia_derive::*;
/// # let mut cx = &mut Context::new();
/// # #[derive(Lens, Default)]
/// # pub struct AppData {
/// #     value: f32,
/// # }
/// # impl Model for AppData {}
/// # AppData::default().build(cx);
/// Slider::new(cx, AppData::value)
///     .on_changing(|cx, value| {
///         println!("Slider on_changing: {}", value);
///     });
/// ```
///
/// ## Slider with Label
/// ```
/// # use vizia_core::prelude::*;
/// # use vizia_derive::*;
/// # let mut cx = &mut Context::new();
/// # #[derive(Lens, Default)]
/// # pub struct AppData {
/// #     value: f32,
/// # }
/// # impl Model for AppData {}
/// # AppData::default().build(cx);
/// HStack::new(cx, |cx|{
///     Slider::new(cx, AppData::value)
///         .on_changing(|cx, value| {
///             println!("Slider on_changing: {}", value);
///         });
///     Label::new(cx, AppData::value.map(|val| format!("{:.2}", val)));
/// });
/// ```
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
    /// Creates a new slider bound to the value targeted by the lens.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_derive::*;
    /// # let mut cx = &mut Context::new();
    /// # #[derive(Lens, Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # AppData::default().build(cx);
    /// Slider::new(cx, AppData::value)
    ///     .on_changing(|cx, value| {
    ///         println!("Slider on_changing: {}", value);
    ///     });
    /// ```
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self {
            p: PhantomData::default(),
            is_dragging: false,

            internal: SliderDataInternal {
                orientation: Orientation::Horizontal,
                thumb_size: 0.0,
                size: 0.0,
                range: 0.0..1.0,
            },

            on_changing: None,
        }
        .build(cx, move |cx| {
            Binding::new(cx, Slider::<L>::internal, move |cx, slider_data| {
                let lens = lens.clone();
                ZStack::new(cx, move |cx| {
                    let slider_data = slider_data.get(cx);
                    let thumb_size = slider_data.thumb_size;
                    let orientation = slider_data.orientation;
                    let size = slider_data.size;
                    let range = slider_data.range;

                    // Active track
                    Element::new(cx).class("active").bind(lens.clone(), move |handle, value| {
                        let val = value.get(handle.cx);
                        let normal_val = (val - range.start) / (range.end - range.start);
                        let min = thumb_size / size;
                        let max = 1.0;
                        let dx = min + normal_val * (max - min);

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
                            let normal_val = (val - range.start) / (range.end - range.start);
                            let px = normal_val * (1.0 - (thumb_size / size));
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
                });
            });
        })
    }
}

impl<L: Lens> View for Slider<L> {
    fn element(&self) -> Option<&'static str> {
        Some("slider")
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|slider_event_internal, _| match slider_event_internal {
            SliderEventInternal::SetThumbSize(width, height) => match self.internal.orientation {
                Orientation::Horizontal => {
                    self.internal.thumb_size = *width;
                }

                Orientation::Vertical => {
                    self.internal.thumb_size = *height;
                }
            },

            SliderEventInternal::SetRange(range) => {
                self.internal.range = range.clone();
            }
        });

        event.map(|window_event, _| match window_event {
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
                let min = self.internal.range.start;
                let max = self.internal.range.end;

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

                let val = min + dx * (max - min);

                if let Some(callback) = self.on_changing.take() {
                    (callback)(cx, val);

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

                    let min = self.internal.range.start;
                    let max = self.internal.range.end;

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

                    let val = min + dx * (max - min);

                    if let Some(callback) = &self.on_changing {
                        (callback)(cx, val);
                    }
                }
            }

            _ => {}
        });
    }
}

impl<L: Lens> Handle<'_, Slider<L>> {
    /// Sets the callback triggered when the slider value is changing (dragging).
    ///
    /// Takes a closure which triggers when the slider value is changing,
    /// either by pressing the track or dragging the thumb along the track.
    ///
    /// # Example
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_derive::*;
    /// # let mut cx = &mut Context::new();
    /// # #[derive(Lens, Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # AppData::default().build(cx);
    /// Slider::new(cx, AppData::value)
    ///     .on_changing(|cx, value| {
    ///         println!("Slider on_changing: {}", value);
    ///     });
    /// ```
    pub fn on_changing<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context, f32),
    {
        self.modify(|slider| slider.on_changing = Some(Box::new(callback)))
    }

    /// Sets the range of the slider.
    ///
    /// If the bound data is outside of the range then the slider will clip to min/max of the range.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_derive::*;
    /// # let mut cx = &mut Context::new();
    /// # #[derive(Lens, Default)]
    /// # pub struct AppData {
    /// #     value: f32,
    /// # }
    /// # impl Model for AppData {}
    /// # AppData::default().build(cx);
    /// Slider::new(cx, AppData::value)
    ///     .range(-20.0..50.0)
    ///     .on_changing(|cx, value| {
    ///         println!("Slider on_changing: {}", value);
    ///     });
    /// ```
    pub fn range(self, range: Range<f32>) -> Self {
        self.entity.emit(self.cx, SliderEventInternal::SetRange(range));

        self
    }
}
