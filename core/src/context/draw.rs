use std::any::Any;
use std::ops::Range;

use femtovg::{ImageId, TextContext};
use fnv::FnvHashMap;
use morphorm::Units;

use crate::cache::{BoundingBox, CachedData};
use crate::events::ViewHandler;
use crate::input::{Modifiers, MouseState};
use crate::prelude::*;
use crate::resource::ResourceManager;
use crate::state::ModelDataStore;
use crate::storage::sparse_set::SparseSet;
use crate::style::{LinearGradient, Style};
use crate::text::Selection;

/// Cached data used for drawing.
pub struct DrawCache {
    pub shadow_image: SparseSet<(ImageId, ImageId)>,
    pub text_lines: SparseSet<Vec<(Range<usize>, femtovg::TextMetrics)>>,
}

impl DrawCache {
    pub fn new() -> Self {
        Self { shadow_image: SparseSet::new(), text_lines: SparseSet::new() }
    }

    pub fn remove(&mut self, entity: Entity) {
        self.shadow_image.remove(entity);
        self.text_lines.remove(entity);
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
    pub tree: &'a Tree,
    pub(crate) data: &'a SparseSet<ModelDataStore>,
    pub views: &'a FnvHashMap<Entity, Box<dyn ViewHandler>>,
    pub resource_manager: &'a ResourceManager,
    pub text_context: &'a TextContext,
    pub modifiers: &'a Modifiers,
    pub mouse: &'a MouseState,
}

macro_rules! style_getter_units {
    ($name:ident) => {
        pub fn $name(&self) -> Option<Units> {
            let result = self.style.$name.get(self.current);
            if let Some(Units::Pixels(p)) = result {
                Some(Units::Pixels(self.logical_to_physical(*p)))
            } else {
                result.copied()
            }
        }
    };
}

macro_rules! style_getter_untranslated {
    ($ty:ty, $name:ident) => {
        pub fn $name(&self) -> Option<&$ty> {
            self.style.$name.get(self.current)
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
            views: &cx.views,
            resource_manager: &cx.resource_manager,
            text_context: &cx.text_context,
            modifiers: &cx.modifiers,
            mouse: &cx.mouse,
        }
    }

    pub fn bounds(&self) -> BoundingBox {
        self.cache.get_bounds(self.current)
    }

    pub fn clip_region(&self) -> BoundingBox {
        self.cache.get_clip_region(self.current)
    }

    /// Returns the name of the default font.
    pub fn default_font(&self) -> &str {
        &self.style.default_font
    }

    /// Returns the font-size of the current entity in physical coordinates.
    pub fn font_size(&self, entity: Entity) -> f32 {
        self.logical_to_physical(self.style.font_size.get(entity).copied().unwrap_or(16.0))
    }

    /// Function to convert logical points to physical pixels.
    pub fn logical_to_physical(&self, logical: f32) -> f32 {
        logical * self.style.dpi_factor as f32
    }

    /// Function to convert physical pixels to logical points.
    pub fn physical_to_logical(&self, physical: f32) -> f32 {
        physical * self.style.dpi_factor as f32
    }

    style_getter_units!(border_width);
    style_getter_units!(border_radius_top_right);
    style_getter_units!(border_radius_top_left);
    style_getter_units!(border_radius_bottom_right);
    style_getter_units!(border_radius_bottom_left);
    style_getter_units!(outline_width);
    style_getter_units!(outline_offset);
    style_getter_units!(outer_shadow_h_offset);
    style_getter_units!(outer_shadow_v_offset);
    style_getter_units!(outer_shadow_blur);
    style_getter_units!(inner_shadow_h_offset);
    style_getter_units!(inner_shadow_v_offset);
    style_getter_units!(inner_shadow_blur);
    style_getter_units!(child_left);
    style_getter_units!(child_right);
    style_getter_units!(child_top);
    style_getter_units!(child_bottom);
    style_getter_untranslated!(Color, background_color);
    style_getter_untranslated!(Color, font_color);
    style_getter_untranslated!(Color, border_color);
    style_getter_untranslated!(Color, outline_color);
    style_getter_untranslated!(Color, outer_shadow_color);
    style_getter_untranslated!(Color, inner_shadow_color);
    style_getter_untranslated!(Color, selection_color);
    style_getter_untranslated!(Color, caret_color);
    style_getter_untranslated!(LinearGradient, background_gradient);
    style_getter_untranslated!(BorderCornerShape, border_shape_top_right);
    style_getter_untranslated!(BorderCornerShape, border_shape_top_left);
    style_getter_untranslated!(BorderCornerShape, border_shape_bottom_right);
    style_getter_untranslated!(BorderCornerShape, border_shape_bottom_left);
    style_getter_untranslated!(String, background_image);
    style_getter_untranslated!(String, text);
    style_getter_untranslated!(String, image);
    style_getter_untranslated!(String, font);
    style_getter_untranslated!(bool, text_wrap);
    style_getter_untranslated!(Selection, text_selection);

    pub fn opacity(&self) -> f32 {
        self.cache.get_opacity(self.current)
    }
}

impl<'a> DataContext for DrawContext<'a> {
    fn data<T: 'static>(&self) -> Option<&T> {
        // return data for the static model
        if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
            return Some(t);
        }

        for entity in self.current.parent_iter(&self.tree) {
            if let Some(data_list) = self.data.get(entity) {
                for (_, model) in data_list.data.iter() {
                    if let Some(data) = model.downcast_ref::<T>() {
                        return Some(data);
                    }
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
