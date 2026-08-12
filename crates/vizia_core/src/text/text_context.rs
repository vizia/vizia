use std::collections::HashMap;

use parley::editing::{PlainEditor, PlainEditorDriver};
use parley::{FontContext as ParleyFontContext, LayoutContext as ParleyLayoutContext};
use skia_safe::textlayout::TypefaceFontProvider;
use skia_safe::{FontMgr, Typeface, textlayout::FontCollection};
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
    /// Cache of Skia [`Typeface`]s built directly from the raw font data that Parley's
    /// fontique-based shaper selected for a given run (keyed by the font blob's unique id
    /// and its collection index). This ensures glyph ids produced by shaping are always
    /// drawn using the exact same physical font (important for script fallback, e.g.
    /// Arabic/CJK, where Skia's own family-name based typeface lookup can otherwise resolve
    /// to a different font than the one Parley actually shaped with, producing garbled text).
    pub(crate) typeface_cache: HashMap<(u64, u32), Typeface>,
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
    pub(crate) fn editor_driver(
        &mut self,
        entity: Entity,
    ) -> Option<PlainEditorDriver<'_, [u8; 4]>> {
        let editor = self.plain_editors.get_mut(entity)?;
        Some(editor.driver(&mut self.parley_font_context, &mut self.parley_layout_context))
    }
}
