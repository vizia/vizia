use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;

use crate::text::{
    Direction, EditableText, Movement, PreeditBackup, Selection, VerticalMovement, apply_movement,
    enforce_text_bounds, ensure_visible, offset_for_delete_backwards, resolved_text_direction,
};
use accesskit::{ActionData, ActionRequest, TextDirection, TextPosition, TextSelection};
use skia_safe::textlayout::{RectHeightStyle, RectWidthStyle};
use skia_safe::{Paint, PaintStyle, Rect};
use unicode_segmentation::UnicodeSegmentation;

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
    caret_timer: Timer,
    selection: Selection,
    preedit_backup: Option<PreeditBackup>,
    text_overflow: Option<TextOverflow>,
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
            caret_timer,
            selection: Selection::new(0, 0),
            preedit_backup: None,
            text_overflow: None,
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
                let handle = handle.modify(|textbox| {
                    textbox.show_placeholder.set_if_changed(txt.is_empty());
                });
                let placeholder_text = placeholder.get().to_string_local(&handle);

                if show_placeholder.get() {
                    handle.text(placeholder_text);
                } else {
                    handle.text(txt);
                }
            });
        })
    }

    fn insert_text(&mut self, cx: &mut EventContext, txt: &str) {
        if let Some(text) = cx.style.text.get_mut(cx.current) {
            if self.show_placeholder.get() && !txt.is_empty() {
                text.clear();
                self.show_placeholder.set(false);
            }

            text.edit(self.selection.range(), txt);

            self.selection = Selection::caret(self.selection.min() + txt.len());

            self.show_placeholder.set(text.is_empty());
            cx.style.needs_text_update(cx.current);
            cx.style.needs_access_update(cx.current);
        }
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

        if let Some(text) = cx.style.text.get_mut(cx.current) {
            if self.show_placeholder.get() {
                text.clear();
                self.show_placeholder.set(false);
            }

            if !self.selection.is_caret() {
                let start = self.selection.min();
                let end = self.selection.max();

                if end > start && end <= text.len() {
                    text.replace_range(start..end, "");
                }
                self.selection = Selection::caret(start);
            }

            let preedit_backup = self
                .preedit_backup
                .get_or_insert_with(|| PreeditBackup::new(String::new(), self.selection));

            let original_selection = preedit_backup.original_selection;
            let prev_preedit_text = &preedit_backup.prev_preedit;

            if prev_preedit_text == preedit_txt {
                // Move the cursor only
                let new_selection = Selection::caret(original_selection.min() + cursor.unwrap().0);
                self.selection = new_selection;
            } else {
                // Bytes index
                let start = original_selection.min();
                let end = start + prev_preedit_text.chars().map(|c| c.len_utf8()).sum::<usize>();

                // Delete old preedit text
                if end > start && end <= text.len() {
                    text.replace_range(start..end, "");
                }

                text.insert_str(start, preedit_txt);

                if let Some((cursor_index, _)) = cursor {
                    let new_caret = original_selection.min() + cursor_index;
                    self.selection = Selection::caret(new_caret);
                } else {
                    // If there is no valid cursor, the default behavior is to move to the end of the text.
                    let new_caret = original_selection.min() + preedit_txt.chars().count();
                    self.selection = Selection::caret(new_caret);
                }

                self.preedit_backup.as_mut().unwrap().set_prev_preedit(preedit_txt.to_string());
            }

            cx.style.needs_text_update(cx.current);
        }
    }

    fn clear_preedit(&mut self, cx: &mut EventContext) {
        if let Some(text) = cx.style.text.get_mut(cx.current) {
            if let Some(preedit_backup) = self.preedit_backup.as_ref() {
                let original_selection = preedit_backup.original_selection;
                let prev_preedit_text = preedit_backup.prev_preedit.clone();

                let start = original_selection.min();
                let end = start + prev_preedit_text.chars().map(|c| c.len_utf8()).sum::<usize>();

                text.replace_range(start..end, "");

                self.selection = original_selection;

                self.preedit_backup = None;
            }
        }
    }

    fn delete_text(&mut self, cx: &mut EventContext, movement: Movement) {
        if self.show_placeholder.get() {
            return;
        }

        if self.preedit_backup.is_some() {
            return;
        }

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
                    cx.style.needs_access_update(cx.current);
                }
            } else if let Some(text) = cx.style.text.get_mut(cx.current) {
                if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
                    let to_delete = apply_movement(movement, self.selection, text, paragraph, true);
                    self.selection = to_delete;
                    let new_cursor_pos = self.selection.min();

                    text.edit(to_delete.range(), "");
                    self.selection = Selection::caret(new_cursor_pos);

                    cx.style.needs_text_update(cx.current);
                    cx.style.needs_access_update(cx.current);
                }
            }
        } else if let Some(text) = cx.style.text.get_mut(cx.current) {
            let del_range = self.selection.range();

            self.selection = Selection::caret(del_range.start);

            text.edit(del_range, "");

            cx.style.needs_text_update(cx.current);
            cx.style.needs_access_update(cx.current);
        }

        if let Some(text) = cx.style.text.get_mut(cx.current) {
            self.show_placeholder.set(text.is_empty());
        }
    }

    fn reset_text(&mut self, cx: &mut EventContext) {
        if let Some(text) = cx.style.text.get_mut(cx.current) {
            text.clear();
            self.selection = Selection::caret(0);
            self.show_placeholder.set(true);
            *text = self.placeholder.get().clone();
            cx.style.needs_text_update(cx.current);
            cx.style.needs_access_update(cx.current);
        }
    }

    /// When IME is enabled, the cursor movement logic will be controlled by [`update_preedit`].
    ///
    /// [`update_preedit`]: Textbox::update_preedit
    fn move_cursor(&mut self, cx: &mut EventContext, movement: Movement, selection: bool) {
        if let Some(text) = cx.style.text.get_mut(cx.current) {
            if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
                let new_selection =
                    apply_movement(movement, self.selection, text, paragraph, selection);
                self.selection = new_selection;
                cx.needs_redraw();
                cx.style.needs_access_update(cx.current);
            }
        }
    }

    fn select_all(&mut self, cx: &mut EventContext) {
        if self.show_placeholder.get() {
            return;
        }
        if let Some(text) = cx.style.text.get(cx.current) {
            self.selection.anchor = 0;
            self.selection.active = text.len();
            cx.needs_redraw();
            cx.style.needs_access_update(cx.current);
        }
    }

    fn select_word(&mut self, cx: &mut EventContext) {
        if self.show_placeholder.get() {
            return;
        }
        self.move_cursor(cx, Movement::Word(Direction::Upstream), false);
        self.move_cursor(cx, Movement::Word(Direction::Downstream), true);
    }

    fn select_paragraph(&mut self, cx: &mut EventContext) {
        if self.show_placeholder.get() {
            return;
        }
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

            if resolved_text_direction(&cx.style, cx.current)
                == crate::style::Direction::RightToLeft
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

            top *= bounds.height() - padding_top - padding_bottom - paragraph.height();

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
                let x = x - self.transform.borrow().0;
                let y = y - self.transform.borrow().1;
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
                cx.style.needs_access_update(cx.current);
            }
        }
    }

    /// This function takes window-global physical coordinates.
    fn drag(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        if let Some(text) = cx.style.text.get(cx.current) {
            if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
                let x = x - self.transform.borrow().0;
                let y = y - self.transform.borrow().1;
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
                cx.style.needs_access_update(cx.current);
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
        if self.show_placeholder.get() {
            return String::new();
        }

        if let Some(text) = cx.style.text.get(cx.current) { text.clone() } else { String::new() }
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

                        let mut padding_left = match cx.padding_left() {
                            Units::Pixels(val) => val,
                            _ => 0.0,
                        };

                        let mut padding_right = match cx.padding_right() {
                            Units::Pixels(val) => val,
                            _ => 0.0,
                        };

                        if resolved_text_direction(&cx.style, cx.current)
                            == crate::style::Direction::RightToLeft
                        {
                            std::mem::swap(&mut padding_left, &mut padding_right);
                        }

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

                let grapheme_count = text.graphemes(true).count();
                let (range_start, range_end, use_trailing_edge) = if current < grapheme_count {
                    (current, current + 1, false)
                } else if current > 0 {
                    // At end-of-text, use the previous grapheme box and place the caret on its trailing edge.
                    (current - 1, current, true)
                } else {
                    // Empty text or no valid grapheme box to anchor the caret.
                    return;
                };

                let rects = paragraph.get_rects_for_range(
                    range_start..range_end,
                    RectHeightStyle::Tight,
                    RectWidthStyle::Tight,
                );

                let Some(cursor_rect) = rects.first() else {
                    return;
                };

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

                let mut padding_left = match cx.padding_left() {
                    Units::Pixels(val) => val,
                    _ => 0.0,
                };

                let mut padding_right = match cx.padding_right() {
                    Units::Pixels(val) => val,
                    _ => 0.0,
                };

                if resolved_text_direction(&cx.style, cx.current)
                    == crate::style::Direction::RightToLeft
                {
                    std::mem::swap(&mut padding_left, &mut padding_right);
                }

                let caret_x =
                    if use_trailing_edge { cursor_rect.rect.right } else { cursor_rect.rect.left };

                let x = (bounds.x + padding_left + caret_x).round();
                let y = (bounds.y + padding_top + cursor_rect.rect.top + top).round();

                let x2 = x + 1.0;
                let y2 = y + (cursor_rect.rect.bottom - cursor_rect.rect.top);

                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                paint.set_style(PaintStyle::Fill);
                paint.set_color(cx.caret_color());

                canvas.draw_rect(Rect::new(x, y, x2, y2), &paint);

                let mut transform = self.transform.borrow_mut();

                let text_bounds = BoundingBox::from_min_max(
                    bounds.x + padding_left,
                    bounds.y + padding_top + top,
                    bounds.x + padding_left + paragraph.max_intrinsic_width(),
                    bounds.y + padding_top + top + paragraph.height(),
                );

                let mut bounds = bounds;

                bounds =
                    bounds.shrink_sides(padding_left, padding_top, padding_right, padding_bottom);

                let (tx, ty) =
                    enforce_text_bounds(&text_bounds, &bounds, (transform.0, transform.1));

                let caret_box = BoundingBox::from_min_max(x, y, x2, y2);

                let (new_tx, new_ty) = ensure_visible(&caret_box, &bounds, (tx, ty));

                if new_tx != transform.0 || new_ty != transform.1 {
                    *transform = (new_tx, new_ty);
                    cx.needs_redraw();
                }
            }
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

        let selection = self.selection;

        let mut selection_active_line = None;
        let mut selection_anchor_line = None;
        let mut selection_active_cursor = 0;
        let mut selection_anchor_cursor = 0;
        let mut first_line_node_id = None;

        let text = if self.show_placeholder.get() {
            ""
        } else {
            cx.style.text.get(cx.current).map(|t| t.as_str()).unwrap_or("")
        };
        // build_paragraph() appends a zero-width space (\u{200B}, 3 UTF-8 bytes)
        // to every paragraph, so skia's line metrics include indices beyond the
        // actual text. We use text.len() as the upper bound for all slicing.
        let text_len = text.len();

        if let Some(paragraph) = cx.text_context.text_paragraphs.get(cx.current) {
            let text_direction = if resolved_text_direction(&cx.style, cx.current)
                == crate::style::Direction::RightToLeft
            {
                TextDirection::RightToLeft
            } else {
                TextDirection::LeftToRight
            };

            let line_metrics = paragraph.get_line_metrics();
            for line in line_metrics.iter() {
                // Skip lines that start beyond the actual text (i.e., the ZWS-only line)
                if line.start_index >= text_len && text_len > 0 {
                    continue;
                }

                // We need a child node per line
                let mut line_node = AccessNode::new_from_parent(node_id, line.line_number);
                line_node.set_role(Role::TextRun);
                line_node.set_text_direction(text_direction);
                line_node.set_bounds(BoundingBox {
                    x: line.left as f32,
                    y: (line.baseline - line.ascent) as f32,
                    w: line.width as f32,
                    h: line.height as f32,
                });

                // Only iterate over glyphs within the actual text range
                let glyph_end = line.end_index.min(text_len);
                let estimated_chars = glyph_end - line.start_index;
                let mut character_lengths: Vec<u8> = Vec::with_capacity(estimated_chars);
                let mut character_positions: Vec<f32> = Vec::with_capacity(estimated_chars);
                let mut character_widths: Vec<f32> = Vec::with_capacity(estimated_chars);
                let mut glyph_pos = line.start_index;

                while glyph_pos < glyph_end {
                    if let Some(cluster_info) = paragraph.get_glyph_cluster_at(glyph_pos) {
                        let length = cluster_info.text_range.end - cluster_info.text_range.start;
                        if length == 0 {
                            break;
                        }

                        character_lengths.push(length as u8);
                        character_positions.push(cluster_info.bounds.left());
                        character_widths.push(cluster_info.bounds.width());

                        glyph_pos += length;
                    } else {
                        break;
                    }
                }

                // Include the newline character for hard breaks, as AccessKit needs it
                let line_end = if line.hard_break {
                    line.end_including_newline.min(text_len)
                } else {
                    glyph_end
                };
                let line_text = text.get(line.start_index..line_end).unwrap_or("").to_owned();

                if line.hard_break && line.end_including_newline <= text_len {
                    character_lengths.push(1);
                    character_positions.push(line.width as f32);
                    character_widths.push(0.0);
                }

                let mut word_starts = Vec::new();
                let mut previous_is_alphanumeric = text
                    .get(..line.start_index)
                    .and_then(|prefix| prefix.graphemes(true).next_back())
                    .and_then(|grapheme| grapheme.chars().next())
                    .is_some_and(|ch| ch.is_alphanumeric());

                for (character_index, grapheme) in line_text.graphemes(true).enumerate() {
                    let current_is_alphanumeric =
                        grapheme.chars().next().is_some_and(|ch| ch.is_alphanumeric());

                    if current_is_alphanumeric
                        && !previous_is_alphanumeric
                        && let Ok(character_index) = u8::try_from(character_index)
                    {
                        word_starts.push(character_index);
                    }

                    previous_is_alphanumeric = current_is_alphanumeric;
                }

                if first_line_node_id.is_none() {
                    first_line_node_id = Some(line_node.node_id());
                }

                // Check if this line contains the selection active (focus) position
                if selection.active >= line.start_index && selection.active <= line_end {
                    selection_active_line = Some(line_node.node_id());
                    selection_active_cursor = byte_offset_to_char_index(
                        &character_lengths,
                        selection.active - line.start_index,
                    );
                }

                // Check if this line contains the selection anchor position
                if selection.anchor >= line.start_index && selection.anchor <= line_end {
                    selection_anchor_line = Some(line_node.node_id());
                    selection_anchor_cursor = byte_offset_to_char_index(
                        &character_lengths,
                        selection.anchor - line.start_index,
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
                        cx.focus_with_visibility(false);
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
                    cx.emit(TextEvent::Submit(false));
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
                        Movement::LineStart
                    } else if cx.modifiers.alt() {
                        Movement::Word(Direction::Left)
                    } else {
                        Movement::Grapheme(Direction::Left)
                    };
                    #[cfg(not(target_os = "macos"))]
                    let movement = if cx.modifiers.ctrl() {
                        Movement::Word(Direction::Left)
                    } else {
                        Movement::Grapheme(Direction::Left)
                    };

                    cx.emit(TextEvent::MoveCursor(movement, cx.modifiers.shift()));
                }

                Code::ArrowRight => {
                    self.reset_caret_timer(cx);

                    #[cfg(target_os = "macos")]
                    let movement = if cx.modifiers.logo() {
                        Movement::LineEnd
                    } else if cx.modifiers.alt() {
                        Movement::Word(Direction::Right)
                    } else {
                        Movement::Grapheme(Direction::Right)
                    };
                    #[cfg(not(target_os = "macos"))]
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
                        #[cfg(target_os = "macos")]
                        let movement = if cx.modifiers.logo() {
                            // Cmd+Backspace deletes from caret to the visual
                            // line start on macOS, matching Cmd+Left cursor
                            // movement (which uses `Movement::LineStart`).
                            Movement::LineStart
                        } else if cx.modifiers.alt() {
                            Movement::Word(Direction::Upstream)
                        } else {
                            Movement::Grapheme(Direction::Upstream)
                        };
                        #[cfg(not(target_os = "macos"))]
                        let movement = if cx.modifiers.ctrl() {
                            Movement::Word(Direction::Upstream)
                        } else {
                            Movement::Grapheme(Direction::Upstream)
                        };

                        cx.emit(TextEvent::DeleteText(movement));
                    }
                }

                Code::Delete => {
                    self.reset_caret_timer(cx);
                    if !cx.is_read_only() {
                        #[cfg(target_os = "macos")]
                        let movement = if cx.modifiers.alt() {
                            Movement::Word(Direction::Downstream)
                        } else {
                            Movement::Grapheme(Direction::Downstream)
                        };
                        #[cfg(not(target_os = "macos"))]
                        let movement = if cx.modifiers.ctrl() {
                            Movement::Word(Direction::Downstream)
                        } else {
                            Movement::Grapheme(Direction::Downstream)
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
                if self.preedit_backup.is_some() {
                    return;
                }

                if self.show_placeholder.get() {
                    self.reset_text(cx);
                }

                self.insert_text(cx, text);

                let text = self.clone_text(cx);

                if let Ok(value) = &text.parse::<T>() {
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

            TextEvent::UpdatePreedit(preedit, cursor) => {
                self.update_preedit(cx, preedit, *cursor);
            }

            TextEvent::ClearPreedit => {
                self.clear_preedit(cx);
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

                    if let Ok(value) = &text.parse::<T>() {
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
                if self.edit && !self.show_placeholder.get() && self.preedit_backup.is_none() {
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
                    cx.focus_with_visibility(false);
                    cx.capture();
                    self.reset_caret_timer(cx);
                    self.reset_ime_position(cx);

                    self.text_overflow = cx.style.text_overflow.get_inline(cx.current).copied();
                    cx.style.text_overflow.remove(cx.current);

                    let text = self.value.get_value(cx);
                    let text = text.to_string_local(cx);

                    if text.is_empty() {
                        self.show_placeholder.set(true);
                        self.selection = Selection::caret(0);
                        cx.style.needs_access_update(cx.current);
                    } else {
                        self.show_placeholder.set(false);
                        self.select_all(cx);
                    }

                    if let Ok(value) = &text.parse::<T>() {
                        if let Some(validate) = &self.validate {
                            cx.set_valid(validate(value));
                        } else {
                            cx.set_valid(true);
                        }
                    } else {
                        cx.set_valid(false);
                    }
                }

                cx.style.needs_text_update(cx.current);
            }

            TextEvent::EndEdit => {
                self.deselect();
                self.edit = false;
                cx.release();
                cx.stop_timer(self.caret_timer);

                let text = self.value.get_value(cx);
                let text = text.to_string_local(cx);
                self.show_placeholder.set(text.is_empty());

                if let Some(text_overflow) = self.text_overflow {
                    cx.style.text_overflow.insert(cx.current, text_overflow);
                } else {
                    cx.style.text_overflow.remove(cx.current);
                }

                self.select_all(cx);

                if let Ok(value) = &text.parse::<T>() {
                    if let Some(validate) = &self.validate {
                        cx.set_valid(validate(value));
                    } else {
                        cx.set_valid(true);
                    }
                } else {
                    cx.set_valid(false);
                }

                // Reset transform to 0,0
                let mut transform = self.transform.borrow_mut();
                *transform = (0.0, 0.0);

                // Reset cursor position
                self.selection = Selection::caret(0);

                cx.style.needs_text_update(cx.current);
                cx.style.needs_access_update(cx.current);
            }

            TextEvent::Blur => {
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
                        if let Ok(value) = text.parse::<T>() {
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

                            if let Ok(value) = &text.parse::<T>() {
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
        canvas.save();
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
