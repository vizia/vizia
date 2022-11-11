use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;
use crate::vg;

pub struct ColorPicker<L> {
    lens: L,
    on_change: Option<Box<dyn Fn(&mut EventContext, Color)>>,
}

pub enum ColorPickerEvent {
    SetColor(Color),
}

impl<L> ColorPicker<L>
where
    L: Lens<Target = Color>,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self { lens: lens.clone(), on_change: None }
            .build(cx, |cx| {
                ColorSelector::new(cx, lens.clone())
                    .on_change(|cx, color| cx.emit(ColorPickerEvent::SetColor(color)))
                    .size(Pixels(200.0))
                    .background_color(Color::red());

                // Hue Slider
                // Alpha Slider
                HStack::new(cx, |cx| {
                    //Dropdown
                    Textbox::new(
                        cx,
                        lens.clone().map(|color| {
                            let (h, s, v) = rgb_to_hsv(
                                color.r() as f64 / 255.0,
                                color.g() as f64 / 255.0,
                                color.b() as f64 / 255.0,
                            );
                            h
                        }),
                    )
                    .width(Stretch(1.0));
                    Textbox::new(
                        cx,
                        lens.clone().map(|color| {
                            let (h, s, v) = rgb_to_hsv(
                                color.r() as f64 / 255.0,
                                color.g() as f64 / 255.0,
                                color.b() as f64 / 255.0,
                            );
                            s
                        }),
                    )
                    .width(Stretch(1.0));
                    Textbox::new(
                        cx,
                        lens.clone().map(|color| {
                            let (h, s, v) = rgb_to_hsv(
                                color.r() as f64 / 255.0,
                                color.g() as f64 / 255.0,
                                color.b() as f64 / 255.0,
                            );
                            v
                        }),
                    )
                    .width(Stretch(1.0));
                });
            })
            .size(Auto)
            .row_between(Pixels(4.0))
    }
}

impl<L> View for ColorPicker<L>
where
    L: Lens<Target = Color>,
{
    fn element(&self) -> Option<&'static str> {
        Some("colorpicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|colorpicker_event, _| match colorpicker_event {
            ColorPickerEvent::SetColor(color) => {
                if let Some(callback) = &self.on_change {
                    (callback)(cx, *color);
                }
            }
        });
    }
}

impl<'v, L> Handle<'v, ColorPicker<L>>
where
    L: Lens<Target = Color>,
{
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Color),
    {
        self.modify(|colorpicker: &mut ColorPicker<L>| {
            colorpicker.on_change = Some(Box::new(callback))
        })
    }
}

// TODO: Think of a better name for this
#[derive(Lens)]
pub struct ColorSelector<L: Lens> {
    lens: L,
    image: Rc<RefCell<Option<vg::ImageId>>>,
    thumb_left: Units,
    thumb_top: Units,
    thumb_checked: bool,
    is_dragging: bool,

    on_change: Option<Box<dyn Fn(&mut EventContext, Color)>>,
}

impl<L> ColorSelector<L>
where
    L: Lens<Target = Color>,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self {
            lens: lens.clone(),
            image: Rc::new(RefCell::new(None)),
            thumb_left: Pixels(0.0),
            thumb_top: Pixels(0.0),
            thumb_checked: false,
            is_dragging: false,
            on_change: None,
        }
        .build(cx, |cx| {
            Element::new(cx)
                .position_type(PositionType::SelfDirected)
                //.left(Self::thumb_left)
                //.top(Self::thumb_top)
                .translate((-5.0, -5.0))
                .checked(Self::thumb_checked)
                .size(Pixels(10.0))
                .border_radius(Percentage(50.0))
                .border_width(Pixels(2.0))
                .border_color(Color::white())
                .hoverable(false)
                .bind(lens.clone(), |handle, color| {
                    let color = color.get(handle.cx);
                    let (h, s, v) = rgb_to_hsv(
                        color.r() as f64 / 255.0,
                        color.g() as f64 / 255.0,
                        color.b() as f64 / 255.0,
                    );

                    //println!("h {} s {} v {}", h, s, v);

                    //let bounds = handle.bounds();
                    // let dx = s as f32 * bounds.w - 5.0;
                    // let dy = v as f32 * bounds.h - 5.0;
                    handle
                        .left(Percentage(s as f32 * 100.0))
                        .top(Percentage((1.0 - v) as f32 * 100.0));
                });
        })
        .overflow(Overflow::Hidden)
    }
}

impl<L> View for ColorSelector<L>
where
    L: Lens<Target = Color>,
{
    fn element(&self) -> Option<&'static str> {
        Some("color-selector")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                let current = cx.current();
                if meta.target == current {
                    cx.capture();
                    let width = cx.cache.get_width(current);
                    let height = cx.cache.get_height(current);

                    let mut dx = (cx.mouse.left.pos_down.0 - cx.cache.get_posx(current))
                        / cx.cache.get_width(current);
                    let mut dy = (cx.mouse.left.pos_down.1 - cx.cache.get_posy(current))
                        / cx.cache.get_height(current);

                    dx = dx.clamp(0.0, 1.0);
                    dy = dy.clamp(0.0, 1.0);

                    let saturation = dx;
                    let value = 1.0 - dy;

                    // self.saturation = dx;
                    // self.value = 1.0 - dy;

                    // self.thumb
                    //     .set_left(state, Pixels(dx * width - 5.0))
                    //     .set_top(state, Pixels(dy * height - 5.0));
                    // let dpi = cx.scale_factor();
                    // self.thumb_left = Pixels(((dx * width - 5.0) / dpi).round());
                    // self.thumb_top = Pixels(((dy * height - 5.0) / dpi).round());

                    // if dx < 0.2 && dy < 0.2 {
                    //     self.thumb_checked = true;
                    // } else {
                    //     self.thumb_checked = false;
                    // }

                    self.is_dragging = true;

                    if let Some(callback) = &self.on_change {
                        let current = self.lens.get(cx);
                        let (h, _, _) = rgb_to_hsv(
                            current.r() as f64 / 255.0,
                            current.g() as f64 / 255.0,
                            current.b() as f64 / 255.0,
                        );
                        let (h, s, l) = hsv_to_hsl(h, saturation as f64, value as f64);
                        let new = Color::hsl(h as f32, s as f32, l as f32);
                        (callback)(cx, new);
                    }
                }
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                cx.set_active(false);
                cx.release();
                self.is_dragging = false;
                if meta.target == cx.current() {
                    cx.release();
                }
            }

            WindowEvent::MouseMove(x, y) => {
                if self.is_dragging {
                    let current = cx.current();
                    let mut dx = (*x - cx.cache.get_posx(current)) / cx.cache.get_width(current);
                    let mut dy = (*y - cx.cache.get_posy(current)) / cx.cache.get_height(current);

                    dx = dx.clamp(0.0, 1.0);
                    dy = dy.clamp(0.0, 1.0);

                    let saturation = dx;
                    let value = 1.0 - dy;

                    // let width = cx.cache.get_width(current);
                    // let height = cx.cache.get_height(current);

                    // let dpi = cx.scale_factor();
                    // self.thumb_left = Pixels(((dx * width - 5.0) / dpi).round());
                    // self.thumb_top = Pixels(((dy * height - 5.0) / dpi).round());

                    // if dx < 0.2 && dy < 0.2 {
                    //     self.thumb_checked = true;
                    // } else {
                    //     self.thumb_checked = false;
                    // }

                    if let Some(callback) = &self.on_change {
                        let current = self.lens.get(cx);
                        let (h, _, _) = rgb_to_hsv(
                            current.r() as f64 / 255.0,
                            current.g() as f64 / 255.0,
                            current.b() as f64 / 255.0,
                        );
                        let (h, s, l) = hsv_to_hsl(h, saturation as f64, value as f64);
                        let new = Color::hsl(h as f32, s as f32, l as f32);
                        (callback)(cx, new);
                    }
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let image_id = if let Some(image_id) = *self.image.borrow() {
            image_id
        } else {
            let image_id = canvas
                .create_image_empty(64, 64, vg::PixelFormat::Rgb8, vg::ImageFlags::empty())
                .expect("Failed to create image");

            canvas.save();
            canvas.reset();
            canvas.reset_scissor();
            canvas.reset_transform();
            if let Ok(size) = canvas.image_size(image_id) {
                canvas.set_render_target(vg::RenderTarget::Image(image_id));

                canvas.clear_rect(0, 0, size.0 as u32, size.1 as u32, femtovg::Color::rgb(0, 0, 0));
                for x in 0..64 {
                    for y in 0..64 {
                        let x_ratio = x as f64 / 63 as f64;
                        let y_ratio = y as f64 / 63 as f64;

                        let (_, s, v) = hsv_to_hsl(0.0, x_ratio, y_ratio);

                        canvas.clear_rect(
                            x as u32,
                            y as u32,
                            1,
                            1,
                            femtovg::Color::hsl(0.0, s as f32, v as f32),
                        );
                    }
                }
            }
            canvas.restore();
            canvas.set_render_target(vg::RenderTarget::Screen);

            image_id
        };

        *self.image.borrow_mut() = Some(image_id);

        let bounds = cx.bounds();

        canvas.save();
        canvas.reset();
        canvas.reset_scissor();
        canvas.reset_transform();
        let mut path = vg::Path::new();
        path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        canvas.fill_path(
            &mut path,
            &vg::Paint::image(image_id, bounds.x, bounds.y, bounds.w, bounds.h, 0.0, 1.0),
        );
        canvas.restore();
    }
}

impl<'v, L> Handle<'v, ColorSelector<L>>
where
    L: Lens<Target = Color>,
{
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Color),
    {
        self.modify(|colorpicker: &mut ColorSelector<L>| {
            colorpicker.on_change = Some(Box::new(callback))
        })
    }
}

fn hsv_to_hsl(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    //   *hh = h;
    let mut ll = (2.0 - s) * v;
    let mut ss = s * v;
    ss /= if ll <= 1.0 { ll } else { 2.0 - ll };
    ll /= 2.0;

    (h, ss, ll)
}

fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let cmax = r.max(g.max(b));
    let cmin = r.min(g.min(b));
    let diff = cmax - cmin;
    let v = cmax;

    let mut h = if cmax == cmin {
        0.0
    } else if cmax == r {
        (g - b) / diff
    } else if cmax == g {
        2.0 + (b - r) / diff
    } else if cmax == b {
        4.0 + (r - g) / diff
    } else {
        0.0
    };

    h *= 60.0;

    if h < 0.0 {
        h += 360.0;
    }

    h /= 360.0;

    let s = if v == 0.0 { 0.0 } else { diff / v };

    (h, s, v)
}
