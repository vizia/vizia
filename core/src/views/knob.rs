use femtovg::{LineCap, Paint, Path, Solidity};
use morphorm::{Hierarchy, Units};

use crate::{Binding, Context, Entity, Handle, Lens, LensExt, Model, Modifiers, MouseButton, Res, Units::*, View, WindowEvent, ZStack};

static DEFAULT_DRAG_SCALAR: f32 = 0.0042;
static DEFAULT_WHEEL_SCALAR: f32 = 0.005;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.04;

use std::f32::consts::PI;

pub struct Knob<L> {
    lens: L,
    default_normal: f32,

    is_dragging: bool,
    prev_drag_y: f32,
    continuous_normal: f32,

    drag_scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,

    on_changing: Option<Box<dyn Fn(&mut Context, f32)>>,
}

impl<L: Lens<Target = f32>> Knob<L> {
    pub fn new(
        cx: &mut Context,
        normalized_default: impl Res<f32>,
        lens: L,
        centered: bool,
    ) -> Handle<Self> {
        Self {
            lens: lens.clone(),
            default_normal: normalized_default.get_val(cx),

            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: *lens.get(cx),

            drag_scalar: DEFAULT_DRAG_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,

            on_changing: None,
        }
        .build2(cx, move |cx| {
            ZStack::new(cx, move |cx| {
                ArcTrack::new(
                    cx,
                    centered,
                    Percentage(50.),
                    Percentage(15.),
                    300.,
                )
                .value(lens)
                .width(Stretch(1.0))
                .height(Stretch(1.0))
                .class("track");

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

    pub fn custom<F, T>(
        cx: &mut Context,
        lens: L,
        default_normal: f32,
        content: F,
    ) -> Handle<'_, Self>
    where
        F: 'static + Fn(&mut Context, f32) -> Handle<T>,
    {
        Self {
            lens: lens.clone(),
            default_normal,

            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: *lens.get(cx),

            drag_scalar: DEFAULT_DRAG_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,

            on_changing: None,
        }
        .build2(cx, move |cx| {
            ZStack::new(cx, move |cx| {
                Binding::new(cx, lens, move |cx, value| {
                    (content)(cx, *value.get(cx))
                        .width(Percentage(100.0))
                        .height(Percentage(100.0));
                });
            });
        })
    }
}

impl<'a, L: Lens<Target = f32>> Handle<'a, Knob<L>> {
    pub fn on_changing<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context, f32),
    {
        if let Some(view) = self.cx.views.get_mut(&self.entity) {
            if let Some(knob) = view.downcast_mut::<Knob<L>>() {
                knob.on_changing = Some(Box::new(callback));
            }
        }

        self
    }
}

impl<L: Lens<Target = f32>> View for Knob<L> {
    fn element(&self) -> Option<String> {
        Some("knob".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        let move_virtual_slider = |self_ref: &mut Self, cx: &mut Context, new_normal: f32| {
            self_ref.continuous_normal = new_normal.clamp(0.0, 1.0);

            // TODO - Remove when done
            //println!("Normalized: {}, Display: {}", self_ref.normalized_value, self_ref.map.normalized_to_display(self_ref.normalized_value));

            if let Some(callback) = self_ref.on_changing.take() {
                (callback)(cx, self_ref.continuous_normal);
                self_ref.on_changing = Some(callback);
            }

            //entity.emit(cx, SliderEvent::ValueChanged(self_ref.normalized_value));

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

                    self.continuous_normal = *self.lens.get(cx);

                    // if let Some(callback) = self.on_press.take() {
                    //     (callback)(self, cx, cx.current);
                    //     self.on_press = Some(callback);
                    // }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    self.is_dragging = false;
                    //self.continuous_normal = self.normalized_value;

                    self.continuous_normal = *self.lens.get(cx);

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
        center: bool,
        radius: Units,
        span: Units,
        arc_len: f32,
    ) -> Handle<Self> {
        Self {
            // angle_start: -150.0,
            // angle_end: 150.0,
            angle_start: -arc_len / 2.,
            angle_end: arc_len / 2.,
            radius,
            span,

            normalized_value: 0.5,

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

        // Calculate arc center
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

impl Handle<'_, ArcTrack> {
    pub fn value<L: Lens<Target = f32>>(self, lens: L) -> Self {
        let entity = self.entity;
        Binding::new(self.cx, lens, move |cx, value| {
            let value = *value.get(cx);
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
