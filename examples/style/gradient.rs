use std::marker::PhantomData;

use vizia::prelude::*;
use vizia::vg;

const STYLE: &str = r#"

    element {
        width: 400px;
        height: 200px;
        space: 1s;
        background-color: blue;
        border-radius: 0%;
    }

    .linear-gradient {
        background-image: linear-gradient(red, red);
    }

    .linear-gradient:hover {
        width: 200px;
        background-image: linear-gradient(red, yellow);
        border-radius: 50%;
        transition: background-image 0.5s 0.5s, width 0.5s, border-radius 0.5s 1.0s;
    }

    .linear-gradient-direction {
        background-image: linear-gradient(to right, red, yellow);
    }

    .linear-gradient-corner {
        background-image: linear-gradient(to bottom right, red, yellow);
    }



"#;

pub trait GradientPoint {
    fn pos(&self) -> f32;
    fn color(&self) -> Color;
}

impl GradientPoint for (f32, Color) {
    fn pos(&self) -> f32 {
        self.0
    }

    fn color(&self) -> Color {
        self.1
    }
}

#[derive(Debug, Lens)]
struct AppData {
    gradient: GradientData,
}

pub enum AppEvent {
    MovePoint(usize, f32, f32),
    AddPoint(f32),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::AddPoint(t) => {
                let mut insert_point = (0, Color::RGBA(RGBA::BLACK));
                let iter = self.gradient.stops.windows(2).enumerate();
                for (index, window) in iter {
                    let first = window[0];
                    let last = window[1];
                    if *t >= first.0 && *t < last.0 {
                        let col = Color::interpolate(
                            &first.1,
                            &last.1,
                            (*t - first.0) / (last.0 - first.0),
                        );
                        insert_point = (index, col);
                    }
                }

                self.gradient.stops.insert(insert_point.0 + 1, (*t, insert_point.1));
            }

            AppEvent::MovePoint(index, x, y) => {
                if *index == 0 {
                    self.gradient.start_point = (*x, *y);
                } else if *index == self.gradient.stops.len() - 1 {
                    self.gradient.end_point = (*x, *y);
                } else {
                    // Convert position into a t
                    let x2 = self.gradient.end_point.0;
                    let x1 = self.gradient.start_point.0;
                    let y2 = self.gradient.end_point.1;
                    let y1 = self.gradient.start_point.1;

                    // Sqaured distance from start to end
                    let d = ((x2 - x1) * (x2 - x1)) + ((y2 - y1) * (y2 - y1));

                    let p1 = (*x - x1, *y - y1);
                    let p2 = (x2 - x1, y2 - y1);
                    let product = p1.0 * p2.0 + p1.1 * p2.1;

                    let mut t = product / d;
                    println!("Index: {}  t: {}", index, t);
                    t = t.clamp(0.0, 1.0);
                    if let Some(stop) = self.gradient.stops.get_mut(*index) {
                        stop.0 = t;
                    }
                    // Project mouse position on to line
                }
                cx.needs_redraw();
            }
        })
    }
}

#[derive(Debug, Data, Clone, Lens)]
pub struct GradientData {
    start_point: (f32, f32),
    end_point: (f32, f32),
    stops: Vec<(f32, Color)>,
}

pub struct GradientControl<L> {
    lens: L,
    is_dragging: bool,
    selected: Option<usize>,
}

impl<L> GradientControl<L>
where
    L: Lens<Target = GradientData>,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self { lens, is_dragging: false, selected: None }.build(cx, |_| {})
    }
}

impl<L> View for GradientControl<L>
where
    L: Lens<Target = GradientData>,
{
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                if let Some(gradient) = self.lens.get_fallible(cx) {
                    self.selected = None;
                    for (index, stop) in gradient.stops.iter().enumerate() {
                        let x2 = gradient.end_point.0;
                        let x1 = gradient.start_point.0;
                        let y2 = gradient.end_point.1;
                        let y1 = gradient.start_point.1;

                        let d = (((x2 - x1) * (x2 - x1)) + ((y2 - y1) * (y2 - y1))).sqrt();
                        let t = stop.pos();

                        let (xt, yt) = (((1.0 - t) * x1 + t * x2), ((1.0 - t) * y1 + t * y2));

                        println!("xt: {}  yt: {}", xt, yt);

                        let mouse_down = cx.mouse.left.pos_down;
                        let dist_x = mouse_down.0 - xt;
                        let dist_y = mouse_down.1 - yt;
                        let squared_dist = dist_x * dist_x + dist_y * dist_y;

                        // println!("{} {}", index, squared_dist);
                        if squared_dist <= 100.0 {
                            self.selected = Some(index);
                            self.is_dragging = true;
                            cx.capture();
                        }
                    }

                    if self.selected.is_none() {
                        let x0 = cx.mouse.left.pos_down.0;
                        let y0 = cx.mouse.left.pos_down.1;
                        let x2 = gradient.end_point.0;
                        let x1 = gradient.start_point.0;
                        let y2 = gradient.end_point.1;
                        let y1 = gradient.start_point.1;

                        let d = ((x2 - x1) * (x2 - x1)) + ((y2 - y1) * (y2 - y1));

                        let dist = ((x2 - x1) * (y1 - y0) - (x1 - x0) * (y2 - y1)).powf(2.0) / d;

                        let p1 = (x0 - x1, y0 - y1);
                        let p2 = (x2 - x1, y2 - y1);
                        let product = p1.0 * p2.0 + p1.1 * p2.1;

                        let mut t = product / d;

                        t = t.clamp(0.0, 1.0);
                        println!("Click line: {} {}", t, dist);

                        if dist < 25.0 {
                            cx.emit(AppEvent::AddPoint(t));
                        }
                    }
                }
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                self.is_dragging = false;
                cx.release();
            }

            WindowEvent::MouseMove(x, y) => {
                if let Some(selected) = self.selected {
                    if self.is_dragging {
                        cx.emit(AppEvent::MovePoint(selected, *x, *y));
                        cx.emit(WindowEvent::Redraw);
                    }
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        if let Some(gradient) = self.lens.get_fallible(cx) {
            let line_paint = vg::Paint::color(RGBA::rgb(110, 180, 255).into()).with_line_width(4.0);
            let line_back_paint = vg::Paint::color(RGBA::WHITE.into()).with_line_width(8.0);

            if gradient.stops.len() >= 2 {
                let first_stop = gradient.stops.first().unwrap();
                let last_stop = gradient.stops.last().unwrap();

                let mut line = vg::Path::new();
                line.move_to(gradient.start_point.0, gradient.start_point.1);
                line.line_to(gradient.end_point.0, gradient.end_point.1);

                canvas.stroke_path(&mut line, &line_back_paint);
                canvas.stroke_path(&mut line, &line_paint);
            }

            for (index, stop) in gradient.stops.iter().enumerate() {
                let x2 = gradient.end_point.0;
                let x1 = gradient.start_point.0;
                let y2 = gradient.end_point.1;
                let y1 = gradient.start_point.1;

                let d = (((x2 - x1) * (x2 - x1)) + ((y2 - y1) * (y2 - y1))).sqrt();
                let t = stop.pos();

                let (xt, yt) = (((1.0 - t) * x1 + t * x2), ((1.0 - t) * y1 + t * y2));

                let mut path = vg::Path::new();
                path.circle(0.0, 0.0, 10.0);

                canvas.save();
                canvas.translate(xt, yt);
                canvas.fill_path(&mut path, &vg::Paint::color(stop.color().into()));
                canvas.stroke_path(&mut path, &line_back_paint);
                canvas.stroke_path(&mut path, &line_paint);
                canvas.restore();
            }
        }
    }
}

pub struct GradientCanvas<L> {
    lens: L,
}

impl<L> GradientCanvas<L>
where
    L: Lens<Target = GradientData>,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self { lens }.build(cx, |_| {})
    }
}

impl<L> View for GradientCanvas<L>
where
    L: Lens<Target = GradientData>,
{
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        if let Some(gradient_data) = self.lens.get_fallible(cx) {
            if gradient_data.stops.len() >= 2 {
                let first_stop = gradient_data.stops.first().unwrap();
                let last_stop = gradient_data.stops.last().unwrap();

                let bounds = cx.bounds();

                let mut bounds_path = vg::Path::new();
                bounds_path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
                let gradient_paint = vg::Paint::linear_gradient_stops(
                    gradient_data.start_point.0,
                    gradient_data.start_point.1,
                    gradient_data.end_point.0,
                    gradient_data.end_point.1,
                    &gradient_data
                        .stops
                        .iter()
                        .map(|stop| (stop.0, stop.1.into()))
                        .collect::<Vec<_>>(),
                );
                canvas.fill_path(&mut bounds_path, &gradient_paint);
            }
        }
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);

        AppData {
            gradient: GradientData {
                start_point: (100.0, 100.0),
                end_point: (300.0, 300.0),
                stops: vec![
                    (0.0, RGBA::GREEN.into()),
                    (0.5, RGBA::BLUE.into()),
                    (1.0, RGBA::RED.into()),
                ],
            },
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Element::new(cx).class("linear-gradient");
            // Element::new(cx).class("linear-gradient-direction");
            // Element::new(cx)
            //     // .class("linear-gradient-corner")
            //     .height(Pixels(400.0))
            //     .width(Pixels(600.0))
            //     .background_image("linear-gradient(red, green)");

            // GradientCanvas::new(cx, AppData::gradient).size(Pixels(300.0)).space(Stretch(1.0));

            // GradientControl::new(cx, AppData::gradient)
            //     .position_type(PositionType::SelfDirected)
            //     .size(Stretch(1.0))
            //     .background_color(RGBA::GREY);
        })
        .row_between(Pixels(10.0));
    })
    .run();
}
