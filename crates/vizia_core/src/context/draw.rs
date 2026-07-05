use skia_safe::canvas::SaveLayerRec;
use skia_safe::path_builder::ArcSize;
use skia_safe::rrect::Corner;
use skia_safe::wrapper::PointerWrapper;
use skia_safe::{
    BlurStyle, ClipOp, MaskFilter, Matrix, Paint, PaintStyle, Path, PathBuilder, PathDirection,
    PathEffect, Point, RRect, Rect, SamplingOptions, TileMode,
};
use std::any::{Any, TypeId};
use std::f32::consts::SQRT_2;
use vizia_style::LengthPercentageOrAuto;

use hashbrown::HashMap;

use crate::cache::CachedData;
use crate::events::ViewHandler;
use crate::prelude::*;
use crate::resource::{ImageOrSvg, ResourceManager};
use crate::text::{TextContext, resolved_text_direction};
use vizia_input::MouseState;

use super::ModelData;

/// A context used when drawing a view.
///
/// The `DrawContext` is provided by the [`draw`](crate::view::View::draw) method in [`View`] and can be used to immutably access the
/// computed style and layout properties of the current view.
///
/// # Example
/// ```
/// # use vizia_core::prelude::*;
/// # use vizia_core::vg;
/// # let cx = &mut Context::default();
///
/// pub struct CustomView {}
///
/// impl CustomView {
///     pub fn new(cx: &mut Context) -> Handle<Self> {
///         Self{}.build(cx, |_|{})
///     }
/// }
///
/// impl View for CustomView {
///     fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
///         // Get the computed bounds after layout of the current view
///         let bounds = cx.bounds();
///         // Draw to the canvas using the bounds of the current view
///         let path = vg::Path::new();
///         path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
///         let mut paint = vg::Paint::default();
///         paint.set_color(Color::rgb(200, 100, 100));
///         canvas.draw_path(&path, &paint);
///     }
/// }
/// ```
pub struct DrawContext<'a> {
    pub(crate) current: Entity,
    pub(crate) style: &'a Style,
    pub(crate) cache: &'a mut CachedData,
    pub(crate) tree: &'a Tree<Entity>,
    pub(crate) models: &'a HashMap<Entity, HashMap<TypeId, Box<dyn ModelData>>>,
    pub(crate) views: &'a mut HashMap<Entity, Box<dyn ViewHandler>>,
    pub(crate) resource_manager: &'a ResourceManager,
    pub(crate) text_context: &'a mut TextContext,
    pub(crate) modifiers: &'a Modifiers,
    pub(crate) mouse: &'a MouseState<Entity>,
    pub(crate) windows: &'a mut HashMap<Entity, WindowState>,
}

macro_rules! get_units_property {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
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

impl DrawContext<'_> {
    pub fn with_current<T>(&mut self, entity: Entity, f: impl FnOnce(&mut DrawContext) -> T) -> T {
        let current = self.current;
        self.current = entity;
        let t = f(self);
        self.current = current;
        t
    }

    /// Returns the bounds of the current view.
    pub fn bounds(&self) -> BoundingBox {
        self.cache.get_bounds(self.current)
    }

    /// Marks the current view as needing to be redrawn.
    pub fn needs_redraw(&mut self) {
        let parent_window = self.tree.get_parent_window(self.current).unwrap_or(Entity::root());
        if let Some(window_state) = self.windows.get_mut(&parent_window) {
            window_state.redraw_list.insert(self.current);
        }
    }

    /// Returns the z-index of the current view.
    pub fn z_index(&self) -> i32 {
        self.style.z_index.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the scale factor.
    pub fn scale_factor(&self) -> f32 {
        self.style.dpi_factor as f32
    }

    /// Returns a reference to the keyboard modifiers state.
    pub fn modifiers(&self) -> &Modifiers {
        self.modifiers
    }

    /// Returns a reference to the mouse state.
    pub fn mouse(&self) -> &MouseState<Entity> {
        self.mouse
    }

    /// Returns the clip path of the current view.
    pub fn clip_path(&self) -> Option<skia_safe::Path> {
        // A cached entry (including None) is authoritative for this entity.
        if let Some(clip_path) = self.cache.clip_path.get(self.current) {
            return clip_path.clone();
        }

        if self.style.ignore_clipping.get(self.current).copied().unwrap_or(false) {
            return None;
        }

        // If there is no cached value yet, walk ancestors to find an inherited clip.
        let mut current = self.current;
        while let Some(parent) = self.tree.get_parent(current) {
            // A cached parent entry (including None) is authoritative.
            if let Some(clip_path) = self.cache.clip_path.get(parent) {
                return clip_path.clone();
            }

            if self.style.ignore_clipping.get(parent).copied().unwrap_or(false) {
                return None;
            }
            current = parent;
        }

        None
    }

    /// Returns the 2D transform of the current view.
    pub fn transform(&self) -> Matrix {
        self.cache.transform.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the visibility of the current view.
    pub fn visibility(&self) -> Option<Visibility> {
        self.style.visibility.get(self.current).copied()
    }

    /// Returns the display of the current view.
    pub fn display(&self) -> Display {
        self.style.display.get(self.current).copied().unwrap_or(Display::Flex)
    }

    /// Returns the opacity of the current view.
    pub fn opacity(&self) -> f32 {
        self.style
            .opacity
            .get_resolved(self.current, &self.style.custom_opacity_props)
            .unwrap_or(Opacity(1.0))
            .0
    }

    /// Returns the lookup pattern to pick the default font.
    pub fn default_font(&self) -> &[FamilyOwned] {
        &self.style.default_font
    }

    /// Returns the font-size of the current view in physical pixels.
    pub fn font_size(&self) -> f32 {
        let fs = self
            .style
            .font_size
            .get_resolved(self.current, &self.style.custom_font_size_props)
            .and_then(|f| f.0.to_px())
            .unwrap_or(16.0);
        self.logical_to_physical(fs)
    }

    /// Returns the font-weight of the current view.
    pub fn font_weight(&self) -> FontWeight {
        self.style.font_weight.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the font-width of the current view.
    pub fn font_width(&self) -> FontWidth {
        self.style.font_width.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the font-slant of the current view.
    pub fn font_slant(&self) -> FontSlant {
        self.style.font_slant.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the font variation settings of the current view.
    pub fn font_variation_settings(&self) -> &[FontVariation] {
        self.style.font_variation_settings.get(self.current).map(Vec::as_slice).unwrap_or_default()
    }

    /// Function to convert logical points to physical pixels.
    pub fn logical_to_physical(&self, logical: f32) -> f32 {
        self.style.logical_to_physical(logical)
    }

    /// Function to convert physical pixels to logical points.
    pub fn physical_to_logical(&self, physical: f32) -> f32 {
        self.style.physical_to_logical(physical)
    }

    /// Returns the top border width of the current view in physical pixels.
    pub fn border_top_width(&self) -> f32 {
        let bounds = self.bounds();
        self.style
            .border_top_width
            .get_resolved(self.current, &self.style.custom_length_props)
            .map(|l| l.to_pixels(bounds.w.min(bounds.h), self.scale_factor()).round())
            .unwrap_or(0.0)
    }

    /// Returns the right border width of the current view in physical pixels.
    pub fn border_right_width(&self) -> f32 {
        let bounds = self.bounds();
        self.style
            .border_right_width
            .get_resolved(self.current, &self.style.custom_length_props)
            .map(|l| l.to_pixels(bounds.w.min(bounds.h), self.scale_factor()).round())
            .unwrap_or(0.0)
    }

    /// Returns the bottom border width of the current view in physical pixels.
    pub fn border_bottom_width(&self) -> f32 {
        let bounds = self.bounds();
        self.style
            .border_bottom_width
            .get_resolved(self.current, &self.style.custom_length_props)
            .map(|l| l.to_pixels(bounds.w.min(bounds.h), self.scale_factor()).round())
            .unwrap_or(0.0)
    }

    /// Returns the left border width of the current view in physical pixels.
    pub fn border_left_width(&self) -> f32 {
        let bounds = self.bounds();
        self.style
            .border_left_width
            .get_resolved(self.current, &self.style.custom_length_props)
            .map(|l| l.to_pixels(bounds.w.min(bounds.h), self.scale_factor()).round())
            .unwrap_or(0.0)
    }

    /// Returns the top border color of the current view.
    pub fn border_top_color(&self) -> Color {
        self.style
            .border_top_color
            .get_resolved(self.current, &self.style.custom_color_props)
            .map(|c| Color::rgba(c.r(), c.g(), c.b(), c.a()))
            .unwrap_or(Color::rgba(0, 0, 0, 0))
    }

    /// Returns the right border color of the current view.
    pub fn border_right_color(&self) -> Color {
        self.style
            .border_right_color
            .get_resolved(self.current, &self.style.custom_color_props)
            .map(|c| Color::rgba(c.r(), c.g(), c.b(), c.a()))
            .unwrap_or(Color::rgba(0, 0, 0, 0))
    }

    /// Returns the bottom border color of the current view.
    pub fn border_bottom_color(&self) -> Color {
        self.style
            .border_bottom_color
            .get_resolved(self.current, &self.style.custom_color_props)
            .map(|c| Color::rgba(c.r(), c.g(), c.b(), c.a()))
            .unwrap_or(Color::rgba(0, 0, 0, 0))
    }

    /// Returns the left border color of the current view.
    pub fn border_left_color(&self) -> Color {
        self.style
            .border_left_color
            .get_resolved(self.current, &self.style.custom_color_props)
            .map(|c| Color::rgba(c.r(), c.g(), c.b(), c.a()))
            .unwrap_or(Color::rgba(0, 0, 0, 0))
    }

    /// Returns the top border style of the current view.
    pub fn border_top_style(&self) -> BorderStyleKeyword {
        self.style.border_top_style.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the right border style of the current view.
    pub fn border_right_style(&self) -> BorderStyleKeyword {
        self.style.border_right_style.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the bottom border style of the current view.
    pub fn border_bottom_style(&self) -> BorderStyleKeyword {
        self.style.border_bottom_style.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the left border style of the current view.
    pub fn border_left_style(&self) -> BorderStyleKeyword {
        self.style.border_left_style.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the outline color of the current view.
    pub fn outline_color(&self) -> Color {
        if let Some(col) =
            self.style.outline_color.get_resolved(self.current, &self.style.custom_color_props)
        {
            Color::rgba(col.r(), col.g(), col.b(), col.a())
        } else {
            Color::rgba(0, 0, 0, 0)
        }
    }

    /// Returns the outline width of the current view in physical pixels.
    pub fn outline_width(&self) -> f32 {
        if let Some(length) =
            self.style.outline_width.get_resolved(self.current, &self.style.custom_length_props)
        {
            let bounds = self.bounds();
            return length.to_pixels(bounds.w.min(bounds.h), self.scale_factor()).round();
        }
        0.0
    }

    /// Returns the outline offset of the current view in physical pixels.
    pub fn outline_offset(&self) -> f32 {
        if let Some(length) =
            self.style.outline_offset.get_resolved(self.current, &self.style.custom_length_props)
        {
            let bounds = self.bounds();
            return length.to_pixels(bounds.w.min(bounds.h), self.scale_factor()).round();
        }
        0.0
    }

    /// Returns the corner radius for the top-left corner of the current view.
    pub fn corner_top_left_radius(&self) -> f32 {
        let bounds = self.bounds();
        let scale = self.scale_factor();
        self.style
            .corner_top_left_radius
            .get_resolved(self.current, &self.style.custom_length_props)
            .map(|l| l.to_pixels(bounds.w.min(bounds.h), scale).round())
            .unwrap_or(0.0)
    }

    /// Returns the corner radius for the top-right corner of the current view.
    pub fn corner_top_right_radius(&self) -> f32 {
        let bounds = self.bounds();
        let scale = self.scale_factor();
        self.style
            .corner_top_right_radius
            .get_resolved(self.current, &self.style.custom_length_props)
            .map(|l| l.to_pixels(bounds.w.min(bounds.h), scale).round())
            .unwrap_or(0.0)
    }

    /// Returns the corner radius for the bottom-left corner of the current view.
    pub fn corner_bottom_left_radius(&self) -> f32 {
        let bounds = self.bounds();
        let scale = self.scale_factor();
        self.style
            .corner_bottom_left_radius
            .get_resolved(self.current, &self.style.custom_length_props)
            .map(|l| l.to_pixels(bounds.w.min(bounds.h), scale).round())
            .unwrap_or(0.0)
    }

    /// Returns the corner radius for the bottom-right corner of the current view.
    pub fn corner_bottom_right_radius(&self) -> f32 {
        let bounds = self.bounds();
        let scale = self.scale_factor();
        self.style
            .corner_bottom_right_radius
            .get_resolved(self.current, &self.style.custom_length_props)
            .map(|l| l.to_pixels(bounds.w.min(bounds.h), scale).round())
            .unwrap_or(0.0)
    }

    /// Returns the corner shape for the top-left corner of the current view.
    pub fn corner_top_left_shape(&self) -> CornerShape {
        self.style.corner_top_left_shape.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the corner shape for the top-left corner of the current view.
    pub fn corner_top_right_shape(&self) -> CornerShape {
        self.style.corner_top_right_shape.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the corner shape for the top-left corner of the current view.
    pub fn corner_bottom_left_shape(&self) -> CornerShape {
        self.style.corner_bottom_left_shape.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the corner shape for the top-left corner of the current view.
    pub fn corner_bottom_right_shape(&self) -> CornerShape {
        self.style.corner_bottom_right_shape.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the corner smoothing for the top-left corner of the current view.
    pub fn corner_top_left_smoothing(&self) -> f32 {
        self.style.corner_top_left_smoothing.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the corner shape for the top-left corner of the current view.
    pub fn corner_top_right_smoothing(&self) -> f32 {
        self.style.corner_top_right_smoothing.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the corner shape for the top-left corner of the current view.
    pub fn corner_bottom_left_smoothing(&self) -> f32 {
        self.style.corner_bottom_left_smoothing.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the corner shape for the top-left corner of the current view.
    pub fn corner_bottom_right_smoothing(&self) -> f32 {
        self.style.corner_bottom_right_smoothing.get(self.current).copied().unwrap_or_default()
    }

    get_units_property!(
        /// Returns the padding-left space of the current view.
        padding_left
    );

    get_units_property!(
        /// Returns the padding-right space of the current view.
        padding_right
    );

    get_units_property!(
        /// Returns the padding-top space of the current view.
        padding_top
    );

    get_units_property!(
        /// Returns the padding-bottom space of the current view.
        padding_bottom
    );

    /// Returns the alignment of the current view.
    pub fn alignment(&self) -> Alignment {
        self.style.alignment.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the background color of the current view.
    pub fn background_color(&self) -> Color {
        if let Some(col) =
            self.style.background_color.get_resolved(self.current, &self.style.custom_color_props)
        {
            Color::rgba(col.r(), col.g(), col.b(), col.a())
        } else {
            Color::rgba(0, 0, 0, 0)
        }
    }

    /// Returns the border color of the current view.
    /// This returns the top border color; use side-specific getters for per-side access.
    pub fn border_color(&self) -> Color {
        self.border_top_color()
    }

    /// Returns the border style of the current view.
    /// This returns the top border style; use side-specific getters for per-side access.
    pub fn border_style(&self) -> BorderStyleKeyword {
        self.border_top_style()
    }

    /// Returns the border width of the current view in physical pixels.
    /// This returns the top border width; use side-specific getters for per-side access.
    pub fn border_width(&self) -> f32 {
        self.border_top_width()
    }

    /// Returns the text selection color for the current view.
    pub fn selection_color(&self) -> Color {
        if let Some(col) =
            self.style.selection_color.get_resolved(self.current, &self.style.custom_color_props)
        {
            Color::rgba(col.r(), col.g(), col.b(), col.a())
        } else {
            Color::rgba(0, 0, 0, 0)
        }
    }

    /// Returns the text caret color for the current view.
    pub fn caret_color(&self) -> Color {
        if let Some(col) =
            self.style.caret_color.get_resolved(self.current, &self.style.custom_color_props)
        {
            Color::rgba(col.r(), col.g(), col.b(), col.a())
        } else {
            Color::rgba(0, 0, 0, 0)
        }
    }

    /// Returns the font color for the current view.
    pub fn font_color(&self) -> Color {
        if let Some(col) =
            self.style.font_color.get_resolved(self.current, &self.style.custom_color_props)
        {
            Color::rgba(col.r(), col.g(), col.b(), col.a())
        } else {
            Color::rgba(0, 0, 0, 0)
        }
    }

    /// Returns whether the current view should have its text wrapped.
    pub fn text_wrap(&self) -> bool {
        self.style.text_wrap.get(self.current).copied().unwrap_or(true)
    }

    /// Returns the text alignment of the current view.
    pub fn text_align(&self) -> TextAlign {
        self.style.text_align.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the text overflow preference of the current view.
    pub fn text_overflow(&self) -> TextOverflow {
        self.style.text_overflow.get(self.current).copied().unwrap_or_default()
    }

    /// Returns the line clamp Of the current view.
    pub fn line_clamp(&self) -> Option<usize> {
        self.style.line_clamp.get(self.current).copied().map(|lc| lc.0 as usize)
    }

    /// Returns the resolved shadows of the current view.
    pub fn shadows(&self) -> Option<Vec<Shadow>> {
        self.style.shadow.get_resolved(self.current, &self.style.custom_shadow_props)
    }

    /// Returns a reference to any filter applied to the current view.
    pub fn filter(&self) -> Option<&Filter> {
        self.style.filter.get(self.current)
    }

    /// Return to reference to any filter applied to the current view.
    pub fn backdrop_filter(&self) -> Option<&Filter> {
        self.style.backdrop_filter.get(self.current)
    }

    /// Returns a reference to any images of the current view.
    pub fn background_images(&self) -> Option<&Vec<ImageOrGradient>> {
        self.style.background_image.get(self.current)
    }

    ///  Returns a list of background sizes for the current view.
    pub fn background_size(&self) -> Vec<BackgroundSize> {
        self.style.background_size.get(self.current).cloned().unwrap_or_default()
    }

    /// Returns a list of background positions for the current view.
    pub fn background_position(&self) -> Vec<Position> {
        self.style.background_position.get(self.current).cloned().unwrap_or_default()
    }

    /// Returns a list of background repeat modes for the current view.
    pub fn background_repeat(&self) -> Vec<BackgroundRepeat> {
        self.style.background_repeat.get(self.current).cloned().unwrap_or_default()
    }

    pub fn path(&mut self) -> Path {
        if self.cache.path.get(self.current).is_none() {
            self.cache.path.insert(self.current, self.build_path(self.bounds(), (0.0, 0.0)));
        }
        let bounds = self.bounds();
        self.cache.path.get(self.current).unwrap().make_offset(bounds.top_left())
    }

    /// Get the vector path of the current view.
    pub fn build_path(&self, bounds: BoundingBox, outset: (f32, f32)) -> Path {
        self.build_path_with_corners(
            bounds,
            outset,
            (
                self.corner_top_left_radius(),
                self.corner_top_right_radius(),
                self.corner_bottom_right_radius(),
                self.corner_bottom_left_radius(),
            ),
            (
                self.corner_top_left_shape(),
                self.corner_top_right_shape(),
                self.corner_bottom_right_shape(),
                self.corner_bottom_left_shape(),
            ),
            (
                self.corner_top_left_smoothing(),
                self.corner_top_right_smoothing(),
                self.corner_bottom_right_smoothing(),
                self.corner_bottom_left_smoothing(),
            ),
        )
    }

    fn build_path_with_corners(
        &self,
        bounds: BoundingBox,
        outset: (f32, f32),
        corner_radii: (f32, f32, f32, f32),
        corner_shapes: (CornerShape, CornerShape, CornerShape, CornerShape),
        corner_smoothing: (f32, f32, f32, f32),
    ) -> Path {
        let (
            corner_top_left_radius,
            corner_top_right_radius,
            corner_bottom_right_radius,
            corner_bottom_left_radius,
        ) = corner_radii;
        let (
            corner_top_left_shape,
            corner_top_right_shape,
            corner_bottom_right_shape,
            corner_bottom_left_shape,
        ) = corner_shapes;
        let (
            corner_top_left_smoothing,
            corner_top_right_smoothing,
            corner_bottom_right_smoothing,
            corner_bottom_left_smoothing,
        ) = corner_smoothing;

        let bounds = BoundingBox::from_min_max(0.0, 0.0, bounds.w, bounds.h);

        let rect: Rect = bounds.into();

        let mut rr = RRect::new_rect_radii(
            rect,
            &[
                Point::new(corner_top_left_radius, corner_top_left_radius),
                Point::new(corner_top_right_radius, corner_top_right_radius),
                Point::new(corner_bottom_right_radius, corner_bottom_right_radius),
                Point::new(corner_bottom_left_radius, corner_bottom_left_radius),
            ],
        );

        rr = rr.with_outset(outset);

        let x = rr.bounds().x();
        let y = rr.bounds().y();
        let width = rr.width();
        let height = rr.height();
        let mut should_offset = true;

        let mut path = PathBuilder::new();

        if width == height
            && corner_bottom_left_radius == width / 2.0
            && corner_bottom_right_radius == width / 2.0
            && corner_top_left_radius == height / 2.0
            && corner_top_right_radius == height / 2.0
        {
            path.add_circle((width / 2.0, height / 2.0), width / 2.0, Some(PathDirection::CW));
        } else if corner_top_left_radius == corner_top_right_radius
            && corner_top_right_radius == corner_bottom_right_radius
            && corner_bottom_right_radius == corner_bottom_left_radius
            && corner_top_left_smoothing == 0.0
            && corner_top_left_smoothing == corner_top_right_smoothing
            && corner_top_right_smoothing == corner_bottom_right_smoothing
            && corner_bottom_right_smoothing == corner_bottom_left_smoothing
            && corner_top_left_shape == CornerShape::Round
            && corner_top_left_shape == corner_top_right_shape
            && corner_top_right_shape == corner_bottom_right_shape
            && corner_bottom_right_shape == corner_bottom_left_shape
        {
            path.add_rrect(rr, None, None);
            should_offset = false;
        } else {
            let top_right = rr.radii(Corner::UpperRight).x;

            if top_right > 0.0 {
                let (a, b, c, d, l, p, radius) =
                    compute_smooth_corner(top_right, corner_top_right_smoothing, width, height);

                path.move_to((f32::max(width / 2.0, width - p), 0.0));
                if corner_top_right_shape == CornerShape::Round {
                    path.cubic_to(
                        (width - (p - a), 0.0),
                        (width - (p - a - b), 0.0),
                        (width - (p - a - b - c), d),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (l, l))
                    .cubic_to(
                        (width, p - a - b),
                        (width, p - a),
                        (width, f32::min(height / 2.0, p)),
                    );
                } else {
                    path.line_to((width, f32::min(height / 2.0, p)));
                }
            } else {
                path.move_to((width / 2.0, 0.0))
                    .line_to((width, 0.0))
                    .line_to((width, height / 2.0));
            }

            let bottom_right = rr.radii(Corner::LowerRight).x;
            if bottom_right > 0.0 {
                let (a, b, c, d, l, p, radius) = compute_smooth_corner(
                    bottom_right,
                    corner_bottom_right_smoothing,
                    width,
                    height,
                );

                path.line_to((width, f32::max(height / 2.0, height - p)));
                if corner_bottom_right_shape == CornerShape::Round {
                    path.cubic_to(
                        (width, height - (p - a)),
                        (width, height - (p - a - b)),
                        (width - d, height - (p - a - b - c)),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (-l, l))
                    .cubic_to(
                        (width - (p - a - b), height),
                        (width - (p - a), height),
                        (f32::max(width / 2.0, width - p), height),
                    );
                } else {
                    path.line_to((f32::max(width / 2.0, width - p), height));
                }
            } else {
                path.line_to((width, height)).line_to((width / 2.0, height));
            }

            let bottom_left = rr.radii(Corner::LowerLeft).x;
            if bottom_left > 0.0 {
                let (a, b, c, d, l, p, radius) =
                    compute_smooth_corner(bottom_left, corner_bottom_left_smoothing, width, height);

                path.line_to((f32::min(width / 2.0, p), height));
                if corner_bottom_left_shape == CornerShape::Round {
                    path.cubic_to(
                        (p - a, height),
                        (p - a - b, height),
                        (p - a - b - c, height - d),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (-l, -l))
                    .cubic_to(
                        (0.0, height - (p - a - b)),
                        (0.0, height - (p - a)),
                        (0.0, f32::max(height / 2.0, height - p)),
                    );
                } else {
                    path.line_to((0.0, f32::max(height / 2.0, height - p)));
                }
            } else {
                path.line_to((0.0, height)).line_to((0.0, height / 2.0));
            }

            let top_left = rr.radii(Corner::UpperLeft).x;
            if top_left > 0.0 {
                let (a, b, c, d, l, p, radius) =
                    compute_smooth_corner(top_left, corner_top_left_smoothing, width, height);

                path.line_to((0.0, f32::min(height / 2.0, p)));
                if corner_top_left_shape == CornerShape::Round {
                    path.cubic_to((0.0, p - a), (0.0, p - a - b), (d, p - a - b - c))
                        .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (l, -l))
                        .cubic_to((p - a - b, 0.0), (p - a, 0.0), (f32::min(width / 2.0, p), 0.0));
                } else {
                    path.line_to((f32::min(width / 2.0, p), 0.0));
                }
            } else {
                path.line_to((0.0, 0.0));
            }

            path.close();
        }

        let path = path.detach();
        if should_offset { path.make_offset((x, y)) } else { path }
    }

    fn corner_oval(center_x: f32, center_y: f32, radius: f32) -> Rect {
        Rect::from_xywh(center_x - radius, center_y - radius, radius * 2.0, radius * 2.0)
    }

    fn round_corner_side_path(
        side_ix: usize,
        bounds: BoundingBox,
        outer_radii: (f32, f32, f32, f32),
        inner_radii: (f32, f32, f32, f32),
        widths: (f32, f32, f32, f32),
    ) -> Option<Path> {
        let bx = bounds.x;
        let by = bounds.y;
        let bw = bounds.w;
        let bh = bounds.h;

        let (r_tl, r_tr, r_br, r_bl) = outer_radii;
        let (ir_tl, ir_tr, ir_br, ir_bl) = inner_radii;
        let (top_width, right_width, bottom_width, left_width) = widths;

        let mut path = PathBuilder::new();
        let diag = SQRT_2.recip();

        let outer_tl_center = Point::new(bx + r_tl, by + r_tl);
        let outer_tr_center = Point::new(bx + bw - r_tr, by + r_tr);
        let outer_br_center = Point::new(bx + bw - r_br, by + bh - r_br);
        let outer_bl_center = Point::new(bx + r_bl, by + bh - r_bl);

        let inner_tl_center = Point::new(bx + left_width + ir_tl, by + top_width + ir_tl);
        let inner_tr_center = Point::new(bx + bw - right_width - ir_tr, by + top_width + ir_tr);
        let inner_br_center =
            Point::new(bx + bw - right_width - ir_br, by + bh - bottom_width - ir_br);
        let inner_bl_center = Point::new(bx + left_width + ir_bl, by + bh - bottom_width - ir_bl);

        match side_ix {
            0 => {
                let outer_start = if r_tl > 0.0 {
                    Point::new(outer_tl_center.x - r_tl * diag, outer_tl_center.y - r_tl * diag)
                } else {
                    Point::new(bx, by)
                };
                path.move_to(outer_start);
                if r_tl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_tl_center.x, outer_tl_center.y, r_tl),
                        225.0,
                        45.0,
                        false,
                    );
                }
                path.line_to((bx + bw - r_tr, by));
                if r_tr > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_tr_center.x, outer_tr_center.y, r_tr),
                        270.0,
                        45.0,
                        false,
                    );
                }

                let inner_split = if ir_tr > 0.0 {
                    Point::new(inner_tr_center.x + ir_tr * diag, inner_tr_center.y - ir_tr * diag)
                } else {
                    Point::new(bx + bw - right_width, by + top_width)
                };
                path.line_to(inner_split);
                if ir_tr > 0.0 {
                    path.arc_to(
                        Self::corner_oval(inner_tr_center.x, inner_tr_center.y, ir_tr),
                        315.0,
                        -45.0,
                        false,
                    );
                }
                path.line_to((bx + left_width + ir_tl, by + top_width));
                if ir_tl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(inner_tl_center.x, inner_tl_center.y, ir_tl),
                        270.0,
                        -45.0,
                        false,
                    );
                }
            }
            1 => {
                let outer_start = if r_tr > 0.0 {
                    Point::new(outer_tr_center.x + r_tr * diag, outer_tr_center.y - r_tr * diag)
                } else {
                    Point::new(bx + bw, by)
                };
                path.move_to(outer_start);
                if r_tr > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_tr_center.x, outer_tr_center.y, r_tr),
                        315.0,
                        45.0,
                        false,
                    );
                }
                path.line_to((bx + bw, by + bh - r_br));
                if r_br > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_br_center.x, outer_br_center.y, r_br),
                        0.0,
                        45.0,
                        false,
                    );
                }

                let inner_split = if ir_br > 0.0 {
                    Point::new(inner_br_center.x + ir_br * diag, inner_br_center.y + ir_br * diag)
                } else {
                    Point::new(bx + bw - right_width, by + bh - bottom_width)
                };
                path.line_to(inner_split);
                if ir_br > 0.0 {
                    path.arc_to(
                        Self::corner_oval(inner_br_center.x, inner_br_center.y, ir_br),
                        45.0,
                        -45.0,
                        false,
                    );
                }
                path.line_to((bx + bw - right_width, by + top_width + ir_tr));
                if ir_tr > 0.0 {
                    path.arc_to(
                        Self::corner_oval(inner_tr_center.x, inner_tr_center.y, ir_tr),
                        0.0,
                        -45.0,
                        false,
                    );
                }
            }
            2 => {
                let outer_start = if r_br > 0.0 {
                    Point::new(outer_br_center.x + r_br * diag, outer_br_center.y + r_br * diag)
                } else {
                    Point::new(bx + bw, by + bh)
                };
                path.move_to(outer_start);
                if r_br > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_br_center.x, outer_br_center.y, r_br),
                        45.0,
                        45.0,
                        false,
                    );
                }
                path.line_to((bx + r_bl, by + bh));
                if r_bl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_bl_center.x, outer_bl_center.y, r_bl),
                        90.0,
                        45.0,
                        false,
                    );
                }

                let inner_split = if ir_bl > 0.0 {
                    Point::new(inner_bl_center.x - ir_bl * diag, inner_bl_center.y + ir_bl * diag)
                } else {
                    Point::new(bx + left_width, by + bh - bottom_width)
                };
                path.line_to(inner_split);
                if ir_bl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(inner_bl_center.x, inner_bl_center.y, ir_bl),
                        135.0,
                        -45.0,
                        false,
                    );
                }
                path.line_to((bx + bw - right_width - ir_br, by + bh - bottom_width));
                if ir_br > 0.0 {
                    path.arc_to(
                        Self::corner_oval(inner_br_center.x, inner_br_center.y, ir_br),
                        90.0,
                        -45.0,
                        false,
                    );
                }
            }
            3 => {
                let outer_start = if r_bl > 0.0 {
                    Point::new(outer_bl_center.x - r_bl * diag, outer_bl_center.y + r_bl * diag)
                } else {
                    Point::new(bx, by + bh)
                };
                path.move_to(outer_start);
                if r_bl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_bl_center.x, outer_bl_center.y, r_bl),
                        135.0,
                        45.0,
                        false,
                    );
                }
                path.line_to((bx, by + r_tl));
                if r_tl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_tl_center.x, outer_tl_center.y, r_tl),
                        180.0,
                        45.0,
                        false,
                    );
                }

                let inner_split = if ir_tl > 0.0 {
                    Point::new(inner_tl_center.x - ir_tl * diag, inner_tl_center.y - ir_tl * diag)
                } else {
                    Point::new(bx + left_width, by + top_width)
                };
                path.line_to(inner_split);
                if ir_tl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(inner_tl_center.x, inner_tl_center.y, ir_tl),
                        225.0,
                        -45.0,
                        false,
                    );
                }
                path.line_to((bx + left_width, by + bh - bottom_width - ir_bl));
                if ir_bl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(inner_bl_center.x, inner_bl_center.y, ir_bl),
                        180.0,
                        -45.0,
                        false,
                    );
                }
            }
            _ => return None,
        }

        path.close();
        Some(path.detach())
    }

    fn bevel_corner_side_path(
        side_ix: usize,
        bounds: BoundingBox,
        outer_radii: (f32, f32, f32, f32),
        inner_radii: (f32, f32, f32, f32),
        widths: (f32, f32, f32, f32),
    ) -> Option<Path> {
        let bx = bounds.x;
        let by = bounds.y;
        let bw = bounds.w;
        let bh = bounds.h;

        let (r_tl, r_tr, r_br, r_bl) = outer_radii;
        let (ir_tl, ir_tr, ir_br, ir_bl) = inner_radii;
        let (top_width, right_width, bottom_width, left_width) = widths;

        let otl = r_tl.min(bw * 0.5).min(bh * 0.5);
        let otr = r_tr.min(bw * 0.5).min(bh * 0.5);
        let obr = r_br.min(bw * 0.5).min(bh * 0.5);
        let obl = r_bl.min(bw * 0.5).min(bh * 0.5);

        let itl = ir_tl
            .max(0.0)
            .min((bw - left_width - right_width).max(0.0) * 0.5)
            .min((bh - top_width - bottom_width).max(0.0) * 0.5);
        let itr = ir_tr
            .max(0.0)
            .min((bw - left_width - right_width).max(0.0) * 0.5)
            .min((bh - top_width - bottom_width).max(0.0) * 0.5);
        let ibr = ir_br
            .max(0.0)
            .min((bw - left_width - right_width).max(0.0) * 0.5)
            .min((bh - top_width - bottom_width).max(0.0) * 0.5);
        let ibl = ir_bl
            .max(0.0)
            .min((bw - left_width - right_width).max(0.0) * 0.5)
            .min((bh - top_width - bottom_width).max(0.0) * 0.5);

        let outer_tl_top = Point::new(bx + otl, by);
        let outer_tl_left = Point::new(bx, by + otl);
        let outer_tr_top = Point::new(bx + bw - otr, by);
        let outer_tr_right = Point::new(bx + bw, by + otr);
        let outer_br_right = Point::new(bx + bw, by + bh - obr);
        let outer_br_bottom = Point::new(bx + bw - obr, by + bh);
        let outer_bl_bottom = Point::new(bx + obl, by + bh);
        let outer_bl_left = Point::new(bx, by + bh - obl);

        let inner_tl_top = Point::new(bx + left_width + itl, by + top_width);
        let inner_tl_left = Point::new(bx + left_width, by + top_width + itl);
        let inner_tr_top = Point::new(bx + bw - right_width - itr, by + top_width);
        let inner_tr_right = Point::new(bx + bw - right_width, by + top_width + itr);
        let inner_br_right = Point::new(bx + bw - right_width, by + bh - bottom_width - ibr);
        let inner_br_bottom = Point::new(bx + bw - right_width - ibr, by + bh - bottom_width);
        let inner_bl_bottom = Point::new(bx + left_width + ibl, by + bh - bottom_width);
        let inner_bl_left = Point::new(bx + left_width, by + bh - bottom_width - ibl);

        let mut path = PathBuilder::new();
        match side_ix {
            0 => {
                path.move_to(outer_tl_left);
                path.line_to(outer_tl_top);
                path.line_to(outer_tr_top);
                path.line_to(outer_tr_right);
                path.line_to(inner_tr_right);
                path.line_to(inner_tr_top);
                path.line_to(inner_tl_top);
                path.line_to(inner_tl_left);
            }
            1 => {
                path.move_to(outer_tr_top);
                path.line_to(outer_tr_right);
                path.line_to(outer_br_right);
                path.line_to(outer_br_bottom);
                path.line_to(inner_br_bottom);
                path.line_to(inner_br_right);
                path.line_to(inner_tr_right);
                path.line_to(inner_tr_top);
            }
            2 => {
                path.move_to(outer_br_right);
                path.line_to(outer_br_bottom);
                path.line_to(outer_bl_bottom);
                path.line_to(outer_bl_left);
                path.line_to(inner_bl_left);
                path.line_to(inner_bl_bottom);
                path.line_to(inner_br_bottom);
                path.line_to(inner_br_right);
            }
            3 => {
                path.move_to(outer_bl_bottom);
                path.line_to(outer_bl_left);
                path.line_to(outer_tl_left);
                path.line_to(outer_tl_top);
                path.line_to(inner_tl_top);
                path.line_to(inner_tl_left);
                path.line_to(inner_bl_left);
                path.line_to(inner_bl_bottom);
            }
            _ => return None,
        }

        path.close();
        Some(path.detach())
    }

    fn round_corner_side_stroke_path(
        side_ix: usize,
        bounds: BoundingBox,
        outer_radii: (f32, f32, f32, f32),
    ) -> Option<Path> {
        let bx = bounds.x;
        let by = bounds.y;
        let bw = bounds.w;
        let bh = bounds.h;

        let (r_tl, r_tr, r_br, r_bl) = outer_radii;

        let mut path = PathBuilder::new();
        let diag = SQRT_2.recip();

        let outer_tl_center = Point::new(bx + r_tl, by + r_tl);
        let outer_tr_center = Point::new(bx + bw - r_tr, by + r_tr);
        let outer_br_center = Point::new(bx + bw - r_br, by + bh - r_br);
        let outer_bl_center = Point::new(bx + r_bl, by + bh - r_bl);

        match side_ix {
            0 => {
                let outer_start = if r_tl > 0.0 {
                    Point::new(outer_tl_center.x - r_tl * diag, outer_tl_center.y - r_tl * diag)
                } else {
                    Point::new(bx, by)
                };
                path.move_to(outer_start);
                if r_tl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_tl_center.x, outer_tl_center.y, r_tl),
                        225.0,
                        45.0,
                        false,
                    );
                }
                path.line_to((bx + bw - r_tr, by));
                if r_tr > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_tr_center.x, outer_tr_center.y, r_tr),
                        270.0,
                        45.0,
                        false,
                    );
                }
            }
            1 => {
                let outer_start = if r_tr > 0.0 {
                    Point::new(outer_tr_center.x + r_tr * diag, outer_tr_center.y - r_tr * diag)
                } else {
                    Point::new(bx + bw, by)
                };
                path.move_to(outer_start);
                if r_tr > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_tr_center.x, outer_tr_center.y, r_tr),
                        315.0,
                        45.0,
                        false,
                    );
                }
                path.line_to((bx + bw, by + bh - r_br));
                if r_br > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_br_center.x, outer_br_center.y, r_br),
                        0.0,
                        45.0,
                        false,
                    );
                }
            }
            2 => {
                let outer_start = if r_br > 0.0 {
                    Point::new(outer_br_center.x + r_br * diag, outer_br_center.y + r_br * diag)
                } else {
                    Point::new(bx + bw, by + bh)
                };
                path.move_to(outer_start);
                if r_br > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_br_center.x, outer_br_center.y, r_br),
                        45.0,
                        45.0,
                        false,
                    );
                }
                path.line_to((bx + r_bl, by + bh));
                if r_bl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_bl_center.x, outer_bl_center.y, r_bl),
                        90.0,
                        45.0,
                        false,
                    );
                }
            }
            3 => {
                let outer_start = if r_bl > 0.0 {
                    Point::new(outer_bl_center.x - r_bl * diag, outer_bl_center.y + r_bl * diag)
                } else {
                    Point::new(bx, by + bh)
                };
                path.move_to(outer_start);
                if r_bl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_bl_center.x, outer_bl_center.y, r_bl),
                        135.0,
                        45.0,
                        false,
                    );
                }
                path.line_to((bx, by + r_tl));
                if r_tl > 0.0 {
                    path.arc_to(
                        Self::corner_oval(outer_tl_center.x, outer_tl_center.y, r_tl),
                        180.0,
                        45.0,
                        false,
                    );
                }
            }
            _ => return None,
        }

        Some(path.detach())
    }

    fn bevel_corner_side_stroke_path(
        side_ix: usize,
        bounds: BoundingBox,
        outer_radii: (f32, f32, f32, f32),
    ) -> Option<Path> {
        let bx = bounds.x;
        let by = bounds.y;
        let bw = bounds.w;
        let bh = bounds.h;

        let (r_tl, r_tr, r_br, r_bl) = outer_radii;

        let otl = r_tl.min(bw * 0.5).min(bh * 0.5);
        let otr = r_tr.min(bw * 0.5).min(bh * 0.5);
        let obr = r_br.min(bw * 0.5).min(bh * 0.5);
        let obl = r_bl.min(bw * 0.5).min(bh * 0.5);

        let outer_tl_top = Point::new(bx + otl, by);
        let outer_tl_left = Point::new(bx, by + otl);
        let outer_tr_top = Point::new(bx + bw - otr, by);
        let outer_tr_right = Point::new(bx + bw, by + otr);
        let outer_br_right = Point::new(bx + bw, by + bh - obr);
        let outer_br_bottom = Point::new(bx + bw - obr, by + bh);
        let outer_bl_bottom = Point::new(bx + obl, by + bh);
        let outer_bl_left = Point::new(bx, by + bh - obl);

        let mut path = PathBuilder::new();
        match side_ix {
            0 => {
                path.move_to(outer_tl_left);
                path.line_to(outer_tl_top);
                path.line_to(outer_tr_top);
                path.line_to(outer_tr_right);
            }
            1 => {
                path.move_to(outer_tr_top);
                path.line_to(outer_tr_right);
                path.line_to(outer_br_right);
                path.line_to(outer_br_bottom);
            }
            2 => {
                path.move_to(outer_br_right);
                path.line_to(outer_br_bottom);
                path.line_to(outer_bl_bottom);
                path.line_to(outer_bl_left);
            }
            3 => {
                path.move_to(outer_bl_bottom);
                path.line_to(outer_bl_left);
                path.line_to(outer_tl_left);
                path.line_to(outer_tl_top);
            }
            _ => return None,
        }

        Some(path.detach())
    }

    fn smooth_corner_side_path(
        side_ix: usize,
        bounds: BoundingBox,
        outer_radii: (f32, f32, f32, f32),
        inner_radii: (f32, f32, f32, f32),
        corner_shapes: (CornerShape, CornerShape, CornerShape, CornerShape),
        corner_smoothing: (f32, f32, f32, f32),
        widths: (f32, f32, f32, f32),
    ) -> Option<Path> {
        let bx = bounds.x;
        let by = bounds.y;
        let bw = bounds.w;
        let bh = bounds.h;

        let (r_tl, r_tr, r_br, r_bl) = outer_radii;
        let (ir_tl, ir_tr, ir_br, ir_bl) = inner_radii;
        let (corner_tl_shape, corner_tr_shape, corner_br_shape, corner_bl_shape) = corner_shapes;
        let (corner_tl_smoothing, corner_tr_smoothing, corner_br_smoothing, corner_bl_smoothing) =
            corner_smoothing;
        let (top_width, right_width, bottom_width, left_width) = widths;

        let mut path = PathBuilder::new();

        match side_ix {
            0 => {
                // Top side: top-left to top-right
                let otl = r_tl.min(bw * 0.5).min(bh * 0.5);
                let otr = r_tr.min(bw * 0.5).min(bh * 0.5);
                let itl = ir_tl
                    .max(0.0)
                    .min((bw - left_width - right_width).max(0.0) * 0.5)
                    .min((bh - top_width - bottom_width).max(0.0) * 0.5);
                let itr = ir_tr
                    .max(0.0)
                    .min((bw - left_width - right_width).max(0.0) * 0.5)
                    .min((bh - top_width - bottom_width).max(0.0) * 0.5);

                path.move_to((bx + left_width, by + top_width + itl));

                // Outer top-left corner
                if otl > 0.0 && corner_tl_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) =
                        compute_smooth_corner(otl, corner_tl_smoothing, bw, bh);
                    let start_x = bx + f32::max(left_width * 0.5, left_width + p - left_width);
                    let start_y = by + top_width;
                    path.line_to((start_x, start_y));
                    path.cubic_to(
                        (bx + left_width + (p - a), by),
                        (bx + left_width + (p - a - b), by),
                        (bx + left_width + (p - a - b - c), d + by),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (l, l))
                    .cubic_to(
                        (bx + left_width + (p - a - b), by + top_width),
                        (bx + left_width + (p - a), by + top_width),
                        (bx + left_width + p.min(bw * 0.5), by + top_width),
                    );
                } else if otl > 0.0 {
                    let p = otl.min((bw * 0.5).min(bh * 0.5));
                    path.line_to((bx + left_width + p, by + top_width));
                } else {
                    path.line_to((bx + left_width, by + top_width));
                }

                // Outer top-right corner
                if otr > 0.0 {
                    let p = otr.min((bw * 0.5).min(bh * 0.5));
                    path.line_to((bx + bw - right_width - p, by + top_width));
                    if corner_tr_shape == CornerShape::Round {
                        let (a, b, c, d, l, radius_val, _) =
                            compute_smooth_corner(otr, corner_tr_smoothing, bw, bh);
                        path.cubic_to(
                            (bx + bw - right_width - (p - a), by),
                            (bx + bw - right_width - (p - a - b), by),
                            (bx + bw - right_width - (p - a - b - c) + d, by),
                        )
                        .r_arc_to(
                            (radius_val, radius_val),
                            0.0,
                            ArcSize::Small,
                            PathDirection::CW,
                            (l, l),
                        )
                        .cubic_to(
                            (bx + bw - right_width - (p - a - b), by + top_width),
                            (bx + bw - right_width - (p - a), by + top_width),
                            (bx + bw - right_width - p.min(bw * 0.5), by + top_width),
                        );
                    }
                } else {
                    path.line_to((bx + bw - right_width, by + top_width));
                }

                path.line_to((bx + bw - right_width, by + top_width + itr));

                // Inner top-right corner
                if itr > 0.0 && corner_tr_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) = compute_smooth_corner(
                        itr,
                        corner_tr_smoothing,
                        bw - left_width - right_width,
                        bh - top_width - bottom_width,
                    );
                    path.cubic_to(
                        (bx + bw - right_width - (p - a), by + top_width),
                        (bx + bw - right_width - (p - a - b), by + top_width),
                        (bx + bw - right_width - (p - a - b - c), by + top_width + d),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (-l, l))
                    .cubic_to(
                        (bx + bw - right_width, by + top_width + (p - a - b)),
                        (bx + bw - right_width, by + top_width + (p - a)),
                        (
                            bx + bw - right_width,
                            by + top_width + p.min((bh - top_width - bottom_width) * 0.5),
                        ),
                    );
                } else if itr > 0.0 {
                    let p = itr.min(
                        ((bh - top_width - bottom_width) * 0.5)
                            .min((bw - left_width - right_width) * 0.5),
                    );
                    path.line_to((bx + bw - right_width, by + top_width + p));
                }

                // Inner top-left corner
                if itl > 0.0 && corner_tl_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) = compute_smooth_corner(
                        itl,
                        corner_tl_smoothing,
                        bw - left_width - right_width,
                        bh - top_width - bottom_width,
                    );
                    path.line_to((bx + left_width + p, by + top_width));
                    path.cubic_to(
                        (bx + left_width + (p - a), by + top_width),
                        (bx + left_width + (p - a - b), by + top_width),
                        (bx + left_width + (p - a - b - c), by + top_width + d),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (-l, -l))
                    .cubic_to(
                        (bx + left_width, by + top_width + (p - a - b)),
                        (bx + left_width, by + top_width + (p - a)),
                        (
                            bx + left_width,
                            by + top_width + p.min((bh - top_width - bottom_width) * 0.5),
                        ),
                    );
                } else if itl > 0.0 {
                    let p = itl.min(
                        ((bh - top_width - bottom_width) * 0.5)
                            .min((bw - left_width - right_width) * 0.5),
                    );
                    path.line_to((bx + left_width, by + top_width + p));
                }
            }
            1 => {
                // Right side: top-right to bottom-right
                let otr = r_tr.min(bw * 0.5).min(bh * 0.5);
                let obr = r_br.min(bw * 0.5).min(bh * 0.5);
                let itr = ir_tr
                    .max(0.0)
                    .min((bw - left_width - right_width).max(0.0) * 0.5)
                    .min((bh - top_width - bottom_width).max(0.0) * 0.5);
                let ibr = ir_br
                    .max(0.0)
                    .min((bw - left_width - right_width).max(0.0) * 0.5)
                    .min((bh - top_width - bottom_width).max(0.0) * 0.5);

                path.move_to((bx + bw - right_width - itr, by + top_width));

                // Outer top-right corner
                if otr > 0.0 && corner_tr_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) =
                        compute_smooth_corner(otr, corner_tr_smoothing, bw, bh);
                    let start_x = bx + bw - right_width;
                    path.line_to((start_x, by + top_width + p));
                    path.cubic_to(
                        (start_x, by + (p - a)),
                        (start_x, by + (p - a - b)),
                        (start_x - d, by + (p - a - b - c)),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (-l, l))
                    .cubic_to(
                        (start_x - (p - a - b), by),
                        (start_x - (p - a), by),
                        (start_x - p.min(bw * 0.5), by),
                    );
                } else if otr > 0.0 {
                    let p = otr.min((bw * 0.5).min(bh * 0.5));
                    path.line_to((bx + bw - right_width, by + top_width + p));
                } else {
                    path.line_to((bx + bw - right_width, by + top_width));
                }

                // Outer bottom-right corner
                if obr > 0.0 {
                    let p = obr.min((bw * 0.5).min(bh * 0.5));
                    path.line_to((bx + bw - right_width, by + bh - bottom_width - p));
                    if corner_br_shape == CornerShape::Round {
                        let (a, b, c, d, l, radius_val, _) =
                            compute_smooth_corner(obr, corner_br_smoothing, bw, bh);
                        path.cubic_to(
                            (bx + bw, by + bh - bottom_width - (p - a)),
                            (bx + bw, by + bh - bottom_width - (p - a - b)),
                            (bx + bw, by + bh - bottom_width - (p - a - b - c) + d),
                        )
                        .r_arc_to(
                            (radius_val, radius_val),
                            0.0,
                            ArcSize::Small,
                            PathDirection::CW,
                            (l, l),
                        )
                        .cubic_to(
                            (bx + bw - (p - a - b), by + bh),
                            (bx + bw - (p - a), by + bh),
                            (bx + bw - p.min(bw * 0.5), by + bh),
                        );
                    }
                } else {
                    path.line_to((bx + bw - right_width, by + bh - bottom_width));
                }

                path.line_to((bx + bw - right_width - ibr, by + bh - bottom_width));

                // Inner bottom-right corner
                if ibr > 0.0 && corner_br_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) = compute_smooth_corner(
                        ibr,
                        corner_br_smoothing,
                        bw - left_width - right_width,
                        bh - top_width - bottom_width,
                    );
                    path.cubic_to(
                        (bx + bw - right_width - (p - a), by + bh - bottom_width),
                        (bx + bw - right_width - (p - a - b), by + bh - bottom_width),
                        (bx + bw - right_width - (p - a - b - c), by + bh - bottom_width + d),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (-l, -l))
                    .cubic_to(
                        (bx + bw - right_width, by + bh - bottom_width - (p - a - b)),
                        (bx + bw - right_width, by + bh - bottom_width - (p - a)),
                        (
                            bx + bw - right_width,
                            by + bh - bottom_width - p.min((bh - top_width - bottom_width) * 0.5),
                        ),
                    );
                } else if ibr > 0.0 {
                    let p = ibr.min(
                        ((bh - top_width - bottom_width) * 0.5)
                            .min((bw - left_width - right_width) * 0.5),
                    );
                    path.line_to((bx + bw - right_width, by + bh - bottom_width - p));
                }

                // Inner top-right corner
                if itr > 0.0 && corner_tr_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) = compute_smooth_corner(
                        itr,
                        corner_tr_smoothing,
                        bw - left_width - right_width,
                        bh - top_width - bottom_width,
                    );
                    path.line_to((bx + bw - right_width, by + top_width + p));
                    path.cubic_to(
                        (bx + bw - right_width, by + top_width + (p - a)),
                        (bx + bw - right_width, by + top_width + (p - a - b)),
                        (bx + bw - right_width + d, by + top_width + (p - a - b - c)),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (l, -l))
                    .cubic_to(
                        (bx + bw - right_width - (p - a - b), by + top_width),
                        (bx + bw - right_width - (p - a), by + top_width),
                        (
                            bx + bw - right_width - p.min((bw - left_width - right_width) * 0.5),
                            by + top_width,
                        ),
                    );
                } else if itr > 0.0 {
                    let p = itr.min(
                        ((bh - top_width - bottom_width) * 0.5)
                            .min((bw - left_width - right_width) * 0.5),
                    );
                    path.line_to((bx + bw - right_width, by + top_width + p));
                }
            }
            2 => {
                // Bottom side: bottom-right to bottom-left
                let obr = r_br.min(bw * 0.5).min(bh * 0.5);
                let obl = r_bl.min(bw * 0.5).min(bh * 0.5);
                let ibr = ir_br
                    .max(0.0)
                    .min((bw - left_width - right_width).max(0.0) * 0.5)
                    .min((bh - top_width - bottom_width).max(0.0) * 0.5);
                let ibl = ir_bl
                    .max(0.0)
                    .min((bw - left_width - right_width).max(0.0) * 0.5)
                    .min((bh - top_width - bottom_width).max(0.0) * 0.5);

                path.move_to((bx + bw - right_width - ibr, by + bh - bottom_width));

                // Outer bottom-right corner
                if obr > 0.0 && corner_br_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) =
                        compute_smooth_corner(obr, corner_br_smoothing, bw, bh);
                    let end_x = bx + bw - right_width;
                    path.line_to((end_x - p, by + bh - bottom_width));
                    path.cubic_to(
                        (end_x - (p - a), by + bh),
                        (end_x - (p - a - b), by + bh),
                        (end_x - (p - a - b - c) + d, by + bh),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (-l, l))
                    .cubic_to(
                        (end_x - (p - a - b), by + bh - bottom_width),
                        (end_x - (p - a), by + bh - bottom_width),
                        (end_x - p.min(bw * 0.5), by + bh - bottom_width),
                    );
                } else if obr > 0.0 {
                    let p = obr.min((bw * 0.5).min(bh * 0.5));
                    path.line_to((bx + bw - right_width - p, by + bh - bottom_width));
                } else {
                    path.line_to((bx + bw - right_width, by + bh - bottom_width));
                }

                // Outer bottom-left corner
                if obl > 0.0 {
                    let p = obl.min((bw * 0.5).min(bh * 0.5));
                    path.line_to((bx + left_width + p, by + bh - bottom_width));
                    if corner_bl_shape == CornerShape::Round {
                        let (a, b, c, d, l, radius_val, _) =
                            compute_smooth_corner(obl, corner_bl_smoothing, bw, bh);
                        path.cubic_to(
                            (bx + left_width + (p - a), by + bh),
                            (bx + left_width + (p - a - b), by + bh),
                            (bx + left_width + (p - a - b - c) - d, by + bh),
                        )
                        .r_arc_to(
                            (radius_val, radius_val),
                            0.0,
                            ArcSize::Small,
                            PathDirection::CW,
                            (-l, -l),
                        )
                        .cubic_to(
                            (bx + left_width + (p - a - b), by + bh - bottom_width),
                            (bx + left_width + (p - a), by + bh - bottom_width),
                            (bx + left_width + p.min(bw * 0.5), by + bh - bottom_width),
                        );
                    }
                } else {
                    path.line_to((bx + left_width, by + bh - bottom_width));
                }

                path.line_to((bx + left_width + ibl, by + bh - bottom_width));

                // Inner bottom-left corner
                if ibl > 0.0 && corner_bl_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) = compute_smooth_corner(
                        ibl,
                        corner_bl_smoothing,
                        bw - left_width - right_width,
                        bh - top_width - bottom_width,
                    );
                    path.cubic_to(
                        (bx + left_width + (p - a), by + bh - bottom_width),
                        (bx + left_width + (p - a - b), by + bh - bottom_width),
                        (bx + left_width + (p - a - b - c), by + bh - bottom_width + d),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (l, -l))
                    .cubic_to(
                        (bx + left_width, by + bh - bottom_width - (p - a - b)),
                        (bx + left_width, by + bh - bottom_width - (p - a)),
                        (
                            bx + left_width,
                            by + bh - bottom_width - p.min((bh - top_width - bottom_width) * 0.5),
                        ),
                    );
                } else if ibl > 0.0 {
                    let p = ibl.min(
                        ((bh - top_width - bottom_width) * 0.5)
                            .min((bw - left_width - right_width) * 0.5),
                    );
                    path.line_to((bx + left_width, by + bh - bottom_width - p));
                }

                // Inner bottom-right corner
                if ibr > 0.0 && corner_br_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) = compute_smooth_corner(
                        ibr,
                        corner_br_smoothing,
                        bw - left_width - right_width,
                        bh - top_width - bottom_width,
                    );
                    path.line_to((bx + bw - right_width - p, by + bh - bottom_width));
                    path.cubic_to(
                        (bx + bw - right_width - (p - a), by + bh - bottom_width),
                        (bx + bw - right_width - (p - a - b), by + bh - bottom_width),
                        (bx + bw - right_width - (p - a - b - c), by + bh - bottom_width + d),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (l, l))
                    .cubic_to(
                        (bx + bw - right_width, by + bh - bottom_width - (p - a - b)),
                        (bx + bw - right_width, by + bh - bottom_width - (p - a)),
                        (
                            bx + bw - right_width,
                            by + bh - bottom_width - p.min((bh - top_width - bottom_width) * 0.5),
                        ),
                    );
                } else if ibr > 0.0 {
                    let p = ibr.min(
                        ((bh - top_width - bottom_width) * 0.5)
                            .min((bw - left_width - right_width) * 0.5),
                    );
                    path.line_to((bx + bw - right_width, by + bh - bottom_width - p));
                }
            }
            3 => {
                // Left side: bottom-left to top-left
                let obl = r_bl.min(bw * 0.5).min(bh * 0.5);
                let otl = r_tl.min(bw * 0.5).min(bh * 0.5);
                let ibl = ir_bl
                    .max(0.0)
                    .min((bw - left_width - right_width).max(0.0) * 0.5)
                    .min((bh - top_width - bottom_width).max(0.0) * 0.5);
                let itl = ir_tl
                    .max(0.0)
                    .min((bw - left_width - right_width).max(0.0) * 0.5)
                    .min((bh - top_width - bottom_width).max(0.0) * 0.5);

                path.move_to((bx + left_width + ibl, by + bh - bottom_width));

                // Outer bottom-left corner
                if obl > 0.0 && corner_bl_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) =
                        compute_smooth_corner(obl, corner_bl_smoothing, bw, bh);
                    path.line_to((bx + left_width, by + bh - bottom_width - p));
                    path.cubic_to(
                        (bx, by + bh - bottom_width - (p - a)),
                        (bx, by + bh - bottom_width - (p - a - b)),
                        (bx, by + bh - bottom_width - (p - a - b - c) - d),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (l, -l))
                    .cubic_to(
                        (bx + (p - a - b), by),
                        (bx + (p - a), by),
                        (bx + p.min(bw * 0.5), by),
                    );
                } else if obl > 0.0 {
                    let p = obl.min((bw * 0.5).min(bh * 0.5));
                    path.line_to((bx + left_width, by + bh - bottom_width - p));
                } else {
                    path.line_to((bx + left_width, by + bh - bottom_width));
                }

                // Outer top-left corner
                if otl > 0.0 {
                    let p = otl.min((bw * 0.5).min(bh * 0.5));
                    path.line_to((bx + left_width, by + top_width + p));
                    if corner_tl_shape == CornerShape::Round {
                        let (a, b, c, d, l, radius_val, _) =
                            compute_smooth_corner(otl, corner_tl_smoothing, bw, bh);
                        path.cubic_to(
                            (bx, by + top_width + (p - a)),
                            (bx, by + top_width + (p - a - b)),
                            (bx, by + top_width + (p - a - b - c) - d),
                        )
                        .r_arc_to(
                            (radius_val, radius_val),
                            0.0,
                            ArcSize::Small,
                            PathDirection::CW,
                            (l, l),
                        )
                        .cubic_to(
                            (bx + (p - a - b), by),
                            (bx + (p - a), by),
                            (bx + p.min(bw * 0.5), by),
                        );
                    }
                } else {
                    path.line_to((bx + left_width, by + top_width));
                }

                path.line_to((bx + left_width + itl, by + top_width));

                // Inner top-left corner
                if itl > 0.0 && corner_tl_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) = compute_smooth_corner(
                        itl,
                        corner_tl_smoothing,
                        bw - left_width - right_width,
                        bh - top_width - bottom_width,
                    );
                    path.cubic_to(
                        (bx + left_width + (p - a), by + top_width),
                        (bx + left_width + (p - a - b), by + top_width),
                        (bx + left_width + (p - a - b - c), by + top_width + d),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (l, l))
                    .cubic_to(
                        (bx + left_width, by + top_width + (p - a - b)),
                        (bx + left_width, by + top_width + (p - a)),
                        (
                            bx + left_width,
                            by + top_width + p.min((bh - top_width - bottom_width) * 0.5),
                        ),
                    );
                } else if itl > 0.0 {
                    let p = itl.min(
                        ((bh - top_width - bottom_width) * 0.5)
                            .min((bw - left_width - right_width) * 0.5),
                    );
                    path.line_to((bx + left_width, by + top_width + p));
                }

                // Inner bottom-left corner
                if ibl > 0.0 && corner_bl_shape == CornerShape::Round {
                    let (a, b, c, d, l, p, radius) = compute_smooth_corner(
                        ibl,
                        corner_bl_smoothing,
                        bw - left_width - right_width,
                        bh - top_width - bottom_width,
                    );
                    path.line_to((bx + left_width, by + bh - bottom_width - p));
                    path.cubic_to(
                        (bx + left_width, by + bh - bottom_width - (p - a)),
                        (bx + left_width, by + bh - bottom_width - (p - a - b)),
                        (bx + left_width - d, by + bh - bottom_width - (p - a - b - c)),
                    )
                    .r_arc_to((radius, radius), 0.0, ArcSize::Small, PathDirection::CW, (-l, -l))
                    .cubic_to(
                        (bx + left_width + (p - a - b), by + bh),
                        (bx + left_width + (p - a), by + bh),
                        (bx + left_width + p.min((bw - left_width - right_width) * 0.5), by + bh),
                    );
                } else if ibl > 0.0 {
                    let p = ibl.min(
                        ((bh - top_width - bottom_width) * 0.5)
                            .min((bw - left_width - right_width) * 0.5),
                    );
                    path.line_to((bx + left_width, by + bh - bottom_width - p));
                }
            }
            _ => return None,
        }

        path.close();
        Some(path.detach())
    }

    /// Draw background color or background image (including gradients) for the current view.
    pub fn draw_background(&mut self, canvas: &Canvas) {
        let background_color = self.background_color();
        if background_color.a() > 0 {
            let path = self.path();

            let mut paint = Paint::default();
            paint.set_color(skia_safe::Color::from_argb(
                background_color.a(),
                background_color.r(),
                background_color.g(),
                background_color.b(),
            ));
            paint.set_anti_alias(true);
            canvas.draw_path(&path, &paint);
        }

        self.draw_background_images(canvas);
    }

    /// Draw the border of the current view.
    ///
    /// Supports individual side widths, colors, and styles. Borders are rendered from a
    /// ring path (outer minus inner), then split into per-side ownership paths using path
    /// intersections. Dashed/dotted use side-local center-line strokes clipped by the
    /// corresponding side ownership path.
    pub fn draw_border(&mut self, canvas: &Canvas) {
        let top_width = self.border_top_width();
        let right_width = self.border_right_width();
        let bottom_width = self.border_bottom_width();
        let left_width = self.border_left_width();

        let top_color = self.border_top_color();
        let right_color = self.border_right_color();
        let bottom_color = self.border_bottom_color();
        let left_color = self.border_left_color();

        let top_style = self.border_top_style();
        let right_style = self.border_right_style();
        let bottom_style = self.border_bottom_style();
        let left_style = self.border_left_style();

        let top_vis = top_width > 0.0 && top_color.a() > 0 && top_style != BorderStyleKeyword::None;
        let right_vis =
            right_width > 0.0 && right_color.a() > 0 && right_style != BorderStyleKeyword::None;
        let bottom_vis =
            bottom_width > 0.0 && bottom_color.a() > 0 && bottom_style != BorderStyleKeyword::None;
        let left_vis =
            left_width > 0.0 && left_color.a() > 0 && left_style != BorderStyleKeyword::None;

        if !top_vis && !right_vis && !bottom_vis && !left_vis {
            return;
        }

        let bounds = self.bounds();
        let bx = bounds.x;
        let by = bounds.y;
        let bw = bounds.w;
        let bh = bounds.h;

        // Outer corner radii (from style).
        let r_tl = self.corner_top_left_radius();
        let r_tr = self.corner_top_right_radius();
        let r_br = self.corner_bottom_right_radius();
        let r_bl = self.corner_bottom_left_radius();

        // Inner corner radii: outer radius minus the thicker adjacent border, clamped to 0.
        let ir_tl = (r_tl - top_width.max(left_width)).max(0.0);
        let ir_tr = (r_tr - top_width.max(right_width)).max(0.0);
        let ir_br = (r_br - bottom_width.max(right_width)).max(0.0);
        let ir_bl = (r_bl - bottom_width.max(left_width)).max(0.0);

        // Inner rect dimensions.
        let inner_w = (bw - left_width - right_width).max(0.0);
        let inner_h = (bh - top_width - bottom_width).max(0.0);
        let inner_is_empty = inner_w <= 0.0 || inner_h <= 0.0;

        let corner_shapes = (
            self.corner_top_left_shape(),
            self.corner_top_right_shape(),
            self.corner_bottom_right_shape(),
            self.corner_bottom_left_shape(),
        );
        let corner_smoothing = (
            self.corner_top_left_smoothing(),
            self.corner_top_right_smoothing(),
            self.corner_bottom_right_smoothing(),
            self.corner_bottom_left_smoothing(),
        );
        let uniform_borders = top_width == right_width
            && right_width == bottom_width
            && bottom_width == left_width
            && top_color == right_color
            && right_color == bottom_color
            && bottom_color == left_color
            && top_style == right_style
            && right_style == bottom_style
            && bottom_style == left_style;
        let uniform_corners = r_tl == r_tr
            && r_tr == r_br
            && r_br == r_bl
            && corner_shapes.0 == corner_shapes.1
            && corner_shapes.1 == corner_shapes.2
            && corner_shapes.2 == corner_shapes.3
            && corner_smoothing.0 == corner_smoothing.1
            && corner_smoothing.1 == corner_smoothing.2
            && corner_smoothing.2 == corner_smoothing.3;
        let exact_round_solid = !inner_is_empty
            && corner_shapes.0 == CornerShape::Round
            && corner_shapes.1 == CornerShape::Round
            && corner_shapes.2 == CornerShape::Round
            && corner_shapes.3 == CornerShape::Round
            && corner_smoothing.0 == 0.0
            && corner_smoothing.1 == 0.0
            && corner_smoothing.2 == 0.0
            && corner_smoothing.3 == 0.0;
        let exact_bevel_solid = !inner_is_empty
            && corner_shapes.0 == CornerShape::Bevel
            && corner_shapes.1 == CornerShape::Bevel
            && corner_shapes.2 == CornerShape::Bevel
            && corner_shapes.3 == CornerShape::Bevel
            && corner_smoothing.0 == 0.0
            && corner_smoothing.1 == 0.0
            && corner_smoothing.2 == 0.0
            && corner_smoothing.3 == 0.0;
        let exact_smooth_solid = !inner_is_empty
            && corner_shapes.0 == CornerShape::Round
            && corner_shapes.1 == CornerShape::Round
            && corner_shapes.2 == CornerShape::Round
            && corner_shapes.3 == CornerShape::Round
            && (corner_smoothing.0 > 0.0
                || corner_smoothing.1 > 0.0
                || corner_smoothing.2 > 0.0
                || corner_smoothing.3 > 0.0);

        // Outer path in world coordinates.
        let outer_path = self
            .build_path_with_corners(
                bounds,
                (0.0, 0.0),
                (r_tl, r_tr, r_br, r_bl),
                corner_shapes,
                corner_smoothing,
            )
            .make_offset(bounds.top_left());

        // Inner path in world coordinates uses the same corner shape/smoothing pipeline as outer.
        let inner_path = if inner_is_empty {
            None
        } else {
            let inner_bounds = BoundingBox::from_min_max(0.0, 0.0, inner_w, inner_h);
            Some(
                self.build_path_with_corners(
                    inner_bounds,
                    (0.0, 0.0),
                    (ir_tl, ir_tr, ir_br, ir_bl),
                    corner_shapes,
                    corner_smoothing,
                )
                .make_offset((bx + left_width, by + top_width)),
            )
        };

        let ring_path = if let Some(inner) = inner_path.as_ref() {
            outer_path
                .op(inner, skia_safe::PathOp::Difference)
                .unwrap_or_else(|| outer_path.clone())
        } else {
            outer_path.clone()
        };

        if uniform_borders && uniform_corners {
            match top_style {
                BorderStyleKeyword::Dashed | BorderStyleKeyword::Dotted => {
                    canvas.save();
                    canvas.clip_path(&ring_path, ClipOp::Intersect, true);

                    let half = top_width * 0.5;
                    let stroke_path = self
                        .build_path_with_corners(
                            bounds,
                            (-half, -half),
                            (r_tl, r_tr, r_br, r_bl),
                            corner_shapes,
                            corner_smoothing,
                        )
                        .make_offset(bounds.top_left());

                    let mut paint = Paint::default();
                    paint.set_style(PaintStyle::Stroke);
                    paint.set_color(top_color);
                    paint.set_stroke_width(top_width);
                    if top_style == BorderStyleKeyword::Dashed {
                        paint.set_path_effect(PathEffect::dash(&[top_width * 2.0, top_width], 0.0));
                    } else {
                        paint.set_path_effect(PathEffect::dash(&[0.0, top_width * 2.0], 0.0));
                        paint.set_stroke_cap(skia_safe::PaintCap::Round);
                    }
                    paint.set_anti_alias(true);
                    canvas.draw_path(&stroke_path, &paint);
                    canvas.restore();
                    return;
                }

                _ => {
                    let mut paint = Paint::default();
                    paint.set_color(top_color);
                    paint.set_anti_alias(true);
                    canvas.draw_path(&ring_path, &paint);
                    return;
                }
            }
        }

        // Side ownership masks split corner ownership at the outer->inner corner join line.
        let mask_top: [Point; 4] = [
            Point::new(bx, by),
            Point::new(bx + bw, by),
            Point::new(bx + bw - right_width, by + top_width),
            Point::new(bx + left_width, by + top_width),
        ];
        let mask_right: [Point; 4] = [
            Point::new(bx + bw - right_width, by + top_width),
            Point::new(bx + bw, by),
            Point::new(bx + bw, by + bh),
            Point::new(bx + bw - right_width, by + bh - bottom_width),
        ];
        let mask_bottom: [Point; 4] = [
            Point::new(bx + left_width, by + bh - bottom_width),
            Point::new(bx + bw - right_width, by + bh - bottom_width),
            Point::new(bx + bw, by + bh),
            Point::new(bx, by + bh),
        ];
        let mask_left: [Point; 4] = [
            Point::new(bx, by),
            Point::new(bx + left_width, by + top_width),
            Point::new(bx + left_width, by + bh - bottom_width),
            Point::new(bx, by + bh),
        ];

        let sides: [(usize, bool, Color, BorderStyleKeyword, f32, [Point; 4]); 4] = [
            (0, top_vis, top_color, top_style, top_width, mask_top),
            (1, right_vis, right_color, right_style, right_width, mask_right),
            (2, bottom_vis, bottom_color, bottom_style, bottom_width, mask_bottom),
            (3, left_vis, left_color, left_style, left_width, mask_left),
        ];

        for (side_ix, vis, color, style, side_width, mask_pts) in sides {
            if !vis {
                continue;
            }

            let mut side_mask = PathBuilder::new();
            side_mask.move_to(mask_pts[0]);
            side_mask.line_to(mask_pts[1]);
            side_mask.line_to(mask_pts[2]);
            side_mask.line_to(mask_pts[3]);
            side_mask.close();
            let side_mask = side_mask.detach();
            let Some(side_region) = ring_path.op(&side_mask, skia_safe::PathOp::Intersect) else {
                continue;
            };

            match style {
                BorderStyleKeyword::Dashed | BorderStyleKeyword::Dotted => {
                    canvas.save();
                    canvas.clip_path(&side_region, ClipOp::Intersect, true);

                    let stroke_path = if exact_round_solid {
                        Self::round_corner_side_stroke_path(
                            side_ix,
                            bounds,
                            (r_tl, r_tr, r_br, r_bl),
                        )
                        .unwrap_or_else(|| {
                            let mut side_path = PathBuilder::new();
                            let (sx, sy, ex, ey) = match side_ix {
                                0 => (
                                    bx + left_width * 0.5,
                                    by + top_width * 0.5,
                                    bx + bw - right_width * 0.5,
                                    by + top_width * 0.5,
                                ),
                                1 => (
                                    bx + bw - right_width * 0.5,
                                    by + top_width * 0.5,
                                    bx + bw - right_width * 0.5,
                                    by + bh - bottom_width * 0.5,
                                ),
                                2 => (
                                    bx + left_width * 0.5,
                                    by + bh - bottom_width * 0.5,
                                    bx + bw - right_width * 0.5,
                                    by + bh - bottom_width * 0.5,
                                ),
                                _ => (
                                    bx + left_width * 0.5,
                                    by + top_width * 0.5,
                                    bx + left_width * 0.5,
                                    by + bh - bottom_width * 0.5,
                                ),
                            };
                            side_path.move_to((sx, sy));
                            side_path.line_to((ex, ey));
                            side_path.detach()
                        })
                    } else if exact_bevel_solid {
                        Self::bevel_corner_side_stroke_path(
                            side_ix,
                            bounds,
                            (r_tl, r_tr, r_br, r_bl),
                        )
                        .unwrap_or_else(|| {
                            let mut side_path = PathBuilder::new();
                            let (sx, sy, ex, ey) = match side_ix {
                                0 => (
                                    bx + left_width * 0.5,
                                    by + top_width * 0.5,
                                    bx + bw - right_width * 0.5,
                                    by + top_width * 0.5,
                                ),
                                1 => (
                                    bx + bw - right_width * 0.5,
                                    by + top_width * 0.5,
                                    bx + bw - right_width * 0.5,
                                    by + bh - bottom_width * 0.5,
                                ),
                                2 => (
                                    bx + left_width * 0.5,
                                    by + bh - bottom_width * 0.5,
                                    bx + bw - right_width * 0.5,
                                    by + bh - bottom_width * 0.5,
                                ),
                                _ => (
                                    bx + left_width * 0.5,
                                    by + top_width * 0.5,
                                    bx + left_width * 0.5,
                                    by + bh - bottom_width * 0.5,
                                ),
                            };
                            side_path.move_to((sx, sy));
                            side_path.line_to((ex, ey));
                            side_path.detach()
                        })
                    } else if exact_smooth_solid {
                        self.build_path_with_corners(
                            bounds,
                            (-side_width * 0.5, -side_width * 0.5),
                            (r_tl, r_tr, r_br, r_bl),
                            corner_shapes,
                            corner_smoothing,
                        )
                        .make_offset(bounds.top_left())
                    } else {
                        let mut side_path = PathBuilder::new();
                        let (sx, sy, ex, ey) = match side_ix {
                            0 => (
                                bx + left_width * 0.5,
                                by + top_width * 0.5,
                                bx + bw - right_width * 0.5,
                                by + top_width * 0.5,
                            ),
                            1 => (
                                bx + bw - right_width * 0.5,
                                by + top_width * 0.5,
                                bx + bw - right_width * 0.5,
                                by + bh - bottom_width * 0.5,
                            ),
                            2 => (
                                bx + left_width * 0.5,
                                by + bh - bottom_width * 0.5,
                                bx + bw - right_width * 0.5,
                                by + bh - bottom_width * 0.5,
                            ),
                            _ => (
                                bx + left_width * 0.5,
                                by + top_width * 0.5,
                                bx + left_width * 0.5,
                                by + bh - bottom_width * 0.5,
                            ),
                        };
                        side_path.move_to((sx, sy));
                        side_path.line_to((ex, ey));
                        side_path.detach()
                    };
                    let mut paint = Paint::default();
                    paint.set_style(PaintStyle::Stroke);
                    paint.set_color(color);
                    paint.set_stroke_width(side_width);
                    if style == BorderStyleKeyword::Dashed {
                        paint.set_path_effect(PathEffect::dash(
                            &[side_width * 2.0, side_width],
                            0.0,
                        ));
                    } else {
                        paint.set_path_effect(PathEffect::dash(&[0.0, side_width * 2.0], 0.0));
                        paint.set_stroke_cap(skia_safe::PaintCap::Round);
                    }
                    paint.set_anti_alias(true);
                    canvas.draw_path(&stroke_path, &paint);
                    canvas.restore();
                }
                _ => {
                    let exact_side_path = if exact_round_solid {
                        Self::round_corner_side_path(
                            side_ix,
                            bounds,
                            (r_tl, r_tr, r_br, r_bl),
                            (ir_tl, ir_tr, ir_br, ir_bl),
                            (top_width, right_width, bottom_width, left_width),
                        )
                    } else if exact_bevel_solid {
                        Self::bevel_corner_side_path(
                            side_ix,
                            bounds,
                            (r_tl, r_tr, r_br, r_bl),
                            (ir_tl, ir_tr, ir_br, ir_bl),
                            (top_width, right_width, bottom_width, left_width),
                        )
                    } else if exact_smooth_solid {
                        Self::smooth_corner_side_path(
                            side_ix,
                            bounds,
                            (r_tl, r_tr, r_br, r_bl),
                            (ir_tl, ir_tr, ir_br, ir_bl),
                            corner_shapes,
                            corner_smoothing,
                            (top_width, right_width, bottom_width, left_width),
                        )
                    } else {
                        None
                    };
                    // Solid and any other opaque style: fill this side's owned part of the ring.
                    let mut paint = Paint::default();
                    paint.set_color(color);
                    paint.set_anti_alias(true);
                    if let Some(path) = exact_side_path {
                        let Some(constrained_path) =
                            path.op(&ring_path, skia_safe::PathOp::Intersect)
                        else {
                            continue;
                        };
                        canvas.draw_path(&constrained_path, &paint);
                    } else {
                        canvas.draw_path(&side_region, &paint);
                    }
                }
            }
        }
    }

    /// Draw the outline of the current view.
    pub fn draw_outline(&mut self, canvas: &Canvas) {
        let outline_width = self.outline_width();
        let outline_color = self.outline_color();

        if outline_width > 0.0 && outline_color.a() != 0 {
            let outline_offset = self.outline_offset();

            let bounds = self.bounds();

            let half_outline_width = outline_width / 2.0;
            let mut outline_path = self.build_path(
                bounds,
                (half_outline_width + outline_offset, half_outline_width + outline_offset),
            );

            outline_path = outline_path.make_offset(self.bounds().top_left());

            let mut outline_paint = Paint::default();
            outline_paint.set_color(outline_color);
            outline_paint.set_stroke_width(outline_width);
            outline_paint.set_style(PaintStyle::Stroke);
            outline_paint.set_anti_alias(true);
            canvas.draw_path(&outline_path, &outline_paint);
        }
    }

    /// Draw shadows for the current view.
    pub fn draw_shadows(&mut self, canvas: &Canvas) {
        if let Some(shadows) = self.shadows() {
            if shadows.is_empty() {
                return;
            }

            let bounds = self.bounds();

            let path = self.build_path(bounds, (0.0, 0.0)).make_offset(bounds.top_left());

            for shadow in shadows.iter().rev() {
                let shadow_color = shadow.color.unwrap_or_default();

                let shadow_x_offset = shadow.x_offset.to_px().unwrap_or(0.0) * self.scale_factor();
                let shadow_y_offset = shadow.y_offset.to_px().unwrap_or(0.0) * self.scale_factor();
                let spread_radius =
                    shadow.spread_radius.as_ref().and_then(|l| l.to_px()).unwrap_or(0.0)
                        * self.scale_factor();

                let blur_radius =
                    shadow.blur_radius.as_ref().and_then(|br| br.to_px()).unwrap_or(0.0);

                if shadow_color.a() == 0
                    || (shadow_x_offset == 0.0
                        && shadow_y_offset == 0.0
                        && spread_radius == 0.0
                        && blur_radius == 0.0)
                {
                    continue;
                }

                let mut shadow_paint = Paint::default();

                let outset = if shadow.inset { -spread_radius } else { spread_radius };

                shadow_paint.set_style(PaintStyle::Fill);

                let mut shadow_path = self.build_path(bounds, (outset, outset));
                shadow_path = shadow_path.make_offset(bounds.top_left());

                shadow_paint.set_color(shadow_color);

                if blur_radius > 0.0 {
                    shadow_paint.set_mask_filter(MaskFilter::blur(
                        BlurStyle::Normal,
                        blur_radius / 2.0,
                        false,
                    ));
                }

                shadow_path = shadow_path.make_offset((shadow_x_offset, shadow_y_offset));

                if shadow.inset {
                    shadow_path = path.op(&shadow_path, skia_safe::PathOp::Difference).unwrap();
                }

                canvas.save();
                canvas.clip_path(
                    &path,
                    if shadow.inset { ClipOp::Intersect } else { ClipOp::Difference },
                    true,
                );
                canvas.draw_path(&shadow_path, &shadow_paint);
                canvas.restore();
            }
        }
    }

    /// Draw background images (including gradients) for the current view.
    fn draw_background_images(&mut self, canvas: &Canvas) {
        let bounds = self.bounds();

        if self.background_images().is_some() {
            let path = self.path();
            if let Some(images) = self.background_images() {
                let image_sizes = self.background_size();
                let image_positions = self.background_position();
                let image_repeats = self.background_repeat();

                for (index, image) in images.iter().enumerate() {
                    match image {
                        ImageOrGradient::Gradient(gradient) => match gradient {
                            Gradient::Linear(linear_gradient) => {
                                let (start, end, parent_length) = match linear_gradient.direction {
                                    LineDirection::Horizontal(horizontal_keyword) => {
                                        match horizontal_keyword {
                                            HorizontalPositionKeyword::Left => (
                                                bounds.center_right(),
                                                bounds.center_left(),
                                                bounds.width(),
                                            ),

                                            HorizontalPositionKeyword::Right => (
                                                bounds.center_left(),
                                                bounds.center_right(),
                                                bounds.width(),
                                            ),
                                        }
                                    }

                                    LineDirection::Vertical(vertical_keyword) => {
                                        match vertical_keyword {
                                            VerticalPositionKeyword::Top => (
                                                bounds.center_bottom(),
                                                bounds.center_top(),
                                                bounds.height(),
                                            ),

                                            VerticalPositionKeyword::Bottom => (
                                                bounds.center_top(),
                                                bounds.center_bottom(),
                                                bounds.height(),
                                            ),
                                        }
                                    }

                                    LineDirection::Corner { horizontal, vertical } => {
                                        match (horizontal, vertical) {
                                            (
                                                HorizontalPositionKeyword::Right,
                                                VerticalPositionKeyword::Bottom,
                                            ) => (
                                                bounds.top_left(),
                                                bounds.bottom_right(),
                                                bounds.diagonal(),
                                            ),

                                            (
                                                HorizontalPositionKeyword::Right,
                                                VerticalPositionKeyword::Top,
                                            ) => (
                                                bounds.bottom_left(),
                                                bounds.top_right(),
                                                bounds.diagonal(),
                                            ),

                                            _ => (bounds.top_left(), bounds.bottom_right(), 0.0),
                                        }
                                    }

                                    LineDirection::Angle(angle) => {
                                        let angle_rad = angle.to_radians();
                                        let start_x = bounds.x
                                            + ((angle_rad.sin() * bounds.w) - bounds.w) / -2.0;
                                        let end_x = bounds.x
                                            + ((angle_rad.sin() * bounds.w) + bounds.w) / 2.0;
                                        let start_y = bounds.y
                                            + ((angle_rad.cos() * bounds.h) + bounds.h) / 2.0;
                                        let end_y = bounds.y
                                            + ((angle_rad.cos() * bounds.h) - bounds.h) / -2.0;

                                        let x = (end_x - start_x).abs();
                                        let y = (end_y - start_y).abs();

                                        let dist = (x * x + y * y).sqrt();

                                        ((start_x, start_y), (end_x, end_y), dist)
                                    }
                                };

                                let num_stops = linear_gradient.stops.len();

                                let mut stops = linear_gradient
                                    .stops
                                    .iter()
                                    .enumerate()
                                    .map(|(index, stop)| {
                                        let pos = if let Some(pos) = &stop.position {
                                            pos.to_pixels(parent_length, self.scale_factor())
                                                / parent_length
                                        } else {
                                            index as f32 / (num_stops - 1) as f32
                                        };
                                        (pos, skia_safe::Color::from(stop.color))
                                    })
                                    .collect::<Vec<_>>();

                                // Insert a stop at the front if the first stop is not at 0.
                                if let Some(first) = stops.first() {
                                    if first.0 != 0.0 {
                                        stops.insert(0, (0.0, first.1));
                                    }
                                }

                                // Insert a stop at the end if the last stop is not at 1.0.
                                if let Some(last) = stops.last() {
                                    if last.0 != 1.0 {
                                        stops.push((1.0, last.1));
                                    }
                                }

                                let (offsets, colors): (Vec<f32>, Vec<skia_safe::Color>) =
                                    stops.into_iter().unzip();
                                let colors4f: Vec<skia_safe::Color4f> =
                                    colors.iter().copied().map(Into::into).collect();

                                let gradient_colors =
                                    skia_safe::gradient_shader::GradientColors::new(
                                        &colors4f,
                                        Some(&offsets[..]),
                                        TileMode::Clamp,
                                        None,
                                    );
                                let gradient = skia_safe::gradient_shader::Gradient::new(
                                    gradient_colors,
                                    skia_safe::gradient_shader::Interpolation::default(),
                                );
                                let shader = skia_safe::shaders::linear_gradient(
                                    (Point::from(start), Point::from(end)),
                                    &gradient,
                                    None,
                                );

                                let mut paint = Paint::default();
                                paint.set_shader(shader);
                                paint.set_anti_alias(true);

                                canvas.draw_path(&path, &paint);
                            }

                            Gradient::Radial(radial_gradient) => {
                                let num_stops = radial_gradient.stops.len();

                                let mut stops = radial_gradient
                                    .stops
                                    .iter()
                                    .enumerate()
                                    .map(|(index, stop)| {
                                        let pos = if let Some(pos) = &stop.position {
                                            pos.to_pixels(bounds.width(), self.scale_factor())
                                                / bounds.width()
                                        } else {
                                            index as f32 / (num_stops - 1) as f32
                                        };

                                        (pos, skia_safe::Color::from(stop.color))
                                    })
                                    .collect::<Vec<_>>();

                                // Insert a stop at the front if the first stop is not at 0.
                                if let Some(first) = stops.first() {
                                    if first.0 != 0.0 {
                                        stops.insert(0, (0.0, first.1));
                                    }
                                }

                                // Insert a stop at the end if the last stop is not at 1.0.
                                if let Some(last) = stops.last() {
                                    if last.0 != 1.0 {
                                        stops.push((1.0, last.1));
                                    }
                                }

                                let (offsets, colors): (Vec<f32>, Vec<skia_safe::Color>) =
                                    stops.into_iter().unzip();
                                let colors4f: Vec<skia_safe::Color4f> =
                                    colors.iter().copied().map(Into::into).collect();

                                let gradient_colors =
                                    skia_safe::gradient_shader::GradientColors::new(
                                        &colors4f,
                                        Some(&offsets[..]),
                                        TileMode::Clamp,
                                        None,
                                    );
                                let gradient = skia_safe::gradient_shader::Gradient::new(
                                    gradient_colors,
                                    skia_safe::gradient_shader::Interpolation::default(),
                                );
                                let shader = skia_safe::shaders::radial_gradient(
                                    (Point::from(bounds.center()), bounds.w.max(bounds.h)),
                                    &gradient,
                                    None,
                                );

                                let mut paint = Paint::default();
                                paint.set_shader(shader);
                                paint.set_anti_alias(true);

                                canvas.draw_path(&path, &paint);
                            }

                            _ => {}
                        },

                        ImageOrGradient::Image(image_name) => {
                            if let Some(image_id) = self.resource_manager.image_ids.get(image_name)
                            {
                                if let Some(image) = self.resource_manager.images.get(image_id) {
                                    match &image.image {
                                        ImageOrSvg::Image(image) => {
                                            let image_width = image.width();
                                            let image_height = image.height();
                                            let (width, height) = if let Some(background_size) =
                                                image_sizes.get(index)
                                            {
                                                match background_size {
                                                    BackgroundSize::Explicit { width, height } => {
                                                        let w = match width {
                                                LengthPercentageOrAuto::LengthPercentage(
                                                    length,
                                                ) => {
                                                    length.to_pixels(bounds.w, self.scale_factor())
                                                }
                                                LengthPercentageOrAuto::Auto => image_width as f32,
                                            };

                                                        let h = match height {
                                                LengthPercentageOrAuto::LengthPercentage(
                                                    length,
                                                ) => {
                                                    length.to_pixels(bounds.h, self.scale_factor())
                                                }
                                                LengthPercentageOrAuto::Auto => image_height as f32,
                                            };

                                                        (w, h)
                                                    }

                                                    BackgroundSize::Contain => {
                                                        let image_ratio = image_width as f32
                                                            / image_height as f32;
                                                        let container_ratio = bounds.w / bounds.h;

                                                        let (w, h) =
                                                            if image_ratio > container_ratio {
                                                                (bounds.w, bounds.w / image_ratio)
                                                            } else {
                                                                (bounds.h * image_ratio, bounds.h)
                                                            };

                                                        (w, h)
                                                    }

                                                    BackgroundSize::Cover => {
                                                        let image_ratio = image_width as f32
                                                            / image_height as f32;
                                                        let container_ratio = bounds.w / bounds.h;

                                                        let (w, h) =
                                                            if image_ratio < container_ratio {
                                                                (bounds.w, bounds.w / image_ratio)
                                                            } else {
                                                                (bounds.h * image_ratio, bounds.h)
                                                            };

                                                        (w, h)
                                                    }
                                                }
                                            } else {
                                                (image_width as f32, image_height as f32)
                                            };

                                            let position = image_positions
                                                .get(index)
                                                .cloned()
                                                .or_else(|| image_positions.last().cloned())
                                                .unwrap_or_default();

                                            let posx =
                                                position.x.to_length_or_percentage().to_pixels(
                                                    bounds.width() - width,
                                                    self.scale_factor(),
                                                );
                                            let posy =
                                                position.y.to_length_or_percentage().to_pixels(
                                                    bounds.height() - height,
                                                    self.scale_factor(),
                                                );
                                            let repeat = image_repeats
                                                .get(index)
                                                .copied()
                                                .or_else(|| image_repeats.last().copied())
                                                .unwrap_or(BackgroundRepeat::Repeat);

                                            if width <= 0.0 || height <= 0.0 {
                                                continue;
                                            }

                                            let mut paint = Paint::default();
                                            paint.set_anti_alias(true);

                                            let origin_x = bounds.left() + posx;
                                            let origin_y = bounds.top() + posy;

                                            let mut start_x = origin_x;
                                            let mut start_y = origin_y;

                                            if matches!(
                                                repeat,
                                                BackgroundRepeat::Repeat
                                                    | BackgroundRepeat::RepeatX
                                            ) {
                                                let tiles_to_left =
                                                    ((bounds.left() - origin_x) / width).floor();
                                                start_x = origin_x + tiles_to_left * width;
                                                if start_x > bounds.left() {
                                                    start_x -= width;
                                                }
                                            }

                                            if matches!(
                                                repeat,
                                                BackgroundRepeat::Repeat
                                                    | BackgroundRepeat::RepeatY
                                            ) {
                                                let tiles_to_top =
                                                    ((bounds.top() - origin_y) / height).floor();
                                                start_y = origin_y + tiles_to_top * height;
                                                if start_y > bounds.top() {
                                                    start_y -= height;
                                                }
                                            }

                                            canvas.save();
                                            canvas.clip_path(&path, ClipOp::Intersect, true);

                                            match repeat {
                                                BackgroundRepeat::NoRepeat => {
                                                    let dst = Rect::new(
                                                        origin_x,
                                                        origin_y,
                                                        origin_x + width,
                                                        origin_y + height,
                                                    );
                                                    canvas.draw_image_rect_with_sampling_options(
                                                        image,
                                                        None,
                                                        dst,
                                                        SamplingOptions::default(),
                                                        &paint,
                                                    );
                                                }

                                                BackgroundRepeat::RepeatX => {
                                                    let mut x = start_x;
                                                    while x < bounds.right() {
                                                        let dst = Rect::new(
                                                            x,
                                                            origin_y,
                                                            x + width,
                                                            origin_y + height,
                                                        );
                                                        canvas
                                                            .draw_image_rect_with_sampling_options(
                                                                image,
                                                                None,
                                                                dst,
                                                                SamplingOptions::default(),
                                                                &paint,
                                                            );
                                                        x += width;
                                                    }
                                                }

                                                BackgroundRepeat::RepeatY => {
                                                    let mut y = start_y;
                                                    while y < bounds.bottom() {
                                                        let dst = Rect::new(
                                                            origin_x,
                                                            y,
                                                            origin_x + width,
                                                            y + height,
                                                        );
                                                        canvas
                                                            .draw_image_rect_with_sampling_options(
                                                                image,
                                                                None,
                                                                dst,
                                                                SamplingOptions::default(),
                                                                &paint,
                                                            );
                                                        y += height;
                                                    }
                                                }

                                                BackgroundRepeat::Repeat => {
                                                    let mut y = start_y;
                                                    while y < bounds.bottom() {
                                                        let mut x = start_x;
                                                        while x < bounds.right() {
                                                            let dst = Rect::new(
                                                                x,
                                                                y,
                                                                x + width,
                                                                y + height,
                                                            );
                                                            canvas.draw_image_rect_with_sampling_options(
                                                                image,
                                                                None,
                                                                dst,
                                                                SamplingOptions::default(),
                                                                &paint,
                                                            );
                                                            x += width;
                                                        }
                                                        y += height;
                                                    }
                                                }
                                            }

                                            canvas.restore();
                                        }

                                        ImageOrSvg::Svg(svg) => {
                                            canvas.save_layer(&SaveLayerRec::default());
                                            canvas.translate((bounds.x, bounds.y));
                                            let (scale_x, scale_y) = (
                                                bounds.width() / svg.inner().fContainerSize.fWidth,
                                                bounds.height()
                                                    / svg.inner().fContainerSize.fHeight,
                                            );

                                            if scale_x.is_finite() && scale_y.is_finite() {
                                                canvas.scale((scale_x, scale_y));
                                            } else {
                                                svg.clone().set_container_size((
                                                    bounds.width(),
                                                    bounds.height(),
                                                ));
                                            }

                                            svg.render(canvas);

                                            if let Some(color) = self.style.fill.get_resolved(
                                                self.current,
                                                &self.style.custom_color_props,
                                            ) {
                                                // Escape hatch for multi-color SVGs (logos,
                                                // illustrations) that would otherwise be flooded
                                                // by the root-level `fill: var(--foreground)`
                                                // tint in the default theme. Setting
                                                // `fill: transparent` on such elements makes the
                                                // SrcIn pass a no-op, so the SVG's own path
                                                // fills render untouched. Icon SVGs that want
                                                // the tint still work — they set a
                                                // non-transparent fill explicitly. See #636.
                                                if color.a() != 0 {
                                                    let mut paint = Paint::default();
                                                    paint.set_anti_alias(true);
                                                    paint.set_blend_mode(
                                                        skia_safe::BlendMode::SrcIn,
                                                    );
                                                    paint.set_color(color);
                                                    canvas.draw_paint(&paint);
                                                }
                                            }
                                            canvas.restore();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Draw any text for the current view.
    pub fn draw_text(&mut self, canvas: &Canvas) {
        if let Some(paragraph) = self.text_context.text_paragraphs.get(self.current) {
            let bounds = self.bounds();

            let alignment = self.alignment();

            let (mut top, _) = match alignment {
                Alignment::TopLeft => (0.0, 0.0),
                Alignment::TopCenter => (0.0, 0.5),
                Alignment::TopRight => (0.0, 1.0),
                Alignment::Left => (0.5, 0.0),
                Alignment::Center => (0.5, 0.5),
                Alignment::Right => (0.5, 1.0),
                Alignment::BottomLeft => (1.0, 0.0),
                Alignment::BottomCenter => (1.0, 0.5),
                Alignment::BottomRight => (1.0, 1.0),
            };

            let padding_top = match self.padding_top() {
                Units::Pixels(val) => val,
                _ => 0.0,
            };

            let padding_bottom = match self.padding_bottom() {
                Units::Pixels(val) => val,
                _ => 0.0,
            };

            top *= bounds.height() - padding_top - padding_bottom - paragraph.height();

            let mut padding_left = match self.padding_left() {
                Units::Pixels(val) => val,
                _ => 0.0,
            };

            let mut padding_right = match self.padding_right() {
                Units::Pixels(val) => val,
                _ => 0.0,
            };

            if resolved_text_direction(self.style, self.current) == Direction::RightToLeft {
                std::mem::swap(&mut padding_left, &mut padding_right);
            }

            paragraph.paint(
                canvas,
                ((bounds.x + padding_left).round(), (bounds.y + padding_top + top).round()),
            );
        }
    }
}

impl DataContext for DrawContext<'_> {
    fn try_data<T: 'static>(&self) -> Option<&T> {
        // Return data for the static model.
        if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
            return Some(t);
        }

        for entity in self.current.parent_iter(self.tree) {
            // Return model data.
            if let Some(models) = self.models.get(&entity) {
                if let Some(model) = models.get(&TypeId::of::<T>()) {
                    return model.downcast_ref::<T>();
                }
            }

            // Return view data.
            if let Some(view_handler) = self.views.get(&entity) {
                if let Some(data) = view_handler.downcast_ref::<T>() {
                    return Some(data);
                }
            }
        }

        None
    }
}

// Helper function for computing a rounded corner with variable smoothing
fn compute_smooth_corner(
    corner_radius: f32,
    smoothing: f32,
    width: f32,
    height: f32,
) -> (f32, f32, f32, f32, f32, f32, f32) {
    let max_p = f32::min(width, height) / 2.0;
    let corner_radius = f32::min(corner_radius, max_p);

    let p = f32::min((1.0 + smoothing) * corner_radius, max_p);

    let angle_alpha: f32;
    let angle_beta: f32;

    if corner_radius <= max_p / 2.0 {
        angle_alpha = 45.0 * smoothing;
        angle_beta = 90.0 * (1.0 - smoothing);
    } else {
        let diff_ratio = (corner_radius - max_p / 2.0) / (max_p / 2.0);

        angle_alpha = 45.0 * smoothing * (1.0 - diff_ratio);
        angle_beta = 90.0 * (1.0 - smoothing * (1.0 - diff_ratio));
    }

    let angle_theta = (90.0 - angle_beta) / 2.0;
    let dist_p3_p4 = corner_radius * (angle_theta / 2.0).to_radians().tan();

    let l = (angle_beta / 2.0).to_radians().sin() * corner_radius * SQRT_2;
    let c = dist_p3_p4 * angle_alpha.to_radians().cos();
    let d = c * angle_alpha.to_radians().tan();
    let b = (p - l - c - d) / 3.0;
    let a = 2.0 * b;

    (a, b, c, d, l, p, corner_radius)
}
