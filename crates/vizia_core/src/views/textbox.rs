use crate::accessibility::IntoNode;
use crate::context::AccessNode;
use crate::layout::BoundingBox;
use crate::prelude::*;

use crate::text::{enforce_text_bounds, ensure_visible, Direction, Movement};
use crate::views::scrollview::SCROLL_SENSITIVITY;
use accesskit::{ActionData, ActionRequest, TextDirection, TextPosition, TextSelection};
use cosmic_text::{Action, Cursor, Edit, Editor, FontSystem};
use unicode_segmentation::UnicodeSegmentation;
use vizia_input::Code;
use vizia_storage::TreeExt;

/// Events for modifying a textbox.
pub enum TextEvent {
    /// Insert a string of text into the textbox.
    InsertText(String),
    /// Reset the text of the textbox to the bound data.
    Clear,
    /// Delete a section of text, determined by the `Movement`.
    DeleteText(Movement),
    /// Move the cursor and selection.
    MoveCursor(Movement, bool),
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
    Hit(f32, f32),
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

    ToggleCaret,
}

/// The `Textbox` view provides an input control for editing a value as a string.
///
/// The textbox takes a lens to some value, which must be a type which can convert to and from a `String`,
/// as determined by the `ToString` and `FromStr` traits. The value type is used for validation and returned by
/// the `on_submit` callback, which is triggered when the textbox is submitted with the enter key or when the textbox
/// loses keyboard focus.
#[derive(Lens)]
pub struct Textbox<L: Lens> {
    lens: L,
    #[lens(ignore)]
    kind: TextboxKind,
    edit: bool,
    transform: (f32, f32),
    on_edit: Option<Box<dyn Fn(&mut EventContext, String) + Send + Sync>>,
    on_submit: Option<Box<dyn Fn(&mut EventContext, L::Target, bool) + Send + Sync>>,
    on_blur: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    on_cancel: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    validate: Option<Box<dyn Fn(&L::Target) -> bool>>,
    placeholder: String,
    placeholder_shown: bool,
    show_caret: bool,
    caret_timer: Timer,
}

// Determines whether the enter key submits the text or inserts a new line.
#[derive(Copy, Clone, PartialEq, Eq)]
enum TextboxKind {
    SingleLine,
    MultiLineUnwrapped,
    MultiLineWrapped,
}

impl<L: Lens> Textbox<L>
where
    <L as Lens>::Target: Data + Clone + ToString + std::str::FromStr,
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
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self::new_core(cx, lens, TextboxKind::SingleLine)
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
    pub fn new_multiline(cx: &mut Context, lens: L, wrap: bool) -> Handle<Self> {
        Self::new_core(
            cx,
            lens,
            if wrap { TextboxKind::MultiLineWrapped } else { TextboxKind::MultiLineUnwrapped },
        )
    }

    fn new_core(cx: &mut Context, lens: L, kind: TextboxKind) -> Handle<Self> {
        let caret_timer = cx.environment().caret_timer;

        Self {
            lens,
            kind,
            edit: false,
            transform: (0.0, 0.0),
            on_edit: None,
            on_submit: None,
            on_blur: None,
            on_cancel: None,
            validate: None,
            placeholder: String::from(""),
            placeholder_shown: true,
            show_caret: true,
            caret_timer,
        }
        .build(cx, move |cx| {
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

            Binding::new(cx, lens, move |cx: &mut Context, text| {
                Binding::new(cx, Self::edit, move |cx, edit| {
                    if !edit.get(cx) {
                        let text_str = text.get(cx).to_string();
                        cx.emit(TextEvent::SelectAll);
                        cx.emit(TextEvent::InsertText(text_str));
                        cx.emit(TextEvent::Scroll(0.0, 0.0));
                    }
                })
            });

            Binding::new(cx, Self::placeholder, |cx, placeholder| {
                let placeholder_string = placeholder.get(cx).to_string();
                if !placeholder_string.is_empty() {
                    Label::new(cx, &placeholder_string)
                        .size(Stretch(1.0))
                        .space(Pixels(0.0))
                        .display(Self::placeholder_shown)
                        .hoverable(false)
                        .position_type(PositionType::SelfDirected)
                        .hidden(true)
                        .class("placeholder");
                }
            });
        })
        .toggle_class("multiline", kind == TextboxKind::MultiLineWrapped)
        .text_wrap(kind == TextboxKind::MultiLineWrapped)
        .navigable(true)
        .role(Role::TextField)
        .text_value(lens)
        .default_action_verb(DefaultActionVerb::Focus)
        .toggle_class("caret", Self::show_caret)
    }

    fn set_caret(&mut self, cx: &mut EventContext) {
        // Calculate visible area for content and container
        let mut text_bounds = cx.text_context.get_bounds(cx.current).unwrap_or_default();
        let mut bounds = cx.bounds();

        let child_left = cx.style.child_left.get(cx.current).copied().unwrap_or_default();
        let child_top = cx.style.child_top.get(cx.current).copied().unwrap_or_default();
        let child_right = cx.style.child_right.get(cx.current).copied().unwrap_or_default();
        let child_bottom = cx.style.child_bottom.get(cx.current).copied().unwrap_or_default();

        let logical_parent_width = cx.physical_to_logical(bounds.w);
        let logical_parent_height = cx.physical_to_logical(bounds.h);

        let child_left = child_left.to_px(logical_parent_width, 0.0) * cx.scale_factor();
        let child_top = child_top.to_px(logical_parent_height, 0.0) * cx.scale_factor();
        let child_right = child_right.to_px(logical_parent_width, 0.0) * cx.scale_factor();
        let child_bottom = child_bottom.to_px(logical_parent_height, 0.0) * cx.scale_factor();

        text_bounds.x = bounds.x;
        text_bounds.y = bounds.y;

        bounds.h -= child_top + child_bottom;
        bounds.w -= child_left + child_right;

        cx.text_context.sync_styles(cx.current, cx.style);

        let (mut tx, mut ty) = self.transform;

        (tx, ty) = enforce_text_bounds(&text_bounds, &bounds, (tx, ty));

        text_bounds.x += child_left;
        text_bounds.y += child_top;

        // TODO justify????
        if let Some((x, y, _, h)) =
            cx.text_context.layout_caret(cx.current, text_bounds, (0., 0.), 1.0 * cx.scale_factor())
        {
            let caret_box = BoundingBox { x, y, w: 0.0, h };
            bounds.x += child_left;
            bounds.y += child_top;

            (tx, ty) = ensure_visible(&caret_box, &bounds, (tx, ty));
        }

        self.transform = (tx.round(), ty.round());
    }

    fn insert_text(&mut self, cx: &mut EventContext, text: &str) {
        cx.text_context.with_editor(cx.current, |_: &mut FontSystem, buf| {
            buf.insert_string(text, None);
        });
        cx.needs_relayout();
        cx.needs_redraw();
    }

    fn delete_text(&mut self, cx: &mut EventContext, movement: Movement) {
        let x = |_: &mut FontSystem, buf: &mut Editor| {
            let no_selection = match (buf.cursor(), buf.select_opt()) {
                (cursor, Some(selection)) => cursor == selection,
                (_, None) => true,
            };
            buf.delete_selection();
            no_selection
        };

        if cx.text_context.with_editor(cx.current, x) {
            self.move_cursor(cx, movement, true);
            cx.text_context.with_editor(cx.current, |_, buf| {
                buf.delete_selection();
            });
        }
        cx.needs_relayout();
        cx.needs_redraw();
    }

    fn reset_text(&mut self, cx: &mut EventContext) {
        self.select_all(cx);
        cx.text_context.with_editor(cx.current, |_, buf| {
            buf.delete_selection();
        });
        cx.needs_relayout();
        cx.needs_redraw();
    }

    fn move_cursor(&mut self, cx: &mut EventContext, movement: Movement, selection: bool) {
        cx.text_context.with_editor(cx.current, |fs, buf| {
            if selection {
                if buf.select_opt().is_none() {
                    buf.set_select_opt(Some(buf.cursor()));
                }
            } else {
                buf.set_select_opt(None);
            }

            buf.action(
                fs,
                match movement {
                    Movement::Grapheme(Direction::Upstream) => Action::Previous,
                    Movement::Grapheme(Direction::Downstream) => Action::Next,
                    Movement::Grapheme(Direction::Left) => Action::Left,
                    Movement::Grapheme(Direction::Right) => Action::Right,
                    Movement::Word(Direction::Upstream) => Action::PreviousWord,
                    Movement::Word(Direction::Downstream) => Action::NextWord,
                    Movement::Word(Direction::Left) => Action::LeftWord,
                    Movement::Word(Direction::Right) => Action::RightWord,
                    Movement::Line(Direction::Upstream) => Action::Up,
                    Movement::Line(Direction::Downstream) => Action::Down,
                    Movement::LineStart => Action::Home,
                    Movement::LineEnd => Action::End,
                    Movement::Page(dir) => {
                        let parent = cx.current.parent(cx.tree).unwrap();
                        let parent_bounds = *cx.cache.bounds.get(parent).unwrap();
                        let sign = if let Direction::Upstream = dir { -1 } else { 1 };
                        Action::Vertical(sign * parent_bounds.h as i32)
                    }
                    Movement::Body(Direction::Upstream) => Action::BufferStart,
                    Movement::Body(Direction::Downstream) => Action::BufferEnd,
                    _ => return,
                },
            );
        });
        cx.needs_relayout();
        cx.needs_redraw();
    }

    fn select_all(&mut self, cx: &mut EventContext) {
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.action(fs, Action::BufferStart);
            buf.set_select_opt(Some(buf.cursor()));
            buf.action(fs, Action::BufferEnd);
        });
        cx.needs_redraw();
    }

    fn select_word(&mut self, cx: &mut EventContext) {
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.action(fs, Action::PreviousWord);
            buf.set_select_opt(Some(buf.cursor()));
            buf.action(fs, Action::NextWord);
        });
        cx.needs_redraw();
    }

    fn select_paragraph(&mut self, cx: &mut EventContext) {
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.action(fs, Action::ParagraphStart);
            buf.set_select_opt(Some(buf.cursor()));
            buf.action(fs, Action::ParagraphEnd);
        });
        cx.needs_redraw();
    }

    fn deselect(&mut self, cx: &mut EventContext) {
        cx.text_context.with_editor(cx.current, |_, buf| {
            buf.set_select_opt(None);
        });
        cx.needs_redraw();
    }

    /// These input coordinates should be physical coordinates, i.e. what the mouse events provide.
    /// The output text coordinates will also be physical, but relative to the top of the text
    /// glyphs, appropriate for passage to cosmic.
    fn coordinates_global_to_text(&self, cx: &mut EventContext, x: f32, y: f32) -> (f32, f32) {
        let bounds = cx.bounds();

        let child_left = cx.style.child_left.get(cx.current).copied().unwrap_or_default();
        let child_top = cx.style.child_top.get(cx.current).copied().unwrap_or_default();
        let _child_right = cx.style.child_right.get(cx.current).copied().unwrap_or_default();
        let child_bottom = cx.style.child_bottom.get(cx.current).copied().unwrap_or_default();

        let justify_y = match (child_top, child_bottom) {
            (Stretch(top), Stretch(bottom)) => {
                if top + bottom == 0.0 {
                    0.5
                } else {
                    top / (top + bottom)
                }
            }
            (Stretch(_), _) => 1.0,
            _ => 0.0,
        };

        let logical_parent_width = cx.physical_to_logical(bounds.w);
        let logical_parent_height = cx.physical_to_logical(bounds.h);

        let child_left = child_left.to_px(logical_parent_width, 0.0) * cx.scale_factor();
        let child_top = child_top.to_px(logical_parent_height, 0.0) * cx.scale_factor();

        let total_height = cx.text_context.with_buffer(cx.current, |_, buffer| {
            buffer.layout_runs().len() as f32 * buffer.metrics().line_height
        });

        let x = x - bounds.x - self.transform.0 - child_left;
        let y = y - self.transform.1 - bounds.y - (bounds.h - total_height) * justify_y - child_top;

        (x, y)
    }

    /// This function takes window-global physical coordinates.
    fn hit(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        let (x, y) = self.coordinates_global_to_text(cx, x, y);
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.action(fs, Action::Click { x: x as i32, y: y as i32 });
        });
        cx.needs_redraw();
    }

    /// This function takes window-global physical coordinates.
    fn drag(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        let (x, y) = self.coordinates_global_to_text(cx, x, y);
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.action(fs, Action::Drag { x: x as i32, y: y as i32 });
        });
        cx.needs_redraw();
    }

    /// This function takes window-global physical dimensions.
    fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        let entity = cx.current;
        let mut bounds = cx.cache.get_bounds(entity);

        let child_left = cx.style.child_left.get(cx.current).copied().unwrap_or_default();
        let child_top = cx.style.child_top.get(cx.current).copied().unwrap_or_default();
        let child_right = cx.style.child_right.get(cx.current).copied().unwrap_or_default();
        let child_bottom = cx.style.child_bottom.get(cx.current).copied().unwrap_or_default();

        let logical_parent_width = cx.physical_to_logical(bounds.w);
        let logical_parent_height = cx.physical_to_logical(bounds.h);

        let child_left = child_left.to_px(logical_parent_width, 0.0) * cx.scale_factor();
        let child_top = child_top.to_px(logical_parent_height, 0.0) * cx.scale_factor();
        let child_right = child_right.to_px(logical_parent_width, 0.0) * cx.scale_factor();
        let child_bottom = child_bottom.to_px(logical_parent_height, 0.0) * cx.scale_factor();

        if let Some(mut text_bounds) = cx.text_context.get_bounds(entity) {
            text_bounds.x = bounds.x;
            text_bounds.y = bounds.y;
            bounds.h -= child_top + child_bottom;
            bounds.w -= child_left + child_right;
            let (mut tx, mut ty) = self.transform;
            tx += x * SCROLL_SENSITIVITY;
            ty += y * SCROLL_SENSITIVITY;
            (tx, ty) = enforce_text_bounds(&text_bounds, &bounds, (tx, ty));
            self.transform = (tx, ty);
            cx.needs_redraw();
        }
    }

    #[allow(dead_code)]
    fn clone_selected(&self, cx: &mut EventContext) -> Option<String> {
        cx.text_context.with_editor(cx.current, |_, buf| buf.copy_selection())
    }

    fn clone_text(&self, cx: &mut EventContext) -> String {
        cx.text_context.with_buffer(cx.current, |_, buf| {
            buf.lines.iter().map(|line| line.text()).collect::<Vec<_>>().join("\n")
        })
    }

    fn reset_caret_timer(&mut self, cx: &mut EventContext) {
        cx.stop_timer(self.caret_timer);
        if !cx.is_read_only() {
            self.show_caret = true;
            cx.start_timer(self.caret_timer);
        }
    }
}

impl<'a, L: Lens> Handle<'a, Textbox<L>> {
    /// Sets the callback triggered when a textbox is edited, i.e. text is inserted/deleted.
    ///
    /// Callback provides the current text of the textbox.
    pub fn on_edit<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, String) + Send + Sync,
    {
        self.modify(|textbox: &mut Textbox<L>| textbox.on_edit = Some(Box::new(callback)))
    }

    /// Sets the callback triggered when a textbox is submitted,
    /// i.e. when the enter key is pressed with a single-line textbox or the textbox loses focus.
    ///
    /// Callback provides the text of the textbox and a flag to indicate if the submit was due to a key press or a loss of focus.
    pub fn on_submit<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, L::Target, bool) + Send + Sync,
    {
        self.modify(|textbox: &mut Textbox<L>| textbox.on_submit = Some(Box::new(callback)))
    }

    /// Sets the callback triggered when a textbox is blurred, i.e. the mouse is pressed outside of the textbox.
    pub fn on_blur<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.modify(|textbox: &mut Textbox<L>| textbox.on_blur = Some(Box::new(callback)))
    }

    /// Sets the callback triggered when a textbox edit is cancelled, i.e. the escape key is pressed while editing.
    pub fn on_cancel<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.modify(|textbox: &mut Textbox<L>| textbox.on_cancel = Some(Box::new(callback)))
    }

    /// Sets a validation closure which is called when the textbox is edited and sets the validity attribute to the output of the closure.
    ///
    /// If a textbox is modified with the validate modifier then the `on_submit` will not be called if the text is invalid.
    pub fn validate<F>(self, is_valid: F) -> Self
    where
        F: 'static + Fn(&L::Target) -> bool + Send + Sync,
    {
        self.modify(|textbox| textbox.validate = Some(Box::new(is_valid)))
    }

    /// Sets the placeholder text that appears when the textbox has no value.
    pub fn placeholder<P: ToString>(self, text: impl Res<P>) -> Self {
        text.set_or_bind(self.cx, self.entity, |cx, val| {
            cx.emit(TextEvent::SetPlaceholder(val.to_string()));
            cx.needs_relayout();
            cx.needs_redraw();
        });

        self
    }
}

impl<L: Lens> View for Textbox<L>
where
    <L as Lens>::Target: Data + ToString + std::str::FromStr,
{
    fn element(&self) -> Option<&'static str> {
        Some("textbox")
    }

    fn accessibility(&self, cx: &mut AccessContext, node: &mut AccessNode) {
        let bounds = cx.bounds();

        let node_id = node.node_id();
        cx.text_context.with_editor(cx.current, |_, editor| {
            let cursor = editor.cursor();
            let selection = editor.select_opt().unwrap_or(cursor);

            let mut selection_active_line = node_id;
            let mut selection_anchor_line = node_id;
            let mut selection_active_cursor = 0;
            let mut selection_anchor_cursor = 0;

            let mut current_cursor = 0;
            let mut prev_line_index = std::usize::MAX;

            for (index, line) in editor.buffer().layout_runs().enumerate() {
                let text = line.text;

                // We need a child node per line
                let mut line_node = AccessNode::new_from_parent(node_id, index);
                line_node.set_role(Role::InlineTextBox);

                let line_height = editor.buffer().metrics().line_height;
                line_node.set_bounds(BoundingBox {
                    x: bounds.x,
                    y: bounds.y + line.line_y - editor.buffer().metrics().font_size,
                    w: line.line_w,
                    h: line_height,
                });
                line_node.set_text_direction(if line.rtl {
                    TextDirection::RightToLeft
                } else {
                    TextDirection::LeftToRight
                });

                let mut character_lengths = Vec::with_capacity(line.glyphs.len());
                let mut character_positions = Vec::with_capacity(line.glyphs.len());
                let mut character_widths = Vec::with_capacity(line.glyphs.len());

                // Get the actual text in the line
                let first_glyph_pos =
                    line.glyphs.first().map(|glyph| glyph.start).unwrap_or_default();
                let last_glyph_pos = line.glyphs.last().map(|glyph| glyph.end).unwrap_or_default();

                let mut line_text = text[first_glyph_pos..last_glyph_pos].to_owned();

                let word_lengths =
                    line_text.unicode_words().map(|word| word.len() as u8).collect::<Vec<_>>();

                let mut line_length = 0;

                for glyph in line.glyphs.iter() {
                    let length = (glyph.end - glyph.start) as u8;

                    line_length += length as usize;

                    let position = glyph.x;
                    let width = glyph.w;

                    character_lengths.push(length);
                    character_positions.push(position);
                    character_widths.push(width);
                }

                // Cosmic strips the newlines but accesskit needs them so we append them back in if line originally ended with a newline
                // If the last glyph position is equal to the end of the buffer line then this layout run is the last one and ends in a newline.
                if last_glyph_pos == line.text.len() {
                    line_text += "\n";
                    character_lengths.push(1);
                    character_positions.push(line.line_w);
                    character_widths.push(0.0);
                }

                // TODO: Might need to append any spaces that were stripped during layout. This can be done by
                // figuring out if the start of the next line is greater than the end of the current line as long
                // as the lines have the same `line_i`. This will require a peekable iterator loop.

                line_node.set_value(line_text.into_boxed_str());
                line_node.set_character_lengths(character_lengths.into_boxed_slice());
                line_node.set_character_positions(character_positions.into_boxed_slice());
                line_node.set_character_widths(character_widths.into_boxed_slice());
                line_node.set_word_lengths(word_lengths.into_boxed_slice());

                if line.line_i != prev_line_index {
                    current_cursor = 0;
                }

                if line.line_i == cursor.line {
                    if prev_line_index != line.line_i {
                        if cursor.index <= line_length {
                            selection_active_line = line_node.node_id();
                            selection_active_cursor = cursor.index;
                        }
                    } else if cursor.index > current_cursor {
                        selection_active_line = line_node.node_id();
                        selection_active_cursor = cursor.index - current_cursor;
                    }
                }

                // Check if the current line contains the cursor or selection
                // This is a mess because a line happens due to soft and hard breaks but
                // the cursor and selected indices are relative to the lines caused by hard breaks only.
                if line.line_i == selection.line {
                    // A previous line index different to the current means that the current line follows a hard break
                    if prev_line_index != line.line_i {
                        if selection.index <= line_length {
                            selection_anchor_line = line_node.node_id();
                            selection_anchor_cursor = selection.index;
                        }
                    } else if selection.index > current_cursor {
                        selection_anchor_line = line_node.node_id();
                        selection_anchor_cursor = selection.index - current_cursor;
                    }
                }

                node.add_child(line_node);

                current_cursor += line_length;
                prev_line_index = line.line_i;
            }

            node.set_text_selection(TextSelection {
                anchor: TextPosition {
                    node: selection_anchor_line,
                    character_index: selection_anchor_cursor,
                },
                focus: TextPosition {
                    node: selection_active_line,
                    character_index: selection_active_cursor,
                },
            });

            match self.kind {
                TextboxKind::MultiLineUnwrapped | TextboxKind::MultiLineWrapped => {
                    node.node_builder.set_multiline();
                }

                _ => {
                    node.node_builder.clear_multiline();
                }
            }

            node.node_builder.set_default_action_verb(DefaultActionVerb::Focus);
        });
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
                        cx.focus_with_visibility(false);
                        cx.capture();
                        cx.set_checked(true);
                        cx.lock_cursor_icon();

                        if !self.edit {
                            cx.emit(TextEvent::StartEdit);
                        }
                        self.reset_caret_timer(cx);
                        cx.emit(TextEvent::Hit(cx.mouse.cursorx, cx.mouse.cursory));
                    }
                } else {
                    cx.emit(TextEvent::Submit(false));
                    cx.release();
                    cx.set_checked(false);

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

            WindowEvent::MouseMove(_, _) => {
                if cx.mouse.left.state == MouseButtonState::Pressed
                    && cx.mouse.left.pressed == cx.current
                {
                    if self.edit {
                        self.reset_caret_timer(cx);
                    }
                    cx.emit(TextEvent::Drag(cx.mouse.cursorx, cx.mouse.cursory));
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
                    !cx.modifiers.contains(Modifiers::CTRL) &&
                    !cx.modifiers.contains(Modifiers::LOGO) &&
                    self.edit &&
                    !cx.is_read_only()
                {
                    self.reset_caret_timer(cx);
                    cx.emit(TextEvent::InsertText(String::from(*c)));
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

                Code::ArrowLeft => {
                    self.reset_caret_timer(cx);
                    let movement = if cx.modifiers.contains(Modifiers::CTRL) {
                        Movement::Word(Direction::Left)
                    } else {
                        Movement::Grapheme(Direction::Left)
                    };

                    cx.emit(TextEvent::MoveCursor(
                        movement,
                        cx.modifiers.contains(Modifiers::SHIFT),
                    ));
                }

                Code::ArrowRight => {
                    self.reset_caret_timer(cx);
                    let movement = if cx.modifiers.contains(Modifiers::CTRL) {
                        Movement::Word(Direction::Right)
                    } else {
                        Movement::Grapheme(Direction::Right)
                    };

                    cx.emit(TextEvent::MoveCursor(
                        movement,
                        cx.modifiers.contains(Modifiers::SHIFT),
                    ));
                }

                Code::ArrowUp => {
                    self.reset_caret_timer(cx);
                    if self.kind != TextboxKind::SingleLine {
                        cx.emit(TextEvent::MoveCursor(
                            Movement::Line(Direction::Upstream),
                            cx.modifiers.contains(Modifiers::SHIFT),
                        ));
                    }
                }

                Code::ArrowDown => {
                    self.reset_caret_timer(cx);
                    if self.kind != TextboxKind::SingleLine {
                        cx.emit(TextEvent::MoveCursor(
                            Movement::Line(Direction::Downstream),
                            cx.modifiers.contains(Modifiers::SHIFT),
                        ));
                    }
                }

                Code::Backspace => {
                    self.reset_caret_timer(cx);
                    if !cx.is_read_only() {
                        if cx.modifiers.contains(Modifiers::CTRL) {
                            cx.emit(TextEvent::DeleteText(Movement::Word(Direction::Upstream)));
                        } else {
                            cx.emit(TextEvent::DeleteText(Movement::Grapheme(Direction::Upstream)));
                        }
                    }
                }

                Code::Delete => {
                    self.reset_caret_timer(cx);
                    if !cx.is_read_only() {
                        if cx.modifiers.contains(Modifiers::CTRL) {
                            cx.emit(TextEvent::DeleteText(Movement::Word(Direction::Downstream)));
                        } else {
                            cx.emit(TextEvent::DeleteText(Movement::Grapheme(
                                Direction::Downstream,
                            )));
                        }
                    }
                }

                Code::Escape => {
                    cx.emit(TextEvent::EndEdit);
                    cx.set_checked(false);
                    if let Some(callback) = &self.on_cancel {
                        (callback)(cx);
                    }
                }

                Code::Home => {
                    self.reset_caret_timer(cx);
                    cx.emit(TextEvent::MoveCursor(
                        Movement::LineStart,
                        cx.modifiers.contains(Modifiers::SHIFT),
                    ));
                }

                Code::End => {
                    self.reset_caret_timer(cx);
                    cx.emit(TextEvent::MoveCursor(
                        Movement::LineEnd,
                        cx.modifiers.contains(Modifiers::SHIFT),
                    ));
                }

                Code::PageUp | Code::PageDown => {
                    self.reset_caret_timer(cx);
                    let direction = if *code == Code::PageUp {
                        Direction::Upstream
                    } else {
                        Direction::Downstream
                    };
                    cx.emit(TextEvent::MoveCursor(
                        if cx.modifiers.contains(Modifiers::CTRL) {
                            Movement::Body(direction)
                        } else {
                            Movement::Page(direction)
                        },
                        cx.modifiers.contains(Modifiers::SHIFT),
                    ));
                }

                Code::KeyA => {
                    #[cfg(target_os = "macos")]
                    let modifier = Modifiers::LOGO;
                    #[cfg(not(target_os = "macos"))]
                    let modifier = Modifiers::CTRL;

                    if cx.modifiers == &modifier {
                        cx.emit(TextEvent::SelectAll);
                    }
                }

                Code::KeyC => {
                    #[cfg(target_os = "macos")]
                    let modifier = Modifiers::LOGO;
                    #[cfg(not(target_os = "macos"))]
                    let modifier = Modifiers::CTRL;

                    if cx.modifiers == &modifier {
                        cx.emit(TextEvent::Copy);
                    }
                }

                Code::KeyV => {
                    #[cfg(target_os = "macos")]
                    let modifier = Modifiers::LOGO;
                    #[cfg(not(target_os = "macos"))]
                    let modifier = Modifiers::CTRL;

                    if cx.modifiers == &modifier {
                        cx.emit(TextEvent::Paste);
                    }
                }

                Code::KeyX => {
                    #[cfg(target_os = "macos")]
                    let modifier = Modifiers::LOGO;
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
                target: _,
                data: Some(ActionData::SetTextSelection(selection)),
            }) => {
                // TODO: This needs testing once I figure out how to trigger it with a screen reader.
                let node_id = cx.current.accesskit_id();
                cx.text_context.with_editor(cx.current, |_, editor| {
                    // let cursor_node = selection.focus.node;
                    let selection_node = selection.anchor.node;

                    // let mut cursor_line_index = 0;
                    // let mut cursor_index = 0;
                    let mut selection_line_index = 0;
                    let mut selection_index = 0;

                    let mut current_cursor = 0;
                    let mut prev_line_index = std::usize::MAX;

                    for (index, line) in editor.buffer().layout_runs().enumerate() {
                        let line_node = AccessNode::new_from_parent(node_id, index);
                        // if line_node.node_id() == cursor_node {
                        //     cursor_line_index = line.line_i;
                        //     cursor_index = selection.focus.character_index + current_cursor;
                        // }

                        if line_node.node_id() == selection_node {
                            selection_line_index = line.line_i;
                            selection_index = selection.anchor.character_index + current_cursor;
                        }

                        if line.line_i != prev_line_index {
                            current_cursor = 0;
                        }

                        let first_glyph_pos =
                            line.glyphs.first().map(|glyph| glyph.start).unwrap_or_default();
                        let last_glyph_pos =
                            line.glyphs.last().map(|glyph| glyph.end).unwrap_or_default();

                        let line_length = last_glyph_pos - first_glyph_pos;

                        current_cursor += line_length;
                        prev_line_index = line.line_i;
                    }

                    let selection_cursor = Cursor::new(selection_line_index, selection_index);
                    editor.set_select_opt(Some(selection_cursor));

                    // TODO: Either add a method to set the cursor by index to cosmic,
                    // or loop over an `Action` to move the cursor to the correct place.
                });

                // println!("Select some text: {:?}", selection);
            }

            _ => {}
        });

        // Textbox Events
        event.map(|text_event, _| match text_event {
            TextEvent::InsertText(text) => {
                self.insert_text(cx, text);
                self.set_caret(cx);

                let text = self.clone_text(cx);

                if let Ok(value) = &text.parse::<L::Target>() {
                    if let Some(validate) = &self.validate {
                        cx.set_valid(validate(value));
                    } else {
                        cx.set_valid(true);
                    }
                } else {
                    cx.set_valid(false);
                }

                self.placeholder_shown = text.is_empty();

                if self.edit {
                    if let Some(callback) = &self.on_edit {
                        (callback)(cx, text);
                    }
                }
            }

            TextEvent::Clear => {
                self.reset_text(cx);
                self.scroll(cx, 0.0, 0.0); // ensure_visible
                cx.needs_relayout();
                cx.needs_redraw();
            }

            TextEvent::DeleteText(movement) => {
                if self.edit {
                    self.delete_text(cx, *movement);
                    self.set_caret(cx);

                    let text = self.clone_text(cx);

                    if let Ok(value) = &text.parse::<L::Target>() {
                        if let Some(validate) = &self.validate {
                            cx.set_valid(validate(value));
                        } else {
                            cx.set_valid(true);
                        }
                    } else {
                        cx.set_valid(false);
                    }

                    self.placeholder_shown = text.is_empty();

                    if let Some(callback) = &self.on_edit {
                        (callback)(cx, text);
                    }
                }
            }

            TextEvent::MoveCursor(movement, selection) => {
                if self.edit {
                    self.move_cursor(cx, *movement, *selection);
                    self.set_caret(cx);
                }
            }

            TextEvent::SetPlaceholder(text) => {
                self.placeholder = text.clone();
            }

            TextEvent::StartEdit => {
                if !cx.is_disabled() && !self.edit {
                    self.edit = true;
                    cx.focus_with_visibility(false);
                    cx.capture();
                    cx.set_checked(true);
                    self.reset_caret_timer(cx);

                    if let Some(source) = cx.data::<L::Source>() {
                        let text = self.lens.view(source, |t| {
                            if let Some(t) = t {
                                t.to_string()
                            } else {
                                String::from("")
                            }
                        });

                        self.placeholder_shown = text.is_empty();

                        self.select_all(cx);
                        self.insert_text(cx, &text);
                        self.set_caret(cx);

                        if let Ok(value) = &text.parse::<L::Target>() {
                            if let Some(validate) = &self.validate {
                                cx.set_valid(validate(value));
                            } else {
                                cx.set_valid(true);
                            }
                        } else {
                            cx.set_valid(false);
                        }
                    };
                }
            }

            TextEvent::EndEdit => {
                self.deselect(cx);
                self.edit = false;
                cx.set_checked(false);
                cx.release();
                cx.stop_timer(self.caret_timer);

                if let Some(source) = cx.data::<L::Source>() {
                    let text = self.lens.view(source, |t| {
                        if let Some(t) = t {
                            t.to_string()
                        } else {
                            "".to_owned()
                        }
                    });

                    self.placeholder_shown = text.is_empty();

                    self.select_all(cx);
                    self.insert_text(cx, &text);
                    self.set_caret(cx);

                    if let Ok(value) = &text.parse::<L::Target>() {
                        if let Some(validate) = &self.validate {
                            cx.set_valid(validate(value));
                        } else {
                            cx.set_valid(true);
                        }
                    } else {
                        cx.set_valid(false);
                    }
                };
            }

            TextEvent::Blur => {
                cx.set_checked(false);
                if let Some(callback) = &self.on_blur {
                    (callback)(cx);
                } else {
                    cx.emit(TextEvent::Submit(false));
                    cx.emit(TextEvent::EndEdit);
                }
            }

            TextEvent::Submit(reason) => {
                if let Some(callback) = &self.on_submit {
                    if cx.is_valid() {
                        let text = self.clone_text(cx);
                        if let Ok(value) = text.parse::<L::Target>() {
                            (callback)(cx, value, *reason);
                        }
                    }
                }
            }

            TextEvent::SelectAll => {
                self.select_all(cx);
                self.set_caret(cx);
            }

            TextEvent::SelectWord => {
                self.select_word(cx);
                self.set_caret(cx);
            }

            TextEvent::SelectParagraph => {
                self.select_paragraph(cx);
                self.set_caret(cx);
            }

            TextEvent::Hit(posx, posy) => {
                self.hit(cx, *posx, *posy);
                self.set_caret(cx);
            }

            TextEvent::Drag(posx, posy) => {
                self.drag(cx, *posx, *posy);
                self.set_caret(cx);
            }

            TextEvent::Scroll(x, y) => {
                self.scroll(cx, *x, *y);
            }

            TextEvent::Copy =>
            {
                #[cfg(feature = "clipboard")]
                if self.edit {
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
                if self.edit {
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
                            cx.set_clipboard(selected_text)
                                .expect("Failed to add text to clipboard");
                            self.delete_text(cx, Movement::Grapheme(Direction::Upstream));

                            let text = self.clone_text(cx);

                            if let Ok(value) = &text.parse::<L::Target>() {
                                if let Some(validate) = &self.validate {
                                    cx.set_valid(validate(value));
                                } else {
                                    cx.set_valid(true);
                                }
                            } else {
                                cx.set_valid(false);
                            }

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
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let mut path = cx.build_path();
        cx.draw_shadows(canvas, &mut path);
        cx.draw_backdrop_filter(canvas, &mut path);
        cx.draw_background(canvas, &mut path);
        cx.draw_border(canvas, &mut path);
        cx.draw_inset_box_shadows(canvas, &mut path);
        cx.draw_outline(canvas);
        canvas.save();
        canvas.translate(self.transform.0, self.transform.1);
        cx.draw_text_and_selection(canvas);
        canvas.restore();
    }
}
