use femtovg::{ImageId, TextContext};
use morphorm::Units;

use crate::cache::CachedData;
use crate::input::{Modifiers, MouseState};
use crate::prelude::*;
use crate::resource::{FontOrId, ImageOrId, ResourceManager};
use crate::storage::sparse_set::SparseSet;
use crate::style::LinearGradient;
use crate::text::Selection;

/// Cached data used for drawing.
pub struct DrawCache {
    shadow_image: SparseSet<(ImageId, ImageId)>,
}

impl DrawCache {
    pub fn shadow_image(&self, entity: Entity) -> Option<&(ImageId, ImageId)> {
        self.shadow_image.get(entity)
    }
}

/// A restricted context used when drawing.
pub struct DrawContext<'a>(&'a mut Context);

macro_rules! style_getter_units {
    ($name:ident) => {
        pub fn $name(&self) -> Option<Units> {
            let result = self.0.style.$name.get(self.0.current);
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
        pub fn $name(&self, entity: Entity) -> Option<&$ty> {
            self.0.style.$name.get(entity)
        }
    };
}

impl<'a> DrawContext<'a> {
    /// Creates a new `DrawContext` from the given `Context`.
    pub fn new(cx: &'a mut Context) -> Self {
        Self(cx)
    }

    /// Returns the current entity of the context.
    pub fn current(&self) -> Entity {
        self.0.current
    }

    /// Returns an immutable reference to the data cache.
    pub fn cache(&self) -> &CachedData {
        &self.0.cache
    }

    pub fn cache_mut(&mut self) -> &mut CachedData {
        &mut self.0.cache
    }

    /// Returns an immutable reference to the entity tree.
    pub fn tree(&self) -> &Tree {
        &self.0.tree
    }

    /// Returns an immutable reference to the resource manager.
    pub fn resource_manager(&self) -> &ResourceManager {
        &self.0.resource_manager
    }

    /// Returns an immutable reference to the text context.
    pub fn text_context(&self) -> &TextContext {
        &self.0.text_context
    }

    /// Returns an immutable reference to the mouse state.
    pub fn mouse(&self) -> &MouseState {
        &self.0.mouse
    }

    /// Returns an immutable reference to the modifiers state.
    pub fn modifiers(&self) -> &Modifiers {
        &self.0.modifiers
    }

    pub fn get_image(&mut self, path: &str) -> &mut ImageOrId {
        self.0.get_image(path)
    }

    // pub fn get_font(&mut self, name: &str) -> &FontOrId {
    //     self.0.get
    // }

    /// Returns the name of the default font.
    pub fn default_font(&self) -> &str {
        &self.0.style.default_font
    }

    /// Returns the font-size of the current entity in physical coordinates.
    pub fn font_size(&self, entity: Entity) -> f32 {
        self.logical_to_physical(self.0.style.font_size.get(entity).copied().unwrap_or(16.0))
    }

    /// Returns true if the current entity matches the given pseudoclass.
    pub fn has_pseudo_class(&self, entity: Entity, cls: PseudoClass) -> bool {
        self.0.has_pseudo_class(entity, cls)
    }

    /// Function to convert logical points to physical pixels.
    pub fn logical_to_physical(&self, logical: f32) -> f32 {
        logical * self.0.style.dpi_factor as f32
    }

    /// Function to convert physical pixels to logical points.
    pub fn physical_to_logical(&self, physical: f32) -> f32 {
        physical * self.0.style.dpi_factor as f32
    }

    style_getter_units!(border_width);
    style_getter_units!(border_top_right_radius);
    style_getter_units!(border_top_left_radius);
    style_getter_units!(border_bottom_right_radius);
    style_getter_units!(border_bottom_left_radius);
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
    style_getter_untranslated!(Color, outer_shadow_color);
    style_getter_untranslated!(Color, inner_shadow_color);
    style_getter_untranslated!(Color, selection_color);
    style_getter_untranslated!(Color, caret_color);
    style_getter_untranslated!(LinearGradient, background_gradient);
    style_getter_untranslated!(BorderCornerShape, border_top_right_shape);
    style_getter_untranslated!(BorderCornerShape, border_top_left_shape);
    style_getter_untranslated!(BorderCornerShape, border_bottom_right_shape);
    style_getter_untranslated!(BorderCornerShape, border_bottom_left_shape);
    style_getter_untranslated!(String, background_image);
    style_getter_untranslated!(String, text);
    style_getter_untranslated!(String, image);
    style_getter_untranslated!(String, font);
    style_getter_untranslated!(bool, text_wrap);
    style_getter_untranslated!(Selection, text_selection);
}

impl<'a> DataContext for DrawContext<'a> {
    fn data<T: 'static>(&self) -> Option<&T> {
        self.0.data()
    }
}
