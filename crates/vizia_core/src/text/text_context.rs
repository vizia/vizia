use skia_safe::textlayout::TypefaceFontProvider;
use skia_safe::{FontMgr, textlayout::FontCollection};
use vizia_storage::SparseSet;

use crate::{entity::Entity, layout::BoundingBox};
use crate::text::shaped_text::ShapedText;

pub struct TextContext {
    pub font_collection: FontCollection,
    pub default_font_manager: FontMgr,
    pub asset_provider: TypefaceFontProvider,
    pub text_bounds: SparseSet<BoundingBox>,
    pub text_shaped: SparseSet<ShapedText>,
}

impl TextContext {
    #[allow(dead_code)]
    pub(crate) fn font_collection(&self) -> &FontCollection {
        &self.font_collection
    }

    pub(crate) fn set_text_bounds(&mut self, entity: Entity, bounds: BoundingBox) {
        self.text_bounds.insert(entity, bounds);
    }
}
