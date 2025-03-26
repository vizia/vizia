// use crate::accessibility::IntoNode;
use crate::prelude::*;

use crate::text::{
    apply_movement, offset_for_delete_backwards, Direction, EditableText, Movement, Selection,
    VerticalMovement,
};
// use crate::views::scrollview::SCROLL_SENSITIVITY;
use accesskit::{ActionData, ActionRequest};
use skia_safe::textlayout::{RectHeightStyle, RectWidthStyle};
use skia_safe::{Paint, PaintStyle, Rect};
use unicode_segmentation::UnicodeSegmentation;

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
    /// Toggle the visibility of the text Caret.
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
    show_placeholder: bool,
    show_caret: bool,
    caret_timer: Timer,
    selection: Selection,
}

// Determines whether the enter key submits the text or inserts a new line.
#[derive(Copy, Clone, PartialEq, Eq)]
enum TextboxKind {
    SingleLine,
    MultiLineUnwrapped,
    MultiLineWrapped,
}

impl<L> Textbox<L>
where
    L: Lens<Target: Data + Clone + ToStringLocalized + std::str::FromStr>,
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
            show_placeholder: true,
            show_caret: true,
            caret_timer,
            selection: Selection::new(0, 0),
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
        })
        .toggle_class("multiline", kind == TextboxKind::MultiLineWrapped)
        .text_wrap(kind == TextboxKind::MultiLineWrapped)
        .navigable(true)
        .role(Role::TextInput)
        .text_value(lens)
        .toggle_class("caret", Self::show_caret)
        .text(lens)
        .placeholder_shown(Self::show_placeholder)
        .bind(lens, |handle, lens| {
            let flag = lens.get(&handle).to_string_local(handle.cx).is_empty();
            handle.modify(|textbox| textbox.show_placeholder = flag).bind(
                Self::placeholder,
                move |handle, placeholder| {
                    let value = placeholder.get(&handle).to_string_local(handle.cx);
                    if flag {
                        handle.text(value);
                    }
                },
            );
        })
    }

    fn insert_text(&mut self, cx: &mut EventContext, txt: &str) {
        if let Some(text) = cx.style.text.get_mut(cx.current) {
            if self.show_placeholder && !txt.is_empty() {
                text.clear();
                self.show_placeholder = false;
            }
            text.edit(self.selection.range(), txt);
            self.selection = Selection::caret(self.selection.min() + txt.len());
            self.show_placeholder = text.is_empty();
            cx.style.needs_text_update(cx.current);
        }
    }

    fn delete_text(&mut self, cx: &mut EventContext, movement: Movement) {
        if self.selection.is_caret() {
            if movement == Movement::Grapheme(Direction::Upstream) {
                if self.selection.active == 0 {
                    return;
                }
                if let Some(text) = cx.style.text.get_mut(cx.current) {
                    let del_offset = offset_for_delete_backwards(&self.selection, text);
                    let del_range = del_offset..self.selection.active;

                    self.selection = Selection::caret(del_range.start);

                    text.edit(del_range, "");

                    cx.style.needs_text_update(cx.current);
                }
            } else if let Some(text) = cx.style.text.get_mut(cx.current) {
                if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
                    let to_delete = apply_movement(movement, self.selection, text, paragraph, true);
                    self.selection = to_delete;
                    let new_cursor_pos = self.selection.min();
                    text.edit(to_delete.range(), "");
                    self.selection = Selection::caret(new_cursor_pos);
                    cx.style.needs_text_update(cx.current);
                }
            }
        } else if let Some(text) = cx.style.text.get_mut(cx.current) {
            let del_range = self.selection.range();
            self.selection = Selection::caret(del_range.start);

            text.edit(del_range, "");

            cx.style.needs_text_update(cx.current);
        }

        if let Some(text) = cx.style.text.get_mut(cx.current) {
            self.show_placeholder = text.is_empty();
            if self.show_placeholder {
                *text = self.placeholder.clone();
                self.selection = Selection::caret(0);
            }
        }
    }

    fn reset_text(&mut self, cx: &mut EventContext) {
        if let Some(text) = cx.style.text.get_mut(cx.current) {
            text.clear();
            self.selection = Selection::caret(0);
            self.show_placeholder = true;
            *text = self.placeholder.clone();
            cx.style.needs_text_update(cx.current);
        }
    }

    fn move_cursor(&mut self, cx: &mut EventContext, movement: Movement, selection: bool) {
        if let Some(text) = cx.style.text.get_mut(cx.current) {
            if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
                let new_selection =
                    apply_movement(movement, self.selection, text, paragraph, selection);
                self.selection = new_selection;
                cx.needs_redraw();
            }
        }
    }

    fn select_all(&mut self, cx: &mut EventContext) {
        if let Some(text) = cx.style.text.get(cx.current) {
            self.selection.anchor = 0;
            self.selection.active = text.len();
            cx.needs_redraw();
        }
    }

    fn select_word(&mut self, cx: &mut EventContext) {
        self.move_cursor(cx, Movement::Word(Direction::Upstream), false);
        self.move_cursor(cx, Movement::Word(Direction::Downstream), true);
    }

    fn select_paragraph(&mut self, cx: &mut EventContext) {
        self.move_cursor(cx, Movement::ParagraphStart, false);
        self.move_cursor(cx, Movement::ParagraphEnd, true);
    }

    fn deselect(&mut self) {
        self.selection = Selection::caret(self.selection.active);
    }

    /// These input coordinates should be physical coordinates, i.e. what the mouse events provide.
    /// The output text coordinates will also be physical, but relative to the top of the text
    /// glyphs, appropriate for passage to cosmic.
    fn coordinates_global_to_text(&self, cx: &EventContext, x: f32, y: f32) -> (f32, f32) {
        let bounds = cx.bounds();

        if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
            let padding_left = cx.style.padding_left.get(cx.current).copied().unwrap_or_default();
            let padding_top = cx.style.padding_top.get(cx.current).copied().unwrap_or_default();
            let _padding_right =
                cx.style.padding_right.get(cx.current).copied().unwrap_or_default();
            let padding_bottom =
                cx.style.padding_bottom.get(cx.current).copied().unwrap_or_default();

            let logical_parent_width = cx.physical_to_logical(bounds.w);
            let logical_parent_height = cx.physical_to_logical(bounds.h);

            let padding_left = padding_left.to_px(logical_parent_width, 0.0) * cx.scale_factor();
            let padding_top = padding_top.to_px(logical_parent_height, 0.0) * cx.scale_factor();
            let padding_bottom =
                padding_bottom.to_px(logical_parent_height, 0.0) * cx.scale_factor();

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

            top *= bounds.height() - padding_top - padding_bottom - paragraph.height();

            // let total_height = cx.text_context.with_buffer(cx.current, |_, buffer| {
            //     buffer.layout_runs().len() as f32 * buffer.metrics().line_height
            // });

            // let x = x - bounds.x - self.transform.0 - padding_left;
            // let y = y - self.transform.1 - bounds.y - (bounds.h - total_height) * justify_y - padding_top;

            let x = x - bounds.x - padding_left;
            let y = y - bounds.y - padding_top - top;

            (x, y)
        } else {
            (x, y)
        }
    }

    /// This function takes window-global physical coordinates.
    fn hit(&mut self, cx: &mut EventContext, x: f32, y: f32, selection: bool) {
        if let Some(text) = cx.style.text.get(cx.current) {
            if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
                let gp = paragraph
                    .get_glyph_position_at_coordinate(self.coordinates_global_to_text(cx, x, y));
                let num_graphemes = text.graphemes(true).count();
                let pos = (gp.position as usize).min(num_graphemes);
                let mut cursor = text.len();
                for (i, (j, _)) in text.grapheme_indices(true).enumerate() {
                    if pos == i {
                        cursor = j;
                        break;
                    }
                }

                if selection {
                    self.selection.active = cursor;
                } else {
                    self.selection = Selection::caret(cursor);
                }

                cx.needs_redraw();
            }
        }
    }

    /// This function takes window-global physical coordinates.
    fn drag(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        if let Some(text) = cx.style.text.get(cx.current) {
            if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
                let gp = paragraph
                    .get_glyph_position_at_coordinate(self.coordinates_global_to_text(cx, x, y));
                let num_graphemes = text.graphemes(true).count();
                let pos = (gp.position as usize).min(num_graphemes);

                let mut cursor = text.len();
                for (i, (j, _)) in text.grapheme_indices(true).enumerate() {
                    if pos == i {
                        cursor = j;
                        break;
                    }
                }

                self.selection.active = cursor;

                cx.needs_redraw();
            }
        }
    }

    // /// This function takes window-global physical dimensions.
    // fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32) {}

    #[cfg(feature = "clipboard")]
    fn clone_selected(&self, cx: &mut EventContext) -> Option<String> {
        if let Some(text) = cx.style.text.get(cx.current) {
            let substring = &text[self.selection.range()];
            return Some(substring.to_string());
        }

        None
    }

    fn clone_text(&self, cx: &mut EventContext) -> String {
        if self.show_placeholder {
            return String::new();
        }

        if let Some(text) = cx.style.text.get(cx.current) {
            text.clone()
        } else {
            String::new()
        }
    }

    fn reset_caret_timer(&mut self, cx: &mut EventContext) {
        cx.stop_timer(self.caret_timer);
        if !cx.is_read_only() {
            self.show_caret = true;
            cx.start_timer(self.caret_timer);
        }
    }

    fn draw_selection(&self, cx: &mut DrawContext, canvas: &Canvas) {
        if !self.selection.is_caret() {
            if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
                if let Some(text) = cx.style.text.get(cx.current) {
                    let min = text.current_grapheme_offset(self.selection.min());
                    let max = text.current_grapheme_offset(self.selection.max());

                    let cursor_rects = paragraph.get_rects_for_range(
                        min..max,
                        RectHeightStyle::Tight,
                        RectWidthStyle::Tight,
                    );

                    for cursor_rect in cursor_rects {
                        let bounds = cx.bounds();

                        let alignment = cx.alignment();

                        let (mut top, left) = match alignment {
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

                        top *= bounds.height() - padding_top - padding_bottom - paragraph.height();

                        let padding_left = match cx.padding_left() {
                            Units::Pixels(val) => val,
                            _ => 0.0,
                        };

                        let x = bounds.x + padding_left + cursor_rect.rect.left + left;
                        let y = bounds.y + padding_top + cursor_rect.rect.top + top;

                        let x2 = x + (cursor_rect.rect.right - cursor_rect.rect.left);
                        let y2 = y + (cursor_rect.rect.bottom - cursor_rect.rect.top);

                        let mut paint = Paint::default();
                        paint.set_anti_alias(true);
                        paint.set_style(PaintStyle::Fill);
                        paint.set_color(cx.selection_color());

                        canvas.draw_rect(Rect::new(x, y, x2, y2), &paint);
                    }
                }
            }
        }
    }

    /// Draw text caret for the current view.
    pub fn draw_text_caret(&self, cx: &mut DrawContext, canvas: &Canvas) {
        if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
            if let Some(text) = cx.style.text.get(cx.current) {
                let bounds = cx.bounds();

                let current = text.current_grapheme_offset(self.selection.active);

                let rects = paragraph.get_rects_for_range(
                    current..current + 1,
                    RectHeightStyle::Tight,
                    RectWidthStyle::Tight,
                );

                let cursor_rect = rects.first().unwrap();

                let alignment = cx.alignment();

                let (mut top, _) = match alignment {
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

                top *= bounds.height() - padding_top - padding_bottom - paragraph.height();

                let padding_left = match cx.padding_left() {
                    Units::Pixels(val) => val,
                    _ => 0.0,
                };

                let x = (bounds.x + padding_left + cursor_rect.rect.left).round();
                let y = (bounds.y + padding_top + cursor_rect.rect.top + top).round();

                let x2 = x + 1.0;
                let y2 = y + (cursor_rect.rect.bottom - cursor_rect.rect.top);

                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                paint.set_style(PaintStyle::Fill);
                paint.set_color(cx.caret_color());

                canvas.draw_rect(Rect::new(x, y, x2, y2), &paint);
            }
        }
    }
}

impl<L: Lens> Handle<'_, Textbox<L>> {
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
    pub fn placeholder<P: ToStringLocalized>(self, text: impl Res<P>) -> Self {
        text.set_or_bind(self.cx, self.entity, move |cx, val| {
            let txt = val.get(cx).to_string_local(cx);
            cx.emit(TextEvent::SetPlaceholder(txt.clone()));
            cx.style.name.insert(cx.current, txt);
            cx.needs_relayout();
            cx.needs_redraw(self.entity);
        });

        self
    }
}

impl<L> View for Textbox<L>
where
    L: Lens<Target: Data + ToStringLocalized + std::str::FromStr>,
{
    fn element(&self) -> Option<&'static str> {
        Some("textbox")
    }

    fn accessibility(&self, cx: &mut AccessContext, node: &mut AccessNode) {
        let _bounds = cx.bounds();

        let node_id = node.node_id();

        let mut _selection = self.selection;

        // let mut selection_active_line = node_id;
        // let mut selection_anchor_line = node_id;
        // let mut selection_active_cursor = 0;
        // let mut selection_anchor_cursor = 0;

        let mut _current_cursor = 0;
        let mut _prev_line_index = usize::MAX;

        if let Some(_text) = cx.style.text.get(cx.current) {
            if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
                let line_metrics = paragraph.get_line_metrics();
                for line in line_metrics.iter() {
                    // We need a child node per line
                    let mut line_node = AccessNode::new_from_parent(node_id, line.line_number);
                    line_node.set_role(Role::TextInput);
                    line_node.set_bounds(BoundingBox {
                        x: line.left as f32,
                        y: (line.baseline - line.ascent) as f32,
                        w: line.width as f32,
                        h: line.height as f32,
                    });
                    // line_node.set_text_direction(if line.ltr {
                    //     TextDirection::RightToLeft
                    // } else {
                    //     TextDirection::LeftToRight
                    // });

                    let mut character_lengths = Vec::new();
                    let mut character_positions = Vec::new();
                    let mut character_widths = Vec::new();

                    // let mut line_text = text[line.start_index..line.end_index].to_owned();

                    // let word_lengths =
                    //     line_text.unicode_words().map(|word| word.len() as u8).collect::<Vec<_>>();

                    // let mut line_length = 0;

                    let mut glyph_pos = line.start_index;

                    for _ in line.start_index..line.end_index {
                        if let Some(cluster_info) = paragraph.get_glyph_cluster_at(glyph_pos) {
                            let length =
                                cluster_info.text_range.end - cluster_info.text_range.start;

                            // line_length += length as usize;

                            let position = cluster_info.bounds.left();
                            let width = cluster_info.bounds.width();

                            character_lengths.push(length as u8);
                            character_positions.push(position);
                            character_widths.push(width);

                            glyph_pos += length;

                            // if glyph_pos >= line.end_index {
                            //     break;
                            // }
                        }
                    }

                    // Cosmic strips the newlines but accesskit needs them so we append them back in if line originally ended with a newline
                    // If the last glyph position is equal to the end of the buffer line then this layout run is the last one and ends in a newline.
                    // if line.hard_break {
                    //     line_text += "\n";
                    //     character_lengths.push(1);
                    //     character_positions.push(line.width as f32);
                    //     character_widths.push(0.0);
                    // }

                    // TODO: Might need to append any spaces that were stripped during layout. This can be done by
                    // figuring out if the start of the next line is greater than the end of the current line as long
                    // as the lines have the same `line_i`. This will require a peekable iterator loop.

                    // line_node.set_value(line_text.into_boxed_str());
                    line_node.set_character_lengths(character_lengths.into_boxed_slice());
                    line_node.set_character_positions(character_positions.into_boxed_slice());
                    line_node.set_character_widths(character_widths.into_boxed_slice());
                    // line_node.set_word_lengths(word_lengths.into_boxed_slice());

                    // if line.line_i != prev_line_index {
                    //     current_cursor = 0;
                    // }

                    // if line.line_i == cursor.line {
                    //     if prev_line_index != line.line_i {
                    //         if cursor.index <= line_length {
                    //             selection_active_line = line_node.node_id();
                    //             selection_active_cursor = cursor.index;
                    //         }
                    //     } else if cursor.index > current_cursor {
                    //         selection_active_line = line_node.node_id();
                    //         selection_active_cursor = cursor.index - current_cursor;
                    //     }
                    // }

                    // Check if the current line contains the cursor or selection
                    // This is a mess because a line happens due to soft and hard breaks but
                    // the cursor and selected indices are relative to the lines caused by hard breaks only.
                    // if selection == Selection::None {
                    //     selection = Selection::Normal(cursor);
                    // }
                    // if let Selection::Normal(selection) = selection {
                    //     if line.line_i == selection.line {
                    //         // A previous line index different to the current means that the current line follows a hard break
                    //         if prev_line_index != line.line_i {
                    //             if selection.index <= line_length {
                    //                 selection_anchor_line = line_node.node_id();
                    //                 selection_anchor_cursor = selection.index;
                    //             }
                    //         } else if selection.index > current_cursor {
                    //             selection_anchor_line = line_node.node_id();
                    //             selection_anchor_cursor = selection.index - current_cursor;
                    //         }
                    //     }
                    // }

                    node.add_child(line_node);

                    // current_cursor += line_length;
                    // prev_line_index = line.line_i;
                }
            }
        }

        // node.set_text_selection(TextSelection {
        //     anchor: TextPosition {
        //         node: selection_anchor_line,
        //         character_index: selection_anchor_cursor,
        //     },
        //     focus: TextPosition {
        //         node: selection_active_line,
        //         character_index: selection_active_cursor,
        //     },
        // });
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
                        cx.emit(TextEvent::Hit(
                            cx.mouse.cursor_x,
                            cx.mouse.cursor_y,
                            cx.modifiers.shift(),
                        ));
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

            WindowEvent::MouseMove(x, y) => {
                if cx.mouse.left.state == MouseButtonState::Pressed
                    && cx.mouse.left.pressed == cx.current
                {
                    if self.edit {
                        self.reset_caret_timer(cx);
                    }
                    if cx.mouse.left.pos_down.0 != *x || cx.mouse.left.pos_down.1 != *y {
                        cx.emit(TextEvent::Drag(cx.mouse.cursor_x, cx.mouse.cursor_y));
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
                    println!("Textbox got IME Commit: {}", text);
                    self.reset_caret_timer(cx);
                    cx.emit(TextEvent::InsertText(text.to_string()));
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

                Code::Space => {
                    cx.emit(TextEvent::InsertText(String::from(" ")));
                }

                Code::ArrowLeft => {
                    self.reset_caret_timer(cx);
                    let movement = if cx.modifiers.ctrl() {
                        Movement::Word(Direction::Left)
                    } else {
                        Movement::Grapheme(Direction::Left)
                    };

                    cx.emit(TextEvent::MoveCursor(movement, cx.modifiers.shift()));
                }

                Code::ArrowRight => {
                    self.reset_caret_timer(cx);

                    let movement = if cx.modifiers.ctrl() {
                        Movement::Word(Direction::Right)
                    } else {
                        Movement::Grapheme(Direction::Right)
                    };

                    cx.emit(TextEvent::MoveCursor(movement, cx.modifiers.shift()));
                }

                Code::ArrowUp => {
                    self.reset_caret_timer(cx);
                    if self.kind != TextboxKind::SingleLine {
                        cx.emit(TextEvent::MoveCursor(
                            Movement::Vertical(VerticalMovement::LineUp),
                            cx.modifiers.shift(),
                        ));
                    }
                }

                Code::ArrowDown => {
                    self.reset_caret_timer(cx);
                    if self.kind != TextboxKind::SingleLine {
                        cx.emit(TextEvent::MoveCursor(
                            Movement::Vertical(VerticalMovement::LineDown),
                            cx.modifiers.shift(),
                        ));
                    }
                }

                Code::Backspace => {
                    self.reset_caret_timer(cx);
                    if !cx.is_read_only() {
                        if cx.modifiers.ctrl() {
                            cx.emit(TextEvent::DeleteText(Movement::Word(Direction::Upstream)));
                        } else {
                            cx.emit(TextEvent::DeleteText(Movement::Grapheme(Direction::Upstream)));
                        }
                    }
                }

                Code::Delete => {
                    self.reset_caret_timer(cx);
                    if !cx.is_read_only() {
                        if cx.modifiers.ctrl() {
                            cx.emit(TextEvent::DeleteText(Movement::Word(Direction::Downstream)));
                        } else {
                            cx.emit(TextEvent::DeleteText(Movement::Grapheme(
                                Direction::Downstream,
                            )));
                        }
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
                    cx.emit(TextEvent::MoveCursor(Movement::LineStart, cx.modifiers.shift()));
                }

                Code::End => {
                    self.reset_caret_timer(cx);
                    cx.emit(TextEvent::MoveCursor(Movement::LineEnd, cx.modifiers.shift()));
                }

                Code::PageUp | Code::PageDown => {
                    self.reset_caret_timer(cx);
                    let direction = if *code == Code::PageUp {
                        Direction::Upstream
                    } else {
                        Direction::Downstream
                    };
                    cx.emit(TextEvent::MoveCursor(
                        if cx.modifiers.ctrl() {
                            Movement::Body(direction)
                        } else {
                            Movement::Page(direction)
                        },
                        cx.modifiers.shift(),
                    ));
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

                    if cx.modifiers == &modifier {
                        cx.emit(TextEvent::Copy);
                    }
                }

                Code::KeyV => {
                    #[cfg(target_os = "macos")]
                    let modifier = Modifiers::SUPER;
                    #[cfg(not(target_os = "macos"))]
                    let modifier = Modifiers::CTRL;

                    if cx.modifiers == &modifier {
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
                target: _,
                data: Some(ActionData::SetTextSelection(_selection)),
            }) => {
                // TODO: This needs testing once I figure out how to trigger it with a screen reader.
                // let node_id = cx.current.accesskit_id();
                // cx.text_context.with_editor(cx.current, |_, editor| {
                //     // let cursor_node = selection.focus.node;
                //     let selection_node = selection.anchor.node;

                //     // let mut cursor_line_index = 0;
                //     // let mut cursor_index = 0;
                //     let mut selection_line_index = 0;
                //     let mut selection_index = 0;

                //     let mut current_cursor = 0;
                //     let mut prev_line_index = usize::MAX;

                //     for (index, line) in editor.buffer().layout_runs().enumerate() {
                //         let line_node = AccessNode::new_from_parent(node_id, index);
                //         // if line_node.node_id() == cursor_node {
                //         //     cursor_line_index = line.line_i;
                //         //     cursor_index = selection.focus.character_index + current_cursor;
                //         // }

                //         if line_node.node_id() == selection_node {
                //             selection_line_index = line.line_i;
                //             selection_index = selection.anchor.character_index + current_cursor;
                //         }

                //         if line.line_i != prev_line_index {
                //             current_cursor = 0;
                //         }

                //         let first_glyph_pos =
                //             line.glyphs.first().map(|glyph| glyph.start).unwrap_or_default();
                //         let last_glyph_pos =
                //             line.glyphs.last().map(|glyph| glyph.end).unwrap_or_default();

                //         let line_length = last_glyph_pos - first_glyph_pos;

                //         current_cursor += line_length;
                //         prev_line_index = line.line_i;
                //     }

                //     let selection_cursor = Cursor::new(selection_line_index, selection_index);
                //     editor.set_selection(Selection::Normal(selection_cursor));

                //     // TODO: Either add a method to set the cursor by index to cosmic,
                //     // or loop over an `Action` to move the cursor to the correct place.
                // });
            }

            _ => {}
        });

        // Textbox Events
        event.map(|text_event, _| match text_event {
            TextEvent::InsertText(text) => {
                if self.show_placeholder {
                    self.reset_text(cx);
                }

                self.insert_text(cx, text);

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

                if self.edit {
                    if let Some(callback) = &self.on_edit {
                        (callback)(cx, text);
                    }
                }
            }

            TextEvent::Clear => {
                self.reset_text(cx);
                // self.scroll(cx, 0.0, 0.0); // ensure_visible
                cx.needs_relayout();
                cx.needs_redraw();
            }

            TextEvent::DeleteText(movement) => {
                if self.edit {
                    self.delete_text(cx, *movement);

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

            TextEvent::MoveCursor(movement, selection) => {
                if self.edit && !self.show_placeholder {
                    self.move_cursor(cx, *movement, *selection);
                }
            }

            TextEvent::SetPlaceholder(text) => self.placeholder.clone_from(text),

            TextEvent::StartEdit => {
                if !cx.is_disabled() && !self.edit {
                    self.edit = true;
                    cx.focus_with_visibility(false);
                    cx.capture();
                    cx.set_checked(true);
                    self.reset_caret_timer(cx);

                    let text = self.lens.get(cx);
                    let text = text.to_string_local(cx);

                    if text.is_empty() {
                        self.show_placeholder = true;
                        self.selection = Selection::caret(0);
                    } else {
                        self.select_all(cx);
                    }

                    if let Ok(value) = &text.parse::<L::Target>() {
                        if let Some(validate) = &self.validate {
                            cx.set_valid(validate(value));
                        } else {
                            cx.set_valid(true);
                        }
                    } else {
                        cx.set_valid(false);
                    }
                }
            }

            TextEvent::EndEdit => {
                self.deselect();
                self.edit = false;
                cx.set_checked(false);
                cx.release();
                cx.stop_timer(self.caret_timer);

                let text = self.lens.get(cx);
                let text = text.to_string_local(cx);

                self.select_all(cx);

                if let Ok(value) = &text.parse::<L::Target>() {
                    if let Some(validate) = &self.validate {
                        cx.set_valid(validate(value));
                    } else {
                        cx.set_valid(true);
                    }
                } else {
                    cx.set_valid(false);
                }
                self.show_placeholder = text.is_empty();
                if self.show_placeholder {
                    cx.style.text.insert(cx.current, self.placeholder.clone());
                } else {
                    cx.style.text.insert(cx.current, text);
                }

                cx.style.needs_text_update(cx.current);
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
            }

            TextEvent::SelectWord => {
                self.select_word(cx);
            }

            TextEvent::SelectParagraph => {
                self.select_paragraph(cx);
            }

            TextEvent::Hit(posx, posy, selection) => {
                if !self.show_placeholder {
                    self.hit(cx, *posx, *posy, *selection);
                }
            }

            TextEvent::Drag(posx, posy) => {
                if !self.show_placeholder {
                    self.drag(cx, *posx, *posy);
                }
            }

            TextEvent::Scroll(_x, _y) => {
                //self.scroll(cx, *x, *y);
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
    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        cx.draw_shadows(canvas);
        cx.draw_background(canvas);
        cx.draw_border(canvas);
        cx.draw_outline(canvas);
        // canvas.save();
        // canvas.translate(self.transform.0, self.transform.1);
        // cx.draw_text_and_selection(canvas);
        cx.draw_text(canvas);
        if self.edit {
            self.draw_selection(cx, canvas);
            self.draw_text_caret(cx, canvas);
        }
        // canvas.restore();
    }
}
