use std::cell::{Cell, RefCell};
use std::ops::Range;
use std::rc::Rc;

use crate::prelude::*;

use crate::text::{
    apply_editor_style, enforce_text_bounds, ensure_visible, pre_shaped_from_editor_layout,
    resolve_parley_alignment, resolved_text_direction, shaped_text::ShapedText,
};
use accesskit::{ActionData, ActionRequest, TextDirection, TextPosition, TextSelection};
use parley::Affinity as ParleyAffinity;
use parley::BreakReason;
use parley::editing::Cursor as ParleyCursor;
use parley::editing::Generation;
use parley::editing::PlainEditor;
use parley::editing::Selection as ParleySelection;
use skia_safe::{ClipOp, Paint, PaintStyle, Rect};
use unicode_segmentation::UnicodeSegmentation;

/// Describes a cursor/selection movement, driving [`parley::editing::PlainEditorDriver`] directly.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextMovement {
    /// Move/extend one grapheme to the left (visually).
    Left,
    /// Move/extend one grapheme to the right (visually).
    Right,
    /// Move/extend to the start of the previous word.
    WordLeft,
    /// Move/extend to the start of the next word.
    WordRight,
    /// Move/extend up one visual line.
    Up,
    /// Move/extend down one visual line.
    Down,
    /// Move/extend to the start of the current visual line.
    LineStart,
    /// Move/extend to the end of the current visual line.
    LineEnd,
}

/// Describes the direction and granularity of a deletion.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeleteMovement {
    /// Delete one grapheme before the cursor.
    BackwardGrapheme,
    /// Delete one word before the cursor.
    BackwardWord,
    /// Delete from the cursor to the start of the current visual line.
    BackwardToLineStart,
    /// Delete one grapheme after the cursor.
    ForwardGrapheme,
    /// Delete one word after the cursor.
    ForwardWord,
}

/// Events for modifying a textbox.
pub enum TextEvent {
    /// Insert a string of text into the textbox.
    InsertText(String),
    /// Update the preedit text of the textbox (for IME input).
    UpdatePreedit(String, Option<(usize, usize)>),
    /// Clear the preedit text of the textbox.
    ClearPreedit,
    /// Reset the text of the textbox to the bound data.
    Clear,
    /// Delete a section of text, determined by the [`DeleteMovement`].
    DeleteText(DeleteMovement),
    /// Move the cursor and selection.
    MoveCursor(TextMovement, bool),
    /// Select all text.
    SelectAll,
    /// Select the word at the current cursor position.
    SelectWord,
    /// Select the paragraph at the current cursor position.
    SelectParagraph,
    /// Toggle the textbox to allow text input.
    StartEdit,
    /// Toggle the textbox to *not* allow text input.
    EndEdit,
    /// Trigger the `on_submit` callback with the current text.
    Submit(bool),
    /// Specify the 'hit' position of the mouse cursor.
    Hit(f32, f32, bool),
    /// Specify the 'drag' position of the mouse cursor.
    Drag(f32, f32),
    /// Specify the scroll offset of the textbox.
    Scroll(f32, f32),
    /// Copy the textbox buffer to the clipboard.
    Copy,
    /// Paste the clipboard buffer into the textbox.
    Paste,
    /// Cut the textbox text and place it in the clipboard.
    Cut,
    /// Set the placeholder text of the textbox.
    SetPlaceholder(String),
    /// Trigger the `on_blur` callback.
    Blur,
    /// Set whether masked text should be visible.
    SetMaskVisible(bool),
    /// Toggle whether masked text should be visible.
    ToggleMaskVisible,
    /// Toggle the visibility of the text Caret.
    ToggleCaret,
}

/// The `Textbox` view provides an input control for editing a value as a string.
///
/// The textbox takes a lens to some value, which must be a type which can convert to and from a `String`,
/// as determined by the `ToString` and `FromStr` traits. The value type is used for validation and returned by
/// the `on_submit` callback, which is triggered when the textbox is submitted with the enter key or when the textbox
/// loses keyboard focus.
pub struct Textbox<R, T> {
    value: R,
    kind: TextboxKind,
    edit: bool,
    transform: Rc<RefCell<(f32, f32)>>,
    on_edit: Option<Box<dyn Fn(&mut EventContext, String) + Send + Sync>>,
    on_submit: Option<Box<dyn Fn(&mut EventContext, T, bool) + Send + Sync>>,
    on_blur: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    on_cancel: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    validate: Option<Box<dyn Fn(&T) -> bool>>,
    placeholder: Signal<String>,
    show_placeholder: Signal<bool>,
    show_caret: Signal<bool>,
    mask_char: Signal<Option<char>>,
    max_length: Signal<Option<usize>>,
    can_copy: Signal<bool>,
    can_paste: Signal<bool>,
    mask_visible: bool,
    real_text: String,
    caret_timer: Timer,
    text_overflow: Option<TextOverflow>,
    edited_since_focus: bool,
    edited_once: bool,
    /// Generation of the entity's `PlainEditor` layout last reflected in
    /// `cx.text_context.text_shaped`. Used to avoid rebuilding the shaped-glyph
    /// cache when nothing has actually changed.
    last_generation: Cell<Generation>,
    /// Last width (in physical pixels) applied to the `PlainEditor` via `set_width`.
    last_width: Cell<Option<f32>>,
    /// Last scale factor applied to the `PlainEditor` via `set_scale`.
    last_scale: Cell<f32>,
    /// Last alignment applied to the `PlainEditor` via `set_alignment`.
    last_align: Cell<Option<parley::Alignment>>,
}

// Determines whether the enter key submits the text or inserts a new line.
#[derive(Copy, Clone, PartialEq, Eq)]
enum TextboxKind {
    SingleLine,
    MultiLineUnwrapped,
    MultiLineWrapped,
}

impl<R, T> Textbox<R, T>
where
    R: Res<T> + 'static,
    T: Clone + ToStringLocalized + std::str::FromStr + 'static,
{
    /// Creates a new single-line textbox.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// #
    /// # #[derive(Lens)]
    /// # struct AppData {
    /// #     text: String,
    /// # }
    /// #
    /// # impl Model for AppData {}
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// # AppData { text: String::from("Hello World") }.build(cx);
    /// #
    /// Textbox::new(cx, AppData::text);
    /// ```
    pub fn new(cx: &mut Context, value: R) -> Handle<Self>
    where
        R: Clone,
    {
        Self::new_core(cx, value, TextboxKind::SingleLine)
    }

    /// Creates a new multi-line textbox.
    ///
    /// The `wrap` parameter determines whether text which is too long for the textbox
    /// should soft-wrap onto multiple lines. If false, then only hard-wraps from line breaks
    /// will cause the text to span multiple lines.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// #
    /// # #[derive(Lens)]
    /// # struct AppData {
    /// #     text: String,
    /// # }
    /// #
    /// # impl Model for AppData {}
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// # AppData { text: String::from("Hello World") }.build(cx);
    /// #
    /// Textbox::new_multiline(cx, AppData::text, true);
    /// ```
    pub fn new_multiline(cx: &mut Context, value: R, wrap: bool) -> Handle<Self>
    where
        R: Clone,
    {
        Self::new_core(
            cx,
            value,
            if wrap { TextboxKind::MultiLineWrapped } else { TextboxKind::MultiLineUnwrapped },
        )
    }

    fn new_core(cx: &mut Context, value: R, kind: TextboxKind) -> Handle<Self>
    where
        R: Clone,
    {
        let value_text = value.clone().to_signal(cx);
        let caret_timer = cx.environment().caret_timer;
        let initial_text = value.get_value(cx).to_string_local(cx);
        let show_caret = Signal::new(false);
        let mask_char = Signal::new(None);
        let max_length = Signal::new(None);
        let can_copy = Signal::new(true);
        let can_paste = Signal::new(true);
        let placeholder = Signal::new(String::from(""));
        let show_placeholder = Signal::new(initial_text.is_empty());

        Self {
            value: value.clone(),
            kind,
            edit: false,
            transform: Rc::new(RefCell::new((0.0, 0.0))),
            on_edit: None,
            on_submit: None,
            on_blur: None,
            on_cancel: None,
            validate: None,
            placeholder,
            show_placeholder,
            show_caret,
            mask_char,
            max_length,
            can_copy,
            can_paste,
            mask_visible: false,
            real_text: initial_text.clone(),
            caret_timer,
            text_overflow: None,
            edited_since_focus: false,
            edited_once: false,
            last_generation: Cell::new(Generation::default()),
            last_width: Cell::new(None),
            last_scale: Cell::new(1.0),
            last_align: Cell::new(None),
        }
        .build(cx, move |cx| {
            let entity = cx.current();
            let font_size = cx
                .style
                .font_size
                .get_resolved(entity, &cx.style.custom_font_size_props)
                .and_then(|size| size.0.to_px())
                .unwrap_or(16.0);
            let mut editor = PlainEditor::new(font_size);
            apply_editor_style(entity, &cx.style, &mut editor);
            cx.text_context.plain_editors.insert(entity, editor);

            cx.add_listener(move |textbox: &mut Self, cx, event| {
                let flag: bool = textbox.edit;
                event.map(|window_event, meta| match window_event {
                    WindowEvent::MouseDown(_) => {
                        if flag && meta.origin != cx.current() && cx.hovered() != cx.current() {
                            cx.emit(TextEvent::Blur);
                        }
                    }

                    _ => {}
                });
            });
        })
        .toggle_class("multiline", kind == TextboxKind::MultiLineWrapped)
        .text_wrap(kind == TextboxKind::MultiLineWrapped)
        .navigable(true)
        .role(if kind == TextboxKind::SingleLine {
            Role::TextInput
        } else {
            Role::MultilineTextInput
        })
        .text_value(value.clone())
        .toggle_class("caret", show_caret)
        .placeholder_shown(show_placeholder)
        .bind(value_text, move |handle| {
            handle.bind(placeholder, move |handle| {
                let text = value_text.get();
                let txt = text.to_string_local(&handle);
                let entity = handle.entity();
                let mut display_text = String::new();
                let mut handle = handle.modify(|textbox| {
                    textbox.real_text = txt.clone();
                    textbox.show_placeholder.set_if_changed(txt.is_empty());
                    display_text = textbox.display_text_from_real();
                });
                let cx = handle.context();
                push_editor_text_and_rebuild(entity, cx, &display_text);
            });
        })
    }

    fn display_text_from_real(&self) -> String {
        if self.show_placeholder.get() {
            return self.placeholder.get().clone();
        }

        self.mask_str(&self.real_text)
    }

    fn mask_str(&self, s: &str) -> String {
        if self.mask_visible {
            return s.to_string();
        }

        let Some(mask) = self.mask_char.get() else {
            return s.to_string();
        };

        let mut masked = String::with_capacity(s.len());
        for grapheme in s.graphemes(true) {
            if grapheme == "\n" {
                masked.push('\n');
            } else {
                masked.push(mask);
            }
        }

        masked
    }

    /// Rebuilds the entity's shaped-glyph cache (`cx.text_context.text_shaped`) from its
    /// `PlainEditor`, but only if the editor's layout generation has changed since the last
    /// rebuild. Marks relayout/redraw/accessibility dirty when a rebuild actually occurs.
    fn rebuild_shaped_cache(&self, cx: &mut EventContext) {
        let entity = cx.current;
        let width = self.last_width.get().unwrap_or(f32::MAX);

        let Some(mut driver) = cx.text_context.editor_driver(entity) else { return };
        // `PlainEditor::generation()` only reflects a pending style/text/width change once the
        // layout has actually been (re)computed (parley marks `layout_dirty` eagerly but bumps
        // `generation` lazily inside `update_layout`). Force that resolution via `driver.layout()`
        // *before* comparing generations, otherwise a style-only change (no text edit) would be
        // compared against a stale, not-yet-bumped generation and incorrectly skipped.
        let layout = driver.layout().clone();
        let generation = driver.editor.generation();
        if generation == self.last_generation.get() {
            return;
        }
        let text = driver.editor.raw_text().to_string();
        drop(driver);

        let pre_shaped =
            pre_shaped_from_editor_layout(entity, cx.style, &text, &layout, &mut cx.text_context);
        let mut shaped = ShapedText::new(pre_shaped);
        shaped.layout(width);
        cx.text_context.text_shaped.insert(entity, shaped);
        self.last_generation.set(generation);

        cx.style.needs_relayout(entity);
        cx.needs_redraw();
        cx.style.needs_access_update(entity);
    }

    /// Resyncs the `PlainEditor`'s width/scale/alignment with the current layout, then
    /// rebuilds the shaped-glyph cache if anything actually changed.
    fn sync_editor_layout(&self, cx: &mut EventContext) {
        let entity = cx.current;

        // Refresh the editor's font/text styling from the entity's resolved CSS style. This is
        // needed (not just on `StartEdit`) because at construction time the entity's inherited
        // style (font-family/weight/etc.) has not been resolved yet by the restyle system, so the
        // very first `apply_editor_style` call (in the `.build()` closure) can read stale/default
        // values. `GeometryChanged` always fires after a layout pass, which always runs after that
        // frame's restyle pass, so by this point the style is guaranteed to be correctly resolved.
        if let Some(editor) = cx.text_context.plain_editors.get_mut(entity) {
            apply_editor_style(entity, cx.style, editor);
        }

        let bounds = cx.bounds();
        let scale_factor = cx.scale_factor();

        let padding_left = cx
            .style
            .padding_left
            .get_resolved(entity, &cx.style.custom_units_props)
            .unwrap_or_default();
        let padding_right = cx
            .style
            .padding_right
            .get_resolved(entity, &cx.style.custom_units_props)
            .unwrap_or_default();

        let logical_width = cx.physical_to_logical(bounds.w);
        let mut padding_left_px = padding_left.to_px(logical_width, 0.0) * scale_factor;
        let mut padding_right_px = padding_right.to_px(logical_width, 0.0) * scale_factor;
        if resolved_text_direction(cx.style, entity) == crate::style::Direction::RightToLeft {
            std::mem::swap(&mut padding_left_px, &mut padding_right_px);
        }
        let avail_w = (bounds.w - padding_left_px - padding_right_px).max(0.0);
        let align = resolve_parley_alignment(cx.style, entity);

        let width_changed =
            self.last_width.get().map(|w| (w - avail_w).abs() > 0.5).unwrap_or(true);
        let scale_changed = (self.last_scale.get() - scale_factor).abs() > f32::EPSILON;
        let align_changed = self.last_align.get() != Some(align);

        if width_changed || scale_changed || align_changed {
            if let Some(driver) = cx.text_context.editor_driver(entity) {
                if width_changed {
                    driver.editor.set_width(Some(avail_w));
                }
                if scale_changed {
                    driver.editor.set_scale(scale_factor);
                }
                if align_changed {
                    driver.editor.set_alignment(align);
                }
            }
            self.last_width.set(Some(avail_w));
            self.last_scale.set(scale_factor);
            self.last_align.set(Some(align));
        }

        self.rebuild_shaped_cache(cx);
    }

    /// Pushes the current display text into the entity's `PlainEditor` (remapping the
    /// selection across the change in grapheme space) if it differs from the editor's
    /// current text, then rebuilds the shaped-glyph cache.
    fn resync_display_text(&self, cx: &mut EventContext) {
        let entity = cx.current;
        let old_display = cx
            .text_context
            .plain_editors
            .get(entity)
            .map(|e| e.text().to_string())
            .unwrap_or_default();
        let new_display = self.display_text_from_real();
        if old_display == new_display {
            return;
        }

        if let Some(mut driver) = cx.text_context.editor_driver(entity) {
            let sel = driver.editor.raw_selection();
            let anchor_g = Self::byte_to_grapheme_index(&old_display, sel.anchor().index());
            let focus_g = Self::byte_to_grapheme_index(&old_display, sel.focus().index());
            driver.editor.set_text(&new_display);
            let anchor_b = Self::grapheme_index_to_byte(&new_display, anchor_g);
            let focus_b = Self::grapheme_index_to_byte(&new_display, focus_g);
            driver.select_byte_range(anchor_b, focus_b);
        }

        cx.style.needs_access_update(entity);
        self.rebuild_shaped_cache(cx);
    }

    fn grapheme_index_to_byte(text: &str, idx: usize) -> usize {
        if idx == 0 {
            return 0;
        }

        for (i, (byte, _)) in text.grapheme_indices(true).enumerate() {
            if i == idx {
                return byte;
            }
        }

        text.len()
    }

    fn byte_to_grapheme_index(text: &str, byte_offset: usize) -> usize {
        let mut idx = 0;
        for (byte, _) in text.grapheme_indices(true) {
            if byte >= byte_offset {
                break;
            }
            idx += 1;
        }
        idx
    }

    /// Converts a byte range expressed in display-text coordinates into the equivalent
    /// byte range in `real_text` coordinates (grapheme-index remapping).
    fn range_display_to_real(
        display_text: &str,
        real_text: &str,
        range: Range<usize>,
    ) -> Range<usize> {
        let start_g = Self::byte_to_grapheme_index(display_text, range.start);
        let end_g = Self::byte_to_grapheme_index(display_text, range.end);
        let start = Self::grapheme_index_to_byte(real_text, start_g);
        let end = Self::grapheme_index_to_byte(real_text, end_g);
        start..end
    }

    /// Computes the removed byte range (in `old`'s coordinates) between two versions of a
    /// string that differ only by a single deletion (a common-prefix/common-suffix diff).
    fn diff_removed_range(old: &str, new: &str) -> Range<usize> {
        let max_common_prefix = old.len().min(new.len());
        let mut prefix = 0;
        while prefix < max_common_prefix && old.as_bytes()[prefix] == new.as_bytes()[prefix] {
            prefix += 1;
        }
        while prefix > 0 && (!old.is_char_boundary(prefix) || !new.is_char_boundary(prefix)) {
            prefix -= 1;
        }

        let old_rem_len = old.len() - prefix;
        let new_rem_len = new.len() - prefix;
        let max_common_suffix = old_rem_len.min(new_rem_len);
        let mut suffix = 0;
        while suffix < max_common_suffix
            && old.as_bytes()[old.len() - 1 - suffix] == new.as_bytes()[new.len() - 1 - suffix]
        {
            suffix += 1;
        }
        while suffix > 0
            && (!old.is_char_boundary(old.len() - suffix)
                || !new.is_char_boundary(new.len() - suffix))
        {
            suffix -= 1;
        }

        let start = prefix;
        let end = (old.len() - suffix).max(start);
        start..end
    }

    fn insert_text(&mut self, cx: &mut EventContext, txt: &str) {
        let entity = cx.current;

        if self.show_placeholder.get() && !txt.is_empty() {
            self.show_placeholder.set(false);
            if let Some(editor) = cx.text_context.plain_editors.get_mut(entity) {
                editor.set_text("");
            }
            if let Some(mut driver) = cx.text_context.editor_driver(entity) {
                driver.select_byte_range(0, 0);
            }
        }

        let old_display = cx
            .text_context
            .plain_editors
            .get(entity)
            .map(|e| e.text().to_string())
            .unwrap_or_default();
        let sel_range = cx
            .text_context
            .plain_editors
            .get(entity)
            .map(|e| e.raw_selection().text_range())
            .unwrap_or(0..0);

        let real_range = Self::range_display_to_real(&old_display, &self.real_text, sel_range);
        let clamped = self.clamp_insert_text(txt, real_range.clone());

        self.real_text.replace_range(real_range, &clamped);
        self.show_placeholder.set(self.real_text.is_empty());

        let masked = self.mask_str(&clamped);
        if let Some(mut driver) = cx.text_context.editor_driver(entity) {
            driver.insert_or_replace_selection(&masked);
        }

        cx.style.needs_access_update(entity);
        self.rebuild_shaped_cache(cx);
    }

    fn update_preedit(
        &mut self,
        cx: &mut EventContext,
        preedit_txt: &str,
        cursor: Option<(usize, usize)>,
    ) {
        if preedit_txt.is_empty() || cursor.is_none() {
            return;
        }
        let entity = cx.current;

        if self.show_placeholder.get() {
            self.show_placeholder.set(false);
            if let Some(editor) = cx.text_context.plain_editors.get_mut(entity) {
                editor.set_text("");
            }
            if let Some(mut driver) = cx.text_context.editor_driver(entity) {
                driver.select_byte_range(0, 0);
            }
        }

        if let Some(mut driver) = cx.text_context.editor_driver(entity) {
            driver.set_compose(preedit_txt, cursor);
        }

        cx.style.needs_access_update(entity);
        self.rebuild_shaped_cache(cx);
    }

    fn clear_preedit(&mut self, cx: &mut EventContext) {
        let entity = cx.current;
        if let Some(mut driver) = cx.text_context.editor_driver(entity) {
            driver.clear_compose();
        }
        self.show_placeholder.set(self.real_text.is_empty());
        cx.style.needs_access_update(entity);
        self.rebuild_shaped_cache(cx);
    }

    fn delete_text(&mut self, cx: &mut EventContext, movement: DeleteMovement) {
        if self.show_placeholder.get() {
            return;
        }

        let entity = cx.current;
        if cx.text_context.plain_editors.get(entity).map(|e| e.is_composing()).unwrap_or(false) {
            return;
        }

        let old_display = cx
            .text_context
            .plain_editors
            .get(entity)
            .map(|e| e.text().to_string())
            .unwrap_or_default();

        let Some(mut driver) = cx.text_context.editor_driver(entity) else { return };
        match movement {
            DeleteMovement::BackwardGrapheme => driver.backdelete(),
            DeleteMovement::BackwardWord => driver.backdelete_word(),
            DeleteMovement::BackwardToLineStart => {
                if driver.editor.raw_selection().is_collapsed() {
                    driver.select_to_line_start();
                }
                driver.delete_selection();
            }
            DeleteMovement::ForwardGrapheme => driver.delete(),
            DeleteMovement::ForwardWord => driver.delete_word(),
        }
        drop(driver);

        let new_display = cx
            .text_context
            .plain_editors
            .get(entity)
            .map(|e| e.text().to_string())
            .unwrap_or_default();

        if old_display != new_display {
            let removed_display_range = Self::diff_removed_range(&old_display, &new_display);
            let removed_real_range =
                Self::range_display_to_real(&old_display, &self.real_text, removed_display_range);
            if removed_real_range.start < removed_real_range.end {
                self.real_text.replace_range(removed_real_range, "");
            }
        }

        self.show_placeholder.set(self.real_text.is_empty());
        cx.style.needs_access_update(entity);
        self.rebuild_shaped_cache(cx);
    }

    fn reset_text(&mut self, cx: &mut EventContext) {
        let entity = cx.current;
        self.real_text.clear();
        self.show_placeholder.set(true);
        let display = self.display_text_from_real();
        if let Some(editor) = cx.text_context.plain_editors.get_mut(entity) {
            editor.set_text(&display);
        }
        if let Some(mut driver) = cx.text_context.editor_driver(entity) {
            driver.select_byte_range(0, 0);
        }
        cx.style.needs_access_update(entity);
        self.rebuild_shaped_cache(cx);
    }

    /// When IME is enabled, the cursor movement logic will be controlled by [`update_preedit`].
    ///
    /// [`update_preedit`]: Textbox::update_preedit
    fn move_cursor(&mut self, cx: &mut EventContext, movement: TextMovement, selection: bool) {
        let entity = cx.current;
        let Some(mut driver) = cx.text_context.editor_driver(entity) else { return };
        match (movement, selection) {
            (TextMovement::Left, false) => driver.move_left(),
            (TextMovement::Left, true) => driver.select_left(),
            (TextMovement::Right, false) => driver.move_right(),
            (TextMovement::Right, true) => driver.select_right(),
            (TextMovement::WordLeft, false) => driver.move_word_left(),
            (TextMovement::WordLeft, true) => driver.select_word_left(),
            (TextMovement::WordRight, false) => driver.move_word_right(),
            (TextMovement::WordRight, true) => driver.select_word_right(),
            (TextMovement::Up, false) => driver.move_up(),
            (TextMovement::Up, true) => driver.select_up(),
            (TextMovement::Down, false) => driver.move_down(),
            (TextMovement::Down, true) => driver.select_down(),
            (TextMovement::LineStart, false) => driver.move_to_line_start(),
            (TextMovement::LineStart, true) => driver.select_to_line_start(),
            (TextMovement::LineEnd, false) => driver.move_to_line_end(),
            (TextMovement::LineEnd, true) => driver.select_to_line_end(),
        }
        drop(driver);
        cx.needs_redraw();
        cx.style.needs_access_update(entity);
    }

    fn select_all(&mut self, cx: &mut EventContext) {
        if self.show_placeholder.get() {
            return;
        }
        let entity = cx.current;
        if let Some(mut driver) = cx.text_context.editor_driver(entity) {
            driver.select_all();
        }
        cx.needs_redraw();
        cx.style.needs_access_update(entity);
    }

    fn select_word(&mut self, cx: &mut EventContext) {
        if self.show_placeholder.get() {
            return;
        }
        self.move_cursor(cx, TextMovement::WordLeft, false);
        self.move_cursor(cx, TextMovement::WordRight, true);
    }

    fn select_paragraph(&mut self, cx: &mut EventContext) {
        if self.show_placeholder.get() {
            return;
        }
        self.move_cursor(cx, TextMovement::LineStart, false);
        self.move_cursor(cx, TextMovement::LineEnd, true);
    }

    /// These input coordinates should be physical coordinates, i.e. what the mouse events provide.
    /// The output text coordinates will also be physical, but relative to the top of the text
    /// glyphs, appropriate for passage to cosmic.
    fn coordinates_global_to_text(&self, cx: &EventContext, x: f32, y: f32) -> (f32, f32) {
        let bounds = cx.bounds();

        if let Some(shaped) = cx.text_context.text_shaped.get(cx.current) {
            let padding_left = cx
                .style
                .padding_left
                .get_resolved(cx.current, &cx.style.custom_units_props)
                .unwrap_or_default();
            let padding_top = cx
                .style
                .padding_top
                .get_resolved(cx.current, &cx.style.custom_units_props)
                .unwrap_or_default();
            let padding_right = cx
                .style
                .padding_right
                .get_resolved(cx.current, &cx.style.custom_units_props)
                .unwrap_or_default();
            let padding_bottom = cx
                .style
                .padding_bottom
                .get_resolved(cx.current, &cx.style.custom_units_props)
                .unwrap_or_default();

            let logical_parent_width = cx.physical_to_logical(bounds.w);
            let logical_parent_height = cx.physical_to_logical(bounds.h);

            let mut padding_left =
                padding_left.to_px(logical_parent_width, 0.0) * cx.scale_factor();
            let mut padding_right =
                padding_right.to_px(logical_parent_width, 0.0) * cx.scale_factor();
            let padding_top = padding_top.to_px(logical_parent_height, 0.0) * cx.scale_factor();
            let padding_bottom =
                padding_bottom.to_px(logical_parent_height, 0.0) * cx.scale_factor();

            if resolved_text_direction(cx.style, cx.current) == crate::style::Direction::RightToLeft
            {
                std::mem::swap(&mut padding_left, &mut padding_right);
            }

            let (mut top, _) = match cx.style.alignment.get(cx.current).copied().unwrap_or_default()
            {
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

            top *= bounds.height() - padding_top - padding_bottom - shaped.height();

            let x = x - bounds.x - padding_left;
            let y = y - bounds.y - padding_top - top;

            (x, y)
        } else {
            (x, y)
        }
    }

    /// This function takes window-global physical coordinates.
    fn hit(&mut self, cx: &mut EventContext, x: f32, y: f32, selection: bool) {
        let entity = cx.current;
        let x = x - self.transform.borrow().0;
        let y = y - self.transform.borrow().1;
        let (local_x, local_y) = self.coordinates_global_to_text(cx, x, y);

        if let Some(mut driver) = cx.text_context.editor_driver(entity) {
            if selection {
                driver.extend_selection_to_point(local_x, local_y);
            } else {
                driver.move_to_point(local_x, local_y);
            }
        } else {
            return;
        }

        cx.needs_redraw();
        cx.style.needs_access_update(entity);
    }

    /// This function takes window-global physical coordinates.
    fn drag(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        let entity = cx.current;
        let x = x - self.transform.borrow().0;
        let y = y - self.transform.borrow().1;
        let (local_x, local_y) = self.coordinates_global_to_text(cx, x, y);

        if let Some(mut driver) = cx.text_context.editor_driver(entity) {
            driver.extend_selection_to_point(local_x, local_y);
        } else {
            return;
        }

        cx.needs_redraw();
        cx.style.needs_access_update(entity);
    }

    // /// This function takes window-global physical dimensions.
    // fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32) {}

    #[cfg(feature = "clipboard")]
    fn clone_selected(&self, cx: &mut EventContext) -> Option<String> {
        let entity = cx.current;
        let editor = cx.text_context.plain_editors.get(entity)?;
        let display = editor.text().to_string();
        let range = editor.raw_selection().text_range();
        let real_range = Self::range_display_to_real(&display, &self.real_text, range);
        let start = real_range.start.min(self.real_text.len());
        let end = real_range.end.min(self.real_text.len());
        if start >= end {
            return Some(String::new());
        }
        Some(self.real_text[start..end].to_string())
    }

    fn clone_text(&self, _cx: &mut EventContext) -> String {
        if self.show_placeholder.get() {
            return String::new();
        }

        self.real_text.clone()
    }

    fn clamp_insert_text(&self, txt: &str, replace_range: Range<usize>) -> String {
        let Some(max_length) = self.max_length.get() else {
            return txt.to_string();
        };

        let current_len = self.real_text.graphemes(true).count();
        let replaced_len = self.real_text[replace_range].graphemes(true).count();
        let preserved_len = current_len.saturating_sub(replaced_len);
        let remaining = max_length.saturating_sub(preserved_len);

        if remaining == 0 {
            return String::new();
        }

        txt.graphemes(true).take(remaining).collect()
    }

    fn is_text_valid(&self, text: &str) -> bool {
        if let Ok(value) = text.parse::<T>() {
            if let Some(validate) = &self.validate { validate(&value) } else { true }
        } else {
            false
        }
    }

    fn reset_caret_timer(&mut self, cx: &mut EventContext) {
        cx.stop_timer(self.caret_timer);
        if !cx.is_read_only() {
            self.show_caret.set(true);
            cx.start_timer(self.caret_timer);
        }
    }

    fn reset_ime_position(&mut self, cx: &mut EventContext) {
        // TODO: Make the position of IME follow the cursor.
        cx.event_queue.push_back(
            Event::new(WindowEvent::SetImeCursorArea(
                (cx.bounds().x as u32, cx.bounds().y as u32),
                ((cx.bounds().width()) as u32, cx.bounds().height() as u32),
            ))
            .target(cx.current),
        );
    }

    fn draw_selection(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let entity = cx.current;
        let is_collapsed =
            cx.text_context.plain_editors.get(entity).map(|e| e.raw_selection().is_collapsed());
        if is_collapsed != Some(false) {
            return;
        }

        let bounds = cx.bounds();
        let alignment = cx.alignment();

        let (mut top, _left) = match alignment {
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

        let padding_top = match cx.padding_top() {
            Units::Pixels(val) => val,
            _ => 0.0,
        };
        let padding_bottom = match cx.padding_bottom() {
            Units::Pixels(val) => val,
            _ => 0.0,
        };

        let text_height =
            cx.text_context.text_shaped.get(entity).map(|s| s.height()).unwrap_or(0.0);

        top *= bounds.height() - padding_top - padding_bottom - text_height;

        let mut padding_left = match cx.padding_left() {
            Units::Pixels(val) => val,
            _ => 0.0,
        };
        let mut padding_right = match cx.padding_right() {
            Units::Pixels(val) => val,
            _ => 0.0,
        };
        if resolved_text_direction(cx.style, entity) == crate::style::Direction::RightToLeft {
            std::mem::swap(&mut padding_left, &mut padding_right);
        }

        let mut rects = Vec::new();
        if let Some(editor) = cx.text_context.plain_editors.get(entity) {
            editor.selection_geometry_with(|bbox, _line_idx| rects.push(bbox));
        }

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(cx.selection_color());

        for bbox in rects {
            let x = bounds.x + padding_left + bbox.x0 as f32;
            let y = bounds.y + padding_top + top + bbox.y0 as f32;
            let x2 = bounds.x + padding_left + bbox.x1 as f32;
            let y2 = bounds.y + padding_top + top + bbox.y1 as f32;
            canvas.draw_rect(Rect::new(x, y, x2, y2), &paint);
        }
    }

    /// Draw text caret for the current view.
    pub fn draw_text_caret(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let entity = cx.current;
        let bounds = cx.bounds();

        let cursor_rect = cx
            .text_context
            .plain_editors
            .get(entity)
            .and_then(|editor| editor.cursor_geometry(1.0));
        let Some(cursor_rect) = cursor_rect else { return };

        let text_height =
            cx.text_context.text_shaped.get(entity).map(|s| s.height()).unwrap_or(0.0);
        let text_max_w =
            cx.text_context.text_shaped.get(entity).map(|s| s.max_intrinsic_width()).unwrap_or(0.0);

        let alignment = cx.alignment();
        let (mut top, _) = match alignment {
            Alignment::TopLeft => (0.0_f32, 0.0),
            Alignment::TopCenter => (0.0, 0.5),
            Alignment::TopRight => (0.0, 1.0),
            Alignment::Left => (0.5, 0.0),
            Alignment::Center => (0.5, 0.5),
            Alignment::Right => (0.5, 1.0),
            Alignment::BottomLeft => (1.0, 0.0),
            Alignment::BottomCenter => (1.0, 0.5),
            Alignment::BottomRight => (1.0, 1.0),
        };
        let padding_top = match cx.padding_top() {
            Units::Pixels(v) => v,
            _ => 0.0,
        };
        let padding_bottom = match cx.padding_bottom() {
            Units::Pixels(v) => v,
            _ => 0.0,
        };
        top *= bounds.height() - padding_top - padding_bottom - text_height;

        let mut padding_left = match cx.padding_left() {
            Units::Pixels(v) => v,
            _ => 0.0,
        };
        let mut padding_right = match cx.padding_right() {
            Units::Pixels(v) => v,
            _ => 0.0,
        };
        if resolved_text_direction(cx.style, entity) == crate::style::Direction::RightToLeft {
            std::mem::swap(&mut padding_left, &mut padding_right);
        }

        let x = (bounds.x + padding_left + cursor_rect.x0 as f32).round();
        let y = (bounds.y + padding_top + top + cursor_rect.y0 as f32).round();
        let x2 = x + 1.0;
        let y2 = y + (cursor_rect.y1 - cursor_rect.y0) as f32;

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(cx.caret_color());
        canvas.draw_rect(Rect::new(x, y, x2, y2), &paint);

        let mut transform = self.transform.borrow_mut();
        let text_bounds = BoundingBox::from_min_max(
            bounds.x + padding_left,
            bounds.y + padding_top + top,
            bounds.x + padding_left + text_max_w,
            bounds.y + padding_top + top + text_height,
        );
        let mut clip_bounds = bounds;
        clip_bounds =
            clip_bounds.shrink_sides(padding_left, padding_top, padding_right, padding_bottom);
        let (tx, ty) = enforce_text_bounds(&text_bounds, &clip_bounds, (transform.0, transform.1));
        let caret_box = BoundingBox::from_min_max(x, y, x2, y2);
        let (new_tx, new_ty) = ensure_visible(&caret_box, &clip_bounds, (tx, ty));
        if new_tx != transform.0 || new_ty != transform.1 {
            *transform = (new_tx, new_ty);
            cx.needs_redraw();
        }
    }
}

impl<R, T> Handle<'_, Textbox<R, T>>
where
    R: Res<T> + 'static,
    T: Clone + ToStringLocalized + std::str::FromStr + 'static,
{
    /// Sets the callback triggered when a textbox is edited, i.e. text is inserted/deleted.
    ///
    /// Callback provides the current text of the textbox.
    pub fn on_edit<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, String) + Send + Sync,
    {
        self.modify(|textbox| textbox.on_edit = Some(Box::new(callback)))
    }

    /// Sets the callback triggered when a textbox is submitted,
    /// i.e. when the enter key is pressed with a single-line textbox or the textbox loses focus.
    ///
    /// Callback provides the text of the textbox and a flag to indicate if the submit was due to a key press or a loss of focus.
    pub fn on_submit<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, T, bool) + Send + Sync,
    {
        self.modify(|textbox| textbox.on_submit = Some(Box::new(callback)))
    }

    /// Sets the callback triggered when a textbox is blurred, i.e. the mouse is pressed outside of the textbox.
    pub fn on_blur<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.modify(|textbox| textbox.on_blur = Some(Box::new(callback)))
    }

    /// Sets the callback triggered when a textbox edit is cancelled, i.e. the escape key is pressed while editing.
    pub fn on_cancel<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.modify(|textbox| textbox.on_cancel = Some(Box::new(callback)))
    }

    /// Sets a validation closure which is called when the textbox is edited and sets the validity attribute to the output of the closure.
    ///
    /// If a textbox is modified with the validate modifier then the `on_submit` will not be called if the text is invalid.
    pub fn validate<F>(self, is_valid: F) -> Self
    where
        F: 'static + Fn(&T) -> bool + Send + Sync,
    {
        self.modify(|textbox| textbox.validate = Some(Box::new(is_valid)))
    }

    /// Sets the placeholder text that appears when the textbox has no value.
    pub fn placeholder<P: ToStringLocalized + Clone + 'static>(
        self,
        text: impl Res<P> + 'static,
    ) -> Self {
        let text = text.to_signal(self.cx);
        self.bind(text, move |mut handle| {
            let text = text.get();
            let txt = text.to_string_local(&handle);
            let entity = handle.entity();
            handle = handle.modify(|textbox| textbox.placeholder.set(txt));
            handle.context().style.needs_access_update(entity);
        })
    }

    /// Sets an optional character used to visually mask textbox text.
    ///
    /// Use `Some('*')` (or any character) to enable masking and `None` to disable it.
    pub fn mask_char<U: Into<Option<char>> + Clone + 'static>(
        self,
        mask: impl Res<U> + 'static,
    ) -> Self {
        let mask = mask.to_signal(self.cx);
        self.bind(mask, move |mut handle| {
            let entity = handle.entity();
            let new_mask = mask.get().into();
            let mut display_text = String::new();
            handle = handle.modify(|textbox| {
                textbox.mask_char.set_if_changed(new_mask);
                display_text = textbox.display_text_from_real();
            });
            let cx = handle.context();
            push_editor_text_and_rebuild(entity, cx, &display_text);
        })
    }

    /// Sets whether masked text should be visible.
    pub fn mask_visible<U: Into<bool> + Clone + 'static>(
        self,
        visible: impl Res<U> + 'static,
    ) -> Self {
        let visible = visible.to_signal(self.cx);
        self.bind(visible, move |mut handle| {
            let entity = handle.entity();
            let new_visible = visible.get().into();
            let mut display_text = String::new();
            let mut changed = false;

            handle = handle.modify(|textbox| {
                if textbox.mask_visible != new_visible {
                    textbox.mask_visible = new_visible;
                    display_text = textbox.display_text_from_real();
                    changed = true;
                }
            });

            if changed {
                let cx = handle.context();
                push_editor_text_and_rebuild(entity, cx, &display_text);
            }
        })
    }

    /// Sets an optional maximum number of graphemes for textbox input.
    ///
    /// Use `Some(n)` to limit input length and `None` to remove the limit.
    pub fn max_length<U: Into<Option<usize>> + Clone + 'static>(
        self,
        max_length: impl Res<U> + 'static,
    ) -> Self {
        let max_length = max_length.to_signal(self.cx);
        self.bind(max_length, move |handle| {
            let value = max_length.get().into();
            handle.modify(|textbox| textbox.max_length.set_if_changed(value));
        })
    }

    /// Sets whether text in this textbox can be copied to the clipboard.
    pub fn can_copy<U: Into<bool> + Clone + 'static>(self, state: impl Res<U> + 'static) -> Self {
        let state = state.to_signal(self.cx);
        self.bind(state, move |handle| {
            let value = state.get().into();
            handle.modify(|textbox| textbox.can_copy.set_if_changed(value));
        })
    }

    /// Sets whether text can be pasted into this textbox from the clipboard.
    pub fn can_paste<U: Into<bool> + Clone + 'static>(self, state: impl Res<U> + 'static) -> Self {
        let state = state.to_signal(self.cx);
        self.bind(state, move |handle| {
            let value = state.get().into();
            handle.modify(|textbox| textbox.can_paste.set_if_changed(value));
        })
    }
}

/// Pushes `display` into `entity`'s `PlainEditor` (if it differs from the editor's current
/// text) and unconditionally rebuilds the entity's shaped-glyph cache. Used from contexts
/// (initial construction, value/placeholder/mask-visibility changes) where only a `Context`
/// (not an `EventContext`, nor a `Textbox`'s cached-generation state) is available.
fn push_editor_text_and_rebuild(entity: Entity, cx: &mut Context, display: &str) {
    if let Some(editor) = cx.text_context.plain_editors.get_mut(entity) {
        if editor.raw_text() != display {
            editor.set_text(display);
        }
    }

    if let Some(mut driver) = cx.text_context.editor_driver(entity) {
        let layout = driver.layout().clone();
        let text = driver.editor.raw_text().to_string();
        drop(driver);
        let pre_shaped =
            pre_shaped_from_editor_layout(entity, &cx.style, &text, &layout, &mut cx.text_context);
        let mut shaped = ShapedText::new(pre_shaped);
        shaped.layout(f32::MAX);
        cx.text_context.text_shaped.insert(entity, shaped);
    }

    cx.style.needs_relayout(entity);
    cx.needs_redraw(entity);
    cx.style.needs_access_update(entity);
}

/// Converts a byte offset (relative to line start) into a character index
/// within the `character_lengths` array for AccessKit text positioning.
fn byte_offset_to_char_index(character_lengths: &[u8], byte_offset: usize) -> usize {
    let mut cumulative = 0;
    for (i, &len) in character_lengths.iter().enumerate() {
        cumulative += len as usize;
        if byte_offset < cumulative {
            return i;
        }
    }
    character_lengths.len()
}

impl<R, T> View for Textbox<R, T>
where
    R: Res<T> + 'static,
    T: Clone + ToStringLocalized + std::str::FromStr + 'static,
{
    fn element(&self) -> Option<&'static str> {
        Some("textbox")
    }

    fn accessibility(&self, cx: &mut AccessContext, node: &mut AccessNode) {
        if !self.placeholder.get().is_empty() {
            node.set_placeholder(self.placeholder.get().clone());
        }

        let node_id = node.node_id();
        let entity = cx.current;

        let Some(editor) = cx.text_context.plain_editors.get(entity) else { return };
        let (selection_anchor, selection_active) = {
            let sel = editor.raw_selection();
            (sel.anchor().index(), sel.focus().index())
        };

        let mut selection_active_line = None;
        let mut selection_anchor_line = None;
        let mut selection_active_cursor = 0;
        let mut selection_anchor_cursor = 0;
        let mut first_line_node_id = None;

        let text =
            if self.show_placeholder.get() { String::new() } else { editor.raw_text().to_string() };
        let text = text.as_str();
        // build_paragraph() appends a zero-width space (\u{200B}, 3 UTF-8 bytes)
        // to every paragraph, so skia's line metrics include indices beyond the
        // actual text. We use text.len() as the upper bound for all slicing.
        let text_len = text.len();

        if cx.text_context.text_shaped.get(cx.current).is_some() {
            let text_direction = if resolved_text_direction(cx.style, cx.current)
                == crate::style::Direction::RightToLeft
            {
                TextDirection::RightToLeft
            } else {
                TextDirection::LeftToRight
            };

            let Some(shaped) = cx.text_context.text_shaped.get(cx.current) else {
                return;
            };
            let parley_layout = &shaped.pre_shaped.parley_layout;

            for (line_number, parley_line) in parley_layout.lines().enumerate() {
                let line_range = parley_line.text_range();
                let line_start = line_range.start.min(text_len);
                let line_end_including_newline = line_range.end.min(text_len);
                if line_start >= text_len && text_len > 0 {
                    continue;
                }

                let break_reason = parley_line.break_reason();
                let line_metrics = parley_line.metrics();

                let line_slice_full =
                    text.get(line_start..line_end_including_newline).unwrap_or("");
                let trimmed_len = line_slice_full.trim_end().len();
                let glyph_end = (line_start + trimmed_len).min(text_len);

                let mut line_node = AccessNode::new_from_parent(node_id, line_number);
                line_node.set_role(Role::TextRun);
                line_node.set_text_direction(text_direction);
                line_node.set_bounds(BoundingBox {
                    x: line_metrics.offset,
                    y: line_metrics.baseline - line_metrics.ascent,
                    w: line_metrics.advance,
                    h: line_metrics.line_height,
                });

                if line_start > glyph_end {
                    continue;
                }
                let line_slice = text.get(line_start..glyph_end).unwrap_or("");
                let estimated_chars = line_slice.graphemes(true).count();
                let mut character_lengths: Vec<u8> = Vec::with_capacity(estimated_chars);
                let mut character_positions: Vec<f32> = Vec::with_capacity(estimated_chars);
                let mut character_widths: Vec<f32> = Vec::with_capacity(estimated_chars);
                for (rel_start, grapheme) in line_slice.grapheme_indices(true) {
                    let start = line_start + rel_start;
                    let end = (start + grapheme.len()).min(glyph_end);
                    if end <= start {
                        continue;
                    }

                    let anchor = ParleyCursor::from_byte_index(
                        parley_layout,
                        start,
                        ParleyAffinity::Downstream,
                    );
                    let focus =
                        ParleyCursor::from_byte_index(parley_layout, end, ParleyAffinity::Upstream);
                    let sel = ParleySelection::new(anchor, focus);

                    let mut geometry = None;
                    sel.geometry_with(parley_layout, |bbox, line_idx| {
                        if geometry.is_none() && line_idx == line_number {
                            geometry = Some(bbox);
                        }
                    });

                    let (pos_x, width) = if let Some(bbox) = geometry {
                        (bbox.x0 as f32, (bbox.x1 - bbox.x0) as f32)
                    } else {
                        // No geometry can happen for zero-width graphemes.
                        (line_metrics.offset, 0.0)
                    };

                    character_lengths.push((end - start) as u8);
                    character_positions.push(pos_x);
                    character_widths.push(width.max(0.0));
                }

                let mut line_end = if matches!(break_reason, BreakReason::Explicit) {
                    line_end_including_newline
                } else {
                    glyph_end
                };
                if line_end < line_start {
                    line_end = line_start;
                }
                let line_text = text.get(line_start..line_end).unwrap_or("").to_owned();

                if matches!(break_reason, BreakReason::Explicit)
                    && line_end_including_newline <= text_len
                {
                    character_lengths.push(1);
                    character_positions.push(line_metrics.advance);
                    character_widths.push(0.0);
                }

                let mut word_starts = Vec::new();
                let mut last_word_start = None;
                for run in parley_line.runs() {
                    for cluster in run.clusters() {
                        if !cluster.is_word_boundary() {
                            continue;
                        }

                        let cluster_start = cluster.text_range().start;
                        if cluster_start < line_start || cluster_start >= line_end {
                            continue;
                        }

                        let rel_byte = cluster_start - line_start;
                        let char_index = byte_offset_to_char_index(&character_lengths, rel_byte);
                        if let Ok(char_index_u8) = u8::try_from(char_index)
                            && last_word_start != Some(char_index_u8)
                        {
                            word_starts.push(char_index_u8);
                            last_word_start = Some(char_index_u8);
                        }
                    }
                }

                if first_line_node_id.is_none() {
                    first_line_node_id = Some(line_node.node_id());
                }

                if selection_active >= line_start && selection_active <= line_end {
                    selection_active_line = Some(line_node.node_id());
                    selection_active_cursor = byte_offset_to_char_index(
                        &character_lengths,
                        selection_active - line_start,
                    );
                }

                if selection_anchor >= line_start && selection_anchor <= line_end {
                    selection_anchor_line = Some(line_node.node_id());
                    selection_anchor_cursor = byte_offset_to_char_index(
                        &character_lengths,
                        selection_anchor - line_start,
                    );
                }

                line_node.set_value(line_text.into_boxed_str());
                line_node.set_character_lengths(character_lengths.into_boxed_slice());
                line_node.set_character_positions(character_positions.into_boxed_slice());
                line_node.set_character_widths(character_widths.into_boxed_slice());
                line_node.set_word_starts(word_starts.into_boxed_slice());
                node.add_child(line_node);
            }
        }

        if let Some(fallback) = first_line_node_id {
            node.set_text_selection(TextSelection {
                anchor: TextPosition {
                    node: selection_anchor_line.unwrap_or(fallback),
                    character_index: selection_anchor_cursor,
                },
                focus: TextPosition {
                    node: selection_active_line.unwrap_or(fallback),
                    character_index: selection_active_cursor,
                },
            });
        }
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        // Window Events
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(MouseButton::Left) => {
                if meta.origin == cx.current {
                    return;
                }

                if cx.is_over() {
                    if !cx.is_disabled() {
                        cx.focus_with_visibility(true);
                        cx.capture();
                        cx.lock_cursor_icon();

                        if !self.edit {
                            cx.emit(TextEvent::StartEdit);
                        }
                        self.reset_caret_timer(cx);
                        cx.emit(TextEvent::Hit(
                            cx.mouse.cursor_x,
                            cx.mouse.cursor_y,
                            cx.modifiers.shift(),
                        ));
                    }
                } else {
                    cx.release();

                    // Forward event to hovered
                    cx.event_queue.push_back(
                        Event::new(WindowEvent::MouseDown(MouseButton::Left)).target(cx.hovered()),
                    );
                    cx.event_queue.push_back(
                        Event::new(WindowEvent::PressDown { mouse: true }).target(cx.hovered()),
                    );
                }
            }

            WindowEvent::FocusIn => {
                if cx.mouse.left.pressed != cx.current()
                    || cx.mouse.left.state == MouseButtonState::Released
                {
                    cx.emit(TextEvent::StartEdit);
                }
            }

            WindowEvent::FocusOut => {
                cx.emit(TextEvent::EndEdit);
            }

            WindowEvent::GeometryChanged(_) => {
                self.sync_editor_layout(cx);
            }

            WindowEvent::MouseDoubleClick(MouseButton::Left) => {
                cx.emit(TextEvent::SelectWord);
            }

            WindowEvent::MouseTripleClick(MouseButton::Left) => {
                cx.emit(TextEvent::SelectParagraph);
            }

            WindowEvent::MouseUp(MouseButton::Left) => {
                self.reset_caret_timer(cx);
                cx.unlock_cursor_icon();
                cx.release();
            }

            WindowEvent::MouseMove(x, y) => {
                if cx.mouse.left.state == MouseButtonState::Pressed
                    && cx.mouse.left.pressed == cx.current
                {
                    if self.edit {
                        self.reset_caret_timer(cx);
                    }
                    if cx.mouse.left.pos_down.0 != *x || cx.mouse.left.pos_down.1 != *y {
                        cx.emit(TextEvent::Drag(*x, *y));
                    }
                }
            }

            WindowEvent::MouseScroll(x, y) => {
                cx.emit(TextEvent::Scroll(*x, *y));
            }

            WindowEvent::CharInput(c) => {
                if *c != '\u{1b}' && // Escape
                    *c != '\u{8}' && // Backspace
                    *c != '\u{9}' && // Tab
                    *c != '\u{7f}' && // Delete
                    *c != '\u{0d}' && // Carriage return
                    !cx.modifiers.ctrl() &&
                    !cx.modifiers.logo() &&
                    self.edit &&
                    !cx.is_read_only()
                {
                    self.reset_caret_timer(cx);
                    cx.emit(TextEvent::InsertText(String::from(*c)));
                }
            }

            WindowEvent::ImeCommit(text) => {
                if !cx.modifiers.ctrl() && !cx.modifiers.logo() && self.edit && !cx.is_read_only() {
                    self.reset_caret_timer(cx);
                    cx.emit(TextEvent::ClearPreedit);
                    cx.emit(TextEvent::InsertText(text.to_string()));

                    self.reset_ime_position(cx);
                }
            }

            WindowEvent::ImePreedit(text, cursor) => {
                if !cx.modifiers.ctrl() && !cx.modifiers.logo() && self.edit && !cx.is_read_only() {
                    self.reset_caret_timer(cx);
                    cx.emit(TextEvent::UpdatePreedit(text.to_string(), *cursor));
                }
            }

            WindowEvent::KeyDown(code, _) => match code {
                Code::Enter => {
                    if matches!(self.kind, TextboxKind::SingleLine) {
                        cx.emit(TextEvent::Submit(true));
                    } else if !cx.is_read_only() {
                        self.reset_caret_timer(cx);
                        cx.emit(TextEvent::InsertText("\n".to_owned()));
                    }
                }

                // Note: no `Code::Space` arm — the space character arrives
                // through `WindowEvent::CharInput(' ')` above, which already
                // inserts it (and correctly suppresses insertion when Ctrl
                // or Cmd is held). Handling it here as well produced double
                // insertion on platforms that emit both events for a plain
                // spacebar press.
                Code::ArrowLeft => {
                    self.reset_caret_timer(cx);
                    // macOS convention: Option (alt) for word movement,
                    // Cmd (logo) for line-boundary movement.
                    // Other platforms: Ctrl for word movement.
                    #[cfg(target_os = "macos")]
                    let movement = if cx.modifiers.logo() {
                        TextMovement::LineStart
                    } else if cx.modifiers.alt() {
                        TextMovement::WordLeft
                    } else {
                        TextMovement::Left
                    };
                    #[cfg(not(target_os = "macos"))]
                    let movement = if cx.modifiers.ctrl() {
                        TextMovement::WordLeft
                    } else {
                        TextMovement::Left
                    };

                    cx.emit(TextEvent::MoveCursor(movement, cx.modifiers.shift()));
                }

                Code::ArrowRight => {
                    self.reset_caret_timer(cx);

                    #[cfg(target_os = "macos")]
                    let movement = if cx.modifiers.logo() {
                        TextMovement::LineEnd
                    } else if cx.modifiers.alt() {
                        TextMovement::WordRight
                    } else {
                        TextMovement::Right
                    };
                    #[cfg(not(target_os = "macos"))]
                    let movement = if cx.modifiers.ctrl() {
                        TextMovement::WordRight
                    } else {
                        TextMovement::Right
                    };

                    cx.emit(TextEvent::MoveCursor(movement, cx.modifiers.shift()));
                }

                Code::ArrowUp => {
                    self.reset_caret_timer(cx);
                    if self.kind != TextboxKind::SingleLine {
                        cx.emit(TextEvent::MoveCursor(TextMovement::Up, cx.modifiers.shift()));
                    }
                }

                Code::ArrowDown => {
                    self.reset_caret_timer(cx);
                    if self.kind != TextboxKind::SingleLine {
                        cx.emit(TextEvent::MoveCursor(TextMovement::Down, cx.modifiers.shift()));
                    }
                }

                Code::Backspace => {
                    self.reset_caret_timer(cx);
                    if !cx.is_read_only() {
                        #[cfg(target_os = "macos")]
                        let movement = if cx.modifiers.logo() {
                            // Cmd+Backspace deletes from caret to the visual
                            // line start on macOS, matching Cmd+Left cursor
                            // movement (which uses `TextMovement::LineStart`).
                            DeleteMovement::BackwardToLineStart
                        } else if cx.modifiers.alt() {
                            DeleteMovement::BackwardWord
                        } else {
                            DeleteMovement::BackwardGrapheme
                        };
                        #[cfg(not(target_os = "macos"))]
                        let movement = if cx.modifiers.ctrl() {
                            DeleteMovement::BackwardWord
                        } else {
                            DeleteMovement::BackwardGrapheme
                        };

                        cx.emit(TextEvent::DeleteText(movement));
                    }
                }

                Code::Delete => {
                    self.reset_caret_timer(cx);
                    if !cx.is_read_only() {
                        #[cfg(target_os = "macos")]
                        let movement = if cx.modifiers.alt() {
                            DeleteMovement::ForwardWord
                        } else {
                            DeleteMovement::ForwardGrapheme
                        };
                        #[cfg(not(target_os = "macos"))]
                        let movement = if cx.modifiers.ctrl() {
                            DeleteMovement::ForwardWord
                        } else {
                            DeleteMovement::ForwardGrapheme
                        };

                        cx.emit(TextEvent::DeleteText(movement));
                    }
                }

                Code::Escape => {
                    if let Some(callback) = &self.on_cancel {
                        (callback)(cx);
                    } else {
                        cx.emit(TextEvent::EndEdit);
                    }
                }

                Code::Home => {
                    self.reset_caret_timer(cx);
                    cx.emit(TextEvent::MoveCursor(TextMovement::LineStart, cx.modifiers.shift()));
                }

                Code::End => {
                    self.reset_caret_timer(cx);
                    cx.emit(TextEvent::MoveCursor(TextMovement::LineEnd, cx.modifiers.shift()));
                }

                Code::KeyA => {
                    #[cfg(target_os = "macos")]
                    let modifier = Modifiers::SUPER;
                    #[cfg(not(target_os = "macos"))]
                    let modifier = Modifiers::CTRL;

                    if cx.modifiers == &modifier {
                        cx.emit(TextEvent::SelectAll);
                    }
                }

                Code::KeyC => {
                    #[cfg(target_os = "macos")]
                    let modifier = Modifiers::SUPER;
                    #[cfg(not(target_os = "macos"))]
                    let modifier = Modifiers::CTRL;

                    if cx.modifiers == &modifier && self.can_copy.get() {
                        cx.emit(TextEvent::Copy);
                    }
                }

                Code::KeyV => {
                    #[cfg(target_os = "macos")]
                    let modifier = Modifiers::SUPER;
                    #[cfg(not(target_os = "macos"))]
                    let modifier = Modifiers::CTRL;

                    if cx.modifiers == &modifier && self.can_paste.get() {
                        cx.emit(TextEvent::Paste);
                    }
                }

                Code::KeyX => {
                    #[cfg(target_os = "macos")]
                    let modifier = Modifiers::SUPER;
                    #[cfg(not(target_os = "macos"))]
                    let modifier = Modifiers::CTRL;

                    if cx.modifiers == &modifier && !cx.is_read_only() {
                        cx.emit(TextEvent::Cut);
                    }
                }

                _ => {}
            },

            WindowEvent::ActionRequest(ActionRequest {
                action: accesskit::Action::SetTextSelection,
                target_tree: _,
                target_node: _,
                data: Some(ActionData::SetTextSelection(_selection)),
            }) => {
                // TODO: Implement SetTextSelection action for screen reader support.
            }

            _ => {}
        });

        // Textbox Events
        event.map(|text_event, _| match text_event {
            TextEvent::InsertText(text) => {
                let entity = cx.current;
                if cx
                    .text_context
                    .plain_editors
                    .get(entity)
                    .map(|e| e.is_composing())
                    .unwrap_or(false)
                {
                    return;
                }

                self.edited_since_focus = true;
                self.edited_once = true;

                if self.show_placeholder.get() {
                    self.reset_text(cx);
                }

                self.insert_text(cx, text);

                let text = self.clone_text(cx);

                cx.set_valid(self.is_text_valid(&text));

                if self.edit {
                    if let Some(callback) = &self.on_edit {
                        (callback)(cx, text);
                    }
                }
            }

            TextEvent::UpdatePreedit(preedit, cursor) => {
                self.update_preedit(cx, preedit, *cursor);
            }

            TextEvent::ClearPreedit => {
                self.clear_preedit(cx);
            }

            TextEvent::Clear => {
                self.reset_text(cx);
                cx.needs_redraw();
            }

            TextEvent::DeleteText(movement) => {
                if self.edit {
                    self.edited_since_focus = true;
                    self.edited_once = true;
                    self.delete_text(cx, *movement);

                    let text = self.clone_text(cx);

                    cx.set_valid(self.is_text_valid(&text));

                    if let Some(callback) = &self.on_edit {
                        (callback)(cx, text);
                    }
                }
            }

            TextEvent::MoveCursor(movement, selection) => {
                let entity = cx.current;
                let is_composing = cx
                    .text_context
                    .plain_editors
                    .get(entity)
                    .map(|e| e.is_composing())
                    .unwrap_or(false);
                if self.edit && !self.show_placeholder.get() && !is_composing {
                    self.move_cursor(cx, *movement, *selection);
                }
            }

            TextEvent::SetPlaceholder(text) => {
                self.placeholder.set(text.clone());
                cx.style.needs_access_update(cx.current);
            }

            TextEvent::StartEdit => {
                if !cx.is_disabled() && !self.edit {
                    self.edit = true;
                    self.edited_since_focus = false;
                    cx.focus_with_visibility(true);
                    cx.capture();
                    self.reset_caret_timer(cx);
                    self.reset_ime_position(cx);

                    self.text_overflow = cx.style.text_overflow.get_inline(cx.current).copied();
                    cx.style.text_overflow.remove(cx.current);

                    let text = self.value.get_value(cx);
                    let text = text.to_string_local(cx);
                    self.real_text = text.clone();

                    let entity = cx.current;
                    if let Some(editor) = cx.text_context.plain_editors.get_mut(entity) {
                        apply_editor_style(entity, cx.style, editor);
                    }

                    if text.is_empty() {
                        self.show_placeholder.set(true);
                    } else {
                        self.show_placeholder.set(false);
                    }

                    let display = self.display_text_from_real();
                    if let Some(editor) = cx.text_context.plain_editors.get_mut(entity) {
                        if editor.raw_text() != display {
                            editor.set_text(&display);
                        }
                    }

                    self.sync_editor_layout(cx);

                    if text.is_empty() {
                        if let Some(mut driver) = cx.text_context.editor_driver(entity) {
                            driver.move_to_text_start();
                        }
                        cx.style.needs_access_update(entity);
                    } else {
                        self.select_all(cx);
                    }

                    // Keep textbox pristine only until first user edit; once edited,
                    // preserve validation across blur/focus cycles.
                    if self.edited_once || !text.is_empty() {
                        cx.set_valid(self.is_text_valid(&text));
                    } else {
                        cx.set_valid(true);
                    }
                }
            }

            TextEvent::EndEdit => {
                self.edit = false;
                cx.release();
                cx.stop_timer(self.caret_timer);

                let text = self.clone_text(cx);
                self.show_placeholder.set(text.is_empty());

                if let Some(text_overflow) = self.text_overflow {
                    cx.style.text_overflow.insert(cx.current, text_overflow);
                } else {
                    cx.style.text_overflow.remove(cx.current);
                }

                if self.edited_since_focus {
                    cx.set_valid(self.is_text_valid(&text));
                }

                // Reset transform to 0,0
                *self.transform.borrow_mut() = (0.0, 0.0);

                // Reset cursor position to the start of the text.
                let entity = cx.current;
                if let Some(mut driver) = cx.text_context.editor_driver(entity) {
                    driver.move_to_text_start();
                }
                self.rebuild_shaped_cache(cx);
                cx.style.needs_access_update(entity);
            }

            TextEvent::Blur => {
                // Clicking outside a textbox can end editing while retaining keyboard focus
                // (for example when clicking non-focusable chrome). Keep focus but remove
                // the visible focus indicator for pointer-driven blur.
                if cx.focused() == cx.current() {
                    cx.focus_with_visibility(false);
                }

                if let Some(callback) = &self.on_blur {
                    (callback)(cx);
                } else {
                    cx.emit(TextEvent::Submit(false));
                    cx.emit(TextEvent::EndEdit);
                }
            }

            TextEvent::SetMaskVisible(visible) => {
                if self.mask_visible != *visible {
                    self.mask_visible = *visible;
                    self.resync_display_text(cx);
                    cx.needs_redraw();
                }
            }

            TextEvent::ToggleMaskVisible => {
                self.mask_visible = !self.mask_visible;
                self.resync_display_text(cx);
                cx.needs_redraw();
            }

            TextEvent::Submit(reason) => {
                if let Some(callback) = &self.on_submit {
                    let text = self.clone_text(cx);
                    let is_valid = self.is_text_valid(&text);
                    cx.set_valid(is_valid);
                    if is_valid && let Ok(value) = text.parse::<T>() {
                        (callback)(cx, value, *reason);
                    }
                }
            }

            TextEvent::SelectAll => {
                self.select_all(cx);
            }

            TextEvent::SelectWord => {
                self.select_word(cx);
            }

            TextEvent::SelectParagraph => {
                self.select_paragraph(cx);
            }

            TextEvent::Hit(posx, posy, selection) => {
                if !self.show_placeholder.get() {
                    self.hit(cx, *posx, *posy, *selection);
                }
            }

            TextEvent::Drag(posx, posy) => {
                if !self.show_placeholder.get() {
                    self.drag(cx, *posx, *posy);
                }
            }

            TextEvent::Scroll(_x, _y) => {
                //self.scroll(cx, *x, *y);
            }

            TextEvent::Copy =>
            {
                #[cfg(feature = "clipboard")]
                if self.edit && self.can_copy.get() {
                    if let Some(selected_text) = self.clone_selected(cx) {
                        if !selected_text.is_empty() {
                            cx.set_clipboard(selected_text)
                                .expect("Failed to add text to clipboard");
                        }
                    }
                }
            }

            TextEvent::Paste =>
            {
                #[cfg(feature = "clipboard")]
                if self.edit && self.can_paste.get() {
                    if let Ok(text) = cx.get_clipboard() {
                        cx.emit(TextEvent::InsertText(text));
                    }
                }
            }

            TextEvent::Cut =>
            {
                #[cfg(feature = "clipboard")]
                if self.edit {
                    if let Some(selected_text) = self.clone_selected(cx) {
                        if !selected_text.is_empty() {
                            self.edited_since_focus = true;
                            self.edited_once = true;
                            cx.set_clipboard(selected_text)
                                .expect("Failed to add text to clipboard");
                            self.delete_text(cx, DeleteMovement::BackwardGrapheme);

                            let text = self.clone_text(cx);

                            cx.set_valid(self.is_text_valid(&text));

                            if let Some(callback) = &self.on_edit {
                                (callback)(cx, text);
                            }
                        }
                    }
                }
            }

            TextEvent::ToggleCaret => {
                self.show_caret ^= true;
            }
        });
    }

    // Use custom drawing for the textbox so a transform can be applied to just the text.
    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        cx.draw_shadows(canvas);
        cx.draw_background(canvas);
        cx.draw_border(canvas);
        cx.draw_outline(canvas);

        // Clip only the text content to the textbox shape so long text is contained
        // without clipping outside effects such as outlines.
        canvas.save();
        let path = cx.path();
        canvas.clip_path(&path, ClipOp::Intersect, true);

        let bounds = cx.bounds();
        let padding_left = match cx.padding_left() {
            Units::Pixels(val) => val,
            _ => 0.0,
        };
        let padding_right = match cx.padding_right() {
            Units::Pixels(val) => val,
            _ => 0.0,
        };
        let padding_top = match cx.padding_top() {
            Units::Pixels(val) => val,
            _ => 0.0,
        };
        let padding_bottom = match cx.padding_bottom() {
            Units::Pixels(val) => val,
            _ => 0.0,
        };

        let content_left = bounds.x + padding_left;
        let content_top = bounds.y + padding_top;
        let content_right = (bounds.x + bounds.w - padding_right).max(content_left);
        let content_bottom = (bounds.y + bounds.h - padding_bottom).max(content_top);
        canvas.clip_rect(
            Rect::new(content_left, content_top, content_right, content_bottom),
            ClipOp::Intersect,
            true,
        );

        let transform = *self.transform.borrow();
        canvas.translate((transform.0, transform.1));
        cx.draw_text(canvas);

        if self.edit {
            self.draw_selection(cx, canvas);
            self.draw_text_caret(cx, canvas);
        }

        canvas.restore();
    }
}
