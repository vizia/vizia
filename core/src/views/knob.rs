use femtovg::{LineCap, Paint, Path, Solidity};
use morphorm::{Hierarchy, Units};

use crate::{
    Binding, Context, Entity, Handle, Model, Modifiers, MouseButton, SliderData, SliderEvent,
    Units::*, View, WindowEvent, ZStack,
};

static DEFAULT_DRAG_SCALAR: f32 = 0.0042;
static DEFAULT_WHEEL_SCALAR: f32 = 0.005;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.04;

use std::f32::consts::PI;

pub struct Knob {
    pub normalized_value: f32,
    default_normal: f32,

    is_dragging: bool,
    prev_drag_y: f32,
    continuous_normal: f32,

    drag_scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,

    on_changing: Option<Box<dyn Fn(&mut Self, &mut Context)>>,
}

impl Knob {
    pub fn new(
        cx: &mut Context,
        normalized_default: f32,
        normalized_value: f32,
        centered: bool,
    ) -> Handle<Self> {
        Self {
            normalized_value,
            default_normal: normalized_default,

            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: normalized_value,

            drag_scalar: DEFAULT_DRAG_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,

            on_changing: None,
        }
        .build2(cx, move |cx| {
            SliderData { value: normalized_value.clamp(0.0, 1.0) }.build(cx);

            ZStack::new(cx, move |cx| {
                Binding::new(cx, SliderData::value, move |cx, value| {
                    //println!("{}", value.get(cx));
                    let height = cx.cache.get_height(cx.current);
                    let width = cx.cache.get_width(cx.current);
                    let radius = height.min(width) / 2.;
                    ArcTrack::new(
                        cx,
                        *value.get(cx),
                        centered,
                        Pixels(radius),
                        Percentage(15.),
                        300.,
                    )
                    .width(Stretch(1.0))
                    .height(Stretch(1.0))
                    .class("track");
                });

                // TODO
                // Element::new(cx)
                //     .width(Pixels(10.0))
                //     .height(Pixels(10.0))
                //     .space(Stretch(1.0))
                //     .background_color(Color::red())
                //     .translate((30.0,0.0))
                //     .rotate(30.0);
            });
        })
    }
    pub fn custom<'a, F, T>(
        cx: &'a mut Context,
        normalized_default: f32,
        normalized_value: f32,
        content: F,
    ) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, f32) -> Handle<T>,
    {
        Self {
            normalized_value,
            default_normal: normalized_default,

            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: normalized_value,

            drag_scalar: DEFAULT_DRAG_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,

            on_changing: None,
        }
        .build2(cx, move |cx| {
            SliderData { value: normalized_value.clamp(0.0, 1.0) }.build(cx);

            ZStack::new(cx, move |cx| {
                Binding::new(cx, SliderData::value, move |cx, value| {
                    (content)(cx, *value.get(cx))
                        .width(Percentage(100.0))
                        .height(Percentage(100.0));
                });
            });
        })
    }
}

impl<'a> Handle<'a, Knob> {
    pub fn on_changing<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Knob, &mut Context),
    {
        if let Some(view) = self.cx.views.get_mut(&self.entity) {
            if let Some(knob) = view.downcast_mut::<Knob>() {
                knob.on_changing = Some(Box::new(callback));
            }
        }

        self
    }
}

impl View for Knob {
    fn element(&self) -> Option<String> {
        Some("knob".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        let move_virtual_slider = |self_ref: &mut Self, cx: &mut Context, new_normal: f32| {
            self_ref.continuous_normal = new_normal.clamp(0.0, 1.0);

            // This will cause the knob to "snap" when using an `IntMap`.
            self_ref.normalized_value = self_ref.continuous_normal;

            // TODO - Remove when done
            //println!("Normalized: {}, Display: {}", self_ref.normalized_value, self_ref.map.normalized_to_display(self_ref.normalized_value));

            if let Some(callback) = self_ref.on_changing.take() {
                (callback)(self_ref, cx);
                self_ref.on_changing = Some(callback);
            }

            //entity.emit(cx, SliderEvent::ValueChanged(self_ref.normalized_value));

            cx.emit(SliderEvent::SetValue(self_ref.normalized_value));

            // if let Some(track) = cx.query::<ArcTrack>(self_ref.value_track) {
            //     track.normalized_value = self_ref.normalized_value;
            // }

            //Entity::root().redraw(cx);
        };

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    self.is_dragging = true;
                    self.prev_drag_y = cx.mouse.left.pos_down.1;

                    cx.captured = cx.current;
                    cx.focused = cx.current;

                    if let Some(slider_data) = cx.data::<SliderData>() {
                        self.continuous_normal = slider_data.value;
                    }

                    // if let Some(callback) = self.on_press.take() {
                    //     (callback)(self, cx, cx.current);
                    //     self.on_press = Some(callback);
                    // }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    self.is_dragging = false;
                    //self.continuous_normal = self.normalized_value;

                    if let Some(slider_data) = cx.data::<SliderData>() {
                        self.continuous_normal = slider_data.value;
                    }

                    cx.captured = Entity::null();

                    // if let Some(callback) = self.on_release.take() {
                    //     (callback)(self, cx, cx.current);
                    //     self.on_release = Some(callback);
                    // }
                }

                WindowEvent::MouseMove(_, y) => {
                    //if event.target == cx.current {
                    if self.is_dragging {
                        let mut delta_normal = (*y - self.prev_drag_y) * self.drag_scalar;

                        self.prev_drag_y = *y;

                        if cx.modifiers.contains(Modifiers::SHIFT) {
                            delta_normal *= self.modifier_scalar;
                        }

                        let new_normal = self.continuous_normal - delta_normal;

                        move_virtual_slider(self, cx, new_normal);
                    }
                    //}
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

                    move_virtual_slider(self, cx, self.default_normal);
                }

                _ => {}
            }
        }
    }
}

// TODO: better name?
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
        .build(cx)
    }
}
impl View for Ticks {
    fn draw(&self, cx: &mut Context, canvas: &mut crate::Canvas) {
        let opacity = cx.cache.get_opacity(cx.current);

        //let mut background_color: femtovg::Color = cx.current.get_background_color(cx).into();
        // background_color.set_alphaf(background_color.a * opacity);

        let mut foreground_color: femtovg::Color =
            cx.style.background_color.get(cx.current).cloned().unwrap_or_default().into();
        foreground_color.set_alphaf(foreground_color.a * opacity);

        // let background_color = femtovg::Color::rgb(54, 54, 54);
        //et mut foreground_color = femtovg::Color::rgb(50, 50, 200);

        let posx = cx.cache.get_posx(cx.current);
        let posy = cx.cache.get_posy(cx.current);
        let width = cx.cache.get_width(cx.current);
        let height = cx.cache.get_height(cx.current);

        // Clalculate arc center
        let centerx = posx + 0.5 * width;
        let centery = posy + 0.5 * height;

        // Convert start and end angles to radians and rotate origin direction to be upwards instead of to the right
        let start = self.angle_start.to_radians() - PI / 2.0;
        let end = self.angle_end.to_radians() - PI / 2.0;

        //let parent = cx.current.get_parent(cx).unwrap();

        let parent = cx.tree.parent(cx.current).unwrap();

        let parent_width = cx.cache.get_width(parent);

        // Convert radius and span into screen coordinates
        let radius = self.radius.value_or(parent_width, 0.0);
        // default value of span is 15 % of radius. Original span value was 16.667%
        let tick_len = self.tick_len.value_or(radius, 0.0);
        let line_width = self.tick_width.value_or(radius, 0.0);
        // Draw ticks
        let mut path = Path::new();
        match self.mode {
            // can't really make ticks for a continuous knob
            KnobMode::Continuous => return,
            KnobMode::Discrete(steps) => {
                for n in 0..steps {
                    let a = n as f32 / (steps - 1) as f32;
                    let angle = start + (end - start) * a;
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
        }
        let mut paint = Paint::color(foreground_color);
        paint.set_line_width(line_width);
        paint.set_line_cap(LineCap::Round);
        canvas.stroke_path(&mut path, paint);
    }
}
/// Makes a round knob with a tick to show the current value
pub struct TickKnob {
    angle_start: f32,
    angle_end: f32,
    radius: Units,
    tick_width: Units,
    normalized_value: f32,
    // steps: u32,
    mode: KnobMode,
}

impl TickKnob {
    pub fn new(
        cx: &mut Context,
        value: f32,
        radius: Units,
        tick_width: Units,
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
            normalized_value: value,
            mode,
        }
        .build(cx)
    }
}
impl View for TickKnob {
    fn draw(&self, cx: &mut Context, canvas: &mut crate::Canvas) {
        let opacity = cx.cache.get_opacity(cx.current);

        //let mut background_color: femtovg::Color = cx.current.get_background_color(cx).into();
        // background_color.set_alphaf(background_color.a * opacity);

        let mut foreground_color: femtovg::Color =
            cx.style.background_color.get(cx.current).cloned().unwrap_or_default().into();
        foreground_color.set_alphaf(foreground_color.a * opacity);

        let background_color = femtovg::Color::rgb(54, 54, 54);
        //et mut foreground_color = femtovg::Color::rgb(50, 50, 200);

        let posx = cx.cache.get_posx(cx.current);
        let posy = cx.cache.get_posy(cx.current);
        let width = cx.cache.get_width(cx.current);
        let height = cx.cache.get_height(cx.current);

        // Clalculate arc center
        let centerx = posx + 0.5 * width;
        let centery = posy + 0.5 * height;

        // Convert start and end angles to radians and rotate origin direction to be upwards instead of to the right
        let start = self.angle_start.to_radians() - PI / 2.0;
        let end = self.angle_end.to_radians() - PI / 2.0;

        //let parent = cx.current.get_parent(cx).unwrap();

        let parent = cx.tree.parent(cx.current).unwrap();

        let parent_width = cx.cache.get_width(parent);

        // Convert radius and span into screen coordinates
        let radius = self.radius.value_or(parent_width, 0.0);

        let tick_width = self.tick_width.value_or(radius, 0.0);

        // Draw the circle
        let mut path = Path::new();
        path.circle(centerx, centery, radius);
        // path.arc(centerx, centery, radius - span / 2.0, end, start, Solidity::Solid);
        let mut paint = Paint::color(background_color);
        paint.set_line_width(tick_width);
        paint.set_line_cap(LineCap::Round);
        canvas.fill_path(&mut path, paint);

        // Draw the tick
        let mut path = Path::new();

        let angle = match self.mode {
            KnobMode::Continuous => start + (end - start) * self.normalized_value,
            // snapping
            KnobMode::Discrete(steps) => {
                start
                    + (end - start) * (self.normalized_value * (steps - 1) as f32).floor()
                        / (steps - 1) as f32
            }
        };

        // TODO: Does radius * 0.75 give a good tick length?
        // Should it maybe be customizable?
        path.move_to(
            centerx + angle.cos() * (radius * 0.70),
            centery + angle.sin() * (radius * 0.70),
        );
        path.line_to(
            centerx + angle.cos() * (radius - tick_width / 2.0),
            centery + angle.sin() * (radius - tick_width / 2.0),
        );

        let mut paint = Paint::color(foreground_color);
        paint.set_line_width(tick_width);
        paint.set_line_cap(LineCap::Round);
        canvas.stroke_path(&mut path, paint);
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
}

impl ArcTrack {
    pub fn new(
        cx: &mut Context,
        value: f32,
        center: bool,
        radius: Units,
        span: Units,
        arc_len: f32,
    ) -> Handle<Self> {
        Self {
            // angle_start: -150.0,
            // angle_end: 150.0,
            angle_start: -arc_len / 2.0,
            angle_end: arc_len / 2.0,
            radius,
            span,

            normalized_value: value,

            center,
        }
        .build(cx)
    }
}

impl View for ArcTrack {
    fn draw(&self, cx: &mut Context, canvas: &mut crate::Canvas) {
        let opacity = cx.cache.get_opacity(cx.current);

        //let mut background_color: femtovg::Color = cx.current.get_background_color(cx).into();
        // background_color.set_alphaf(background_color.a * opacity);

        let mut foreground_color: femtovg::Color =
            cx.style.background_color.get(cx.current).cloned().unwrap_or_default().into();
        foreground_color.set_alphaf(foreground_color.a * opacity);

        let background_color = femtovg::Color::rgb(54, 54, 54);
        //et mut foreground_color = femtovg::Color::rgb(50, 50, 200);

        let posx = cx.cache.get_posx(cx.current);
        let posy = cx.cache.get_posy(cx.current);
        let width = cx.cache.get_width(cx.current);
        let height = cx.cache.get_height(cx.current);

        // Clalculate arc center
        let centerx = posx + 0.5 * width;
        let centery = posy + 0.5 * height;

        // Convert start and end angles to radians and rotate origin direction to be upwards instead of to the right
        let start = self.angle_start.to_radians() - PI / 2.0;
        let end = self.angle_end.to_radians() - PI / 2.0;

        //let parent = cx.current.get_parent(cx).unwrap();

        let parent = cx.tree.parent(cx.current).unwrap();

        let parent_width = cx.cache.get_width(parent);

        // Convert radius and span into screen coordinates
        let radius = self.radius.value_or(parent_width, 0.0);
        // default value of span is 15 % of radius. Original span value was 16.667%
        let span = self.span.value_or(radius, 0.0);

        // Draw the track arc
        let mut path = Path::new();
        path.arc(centerx, centery, radius - span / 2.0, end, start, Solidity::Solid);
        let mut paint = Paint::color(background_color);
        paint.set_line_width(span);
        paint.set_line_cap(LineCap::Round);
        canvas.stroke_path(&mut path, paint);

        // Draw the active arc
        let mut path = Path::new();

        if self.center {
            let center = -PI / 2.0;

            if self.normalized_value <= 0.5 {
                let current = self.normalized_value * 2.0 * (center - start) + start;
                path.arc(centerx, centery, radius - span / 2.0, center, current, Solidity::Solid);
            } else {
                let current = (self.normalized_value * 2.0 - 1.0) * (end - center) + center;
                path.arc(centerx, centery, radius - span / 2.0, current, center, Solidity::Solid);
            }
        } else {
            let current = self.normalized_value * (end - start) + start;
            path.arc(centerx, centery, radius - span / 2.0, current, start, Solidity::Solid);
        }

        let mut paint = Paint::color(foreground_color);
        paint.set_line_width(span);
        paint.set_line_cap(LineCap::Round);
        canvas.stroke_path(&mut path, paint);
    }
}
// TODO: better name?
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KnobMode {
    Discrete(usize),
    Continuous,
}
impl Default for KnobMode {
    fn default() -> Self {
        KnobMode::Continuous
    }
}
