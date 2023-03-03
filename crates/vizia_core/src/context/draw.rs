use cosmic_text::FamilyOwned;
use femtovg::Transform2D;
use std::any::{Any, TypeId};

use fnv::FnvHashMap;
use morphorm::Units;

use crate::cache::{BoundingBox, CachedData};
use crate::events::ViewHandler;
use crate::prelude::*;
use crate::resource::ResourceManager;
use crate::state::ModelDataStore;
use crate::style::{IntoTransform, Style};
use crate::text::{TextConfig, TextContext};
use crate::vg::{ImageId, Paint, Path};
use vizia_input::{Modifiers, MouseState};
use vizia_storage::SparseSet;
use vizia_style::{
    BoxShadow, Clip, Gradient, HorizontalPositionKeyword, LineDirection, VerticalPositionKeyword,
};

/// Cached data used for drawing.
pub struct DrawCache {
    pub shadow_image: SparseSet<Vec<(ImageId, ImageId)>>,
}

impl DrawCache {
    pub fn new() -> Self {
        Self { shadow_image: SparseSet::new() }
    }

    pub fn remove(&mut self, entity: Entity) {
        self.shadow_image.remove(entity);
    }
}

/// A restricted context used when drawing.
pub struct DrawContext<'a> {
    pub(crate) current: Entity,
    pub captured: &'a Entity,
    pub focused: &'a Entity,
    pub hovered: &'a Entity,
    pub style: &'a Style,
    pub cache: &'a CachedData,
    pub draw_cache: &'a mut DrawCache,
    pub tree: &'a Tree<Entity>,
    pub(crate) data: &'a SparseSet<ModelDataStore>,
    pub views: &'a mut FnvHashMap<Entity, Box<dyn ViewHandler>>,
    pub resource_manager: &'a ResourceManager,
    pub text_context: &'a mut TextContext,
    pub text_config: &'a TextConfig,
    pub modifiers: &'a Modifiers,
    pub mouse: &'a MouseState<Entity>,
}

macro_rules! style_getter_units {
    ($name:ident) => {
        pub fn $name(&self) -> Units {
            let result = self.style.$name.get(self.current);
            if let Some(Units::Pixels(p)) = result {
                Units::Pixels(self.logical_to_physical(*p))
            } else {
                result.copied().unwrap_or_default()
            }
        }
    };
}

// macro_rules! get_property {
//     ($ty:ty, $name:ident) => {
//         pub fn $name(&self) -> $ty {
//             self.style.$name.get(self.current).copied().unwrap_or_default()
//         }
//     };
// }

macro_rules! get_color_property {
    ($ty:ty, $name:ident) => {
        pub fn $name(&self) -> $ty {
            let opacity = self.cache.get_opacity(self.current);
            if let Some(col) = self.style.$name.get(self.current) {
                Color::rgba(col.r(), col.g(), col.b(), (opacity * col.a() as f32) as u8)
            } else {
                Color::rgba(0, 0, 0, 0)
            }
        }
    };
}

macro_rules! get_length_property {
    ($name:ident) => {
        pub fn $name(&self) -> f32 {
            if let Some(length) = self.style.$name.get(self.current) {
                let bounds = self.bounds();

                let px = length.to_pixels(bounds.w.min(bounds.h));
                return self.logical_to_physical(px).round();
            }

            0.0
        }
    };
}

impl<'a> DrawContext<'a> {
    /// Creates a new `DrawContext` from the given `Context`.
    pub fn new(cx: &'a mut Context) -> Self {
        Self {
            current: cx.current,
            captured: &cx.captured,
            focused: &cx.focused,
            hovered: &cx.hovered,
            style: &cx.style,
            cache: &mut cx.cache,
            draw_cache: &mut cx.draw_cache,
            tree: &cx.tree,
            data: &cx.data,
            views: &mut cx.views,
            resource_manager: &cx.resource_manager,
            text_context: &mut cx.text_context,
            text_config: &cx.text_config,
            modifiers: &cx.modifiers,
            mouse: &cx.mouse,
        }
    }

    pub fn bounds(&self) -> BoundingBox {
        self.cache.get_bounds(self.current)
    }

    pub fn clip_region(&self) -> BoundingBox {
        let bounds = self.bounds();
        let overflowx = self.style.overflowx.get(self.current).copied().unwrap_or_default();
        let overflowy = self.style.overflowy.get(self.current).copied().unwrap_or_default();

        let root_bounds = self.cache.get_bounds(Entity::root());

        let clip_bounds = self
            .style
            .clip
            .get(self.current)
            .map(|clip| match clip {
                Clip::Auto => bounds,
                Clip::Shape(rect) => bounds.shrink_sides(
                    self.logical_to_physical(rect.3.to_px().unwrap()),
                    self.logical_to_physical(rect.0.to_px().unwrap()),
                    self.logical_to_physical(rect.1.to_px().unwrap()),
                    self.logical_to_physical(rect.2.to_px().unwrap()),
                ),
            })
            .unwrap_or(bounds);

        match (overflowx, overflowy) {
            (Overflow::Visible, Overflow::Visible) => root_bounds,
            (Overflow::Hidden, Overflow::Visible) => {
                let left = clip_bounds.left();
                let right = clip_bounds.right();
                let top = root_bounds.top();
                let bottom = root_bounds.bottom();
                BoundingBox::from_min_max(left, top, right, bottom)
            }
            (Overflow::Visible, Overflow::Hidden) => {
                let left = root_bounds.left();
                let right = root_bounds.right();
                let top = clip_bounds.top();
                let bottom = clip_bounds.bottom();
                BoundingBox::from_min_max(left, top, right, bottom)
            }
            (Overflow::Hidden, Overflow::Hidden) => clip_bounds,
        }
    }

    pub fn transform(&self) -> Option<Transform2D> {
        if let Some(transforms) = self.style.transform.get(self.current) {
            let bounds = self.bounds();
            let scale_factor = self.scale_factor();
            let mut translate = Transform2D::new_translation(bounds.center().0, bounds.center().1);

            let mut transform = Transform2D::identity();
            transform.premultiply(&translate);

            translate.inverse();

            // Check if the transform is currently animating
            // Get the animation state
            // Manually interpolate the value to get the overall transform for the current frame
            if let Some(animation_state) = self.style.transform.get_active_animation(self.current) {
                if let Some(start) = animation_state.keyframes.first() {
                    if let Some(end) = animation_state.keyframes.last() {
                        let start_transform = start.1.into_transform(bounds, scale_factor);
                        let end_transform = end.1.into_transform(bounds, scale_factor);
                        let t = animation_state.t;
                        let animated_transform =
                            Transform2D::interpolate(&start_transform, &end_transform, t);
                        transform.premultiply(&animated_transform);
                    }
                }
            } else {
                transform.premultiply(&transforms.into_transform(bounds, scale_factor));
            }

            transform.premultiply(&translate);

            return Some(transform);
        }

        None
    }

    pub fn visibility(&self) -> Option<Visibility> {
        self.style.visibility.get(self.current).copied()
    }

    /// Returns the lookup pattern to pick the default font.
    pub fn default_font(&self) -> &[FamilyOwned] {
        &self.style.default_font
    }

    /// Returns the font-size of the current entity in physical coordinates.
    pub fn font_size(&self, entity: Entity) -> f32 {
        self.logical_to_physical(
            self.style.font_size.get(entity).copied().map(|f| f.0).unwrap_or(16.0),
        )
    }

    /// Function to convert logical points to physical pixels.
    pub fn logical_to_physical(&self, logical: f32) -> f32 {
        self.style.logical_to_physical(logical)
    }

    /// Function to convert physical pixels to logical points.
    pub fn physical_to_logical(&self, physical: f32) -> f32 {
        self.style.physical_to_logical(physical)
    }

    get_length_property!(border_width);
    get_length_property!(outline_width);
    get_length_property!(outline_offset);
    get_length_property!(border_top_left_radius);
    get_length_property!(border_top_right_radius);
    get_length_property!(border_bottom_left_radius);
    get_length_property!(border_bottom_right_radius);

    pub fn border_top_left_shape(&self) -> BorderCornerShape {
        self.style.border_top_left_shape.get(self.current).copied().unwrap_or_default()
    }

    pub fn border_top_right_shape(&self) -> BorderCornerShape {
        self.style.border_top_right_shape.get(self.current).copied().unwrap_or_default()
    }

    pub fn border_bottom_left_shape(&self) -> BorderCornerShape {
        self.style.border_bottom_left_shape.get(self.current).copied().unwrap_or_default()
    }

    pub fn border_bottom_right_shape(&self) -> BorderCornerShape {
        self.style.border_bottom_right_shape.get(self.current).copied().unwrap_or_default()
    }

    style_getter_units!(child_left);
    style_getter_units!(child_right);
    style_getter_units!(child_top);
    style_getter_units!(child_bottom);
    get_color_property!(Color, background_color);
    // get_color_property!(Color, font_color);
    get_color_property!(Color, border_color);
    get_color_property!(Color, outline_color);
    get_color_property!(Color, selection_color);
    get_color_property!(Color, caret_color);

    pub fn font_color(&self) -> Color {
        let opacity = self.cache.get_opacity(self.current);
        if let Some(col) = self.style.font_color.get(self.current) {
            Color::rgba(col.r(), col.g(), col.b(), (opacity * col.a() as f32) as u8)
        } else {
            Color::rgba(0, 0, 0, 255)
        }
    }

    pub fn text_wrap(&self) -> bool {
        self.style.text_wrap.get(self.current).copied().unwrap_or(true)
    }

    // pub fn font(&self) -> Option<&String> {
    //     self.style.font.get(self.current)
    // }

    pub fn image(&self) -> Option<&String> {
        self.style.image.get(self.current)
    }

    pub fn box_shadows(&self) -> Option<&Vec<BoxShadow>> {
        self.style.box_shadow.get(self.current)
    }

    // pub fn text(&self) -> Option<&String> {
    //     self.style.text.get(self.current)
    // }

    pub fn opacity(&self) -> f32 {
        self.cache.get_opacity(self.current)
    }

    pub fn scale_factor(&self) -> f32 {
        self.style.dpi_factor as f32
    }

    pub fn build_path(&mut self) -> Path {
        // Length proportional to radius of a cubic bezier handle for 90deg arcs.
        const KAPPA90: f32 = 0.5522847493;

        let bounds = self.bounds();

        let border_width = self.border_width();

        let border_top_left_radius = self.border_top_left_radius();
        let border_top_right_radius = self.border_top_right_radius();
        let border_bottom_right_radius = self.border_bottom_right_radius();
        let border_bottom_left_radius = self.border_bottom_left_radius();

        let border_top_left_shape = self.border_top_left_shape();
        let border_top_right_shape = self.border_top_right_shape();
        let border_bottom_right_shape = self.border_bottom_right_shape();
        let border_bottom_left_shape = self.border_bottom_left_shape();

        //TODO: Cache the path a regenerate if the bounds change
        let mut path = Path::new();

        if bounds.w == bounds.h
            && border_bottom_left_radius == (bounds.w - 2.0 * border_width) / 2.0
            && border_bottom_right_radius == (bounds.w - 2.0 * border_width) / 2.0
            && border_top_left_radius == (bounds.w - 2.0 * border_width) / 2.0
            && border_top_right_radius == (bounds.w - 2.0 * border_width) / 2.0
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

            let rx_bl = border_bottom_left_radius.min(halfw) * w.signum();
            let ry_bl = border_bottom_left_radius.min(halfh) * h.signum();

            let rx_br = border_bottom_right_radius.min(halfw) * w.signum();
            let ry_br = border_bottom_right_radius.min(halfh) * h.signum();

            let rx_tr = border_top_right_radius.min(halfw) * w.signum();
            let ry_tr = border_top_right_radius.min(halfh) * h.signum();

            let rx_tl = border_top_left_radius.min(halfw) * w.signum();
            let ry_tl = border_top_left_radius.min(halfh) * h.signum();

            path.move_to(x, y + ry_tl);
            path.line_to(x, y + h - ry_bl);
            if border_bottom_left_radius != 0.0 {
                if border_bottom_left_shape == BorderCornerShape::Round {
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

            if border_bottom_right_radius != 0.0 {
                if border_bottom_right_shape == BorderCornerShape::Round {
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

            if border_top_right_radius != 0.0 {
                if border_top_right_shape == BorderCornerShape::Round {
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

            if border_top_left_radius != 0.0 {
                if border_top_left_shape == BorderCornerShape::Round {
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

        path
    }

    pub fn draw_background(&mut self, canvas: &mut Canvas, path: &mut Path) {
        let background_color = self.background_color();
        let paint = Paint::color(background_color.into());
        canvas.fill_path(path, &paint);
    }

    pub fn draw_text_and_selection(&mut self, canvas: &mut Canvas) {
        if self.text_context.has_buffer(self.current) {
            let bounds = self.bounds();
            let border_width = self.border_width();

            let mut box_x = bounds.x + border_width;
            let mut box_y = bounds.y + border_width;
            let mut box_w = bounds.w - border_width * 2.0;
            let mut box_h = bounds.h - border_width * 2.0;

            let child_left = self.child_left();
            let child_right = self.child_right();
            let child_top = self.child_top();
            let child_bottom = self.child_bottom();

            // shrink the bounding box based on pixel values
            if let Pixels(val) = child_left {
                box_x += val;
                box_w -= val;
            }
            if let Pixels(val) = child_right {
                box_w -= val;
            }
            if let Pixels(val) = child_top {
                box_y += val;
                box_h -= val;
            }
            if let Pixels(val) = child_bottom {
                box_h -= val;
            }

            // Draw text

            let justify_x = match (child_left, child_right) {
                (Stretch(left), Stretch(right)) => {
                    if left + right == 0.0 {
                        0.5
                    } else {
                        left / (left + right)
                    }
                }
                (Stretch(_), _) => 1.0,
                _ => 0.0,
            };
            let justify_y = match (child_top, child_bottom) {
                (Stretch(top), Stretch(bottom)) => {
                    if top + bottom == 0.0 {
                        0.5
                    } else {
                        top / (top + bottom)
                    }
                }
                (Stretch(_), _) => 1.0,
                _ => 0.0,
            };

            let origin_x = box_x + box_w * justify_x;
            let origin_y = box_y + (box_h * justify_y).round();

            self.text_context.sync_styles(self.current, &self.style);

            self.draw_highlights(canvas, (origin_x, origin_y), (justify_x, justify_y));
            self.draw_caret(canvas, (origin_x, origin_y), (justify_x, justify_y), 1.0);
            self.draw_text(canvas, (origin_x, origin_y), (justify_x, justify_y));
        }
    }

    pub fn draw_border(&mut self, canvas: &mut Canvas, path: &mut Path) {
        let border_color = self.border_color();
        let border_width = self.border_width();

        let mut paint = Paint::color(border_color.into());
        paint.set_line_width(border_width);
        canvas.stroke_path(path, &paint);
    }

    pub fn draw_outline(&mut self, canvas: &mut Canvas) {
        let bounds = self.bounds();

        let border_top_left_radius = self.border_top_left_radius();
        let border_top_right_radius = self.border_top_left_radius();
        let border_bottom_right_radius = self.border_top_left_radius();
        let border_bottom_left_radius = self.border_top_left_radius();

        let outline_width = self.outline_width();
        let outline_offset = self.outline_offset();
        let outline_color = self.outline_color();

        let mut outline_path = Path::new();
        let half_outline_width = outline_width / 2.0;
        outline_path.rounded_rect_varying(
            bounds.x - half_outline_width - outline_offset,
            bounds.y - half_outline_width - outline_offset,
            bounds.w + outline_width + 2.0 * outline_offset,
            bounds.h + outline_width + 2.0 * outline_offset,
            border_top_left_radius * 1.5,
            border_top_right_radius * 1.5,
            border_bottom_right_radius * 1.5,
            border_bottom_left_radius * 1.5,
        );
        let mut outline_paint = Paint::color(outline_color.into());
        outline_paint.set_line_width(outline_width);
        canvas.stroke_path(&mut outline_path, &outline_paint);
    }

    pub fn draw_inset_box_shadows(&self, canvas: &mut Canvas, path: &mut Path) {
        if let Some(box_shadows) = self.box_shadows() {
            for box_shadow in box_shadows.iter().rev().filter(|shadow| shadow.inset) {
                let color = box_shadow.color.unwrap_or_default();
                let x_offset = box_shadow.x_offset.to_px().unwrap_or(0.0) * self.scale_factor();
                let y_offset = box_shadow.y_offset.to_px().unwrap_or(0.0) * self.scale_factor();
                let spread_radius =
                    box_shadow.spread_radius.as_ref().and_then(|l| l.to_px()).unwrap_or(0.0)
                        * self.scale_factor();

                let blur_radius =
                    box_shadow.blur_radius.as_ref().and_then(|br| br.to_px()).unwrap_or(0.0);
                let sigma = blur_radius / 2.0;
                let d = (sigma * 5.0).ceil() + 2.0 * spread_radius + 20.0;

                let bounds = self.bounds();

                // TODO: Cache shadow images
                let (source, target) = {
                    (
                        canvas
                            .create_image_empty(
                                (bounds.w + d) as usize,
                                (bounds.h + d) as usize,
                                femtovg::PixelFormat::Rgba8,
                                femtovg::ImageFlags::FLIP_Y | femtovg::ImageFlags::PREMULTIPLIED,
                            )
                            .unwrap(),
                        canvas
                            .create_image_empty(
                                (bounds.w + d) as usize,
                                (bounds.h + d) as usize,
                                femtovg::PixelFormat::Rgba8,
                                femtovg::ImageFlags::FLIP_Y | femtovg::ImageFlags::PREMULTIPLIED,
                            )
                            .unwrap(),
                    )
                };

                canvas.save();
                canvas.set_render_target(femtovg::RenderTarget::Image(source));
                canvas.reset_scissor();
                canvas.reset_transform();
                canvas.clear_rect(
                    0,
                    0,
                    (bounds.w + d) as u32,
                    (bounds.h + d) as u32,
                    femtovg::Color::rgba(0, 0, 0, 0),
                );

                let scalex = 1.0 - (2.0 * spread_radius / bounds.w);
                let scaley = 1.0 - (2.0 * spread_radius / bounds.h);
                canvas.translate(
                    (-bounds.x - bounds.w / 2.0) * scalex,
                    (-bounds.y - bounds.h / 2.0) * scaley,
                );
                canvas.scale(scalex, scaley);
                canvas.translate(
                    (bounds.w / 2.0 + d / 2.0) / scalex,
                    (bounds.h / 2.0 + d / 2.0) / scaley,
                );
                let paint = Paint::color(color.into());
                let mut shadow_path = path.clone();
                shadow_path.rect(
                    bounds.x - d / 2.0,
                    bounds.y - d / 2.0,
                    bounds.w + d,
                    bounds.h + d,
                );

                shadow_path.solidity(femtovg::Solidity::Hole);
                canvas.fill_path(&mut shadow_path, &paint);
                canvas.restore();

                let target_image = if blur_radius > 0.0 {
                    canvas.filter_image(
                        target,
                        femtovg::ImageFilter::GaussianBlur { sigma },
                        source,
                    );
                    target
                } else {
                    source
                };

                canvas.set_render_target(femtovg::RenderTarget::Screen);
                canvas.save();
                let mut shadow_path = Path::new();
                shadow_path.rect(
                    bounds.x - d / 2.0,
                    bounds.y - d / 2.0,
                    bounds.w + d,
                    bounds.h + d,
                );

                let paint = Paint::image(
                    target_image,
                    bounds.x - d / 2.0 + x_offset - 1.5,
                    bounds.y - d / 2.0 + y_offset - 1.5,
                    bounds.w + d + 3.0,
                    bounds.h + d + 3.0,
                    0f32,
                    1f32,
                );

                canvas.fill_path(path, &paint);

                canvas.restore();

                // canvas.delete_image(source);
                // canvas.delete_image(target);
            }
        }
    }

    pub fn draw_shadows(&mut self, canvas: &mut Canvas, path: &mut Path) {
        if let Some(box_shadows) = self.box_shadows() {
            for box_shadow in box_shadows.iter().rev().filter(|shadow| !shadow.inset) {
                let color = box_shadow.color.unwrap_or_default();
                let x_offset = box_shadow.x_offset.to_px().unwrap_or(0.0) * self.scale_factor();
                let y_offset = box_shadow.y_offset.to_px().unwrap_or(0.0) * self.scale_factor();
                let spread_radius =
                    box_shadow.spread_radius.as_ref().and_then(|l| l.to_px()).unwrap_or(0.0)
                        * self.scale_factor();

                let blur_radius =
                    box_shadow.blur_radius.as_ref().and_then(|br| br.to_px()).unwrap_or(0.0);
                let sigma = blur_radius / 2.0;
                let d = (sigma * 5.0).ceil() + 2.0 * spread_radius;

                let bounds = self.bounds();

                // TODO: Cache shadow images
                let (source, target) = {
                    (
                        canvas
                            .create_image_empty(
                                (bounds.w + d) as usize,
                                (bounds.h + d) as usize,
                                femtovg::PixelFormat::Rgba8,
                                femtovg::ImageFlags::FLIP_Y | femtovg::ImageFlags::PREMULTIPLIED,
                            )
                            .unwrap(),
                        canvas
                            .create_image_empty(
                                (bounds.w + d) as usize,
                                (bounds.h + d) as usize,
                                femtovg::PixelFormat::Rgba8,
                                femtovg::ImageFlags::FLIP_Y | femtovg::ImageFlags::PREMULTIPLIED,
                            )
                            .unwrap(),
                    )
                };

                canvas.save();
                canvas.set_render_target(femtovg::RenderTarget::Image(source));
                canvas.reset_scissor();
                canvas.reset_transform();
                canvas.clear_rect(
                    0,
                    0,
                    (bounds.w + d) as u32,
                    (bounds.h + d) as u32,
                    femtovg::Color::rgba(0, 0, 0, 0),
                );

                let scalex = 1.0 + (2.0 * spread_radius / bounds.w);
                let scaley = 1.0 + (2.0 * spread_radius / bounds.h);
                canvas.translate(
                    (-bounds.x - bounds.w / 2.0) * scalex,
                    (-bounds.y - bounds.h / 2.0) * scaley,
                );
                canvas.scale(scalex, scaley);
                canvas.translate(
                    (bounds.w / 2.0 + d / 2.0) / scalex,
                    (bounds.h / 2.0 + d / 2.0) / scaley,
                );
                let paint = Paint::color(color.into());
                canvas.fill_path(&mut path.clone(), &paint);
                canvas.restore();

                let target_image = if blur_radius > 0.0 {
                    canvas.filter_image(
                        target,
                        femtovg::ImageFilter::GaussianBlur { sigma },
                        source,
                    );
                    target
                } else {
                    source
                };

                canvas.set_render_target(femtovg::RenderTarget::Screen);
                canvas.save();
                canvas.translate(x_offset, y_offset);
                let mut shadow_path = Path::new();
                shadow_path.rect(
                    bounds.x - d / 2.0,
                    bounds.y - d / 2.0,
                    bounds.w + d,
                    bounds.h + d,
                );

                canvas.fill_path(
                    &mut shadow_path,
                    &Paint::image(
                        target_image,
                        bounds.x - d / 2.0,
                        bounds.y - d / 2.0,
                        bounds.w + d,
                        bounds.h + d,
                        0f32,
                        1f32,
                    ),
                );

                canvas.restore();

                // canvas.delete_image(source);
                // canvas.delete_image(target);
            }
        }
    }

    pub fn draw_gradients(&self, canvas: &mut Canvas, path: &mut Path) {
        let bounds = self.bounds();

        let parent = self
            .tree
            .get_layout_parent(self.current)
            .expect(&format!("Failed to find parent somehow: {}", self.current));

        let parent_width = self.cache.get_width(parent);
        let parent_height = self.cache.get_height(parent);

        if let Some(gradients) = self.style.background_gradient.get(self.current) {
            for gradient in gradients.iter() {
                match gradient {
                    Gradient::Linear(linear_gradient) => {
                        let (_, _, end_x, end_y, parent_length) = match linear_gradient.direction {
                            LineDirection::Horizontal(horizontal_keyword) => {
                                match horizontal_keyword {
                                    HorizontalPositionKeyword::Left => {
                                        (0.0, 0.0, bounds.w, 0.0, parent_width)
                                    }

                                    HorizontalPositionKeyword::Right => {
                                        (0.0, 0.0, bounds.w, 0.0, parent_width)
                                    }
                                }
                            }

                            LineDirection::Vertical(vertical_keyword) => match vertical_keyword {
                                VerticalPositionKeyword::Bottom => {
                                    (0.0, 0.0, 0.0, bounds.h, parent_height)
                                }

                                VerticalPositionKeyword::Top => {
                                    (0.0, 0.0, 0.0, bounds.h, parent_height)
                                }
                            },

                            LineDirection::Corner { horizontal, vertical } => {
                                match (horizontal, vertical) {
                                    (
                                        HorizontalPositionKeyword::Right,
                                        VerticalPositionKeyword::Bottom,
                                    ) => (0.0, 0.0, bounds.w, bounds.h, parent_width),

                                    _ => (0.0, 0.0, 0.0, 0.0, 0.0),
                                }
                            }

                            _ => (0.0, 0.0, 0.0, 0.0, 0.0),
                        };

                        let num_stops = linear_gradient.stops.len();

                        let stops = linear_gradient
                            .stops
                            .iter()
                            .enumerate()
                            .map(|(index, stop)| {
                                let pos = if let Some(pos) = &stop.position {
                                    pos.to_pixels(parent_length) / parent_length
                                } else {
                                    index as f32 / (num_stops - 1) as f32
                                };
                                let col: femtovg::Color = stop.color.into();
                                (pos, col)
                            })
                            .collect::<Vec<_>>();

                        let paint = Paint::linear_gradient_stops(
                            bounds.x,
                            bounds.y,
                            bounds.x + end_x,
                            bounds.y + end_y,
                            stops.into_iter(),
                        );

                        canvas.fill_path(path, &paint);
                    }

                    _ => {}
                }
            }
        }
    }

    pub fn sync_text_styles(&mut self) {
        self.text_context.sync_styles(self.current, self.style);
    }

    pub fn draw_text(&mut self, canvas: &mut Canvas, origin: (f32, f32), justify: (f32, f32)) {
        if let Ok(draw_commands) =
            self.text_context.fill_to_cmds(canvas, self.current, origin, justify, *self.text_config)
        {
            for (color, cmds) in draw_commands.into_iter() {
                let temp_paint =
                    Paint::color(femtovg::Color::rgba(color.r(), color.g(), color.b(), color.a()));
                canvas.draw_glyph_commands(cmds, &temp_paint, 1.0);
            }
        }
    }

    pub fn draw_highlights(
        &mut self,
        canvas: &mut Canvas,
        origin: (f32, f32),
        justify: (f32, f32),
    ) {
        let selection_color = self.selection_color();
        let mut path = Path::new();
        for (x, y, w, h) in self.text_context.layout_selection(self.current, origin, justify) {
            path.rect(x, y, w, h);
        }
        canvas.fill_path(&mut path, &Paint::color(selection_color.into()));
    }

    pub fn draw_caret(
        &mut self,
        canvas: &mut Canvas,
        origin: (f32, f32),
        justify: (f32, f32),
        width: f32,
    ) {
        let caret_color = self.caret_color();
        if let Some((x, y, w, h)) = self.text_context.layout_caret(
            self.current,
            origin,
            justify,
            self.logical_to_physical(width),
        ) {
            let mut path = Path::new();
            path.rect(x, y, w, h);
            canvas.fill_path(&mut path, &Paint::color(caret_color.into()));
        }
    }
}

impl<'a> DataContext for DrawContext<'a> {
    fn data<T: 'static>(&self) -> Option<&T> {
        // return data for the static model
        if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
            return Some(t);
        }

        for entity in self.current.parent_iter(self.tree) {
            if let Some(model_data_store) = self.data.get(entity) {
                if let Some(model) = model_data_store.models.get(&TypeId::of::<T>()) {
                    return model.downcast_ref::<T>();
                }
            }

            if let Some(view_handler) = self.views.get(&entity) {
                if let Some(data) = view_handler.downcast_ref::<T>() {
                    return Some(data);
                }
            }
        }

        None
    }
}
