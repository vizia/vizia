use parley::editing::{PlainEditor, PlainEditorDriver};
use parley::{FontContext as ParleyFontContext, LayoutContext as ParleyLayoutContext};
use skia_safe::textlayout::TypefaceFontProvider;
use skia_safe::{FontMgr, textlayout::FontCollection};
use vizia_storage::SparseSet;

use crate::text::shaped_text::ShapedText;
use crate::{entity::Entity, layout::BoundingBox};

pub struct TextContext {
    pub parley_font_context: ParleyFontContext,
    pub parley_layout_context: ParleyLayoutContext,
    pub font_collection: FontCollection,
    pub default_font_manager: FontMgr,
    pub asset_provider: TypefaceFontProvider,
    pub text_bounds: SparseSet<BoundingBox>,
    pub text_shaped: SparseSet<ShapedText>,
    pub plain_editors: SparseSet<PlainEditor<[u8; 4]>>,
}

impl TextContext {
    #[allow(dead_code)]
    pub(crate) fn font_collection(&self) -> &FontCollection {
        &self.font_collection
    }

    pub(crate) fn set_text_bounds(&mut self, entity: Entity, bounds: BoundingBox) {
        self.text_bounds.insert(entity, bounds);
    }

    /// Borrow a [`PlainEditorDriver`] for `entity`'s editor, disjointly borrowing the
    /// editor itself alongside the shared parley font/layout contexts.
    pub(crate) fn editor_driver(&mut self, entity: Entity) -> Option<PlainEditorDriver<'_, [u8; 4]>> {
        let editor = self.plain_editors.get_mut(entity)?;
        Some(editor.driver(&mut self.parley_font_context, &mut self.parley_layout_context))
    }
}

