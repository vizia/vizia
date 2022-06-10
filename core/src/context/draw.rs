use femtovg::TextContext;
use morphorm::Units;

use crate::cache::{BoundingBox, CachedData};
use crate::input::{Modifiers, MouseState};
use crate::prelude::*;
use crate::resource::{ImageOrId, ResourceManager};
use crate::style::LinearGradient;
use crate::text::Selection;

/// A context used when drawing.
pub struct DrawContext<'a>(&'a mut Context);

macro_rules! style_getter_units {
    ($name:ident) => {
        pub fn $name(&self, entity: Entity) -> Option<Units> {
            let result = self.0.style.$name.get(entity);
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
    pub fn new(cx: &'a mut Context) -> Self {
        Self(cx)
    }

    /// Returns the current entity
    pub fn current(&self) -> Entity {
        self.0.current
    }

    /// Returns a reference to the cache
    pub fn cache(&mut self) -> &mut CachedData {
        &mut self.0.cache
    }

    /// Returns a reference to the tree
    pub fn tree(&self) -> &Tree {
        &self.0.tree
    }

    /// Returns a reference to the resource manager
    pub(crate) fn resource_manager(&self) -> &ResourceManager {
        &self.0.resource_manager
    }

    /// Returns a reference to the text context
    pub fn text_context(&self) -> &TextContext {
        &self.0.text_context
    }

    /// Returns a reference to the mouse state
    pub fn mouse(&self) -> &MouseState {
        &self.0.mouse
    }

    /// Returns a reference to the modifiers state
    pub fn modifiers(&self) -> &Modifiers {
        &self.0.modifiers
    }

    pub fn get_image(&mut self, path: &str) -> &mut ImageOrId {
        self.0.get_image(path)
    }

    pub fn default_font(&self) -> &str {
        &self.0.style.default_font
    }

    pub fn font_size(&self, entity: Entity) -> f32 {
        self.logical_to_physical(self.0.style.font_size.get(entity).copied().unwrap_or(16.0))
    }

    pub fn has_pseudo_class(&self, entity: Entity, cls: PseudoClass) -> bool {
        self.0.has_pseudo_class(entity, cls)
    }

    pub fn logical_to_physical(&self, logical: f32) -> f32 {
        logical * self.0.style.dpi_factor as f32
    }

    /// Returns the current application scale factor
    pub fn scale_factor(&self) -> f32 {
        self.0.style_ref().dpi_factor as f32
    }

    /// Returns the bounds of the current entity
    pub fn bounds(&self) -> BoundingBox {
        self.0.cache_ref().get_bounds(self.current())
    }

    style_getter_units!(border_width);
    style_getter_units!(border_radius_top_right);
    style_getter_units!(border_radius_top_left);
    style_getter_units!(border_radius_bottom_right);
    style_getter_units!(border_radius_bottom_left);
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
}

impl<'a> DataContext for DrawContext<'a> {
    fn data<T: 'static>(&self) -> Option<&T> {
        self.0.data()
    }
}
