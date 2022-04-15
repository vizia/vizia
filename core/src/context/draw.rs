use crate::style::LinearGradient;
use crate::{
    BorderCornerShape, CachedData, Color, Context, Entity, ImageOrId, ResourceManager, Selection,
    Tree,
};
use femtovg::TextContext;
use morphorm::Units;

pub struct DrawContext<'a>(&'a mut Context);

macro_rules! style_getter_units {
    ($name:ident) => {
        pub fn $name(&self, entity: Entity) -> Option<Units> {
            let result = self.0.style.$name.get(entity);
            if let Some(Units::Pixels(p)) = result {
                Some(Units::Pixels(*p * self.0.style.dpi_factor as f32))
            } else {
                result.copied()
            }
        }
    };
}

macro_rules! style_getter_f32 {
    ($name:ident) => {
        pub fn $name(&self, entity: Entity) -> Option<f32> {
            self.0.style.$name.get(entity).map(|f| *f * self.0.style.dpi_factor as f32)
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

    pub fn current(&self) -> Entity {
        self.0.current
    }

    pub fn cache(&mut self) -> &mut CachedData {
        &mut self.0.cache
    }

    pub fn tree(&self) -> &Tree {
        &self.0.tree
    }

    pub fn resource_manager(&self) -> &ResourceManager {
        &self.0.resource_manager
    }

    pub fn text_context(&self) -> &TextContext {
        &self.0.text_context
    }

    pub fn get_image(&mut self, path: &str) -> &mut ImageOrId {
        self.0.get_image(path)
    }

    pub fn default_font(&self) -> &str {
        &self.0.style.default_font
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
    style_getter_f32!(font_size);
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
