#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::vg;
use accesskit::ActionData;
use morphorm::Units;

use crate::prelude::*;

static DEFAULT_DRAG_SCALAR: f32 = 0.0042;
static DEFAULT_WHEEL_SCALAR: f32 = 0.005;
static DEFAULT_ARROW_SCALAR: f32 = 0.1;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.04;

use std::{default, f32::consts::PI};

/// A circular view which represents a value.
pub struct Knob<T> {
    value: T,
    default_normal: f32,

    is_dragging: bool,
    drag_anchor_x: f32,
    drag_anchor_y: f32,
    continuous_normal: f32,

    drag_scalar: f32,
    wheel_scalar: f32,
    arrow_scalar: f32,
    modifier_scalar: f32,

    on_changing: Option<Box<dyn Fn(&mut EventContext, f32)>>,
}

impl<R: Res<f32> + Clone + 'static> Knob<R> {
    /// Create a new [Knob] view.
    pub fn new(
        cx: &mut Context,
        normalized_default: impl Res<f32>,
        value: R,
        centered: bool,
    ) -> Handle<Self> {
        let value_for_track = value.clone().to_signal(cx);
        let value_for_head = value.clone().to_signal(cx);

        Self {
            value: value.clone(),
            default_normal: normalized_default.get_value(cx),

            is_dragging: false,
            drag_anchor_x: 0.0,
            drag_anchor_y: 0.0,
            continuous_normal: value.get_value(cx),

            drag_scalar: DEFAULT_DRAG_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            arrow_scalar: DEFAULT_ARROW_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,

            on_changing: None,
        }
        .build(cx, move |cx| {
            ZStack::new(cx, move |cx| {
                ArcTrack::new(
                    cx,
                    centered,
                    Percentage(100.0),
                    Percentage(15.0),
                    -240.,
                    60.,
                    KnobMode::Continuous,
                )
                .value(value_for_track)
                .class("knob-track");

                HStack::new(cx, |cx| {
                    Element::new(cx).class("knob-tick");
                })
                .bind(value_for_head, move |handle| {
                    let value = value_for_head.get();
                    handle.rotate(Angle::Deg(value * 300.0 - 150.0));
                })
                .class("knob-head");
            });
        })
        .navigable(true)
        .role(Role::Slider)
        .numeric_value(value_for_track.map(|val| (*val as f64 * 100.0).round()))
    }
}

impl<R: Res<f32> + Clone + 'static> Knob<R> {
    /// Create a custom [Knob] view.
    pub fn custom<F, V: View>(
        cx: &mut Context,
        default_normal: f32,
        value: R,
        content: F,
    ) -> Handle<'_, Self>
    where
        F: 'static + Fn(&mut Context, R) -> Handle<V>,
    {
        let value_for_content = value.clone();

        Self {
            value: value.clone(),
            default_normal,

            is_dragging: false,
            drag_anchor_x: 0.0,
            drag_anchor_y: 0.0,
            continuous_normal: value.get_value(cx),

            drag_scalar: DEFAULT_DRAG_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            arrow_scalar: DEFAULT_ARROW_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,

            on_changing: None,
        }
        .build(cx, move |cx| {
            ZStack::new(cx, move |cx| {
                (content)(cx, value_for_content.clone())
                    .width(Percentage(100.0))
                    .height(Percentage(100.0));
            });
        })
    }
}

impl<T: Res<f32> + 'static> Handle<'_, Knob<T>> {
    /// Sets the callback triggered when the knob value is changed.
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, f32),
    {
        if let Some(view) = self.cx.views.get_mut(&self.entity) {
            if let Some(knob) = view.downcast_mut::<Knob<T>>() {
                knob.on_changing = Some(Box::new(callback));
            }
        }

        self
    }
}

impl<T: Res<f32> + 'static> View for Knob<T> {
    fn element(&self) -> Option<&'static str> {
        Some("knob")
    }

    fn accessibility(&self, _cx: &mut AccessContext, node: &mut AccessNode) {
        node.set_min_numeric_value(0.0);
        node.set_max_numeric_value(100.0);
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        let move_virtual_slider = |self_ref: &mut Self, cx: &mut EventContext, new_normal: f32| {
            self_ref.continuous_normal = new_normal;

            if let Some(callback) = &self_ref.on_changing {
                (callback)(cx, self_ref.continuous_normal.clamp(0.0, 1.0));
            }
        };

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                self.is_dragging = true;
                self.drag_anchor_x = cx.mouse.left.pos_down.0;
                self.drag_anchor_y = cx.mouse.left.pos_down.1;

                cx.capture();
                cx.lock_cursor_icon();
                cx.emit(WindowEvent::SetCursor(CursorIcon::None));
                cx.focus_with_visibility(false);

                self.continuous_normal = self.value.get_value(cx);
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                self.is_dragging = false;

                self.continuous_normal = self.value.get_value(cx);

                cx.unlock_cursor_icon();
                cx.release();
            }

            WindowEvent::MouseMove(_, y) => {
                if self.is_dragging && !cx.is_disabled() {
                    let mut delta_normal = (*y - self.drag_anchor_y) * self.drag_scalar;

                    if cx.modifiers.shift() {
                        delta_normal *= self.modifier_scalar;
                    }

                    let new_normal = self.continuous_normal - delta_normal;

                    move_virtual_slider(self, cx, new_normal);

                    if delta_normal != 0.0 {
                        let anchor_x = self.drag_anchor_x.max(0.0) as u32;
                        let anchor_y = self.drag_anchor_y.max(0.0) as u32;
                        cx.emit(WindowEvent::SetCursorPosition(anchor_x, anchor_y));
                    }
                }
            }

            WindowEvent::MouseScroll(_, y) => {
                if *y != 0.0 {
                    let delta_normal = -*y * self.wheel_scalar;

                    let new_normal = self.continuous_normal - delta_normal;

                    move_virtual_slider(self, cx, new_normal);
                }
            }

            WindowEvent::MouseDoubleClick(button) if *button == MouseButton::Left => {
                self.is_dragging = false;
                cx.unlock_cursor_icon();

                move_virtual_slider(self, cx, self.default_normal);
            }

            WindowEvent::KeyDown(Code::ArrowUp | Code::ArrowRight, _) => {
                self.continuous_normal = self.value.get_value(cx);
                move_virtual_slider(self, cx, self.continuous_normal + self.arrow_scalar);
            }

            WindowEvent::KeyDown(Code::ArrowDown | Code::ArrowLeft, _) => {
                self.continuous_normal = self.value.get_value(cx);
                move_virtual_slider(self, cx, self.continuous_normal - self.arrow_scalar);
            }

            WindowEvent::ActionRequest(action) => match action.action {
                Action::Increment => {
                    self.continuous_normal = self.value.get_value(cx);
                    move_virtual_slider(self, cx, self.continuous_normal + self.arrow_scalar);
                }

                Action::Decrement => {
                    self.continuous_normal = self.value.get_value(cx);
                    move_virtual_slider(self, cx, self.continuous_normal - self.arrow_scalar);
                }

                Action::SetValue => {
                    if let Some(ActionData::NumericValue(val)) = action.data {
                        let val = (val as f32).clamp(0.0, 1.0);
                        move_virtual_slider(self, cx, val);
                    }
                }

                _ => {}
            },

            _ => {}
        });
    }
}

/// Makes a knob that represents the current value with an arc
pub struct ArcTrack {
    angle_start: f32,
    angle_end: f32,
    radius: Units,
    span: Units,
    normalized_value: f32,

    center: bool,
    mode: KnobMode,
}

impl ArcTrack {
    /// Creates a new [ArcTrack] view.
    pub fn new(
        cx: &mut Context,
        center: bool,
        radius: Units,
        span: Units,
        angle_start: f32,
        angle_end: f32,
        mode: KnobMode,
    ) -> Handle<Self> {
        Self {
            // angle_start: -150.0,
            // angle_end: 150.0,
            angle_start,
            angle_end,
            radius,
            span,

            normalized_value: 0.5,

            center,
            mode,
        }
        .build(cx, |_| {})
    }
}

impl View for ArcTrack {
    fn element(&self) -> Option<&'static str> {
        Some("arctrack")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let opacity = cx.opacity();

        let foreground_color = cx.font_color();

        let background_color = cx.background_color();

        let bounds = cx.bounds();

        // Calculate arc center
        let centerx = bounds.x + 0.5 * bounds.w;
        let centery = bounds.y + 0.5 * bounds.h;

        // Convert start and end angles to radians and rotate origin direction to be upwards instead of to the right
        let start = self.angle_start;
        let end = self.angle_end;

        let parent = cx.tree.get_parent(cx.current).unwrap();

        let parent_width = cx.cache.get_width(parent);

        // Convert radius and span into screen coordinates
        let radius = self.radius.to_px(parent_width / 2.0, 0.0);
        // default value of span is 15 % of radius. Original span value was 16.667%
        let span = self.span.to_px(radius, 0.0);

        // Draw the track arc
        let path = vg::Path::new();
        // path.arc(centerx, centery, radius - span / 2.0, end, start, Solidity::Solid);
        let oval = vg::Rect::new(bounds.left(), bounds.top(), bounds.right(), bounds.bottom());

        let mut paint = vg::Paint::default();
        paint.set_color(background_color);
        paint.set_stroke_width(span);
        paint.set_stroke_cap(vg::PaintCap::Round);
        paint.set_style(vg::PaintStyle::Stroke);
        // canvas.draw_path(&path, &paint);
        canvas.draw_arc(oval, start, end - start, true, &paint);

        // Draw the active arc
        let mut path = vg::PathBuilder::new();

        let value = match self.mode {
            KnobMode::Continuous => self.normalized_value,
            // snapping
            KnobMode::Discrete(steps) => {
                (self.normalized_value * (steps - 1) as f32).floor() / (steps - 1) as f32
            }
        };

        if self.center {
            let center = -90.0;

            if value <= 0.5 {
                let current = value * 2.0 * (center - start) + start;
                path.arc_to(oval.with_inset((span / 2.0, span / 2.0)), start, current, false);
            } else {
                let current = (value * 2.0 - 1.0) * (end - center);
                path.arc_to(oval.with_inset((span / 2.0, span / 2.0)), center, current, false);
            }
        } else {
            let current = value * (end - start) + start;
            path.arc_to(oval.with_inset((span / 2.0, span / 2.0)), start, current - start, false);
        }

        let mut paint = vg::Paint::default();
        paint.set_color(foreground_color);
        paint.set_stroke_width(span);
        paint.set_stroke_cap(vg::PaintCap::Round);
        paint.set_style(vg::PaintStyle::Stroke);
        paint.set_anti_alias(true);
        let path = path.detach();
        canvas.draw_path(&path, &paint);
    }
}

impl Handle<'_, ArcTrack> {
    pub fn value<R: Res<f32>>(self, value: R) -> Self {
        let entity = self.entity;
        value.set_or_bind(self.cx, move |cx, value| {
            let value = Res::get_value(&value, cx);
            if let Some(view) = cx.views.get_mut(&entity) {
                if let Some(knob) = view.downcast_mut::<ArcTrack>() {
                    knob.normalized_value = value;
                    cx.needs_redraw(entity);
                }
            }
        });

        self
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum KnobMode {
    Discrete(usize),
    #[default]
    Continuous,
}

/// Adds tickmarks to a knob to show the steps that a knob can be set to.
/// When added to a knob, the knob should be made smaller (depending on span),
/// so the knob doesn't overlap with the tick marks
pub struct Ticks {
    angle_start: f32,
    angle_end: f32,
    radius: Units,
    // TODO: should this be renamed to inner_radius?
    tick_len: Units,
    tick_width: Units,
    // steps: u32,
    mode: KnobMode,
}
impl Ticks {
    /// Creates a new [Ticks] view.
    pub fn new(
        cx: &mut Context,
        radius: Units,
        tick_len: Units,
        tick_width: Units,
        arc_len: f32,
        mode: KnobMode,
    ) -> Handle<Self> {
        Self {
            // angle_start: -150.0,
            // angle_end: 150.0,
            angle_start: -arc_len / 2.0,
            angle_end: arc_len / 2.0,
            radius,
            tick_len,
            tick_width,
            mode,
        }
        .build(cx, |_| {})
    }
}

impl View for Ticks {
    fn element(&self) -> Option<&'static str> {
        Some("ticks")
    }
    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let opacity = cx.opacity();
        //let mut background_color: femtovg::Color = cx.current.get_background_color(cx).into();
        // background_color.set_alphaf(background_color.a * opacity);
        let foreground_color = cx.background_color();
        // let background_color = femtovg::Color::rgb(54, 54, 54);
        //et mut foreground_color = femtovg::Color::rgb(50, 50, 200);
        let bounds = cx.bounds();
        // Clalculate arc center
        let centerx = bounds.x + 0.5 * bounds.w;
        let centery = bounds.y + 0.5 * bounds.h;
        // Convert start and end angles to radians and rotate origin direction to be upwards instead of to the right
        let start = self.angle_start.to_radians() - PI / 2.0;
        let end = self.angle_end.to_radians() - PI / 2.0;
        let parent = cx.tree.get_parent(cx.current).unwrap();
        let parent_width = cx.cache.get_width(parent);
        // Convert radius and span into screen coordinates
        let radius = self.radius.to_px(parent_width / 2.0, 0.0);
        // default value of span is 15 % of radius. Original span value was 16.667%
        let tick_len = self.tick_len.to_px(radius, 0.0);
        let line_width = self.tick_width.to_px(radius, 0.0);
        // Draw ticks
        let mut path = vg::PathBuilder::new();
        match self.mode {
            // can't really make ticks for a continuous knob
            KnobMode::Continuous => return,
            KnobMode::Discrete(steps) => {
                for n in 0..steps {
                    let a = n as f32 / (steps - 1) as f32;
                    let angle = start + (end - start) * a;
                    path.move_to((
                        centerx + angle.cos() * (radius - tick_len),
                        centery + angle.sin() * (radius - tick_len),
                    ));
                    path.line_to((
                        centerx + angle.cos() * (radius - line_width / 2.0),
                        centery + angle.sin() * (radius - line_width / 2.0),
                    ));
                }
            }
        }
        let mut paint = vg::Paint::default();
        paint.set_color(foreground_color);
        paint.set_stroke_width(line_width);
        paint.set_stroke_cap(vg::PaintCap::Round);
        paint.set_style(vg::PaintStyle::Stroke);
        let path = path.detach();
        canvas.draw_path(&path, &paint);
    }
}

/// Makes a round knob with a tick to show the current value
pub struct TickKnob {
    angle_start: f32,
    angle_end: f32,
    radius: Units,
    tick_width: Units,
    tick_len: Units,
    normalized_value: f32,
    mode: KnobMode,
}
impl TickKnob {
    /// Creates a new [TickKnob] view.
    pub fn new(
        cx: &mut Context,
        radius: Units,
        tick_width: Units,
        tick_len: Units,
        arc_len: f32,
        // steps: u32,
        mode: KnobMode,
    ) -> Handle<Self> {
        Self {
            // angle_start: -150.0,
            // angle_end: 150.0,
            angle_start: -arc_len / 2.0,
            angle_end: arc_len / 2.0,
            radius,
            tick_width,
            tick_len,
            normalized_value: 0.5,
            mode,
        }
        .build(cx, |_| {})
    }
}

impl View for TickKnob {
    fn element(&self) -> Option<&'static str> {
        Some("tickknob")
    }
    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let opacity = cx.opacity();
        //let mut background_color: femtovg::Color = cx.current.get_background_color(cx).into();
        // background_color.set_alphaf(background_color.a * opacity);
        let foreground_color = cx.font_color();
        let background_color = cx.background_color();
        //et mut foreground_color = femtovg::Color::rgb(50, 50, 200);
        let bounds = cx.bounds();
        // Calculate arc center
        let centerx = bounds.x + 0.5 * bounds.w;
        let centery = bounds.y + 0.5 * bounds.h;
        // Convert start and end angles to radians and rotate origin direction to be upwards instead of to the right
        let start = self.angle_start.to_radians() - PI / 2.0;
        let end = self.angle_end.to_radians() - PI / 2.0;
        let parent = cx.tree.get_parent(cx.current).unwrap();
        let parent_width = cx.cache.get_width(parent);
        // Convert radius and span into screen coordinates
        let radius = self.radius.to_px(parent_width / 2.0, 0.0);
        let tick_width = self.tick_width.to_px(radius, 0.0);
        let tick_len = self.tick_len.to_px(radius, 0.0);
        // Draw the circle
        let mut path = vg::PathBuilder::new();
        path.add_circle((centerx, centery), radius, None);
        // path.arc(centerx, centery, radius - span / 2.0, end, start, Solidity::Solid);
        let mut paint = vg::Paint::default();
        paint.set_color(background_color);
        paint.set_stroke_width(tick_width);
        paint.set_stroke_cap(vg::PaintCap::Round);
        paint.set_style(vg::PaintStyle::Stroke);
        let path = path.detach();
        canvas.draw_path(&path, &paint);
        // Draw the tick
        let mut path = vg::PathBuilder::new();
        let angle = match self.mode {
            KnobMode::Continuous => start + (end - start) * self.normalized_value,
            // snapping
            KnobMode::Discrete(steps) => {
                start
                    + (end - start) * (self.normalized_value * (steps - 1) as f32).floor()
                        / (steps - 1) as f32
            }
        };
        path.move_to(
            // centerx + angle.cos() * (radius * 0.70),
            (
                centerx + angle.cos() * (radius - tick_len),
                centery + angle.sin() * (radius - tick_len),
            ),
        );
        path.line_to((
            centerx + angle.cos() * (radius - tick_width / 2.0),
            centery + angle.sin() * (radius - tick_width / 2.0),
        ));
        let mut paint = vg::Paint::default();
        paint.set_color(foreground_color);
        paint.set_stroke_width(tick_width);
        paint.set_stroke_cap(vg::PaintCap::Round);
        paint.set_style(vg::PaintStyle::Stroke);
        let path = path.detach();
        canvas.draw_path(&path, &paint);
    }
}

impl Handle<'_, TickKnob> {
    pub fn value<R: Res<f32>>(self, value: R) -> Self {
        let entity = self.entity;
        value.set_or_bind(self.cx, move |cx, value| {
            let value = Res::get_value(&value, cx);
            if let Some(view) = cx.views.get_mut(&entity) {
                if let Some(knob) = view.downcast_mut::<TickKnob>() {
                    knob.normalized_value = value;
                    cx.needs_redraw(entity);
                }
            }
        });
        self
    }
}
