use skia_safe::textlayout::Paragraph;
use skia_safe::{textlayout::FontCollection, FontMgr};
use vizia_storage::SparseSet;

use crate::{entity::Entity, layout::BoundingBox};

pub struct TextContext {
    pub font_collection: FontCollection,
    pub default_font_manager: FontMgr,
    pub text_bounds: SparseSet<BoundingBox>,
    pub text_paragraphs: SparseSet<Paragraph>,
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
