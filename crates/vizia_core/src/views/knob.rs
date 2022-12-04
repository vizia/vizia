use femtovg::{LineCap, Paint, Path, Solidity};
use morphorm::Hierarchy;

use crate::prelude::*;

static DEFAULT_DRAG_SCALAR: f32 = 0.0042;
static DEFAULT_WHEEL_SCALAR: f32 = 0.005;
static DEFAULT_ARROW_SCALAR: f32 = 0.1;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.04;

use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, Default, Data, PartialEq)]
pub enum KnobType {
    #[default]
    Arc,
    Tick,
}

#[derive(Lens)]
pub struct Knob<L: Lens<Target = D>, D: Copy + Clone + Into<f32> + 'static> {
    lens: L,
    is_dragging: bool,
    prev_drag_y: f32,
    cursor: D,
    default_normal: D,
    discrete_steps: u8,

    drag_scalar: f32,
    wheel_scalar: D,
    arrow_scalar: D,
    modifier_scalar: f32,

    knob_type: KnobType,

    on_changing_continuous: Option<Box<dyn Fn(&mut EventContext, f32)>>,
    on_changing_discrete: Option<Box<dyn Fn(&mut EventContext, u8)>>,
}

// Continuous Knob
impl<L> Knob<L, f32>
where
    L: Lens<Target = f32>,
{
    pub fn construct(cx: &mut Context, lens: L, normalized_default: impl Res<f32>) -> Self {
        Self {
            lens: lens.clone(),
            is_dragging: false,
            prev_drag_y: 0.0,
            cursor: lens.get(cx),
            default_normal: normalized_default.get_val(cx),
            discrete_steps: 0,

            drag_scalar: DEFAULT_DRAG_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            arrow_scalar: DEFAULT_ARROW_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,

            knob_type: KnobType::Arc,

            on_changing_continuous: None,
            on_changing_discrete: None,
        }
    }

    pub fn new(
        cx: &mut Context,
        lens: L,
        normalized_default: impl Res<f32>,
        arc_length: f32,
        arc_offset: f32,
        centered: bool,
    ) -> Handle<Self> {
        Self::construct(cx, lens, normalized_default)
            .build(cx, move |cx| {
                ZStack::new(cx, move |cx| {
                    ArcTrack::new(cx, centered, arc_length, arc_offset)
                        .value(lens)
                        .class("knob-track");

                    HStack::new(cx, |cx| {
                        Element::new(cx).class("knob-head-tick");
                    })
                    .rotate(lens.clone().map(|v| *v * 300.0 - 150.0))
                    .class("knob-head");
                });
            })
            .navigable(true)
    }

    pub fn custom<F, V: View>(
        cx: &mut Context,
        lens: L,
        normalized_default: impl Res<f32>,
        content: F,
    ) -> Handle<'_, Self>
    where
        F: 'static + Fn(&mut Context, L) -> Handle<V>,
    {
        Self::construct(cx, lens, normalized_default)
            .build(cx, move |cx| {
                ZStack::new(cx, move |cx| {
                    (content)(cx, lens).width(Percentage(100.0)).height(Percentage(100.0));
                });
            })
            .navigable(true)
    }
}

// Discrete Knob
impl<L> Knob<L, u8>
where
    L: Lens<Target = u8>,
{
    pub fn construct_discrete(
        cx: &mut Context,
        lens: L,
        normalized_default: impl Res<u8>,
        discrete_steps: u8,
    ) -> Self {
        Self {
            lens: lens.clone(),
            is_dragging: false,
            prev_drag_y: 0.0,
            cursor: lens.get(cx),
            default_normal: normalized_default.get_val(cx),
            discrete_steps,

            drag_scalar: DEFAULT_DRAG_SCALAR,
            wheel_scalar: 1,
            arrow_scalar: 1,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,

            knob_type: KnobType::Tick,

            on_changing_continuous: None,
            on_changing_discrete: None,
        }
    }

    pub fn new_discrete(
        cx: &mut Context,
        lens: L,
        normalized_default: impl Res<u8>,
        arc_length: f32,
        arc_offset: f32,
        discrete_steps: u8,
        centered: bool,
    ) -> Handle<Self> {
        Self::construct_discrete(cx, lens, normalized_default, discrete_steps)
            .build(cx, move |cx| {
                ZStack::new(cx, move |cx| {
                    Binding::new(cx, Knob::<L, u8>::knob_type, move |cx, knob_type| {
                        let knob_type = knob_type.get(cx);
                        match knob_type {
                            KnobType::Arc => {
                                ArcTrack::new(cx, centered, arc_length, arc_offset)
                                    .value(
                                        lens.map(move |v| {
                                            *v as f32 / (discrete_steps as f32 - 1.0)
                                        }),
                                    )
                                    .class("knob-track");
                            }
                            KnobType::Tick => {
                                Ticks::new(cx, arc_length, arc_offset, discrete_steps).value(lens);
                            }
                        }
                        let head = HStack::new(cx, |cx| {
                            Element::new(cx).class("knob-head-tick");
                        })
                        .rotate(lens.clone().map(move |v| {
                            (*v as f32 / (discrete_steps as f32 - 1.0)) * arc_length + arc_offset
                                - arc_length / 2.0
                        }))
                        .class("knob-head");

                        if let KnobType::Tick = knob_type {
                            head.class("knob-discrete");
                        }
                    });
                });
            })
            .navigable(true)
    }

    pub fn custom_discrete<F, V: View>(
        cx: &mut Context,
        lens: L,
        normalized_default: impl Res<u8>,
        discrete_steps: u8,
        content: F,
    ) -> Handle<'_, Self>
    where
        F: 'static + Fn(&mut Context, L) -> Handle<V>,
    {
        Self::construct_discrete(cx, lens, normalized_default, discrete_steps).build(
            cx,
            move |cx| {
                ZStack::new(cx, move |cx| {
                    (content)(cx, lens).width(Percentage(100.0)).height(Percentage(100.0));
                });
            },
        )
    }
}

impl<'a, L> Handle<'a, Knob<L, f32>>
where
    L: Lens<Target = f32>,
{
    pub fn on_changing<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, f32),
    {
        if let Some(view) = self.cx.views.get_mut(&self.entity) {
            if let Some(knob) = view.downcast_mut::<Knob<L, f32>>() {
                knob.on_changing_continuous = Some(Box::new(callback));
            }
        }

        self
    }
}

impl<L> View for Knob<L, f32>
where
    L: Lens<Target = f32>,
{
    fn element(&self) -> Option<&'static str> {
        Some("knob")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        let move_virtual_slider = |self_ref: &mut Self, cx: &mut EventContext, new_normal: f32| {
            self_ref.cursor = new_normal.clamp(0.0, 1.0);

            if let Some(callback) = &self_ref.on_changing_continuous {
                (callback)(cx, self_ref.cursor);
            }
        };

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                self.is_dragging = true;
                self.prev_drag_y = cx.mouse.left.pos_down.1;

                cx.capture();
                cx.focus_with_visibility(false);

                self.cursor = self.lens.get(cx);
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                self.is_dragging = false;

                self.cursor = self.lens.get(cx);

                cx.release();
            }

            WindowEvent::MouseMove(_, y) => {
                if self.is_dragging {
                    let mut delta_normal = (*y - self.prev_drag_y) * self.drag_scalar;

                    self.prev_drag_y = *y;

                    if cx.modifiers.contains(Modifiers::SHIFT) {
                        delta_normal *= self.modifier_scalar;
                    }

                    let new_normal = self.cursor - delta_normal;

                    move_virtual_slider(self, cx, new_normal);
                }
            }

            WindowEvent::MouseScroll(_, y) => {
                if *y != 0.0 {
                    let delta_normal = -*y * self.wheel_scalar;

                    let new_normal = self.cursor - delta_normal;

                    move_virtual_slider(self, cx, new_normal);
                }
            }

            WindowEvent::MouseDoubleClick(button) if *button == MouseButton::Left => {
                self.is_dragging = false;

                move_virtual_slider(self, cx, self.default_normal.into());
            }

            WindowEvent::KeyDown(Code::ArrowUp | Code::ArrowRight, _) => {
                self.cursor = self.lens.get(cx).into();
                move_virtual_slider(self, cx, self.cursor + self.arrow_scalar);
            }

            WindowEvent::KeyDown(Code::ArrowDown | Code::ArrowLeft, _) => {
                self.cursor = self.lens.get(cx).into();
                move_virtual_slider(self, cx, self.cursor - self.arrow_scalar);
            }

            _ => {}
        });
    }
}

impl<L> View for Knob<L, u8>
where
    L: Lens<Target = u8>,
{
    fn element(&self) -> Option<&'static str> {
        Some("knob")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        let move_virtual_slider = |self_ref: &mut Self, cx: &mut EventContext, new_normal: u8| {
            self_ref.cursor = new_normal.clamp(0, self_ref.discrete_steps - 1);

            if let Some(callback) = &self_ref.on_changing_discrete {
                (callback)(cx, self_ref.cursor);
            }
        };

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                self.is_dragging = true;
                self.prev_drag_y = cx.mouse.left.pos_down.1;

                cx.capture();
                cx.focus_with_visibility(false);
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                self.is_dragging = false;

                cx.release();
            }

            WindowEvent::MouseMove(_, y) => {
                if self.is_dragging {
                    let mut delta_normal = (*y - self.prev_drag_y) * self.drag_scalar * 4.0;

                    if cx.modifiers.contains(Modifiers::SHIFT) {
                        delta_normal *= self.modifier_scalar;
                    }

                    if delta_normal >= 1.0 {
                        self.prev_drag_y = *y;
                        if self.cursor > 0 {
                            let new_normal = self.cursor - delta_normal as u8;
                            move_virtual_slider(self, cx, new_normal);
                        }
                    } else if delta_normal <= -1.0 {
                        self.prev_drag_y = *y;
                        let new_normal = self.cursor + delta_normal.abs() as u8;
                        move_virtual_slider(self, cx, new_normal);
                    }
                }
            }

            WindowEvent::MouseScroll(_, y) => {
                if *y != 0.0 {
                    let new_normal = if *y > 0.0 {
                        self.cursor + self.wheel_scalar
                    } else {
                        if self.cursor > 0 {
                            self.cursor - self.wheel_scalar
                        } else {
                            self.cursor
                        }
                    };
                    move_virtual_slider(self, cx, new_normal);
                }
            }

            WindowEvent::MouseDoubleClick(button) if *button == MouseButton::Left => {
                self.is_dragging = false;

                move_virtual_slider(self, cx, self.default_normal.into());
            }

            WindowEvent::KeyDown(Code::ArrowUp | Code::ArrowRight, _) => {
                self.cursor = self.lens.get(cx).into();
                move_virtual_slider(self, cx, self.cursor + self.arrow_scalar);
            }

            WindowEvent::KeyDown(Code::ArrowDown | Code::ArrowLeft, _) => {
                self.cursor = self.lens.get(cx).into();
                if self.cursor > 0 {
                    move_virtual_slider(self, cx, self.cursor - self.arrow_scalar);
                }
            }

            _ => {}
        });
    }
}

impl<'a, L> Handle<'a, Knob<L, u8>>
where
    L: Lens<Target = u8>,
{
    pub fn on_changing<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, u8),
    {
        if let Some(view) = self.cx.views.get_mut(&self.entity) {
            if let Some(knob) = view.downcast_mut::<Knob<L, u8>>() {
                knob.on_changing_discrete = Some(Box::new(callback));
            }
        }

        self
    }

    pub fn knob_type(self, knob_type: KnobType) -> Self {
        self.modify(|knob| knob.knob_type = knob_type)
    }
}

/// Adds tickmarks to a knob to show the steps that a knob can be set to.
/// When added to a knob, the knob should be made smaller (depending on span),
/// so the knob doesn't overlap with the tick marks
pub struct Ticks {
    arc_length: f32,
    arc_offset: f32,
    ticks: u8,
    selected_tick: u8,
}

impl Ticks {
    pub fn new(cx: &mut Context, arc_length: f32, arc_offset: f32, ticks: u8) -> Handle<Self> {
        Self { arc_length, arc_offset, ticks, selected_tick: 0 }.build(cx, |_| {})
    }
}
impl View for Ticks {
    fn element(&self) -> Option<&'static str> {
        Some("ticks")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let opacity = cx.opacity();

        //let mut background_color: femtovg::Color = cx.current.get_background_color(cx).into();
        // background_color.set_alphaf(background_color.a * opacity);

        let mut idle_color: femtovg::Color =
            cx.background_color().cloned().unwrap_or_default().into();
        idle_color.set_alphaf(idle_color.a * opacity);

        let mut selected_color: femtovg::Color =
            cx.border_color().cloned().unwrap_or_default().into();
        selected_color.set_alphaf(selected_color.a * opacity);

        let bounds = cx.bounds();

        // Clalculate arc center
        let centerx = bounds.x + 0.5 * bounds.w;
        let centery = bounds.y + 0.5 * bounds.h;

        let angle_start = self.arc_offset - self.arc_length / 2.0;
        let angle_end = self.arc_offset + self.arc_length / 2.0;

        // Convert start and end angles to radians and rotate origin direction to be upwards instead of to the right
        let start = angle_start.to_radians() - PI / 2.0;
        let end = angle_end.to_radians() - PI / 2.0;

        let parent = cx.tree.parent(cx.current).unwrap();

        let parent_width = cx.cache.get_width(parent);

        // Convert radius and span into screen coordinates
        let radius = parent_width / 2.0;
        // default value of span is 15 % of radius. Original span value was 16.667%
        let tick_len = bounds.w / 2.;
        let line_width = cx.border_width().unwrap_or(Pixels(2.0)).value_or(2., 2.);
        // Draw ticks
        let mut path = Path::new();
        let mut selected_path = Path::new();
        for n in 0..self.ticks {
            let a = n as f32 / (self.ticks - 1) as f32;
            let angle = start + (end - start) * a;

            if n == self.selected_tick {
                selected_path.move_to(
                    centerx + angle.cos() * (radius - tick_len),
                    centery + angle.sin() * (radius - tick_len),
                );
                selected_path.line_to(
                    centerx + angle.cos() * (radius - line_width / 2.0),
                    centery + angle.sin() * (radius - line_width / 2.0),
                );
            } else {
                path.move_to(
                    centerx + angle.cos() * (radius - tick_len),
                    centery + angle.sin() * (radius - tick_len),
                );
                path.line_to(
                    centerx + angle.cos() * (radius - line_width / 2.0),
                    centery + angle.sin() * (radius - line_width / 2.0),
                );
            }
        }

        let mut paint = Paint::color(idle_color);
        paint.set_line_width(line_width);
        paint.set_line_cap(LineCap::Round);
        canvas.stroke_path(&mut path, &paint);

        let mut paint = Paint::color(selected_color);
        paint.set_line_width(line_width);
        paint.set_line_cap(LineCap::Round);
        canvas.stroke_path(&mut selected_path, &paint);
    }
}

impl Handle<'_, Ticks> {
    pub fn value<L: Lens<Target = u8>>(self, lens: L) -> Self {
        let entity = self.entity;
        Binding::new(self.cx, lens, move |cx, value| {
            let value = value.get(cx);
            if let Some(view) = cx.views.get_mut(&entity) {
                if let Some(knob) = view.downcast_mut::<Ticks>() {
                    knob.selected_tick = value;
                    cx.style.needs_redraw = true;
                }
            }
        });

        self
    }
}

/// Makes a knob that represents the current value with an arc
pub struct ArcTrack {
    arc_length: f32,
    arc_offset: f32,
    normalized_value: f32,

    center: bool,
}

impl ArcTrack {
    pub fn new(cx: &mut Context, center: bool, arc_length: f32, arc_offset: f32) -> Handle<Self> {
        Self { arc_length, arc_offset, normalized_value: 0.5, center }.build(cx, |_| {})
    }
}

impl View for ArcTrack {
    fn element(&self) -> Option<&'static str> {
        Some("arctrack")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let opacity = cx.opacity();

        let mut background_color: femtovg::Color =
            cx.background_color().cloned().unwrap_or_default().into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut selected_color: femtovg::Color =
            cx.border_color().cloned().unwrap_or_default().into();
        selected_color.set_alphaf(selected_color.a * opacity);

        let bounds = cx.bounds();

        // Calculate arc center
        let centerx = bounds.x + 0.5 * bounds.w;
        let centery = bounds.y + 0.5 * bounds.h;

        // Convert start and end angles to radians and rotate origin direction to be upwards instead of to the right
        let start = (-self.arc_length / 2.0 - 90.0 + self.arc_offset).to_radians();
        let end = (self.arc_length / 2.0 - 90.0 + self.arc_offset).to_radians();

        let parent = cx.tree.parent(cx.current).unwrap();

        let parent_width = cx.cache.get_width(parent);

        // Convert radius and span into screen coordinates
        let radius = parent_width / 2.0;
        // default value of span is 15 % of radius. Original span value was 16.667%
        let span = radius * 0.15;

        // Draw the track arc
        let mut path = Path::new();
        path.arc(centerx, centery, radius - span / 2.0, end, start, Solidity::Solid);
        let mut paint = Paint::color(background_color);
        paint.set_line_width(span);
        paint.set_line_cap(LineCap::Round);
        canvas.stroke_path(&mut path, &paint);

        // Draw the active arc
        let mut path = Path::new();

        let value = self.normalized_value;

        if self.center {
            let center = -PI / 2.0;

            if value <= 0.5 {
                let current = value * 2.0 * (center - start) + start;
                path.arc(centerx, centery, radius - span / 2.0, center, current, Solidity::Solid);
            } else {
                let current = (value * 2.0 - 1.0) * (end - center) + center;
                path.arc(centerx, centery, radius - span / 2.0, current, center, Solidity::Solid);
            }
        } else {
            let current = value * (end - start) + start;
            path.arc(centerx, centery, radius - span / 2.0, current, start, Solidity::Solid);
        }

        let mut paint = Paint::color(selected_color);
        paint.set_line_width(span);
        paint.set_line_cap(LineCap::Round);
        canvas.stroke_path(&mut path, &paint);
    }
}

impl Handle<'_, ArcTrack> {
    pub fn value<L: Lens<Target = f32>>(self, lens: L) -> Self {
        let entity = self.entity;
        Binding::new(self.cx, lens, move |cx, value| {
            let value = value.get(cx);
            if let Some(view) = cx.views.get_mut(&entity) {
                if let Some(knob) = view.downcast_mut::<ArcTrack>() {
                    knob.normalized_value = value;
                    cx.style.needs_redraw = true;
                }
            }
        });

        self
    }
}
