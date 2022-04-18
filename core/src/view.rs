use std::{any::Any, collections::HashMap};

use crate::{
    idx_to_pos, measure_text_lines,
    style::{BorderCornerShape, GradientDirection},
    text_layout, text_paint_draw, Context, DrawContext, Event, Handle, ModelDataStore, ViewHandler,
};

use femtovg::{
    renderer::OpenGl, Align, Baseline, ImageFlags, Paint, Path, PixelFormat, RenderTarget,
    TextMetrics,
};
use morphorm::Units;

pub type Canvas = femtovg::Canvas<OpenGl>;

// Length proportional to radius of a cubic bezier handle for 90deg arcs.
const KAPPA90: f32 = 0.5522847493;

pub trait View: 'static + Sized {
    #[allow(unused_variables)]
    fn body(&mut self, cx: &mut Context) {}
    fn build<F>(self, cx: &mut Context, builder: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        let id = cx.entity_manager.create();
        cx.tree.add(id, cx.current).expect("Failed to add to tree");
        cx.cache.add(id).expect("Failed to add to cache");
        cx.style.add(id);
        cx.views.insert(id, Box::new(self));

        cx.data
            .insert(
                id,
                ModelDataStore {
                    data: HashMap::default(),
                    lenses_dedup: HashMap::default(),
                    lenses_dup: vec![],
                },
            )
            .expect("Failed to insert model data store");

        let handle = Handle { entity: id, p: Default::default(), cx };

        let prev = handle.cx.current;
        handle.cx.current = handle.entity;

        (builder)(handle.cx);

        handle.cx.current = prev;

        handle
    }

    fn element(&self) -> Option<String> {
        None
    }

    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut Context, event: &mut Event) {}

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        //println!("{}", debug(&mut context, entity));
        let entity = cx.current();

        let bounds = cx.cache().get_bounds(entity);

        //Skip widgets with no width or no height
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let background_color = cx.background_color(entity).cloned().unwrap_or_default();

        let font_color = cx.font_color(entity).cloned().unwrap_or(crate::Color::rgb(0, 0, 0));

        let border_color = cx.border_color(entity).cloned().unwrap_or_default();

        let parent = cx
            .tree()
            .get_layout_parent(entity)
            .expect(&format!("Failed to find parent somehow: {}", entity));

        let parent_width = cx.cache().get_width(parent);
        let parent_height = cx.cache().get_height(parent);

        let border_shape_top_left = cx.border_shape_top_left(entity).cloned().unwrap_or_default();

        let border_shape_top_right = cx.border_shape_top_right(entity).cloned().unwrap_or_default();

        let border_shape_bottom_left =
            cx.border_shape_bottom_left(entity).cloned().unwrap_or_default();

        let border_shape_bottom_right =
            cx.border_shape_bottom_right(entity).cloned().unwrap_or_default();

        let border_radius_top_left = cx
            .border_radius_top_left(entity)
            .unwrap_or_default()
            .value_or(bounds.w.min(bounds.h), 0.0);

        let border_radius_top_right = cx
            .border_radius_top_right(entity)
            .unwrap_or_default()
            .value_or(bounds.w.min(bounds.h), 0.0);

        let border_radius_bottom_left = cx
            .border_radius_bottom_left(entity)
            .unwrap_or_default()
            .value_or(bounds.w.min(bounds.h), 0.0);
        let border_radius_bottom_right = cx
            .border_radius_bottom_right(entity)
            .unwrap_or_default()
            .value_or(bounds.w.min(bounds.h), 0.0);

        let opacity = cx.cache().get_opacity(entity);

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut border_color: femtovg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);

        let border_width =
            cx.border_width(entity).unwrap_or_default().value_or(bounds.w.min(bounds.h), 0.0);

        let outer_shadow_h_offset =
            cx.outer_shadow_h_offset(entity).unwrap_or_default().value_or(bounds.w, 0.0);
        let outer_shadow_v_offset =
            cx.outer_shadow_v_offset(entity).unwrap_or_default().value_or(bounds.w, 0.0);
        let outer_shadow_blur =
            cx.outer_shadow_blur(entity).unwrap_or_default().value_or(bounds.w, 0.0);

        let outer_shadow_color = cx.outer_shadow_color(entity).cloned().unwrap_or_default();

        let mut outer_shadow_color: femtovg::Color = outer_shadow_color.into();
        outer_shadow_color.set_alphaf(outer_shadow_color.a * opacity);

        let _inner_shadow_h_offset =
            cx.inner_shadow_h_offset(entity).unwrap_or_default().value_or(bounds.w, 0.0);

        let _inner_shadow_v_offset =
            cx.inner_shadow_v_offset(entity).unwrap_or_default().value_or(bounds.w, 0.0);

        let _inner_shadow_blur =
            cx.inner_shadow_blur(entity).unwrap_or_default().value_or(bounds.w, 0.0);

        let inner_shadow_color = cx.inner_shadow_color(entity).cloned().unwrap_or_default();

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

        if cx.outer_shadow_color(entity).is_some() {
            let sigma = outer_shadow_blur / 2.0;
            let d = (sigma * 5.0).ceil();

            let shadow_image = cx.cache().shadow_image.get(&entity).cloned().unwrap_or_else(|| {
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

            cx.cache().shadow_image.insert(entity, (source, target));

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
        if let Some(background_gradient) = cx.background_gradient(entity) {
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
        if let Some(background_image) = cx.background_image(entity) {
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
        if cx.text(entity).is_some() || cx.image(entity).is_some() {
            let mut x = bounds.x;
            let mut y = bounds.y;
            let mut w = bounds.w;
            let mut h = bounds.h;

            // TODO - Move this to a text layout system and include constraints
            let child_left = cx.child_left(entity).unwrap_or_default();
            let child_right = cx.child_right(entity).unwrap_or_default();
            let child_top = cx.child_top(entity).unwrap_or_default();
            let child_bottom = cx.child_bottom(entity).unwrap_or_default();

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
                        Baseline::Middle
                    }

                    _ => Baseline::Bottom,
                },

                _ => Baseline::Top,
            };

            // Draw image
            if let Some(image) = cx.image(entity).cloned() {
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

            if let Some(text) = cx.text(entity).cloned() {
                // let mut x = posx + (border_width / 2.0);
                // let mut y = posy + (border_width / 2.0);

                let mut font_color: femtovg::Color = font_color.into();
                font_color.set_alphaf(font_color.a * opacity);

                let text_wrap = cx.text_wrap(entity).cloned().unwrap_or(true);

                let mut paint = text_paint_draw(cx, entity);
                paint.set_color(font_color);
                paint.set_text_align(align);
                paint.set_text_baseline(baseline);

                let font_metrics =
                    cx.text_context().measure_font(paint).expect("Failed to read font metrics");

                let text_width = if text_wrap { w } else { f32::MAX };

                if let Ok(lines) = text_layout(text_width, &text, paint, cx.text_context()) {
                    // difference between first line and last line
                    let delta_height = font_metrics.height() * (lines.len() - 1) as f32;
                    let first_line_y = match baseline {
                        Baseline::Top => y,
                        Baseline::Middle => y - delta_height / 2.0,
                        Baseline::Alphabetic | Baseline::Bottom => y - delta_height,
                    };
                    let metrics = measure_text_lines(
                        &text,
                        paint,
                        &lines,
                        x,
                        first_line_y,
                        cx.text_context(),
                    );
                    let cached: Vec<(std::ops::Range<usize>, TextMetrics)> =
                        lines.into_iter().zip(metrics.into_iter()).collect();
                    let selection = cx.text_selection(entity);
                    let (anchor, active) = if let Some(cursor) = selection {
                        (
                            idx_to_pos(cursor.anchor, cached.iter()),
                            idx_to_pos(cursor.active, cached.iter()),
                        )
                    } else {
                        ((usize::MAX, (f32::MAX, f32::MAX)), (usize::MAX, (f32::MAX, f32::MAX)))
                    };
                    let (first, last) = if let Some(cursor) = &selection {
                        if cursor.anchor < cursor.active {
                            (anchor, active)
                        } else {
                            (active, anchor)
                        }
                    } else {
                        (anchor, active)
                    };
                    let selection_color = cx.selection_color(entity);
                    let cursor_color = cx.caret_color(entity);
                    for (line, (range, metrics)) in cached.iter().enumerate() {
                        let y = first_line_y + line as f32 * font_metrics.height();
                        let min_y = match baseline {
                            Baseline::Top => y,
                            Baseline::Middle => y - font_metrics.height() / 2.0,
                            Baseline::Alphabetic | Baseline::Bottom => y - font_metrics.height(),
                        };
                        // should we draw part of the selection?
                        if let Some(color) = selection_color {
                            if line >= first.0 && line <= last.0 && first != last {
                                let first_x = if line == first.0 {
                                    first.1 .0
                                } else if let Some(glyph) = metrics.glyphs.first() {
                                    glyph.x
                                } else {
                                    x
                                };
                                let last_x = if line == last.0 {
                                    last.1 .0
                                } else if let Some(glyph) = metrics.glyphs.last() {
                                    glyph.x + glyph.advance_x
                                } else {
                                    x + 10.0
                                };
                                let min_x = first_x.min(last_x).round();
                                let max_x = first_x.max(last_x).round();
                                let mut path = Path::new();
                                path.rect(min_x, min_y, max_x - min_x, font_metrics.height());
                                canvas.fill_path(&mut path, Paint::color(color.clone().into()));
                            }
                        }
                        // should we draw the cursor?
                        if let Some(color) = cursor_color {
                            if line == active.0 {
                                let (x, _) = active.1;
                                let x = x.round();
                                let mut path = Path::new();
                                path.rect(
                                    x,
                                    min_y,
                                    cx.logical_to_physical(1.0),
                                    font_metrics.height(),
                                );
                                canvas.fill_path(&mut path, Paint::color(color.clone().into()));
                            }
                        }
                        canvas.fill_text(x, y, &text[range.clone()], paint).ok();
                    }

                    cx.cache().text_lines.insert(entity, cached).unwrap();
                }
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

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        <T as View>::draw(self, cx, canvas);
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
