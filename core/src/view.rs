use crate::{
    style::{BorderCornerShape, GradientDirection},
    Context, Event, FontOrId, Handle, ViewHandler,
};

use femtovg::{
    renderer::OpenGl, Align, Baseline, ImageFlags, Paint, Path, PixelFormat, RenderTarget,
};
use morphorm::Units;

pub type Canvas = femtovg::Canvas<OpenGl>;

// Length proportional to radius of a cubic bezier handle for 90deg arcs.
const KAPPA90: f32 = 0.5522847493;

pub trait View: 'static + Sized {
    #[allow(unused_variables)]
    fn body(&mut self, cx: &mut Context) {}
    fn build2<F>(self, cx: &mut Context, builder: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        // Add the instance to context even if it already exists

        let id = cx.entity_manager.create();
        cx.tree.add(id, cx.current).expect("Failed to add to tree");
        cx.cache.add(id).expect("Failed to add to cache");
        cx.style.add(id);
        cx.views.insert(id, Box::new(self));

        let handle = Handle { entity: id, p: Default::default(), cx };

        // ...and this part
        let prev = handle.cx.current;
        handle.cx.current = handle.entity;

        (builder)(handle.cx);

        // This part will also be moved somewhere else
        handle.cx.current = prev;

        handle
    }

    fn build(mut self, cx: &mut Context) -> Handle<Self> {
        let id = cx.entity_manager.create();
        cx.tree.add(id, cx.current).expect("Failed to add to tree");
        cx.cache.add(id).expect("Failed to add to cache");
        cx.style.add(id);
        let prev = cx.current;
        cx.current = id;
        self.body(cx);
        cx.current = prev;
        cx.views.insert(id, Box::new(self));

        Handle { entity: id, p: Default::default(), cx }
    }

    fn element(&self) -> Option<String> {
        None
    }

    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut Context, event: &mut Event) {}

    fn draw(&self, cx: &mut Context, canvas: &mut Canvas) {
        //println!("{}", debug(&mut context, entity));
        let entity = cx.current;

        let bounds = cx.cache.get_bounds(entity);

        //Skip widgets with no width or no height
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let _padding_left = match cx.style.child_left.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let _padding_right = match cx.style.child_right.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let _padding_top = match cx.style.child_top.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let _padding_bottom = match cx.style.child_bottom.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let background_color = cx.style.background_color.get(entity).cloned().unwrap_or_default();

        let font_color =
            cx.style.font_color.get(entity).cloned().unwrap_or(crate::Color::rgb(0, 0, 0));

        let border_color = cx.style.border_color.get(entity).cloned().unwrap_or_default();

        let parent = cx
            .tree
            .get_layout_parent(entity)
            .expect(&format!("Failed to find parent somehow: {}", entity));

        let parent_width = cx.cache.get_width(parent);
        let parent_height = cx.cache.get_height(parent);

        let border_shape_top_left =
            cx.style.border_shape_top_left.get(entity).cloned().unwrap_or_default();

        let border_shape_top_right =
            cx.style.border_shape_top_right.get(entity).cloned().unwrap_or_default();

        let border_shape_bottom_left =
            cx.style.border_shape_bottom_left.get(entity).cloned().unwrap_or_default();

        let border_shape_bottom_right =
            cx.style.border_shape_bottom_right.get(entity).cloned().unwrap_or_default();

        let border_radius_top_left =
            match cx.style.border_radius_top_left.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
                _ => 0.0,
            };

        let border_radius_top_right =
            match cx.style.border_radius_top_right.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
                _ => 0.0,
            };

        let border_radius_bottom_left =
            match cx.style.border_radius_bottom_left.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
                _ => 0.0,
            };

        let border_radius_bottom_right =
            match cx.style.border_radius_bottom_right.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
                _ => 0.0,
            };

        let opacity = cx.cache.get_opacity(entity);

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut border_color: femtovg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);

        let border_width = match cx.style.border_width.get(entity).cloned().unwrap_or_default() {
            Units::Pixels(val) => val,
            Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
            _ => 0.0,
        };

        let outer_shadow_h_offset =
            match cx.style.outer_shadow_h_offset.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let outer_shadow_v_offset =
            match cx.style.outer_shadow_v_offset.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let outer_shadow_blur =
            match cx.style.outer_shadow_blur.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let outer_shadow_color =
            cx.style.outer_shadow_color.get(entity).cloned().unwrap_or_default();

        let mut outer_shadow_color: femtovg::Color = outer_shadow_color.into();
        outer_shadow_color.set_alphaf(outer_shadow_color.a * opacity);

        let _inner_shadow_h_offset =
            match cx.style.inner_shadow_h_offset.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let _inner_shadow_v_offset =
            match cx.style.inner_shadow_v_offset.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let _inner_shadow_blur =
            match cx.style.inner_shadow_blur.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let inner_shadow_color =
            cx.style.inner_shadow_color.get(entity).cloned().unwrap_or_default();

        let mut inner_shadow_color: femtovg::Color = inner_shadow_color.into();
        inner_shadow_color.set_alphaf(inner_shadow_color.a * opacity);

        // // Draw outer shadow
        // let mut path = Path::new();
        // path.rounded_rect_varying(
        //     bounds.x - outer_shadow_blur + outer_shadow_h_offset,
        //     bounds.y - outer_shadow_blur + outer_shadow_v_offset,
        //     bounds.w + 2.0 * outer_shadow_blur,
        //     bounds.h + 2.0 * outer_shadow_blur,
        //     border_radius_top_left,
        //     border_radius_top_right,
        //     border_radius_bottom_right,
        //     border_radius_bottom_left,
        // );
        // path.rounded_rect_varying(
        //     bounds.x,
        //     bounds.y,
        //     bounds.w,
        //     bounds.h,
        //     border_radius_top_left,
        //     border_radius_top_right,
        //     border_radius_bottom_right,
        //     border_radius_bottom_left,
        // );
        // path.solidity(Solidity::Hole);

        // let mut paint = Paint::box_gradient(
        //     bounds.x + outer_shadow_h_offset,
        //     bounds.y + outer_shadow_v_offset,
        //     bounds.w,
        //     bounds.h,
        //     border_radius_top_left
        //         .max(border_radius_top_right)
        //         .max(border_radius_bottom_left)
        //         .max(border_radius_bottom_right),
        //     outer_shadow_blur,
        //     outer_shadow_color,
        //     femtovg::Color::rgba(0, 0, 0, 0),
        // );

        // canvas.fill_path(&mut path, paint);

        //let start = instant::Instant::now();
        let mut path = Path::new();

        if bounds.w == bounds.h
            && border_radius_bottom_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_bottom_right == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_right == (bounds.w - 2.0 * border_width) / 2.0
        {
            path.circle(
                bounds.x + (border_width / 2.0) + (bounds.w - border_width) / 2.0,
                bounds.y + (border_width / 2.0) + (bounds.h - border_width) / 2.0,
                bounds.w / 2.0,
            );
        } else {
            let x = bounds.x + border_width / 2.0;
            let y = bounds.y + border_width / 2.0;
            let w = bounds.w - border_width;
            let h = bounds.h - border_width;
            let halfw = w.abs() * 0.5;
            let halfh = h.abs() * 0.5;

            let rx_bl = border_radius_bottom_left.min(halfw) * w.signum();
            let ry_bl = border_radius_bottom_left.min(halfh) * h.signum();

            let rx_br = border_radius_bottom_right.min(halfw) * w.signum();
            let ry_br = border_radius_bottom_right.min(halfh) * h.signum();

            let rx_tr = border_radius_top_right.min(halfw) * w.signum();
            let ry_tr = border_radius_top_right.min(halfh) * h.signum();

            let rx_tl = border_radius_top_left.min(halfw) * w.signum();
            let ry_tl = border_radius_top_left.min(halfh) * h.signum();

            path.move_to(x, y + ry_tl);
            path.line_to(x, y + h - ry_bl);
            if border_radius_bottom_left != 0.0 {
                if border_shape_bottom_left == BorderCornerShape::Round {
                    path.bezier_to(
                        x,
                        y + h - ry_bl * (1.0 - KAPPA90),
                        x + rx_bl * (1.0 - KAPPA90),
                        y + h,
                        x + rx_bl,
                        y + h,
                    );
                } else {
                    path.line_to(x + rx_bl, y + h);
                }
            }

            path.line_to(x + w - rx_br, y + h);

            if border_radius_bottom_right != 0.0 {
                if border_shape_bottom_right == BorderCornerShape::Round {
                    path.bezier_to(
                        x + w - rx_br * (1.0 - KAPPA90),
                        y + h,
                        x + w,
                        y + h - ry_br * (1.0 - KAPPA90),
                        x + w,
                        y + h - ry_br,
                    );
                } else {
                    path.line_to(x + w, y + h - ry_br);
                }
            }

            path.line_to(x + w, y + ry_tr);

            if border_radius_top_right != 0.0 {
                if border_shape_top_right == BorderCornerShape::Round {
                    path.bezier_to(
                        x + w,
                        y + ry_tr * (1.0 - KAPPA90),
                        x + w - rx_tr * (1.0 - KAPPA90),
                        y,
                        x + w - rx_tr,
                        y,
                    );
                } else {
                    path.line_to(x + w - rx_tr, y);
                }
            }

            path.line_to(x + rx_tl, y);

            if border_radius_top_left != 0.0 {
                if border_shape_top_left == BorderCornerShape::Round {
                    path.bezier_to(
                        x + rx_tl * (1.0 - KAPPA90),
                        y,
                        x,
                        y + ry_tl * (1.0 - KAPPA90),
                        x,
                        y + ry_tl,
                    );
                } else {
                    path.line_to(x, y + ry_tl);
                }
            }

            path.close();
        }

        // Draw outer shadow

        if cx.style.outer_shadow_color.get(entity).is_some() {
            let sigma = outer_shadow_blur / 2.0;
            let d = (sigma * 5.0).ceil();

            let shadow_image = cx.cache.shadow_image.get(&entity).cloned().unwrap_or_else(|| {
                (
                    canvas
                        .create_image_empty(
                            (bounds.w + d) as usize,
                            (bounds.h + d) as usize,
                            PixelFormat::Rgba8,
                            ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                        )
                        .expect("Failed to create image"),
                    canvas
                        .create_image_empty(
                            (bounds.w + d) as usize,
                            (bounds.h + d) as usize,
                            PixelFormat::Rgba8,
                            ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                        )
                        .expect("Failed to create image"),
                )
            });

            canvas.save();

            let size = canvas.image_size(shadow_image.0).expect("Failed to get image");

            let (source, target) =
                if size.0 != (bounds.w + d) as usize || size.1 != (bounds.h + d) as usize {
                    canvas.delete_image(shadow_image.0);
                    canvas.delete_image(shadow_image.1);

                    (
                        canvas
                            .create_image_empty(
                                (bounds.w + d) as usize,
                                (bounds.h + d) as usize,
                                PixelFormat::Rgba8,
                                ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                            )
                            .expect("Failed to create image"),
                        canvas
                            .create_image_empty(
                                (bounds.w + d) as usize,
                                (bounds.h + d) as usize,
                                PixelFormat::Rgba8,
                                ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                            )
                            .expect("Failed to create image"),
                    )
                } else {
                    (shadow_image.0, shadow_image.1)
                };

            cx.cache.shadow_image.insert(entity, (source, target));

            canvas.set_render_target(RenderTarget::Image(source));
            canvas.clear_rect(0, 0, size.0 as u32, size.1 as u32, femtovg::Color::rgba(0, 0, 0, 0));
            canvas.translate(-bounds.x + d / 2.0, -bounds.y + d / 2.0);
            let mut outer_shadow = path.clone();
            let paint = Paint::color(outer_shadow_color);
            canvas.fill_path(&mut outer_shadow, paint);

            canvas.restore();

            let target_image = if outer_shadow_blur > 0.0 {
                canvas.filter_image(target, femtovg::ImageFilter::GaussianBlur { sigma }, source);
                target
            } else {
                source
            };

            canvas.set_render_target(RenderTarget::Screen);

            canvas.save();
            canvas.translate(outer_shadow_h_offset, outer_shadow_v_offset);
            let mut path = Path::new();
            path.rect(bounds.x - d / 2.0, bounds.y - d / 2.0, bounds.w + d, bounds.h + d);

            canvas.fill_path(
                &mut path,
                Paint::image(
                    target_image,
                    bounds.x - d / 2.0,
                    bounds.y - d / 2.0,
                    bounds.w + d,
                    bounds.h + d,
                    0f32,
                    1f32,
                ),
            );
            //canvas.fill_path(&mut path, Paint::color(femtovg::Color::rgb(0,0,0)));
            canvas.restore();
        }

        // Fill with background color
        let mut paint = Paint::color(background_color);

        // Gradient overrides background color
        if let Some(background_gradient) = cx.style.background_gradient.get(entity) {
            let (_, _, end_x, end_y, parent_length) = match background_gradient.direction {
                GradientDirection::LeftToRight => (0.0, 0.0, bounds.w, 0.0, parent_width),
                GradientDirection::TopToBottom => (0.0, 0.0, 0.0, bounds.h, parent_height),
                _ => (0.0, 0.0, bounds.w, 0.0, parent_width),
            };

            paint = Paint::linear_gradient_stops(
                bounds.x,
                bounds.y,
                bounds.x + end_x,
                bounds.y + end_y,
                background_gradient
                    .get_stops(parent_length)
                    .iter()
                    .map(|stop| {
                        let col: femtovg::Color = stop.1.into();
                        (stop.0, col)
                    })
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
        }

        // background-image overrides gradient
        // TODO should we draw image on top of colors?
        if let Some(background_image) = cx.style.background_image.get(entity) {
            let background_image = background_image.clone(); // not ideal
            let img = cx.get_image(&background_image);

            let dim = img.dimensions();
            let id = img.id(canvas);
            paint = Paint::image(id, bounds.x, bounds.y, dim.0 as f32, dim.1 as f32, 0.0, 1.0);
        }

        //canvas.global_composite_blend_func(BlendFactor::DstColor, BlendFactor::OneMinusSrcAlpha);

        // Fill the quad
        canvas.fill_path(&mut path, paint);

        //println!("{:.2?} seconds for whatever you did.", start.elapsed());

        // Draw border
        let mut paint = Paint::color(border_color);
        paint.set_line_width(border_width);
        canvas.stroke_path(&mut path, paint);

        // // Draw inner shadow
        // let mut path = Path::new();
        // path.rounded_rect_varying(
        //     0.0 + border_width,
        //     0.0 + border_width,
        //     bounds.w - border_width * 2.0,
        //     bounds.h - border_width * 2.0,
        //     border_radius_top_left,
        //     border_radius_top_right,
        //     border_radius_bottom_right,
        //     border_radius_bottom_left,
        // );

        // let mut paint = Paint::box_gradient(
        //     0.0 + inner_shadow_h_offset + border_width,
        //     0.0 + inner_shadow_v_offset + border_width,
        //     bounds.w - border_width * 2.0,
        //     bounds.h - border_width * 2.0,
        //     border_radius_top_left
        //         .max(border_radius_top_right)
        //         .max(border_radius_bottom_left)
        //         .max(border_radius_bottom_right),
        //     inner_shadow_blur,
        //     femtovg::Color::rgba(0, 0, 0, 0),
        //     inner_shadow_color,
        // );
        // canvas.fill_path(&mut path, paint);

        // Draw text and image
        if cx.style.text.get(entity).is_some() || cx.style.image.get(entity).is_some() {
            let mut x = bounds.x;
            let mut y = bounds.y;
            let mut w = bounds.w;
            let mut h = bounds.h;

            // TODO - Move this to a text layout system and include constraints
            let child_left = cx.style.child_left.get(entity).cloned().unwrap_or_default();
            let child_right = cx.style.child_right.get(entity).cloned().unwrap_or_default();
            let child_top = cx.style.child_top.get(entity).cloned().unwrap_or_default();
            let child_bottom = cx.style.child_bottom.get(entity).cloned().unwrap_or_default();

            let align = match child_left {
                Units::Pixels(val) => match child_right {
                    Units::Stretch(_) | Units::Auto => {
                        x += val + border_width;
                        w -= val + border_width;
                        Align::Left
                    }

                    _ => Align::Left,
                },

                Units::Stretch(_) => match child_right {
                    Units::Pixels(val) => {
                        x += bounds.w - val - border_width;
                        w -= val - border_width;
                        Align::Right
                    }

                    Units::Stretch(_) => {
                        x += 0.5 * bounds.w;
                        w -= 0.5 * bounds.w;
                        Align::Center
                    }

                    _ => Align::Right,
                },

                _ => Align::Left,
            };

            let baseline = match child_top {
                Units::Pixels(val) => match child_bottom {
                    Units::Stretch(_) | Units::Auto => {
                        y += val + border_width;
                        h -= val + border_width;
                        Baseline::Top
                    }

                    _ => Baseline::Top,
                },

                Units::Stretch(_) => match child_bottom {
                    Units::Pixels(val) => {
                        y += bounds.h - val - border_width;
                        h -= val - border_width;
                        Baseline::Bottom
                    }

                    Units::Stretch(_) => {
                        y += 0.5 * bounds.h;
                        h -= 0.5 * bounds.h;
                        Baseline::Middle
                    }

                    _ => Baseline::Bottom,
                },

                _ => Baseline::Top,
            };

            // Draw image
            if let Some(image) = cx.style.image.get(entity).cloned() {
                let image = cx.get_image(&image);
                let x = match align {
                    Align::Left => x,
                    Align::Center => x - w * 0.5,
                    Align::Right => x - w,
                };
                let y = match baseline {
                    Baseline::Top => y,
                    Baseline::Middle => y - h * 0.5,
                    Baseline::Alphabetic | Baseline::Bottom => y - h,
                };

                let mut path = Path::new();
                path.rect(x, y, w, h);

                let paint = Paint::image(image.id(canvas), x, y, w, h, 0.0, 1.0);
                canvas.fill_path(&mut path, paint);
            }

            if let Some(text) = cx.style.text.get(entity).cloned() {
                let font = cx.style.font.get(entity).cloned().unwrap_or_default();

                // TODO - This should probably be cached in cx to save look-up time
                let default_font = cx
                    .resource_manager
                    .fonts
                    .get(&cx.style.default_font)
                    .and_then(|font| match font {
                        FontOrId::Id(id) => Some(id),
                        _ => None,
                    })
                    .expect("Failed to find default font");

                let font_id = cx
                    .resource_manager
                    .fonts
                    .get(&font)
                    .and_then(|font| match font {
                        FontOrId::Id(id) => Some(id),
                        _ => None,
                    })
                    .unwrap_or(default_font);

                // let mut x = posx + (border_width / 2.0);
                // let mut y = posy + (border_width / 2.0);

                let mut font_color: femtovg::Color = font_color.into();
                font_color.set_alphaf(font_color.a * opacity);

                let font_size = cx.style.font_size.get(entity).cloned().unwrap_or(16.0);

                let mut paint = Paint::color(font_color);
                paint.set_font_size(font_size);
                paint.set_font(&[font_id.clone()]);
                paint.set_text_align(align);
                paint.set_text_baseline(baseline);
                paint.set_anti_alias(false);

                canvas.fill_text(x, y, &text, paint).unwrap();
            }
        }
    }
}

impl<T: View> ViewHandler for T
where
    T: std::marker::Sized + View + 'static,
{
    fn element(&self) -> Option<String> {
        <T as View>::element(&self)
    }

    fn body(&mut self, cx: &mut Context) {
        <T as View>::body(self, cx);
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        <T as View>::event(self, cx, event);
    }

    fn draw(&self, cx: &mut Context, canvas: &mut Canvas) {
        <T as View>::draw(self, cx, canvas);
    }
}
