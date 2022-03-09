use std::sync::Arc;

#[cfg(feature = "clipboard")]
use copypasta::ClipboardProvider;

use femtovg::{Align, Baseline, Paint};
use keyboard_types::Code;
use morphorm::{PositionType, Units};
use unicode_segmentation::UnicodeSegmentation;

use crate::style::PropGet;
use crate::{
    Binding, Context, CursorIcon, Data, EditableText, Element, Event, FontOrId, Handle, Lens,
    LensExt, Model, Modifiers, MouseButton, Movement, Overflow, PropSet, Selection, TreeExt,
    Units::*, View, WindowEvent,
};

use crate::text::Direction;

#[derive(Lens)]
pub struct TextboxData {
    text: String,
    selection: Selection,
    edit: bool,
    hitx: f32,
    dragx: f32,
    caret_x: f32,
    selection_x: f32,
    selection_width: f32,
    transform_x: f32,
    line_height: f32,
    on_edit: Option<Arc<dyn Fn(&mut Context, String) + Send + Sync>>,
    on_submit: Option<Arc<dyn Fn(&mut Context, String) + Send + Sync>>,
}

impl TextboxData {
    pub fn new(text: String) -> Self {
        let text_length = text.as_str().len();
        Self {
            text: text.clone(),
            selection: Selection::new(0, text_length),
            edit: false,
            hitx: -1.0,
            dragx: -1.0,
            caret_x: 0.0,
            selection_x: 0.0,
            selection_width: 0.0,
            transform_x: 0.0,
            line_height: 0.0,
            on_edit: None,
            on_submit: None,
        }
    }

    fn set_caret(&mut self, cx: &mut Context) {
        let entity = cx.current;

        let box_x = cx.cache.get_posx(entity);
        let box_y = cx.cache.get_posy(entity);
        let box_width = cx.cache.get_width(entity);
        let box_height = cx.cache.get_height(entity);

        let font = cx.style.font.get(entity).cloned().unwrap_or_default();

        let default_font = cx
            .resource_manager
            .fonts
            .get(&cx.style.default_font)
            .and_then(|font| match font {
                FontOrId::Id(id) => Some(id),
                _ => None,
            })
            .expect("Failed to find default font");

        let font_id = cx
            .resource_manager
            .fonts
            .get(&font)
            .and_then(|font| match font {
                FontOrId::Id(id) => Some(id),
                _ => None,
            })
            .unwrap_or(default_font);

        let mut origin_x = box_x;
        let mut origin_y = box_y;

        let text_string = self.text.to_owned();

        let font_size = cx.style.font_size.get(entity).cloned().unwrap_or(16.0);

        let mut paint = Paint::default();
        paint.set_font_size(font_size);
        paint.set_font(&[font_id.clone()]);

        let font_metrics =
            cx.text_context.measure_font(paint).expect("Failed to read font metrics");

        let parent = cx.tree.get_parent(entity).expect("Failed to find parent somehow");

        let parent_width = cx.cache.get_width(parent);

        let border_width = match cx.style.border_width.get(entity).cloned().unwrap_or_default() {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        // these are the x-coordinates of the bounds of the acceptable position of the cursor
        let mut bounds_left = box_x + border_width;
        let mut bounds_right = box_x + box_width - border_width;

        let child_left = cx.style.child_left.get(entity).cloned().unwrap_or_default();
        let child_right = cx.style.child_right.get(entity).cloned().unwrap_or_default();
        let child_top = cx.style.child_top.get(entity).cloned().unwrap_or_default();
        let child_bottom = cx.style.child_bottom.get(entity).cloned().unwrap_or_default();

        bounds_left += match child_left {
            Units::Pixels(val) => val,
            _ => 5.0,
        };
        bounds_right -= match child_right {
            Units::Pixels(val) => val,
            _ => 5.0,
        };

        let align = match child_left {
            Units::Pixels(val) => match child_right {
                Units::Stretch(_) | Units::Auto | Units::Pixels(_) => {
                    origin_x += val + border_width;
                    Align::Left
                }

                _ => Align::Left,
            },

            Units::Stretch(_) => match child_right {
                Units::Pixels(val) => {
                    origin_x += box_width - val - border_width;
                    Align::Right
                }

                Units::Stretch(_) => {
                    origin_x += 0.5 * box_width;
                    Align::Center
                }

                _ => Align::Right,
            },

            _ => Align::Left,
        };

        let baseline = match child_top {
            Units::Pixels(val) => match child_bottom {
                Units::Stretch(_) => {
                    origin_y += val + border_width;
                    Baseline::Top
                }

                _ => Baseline::Top,
            },

            Units::Stretch(_) => match child_bottom {
                Units::Pixels(val) => {
                    origin_y += box_height - val - border_width;
                    Baseline::Bottom
                }

                Units::Stretch(_) => {
                    origin_y += 0.5 * box_height;
                    Baseline::Middle
                }

                _ => Baseline::Top,
            },

            _ => Baseline::Top,
        };

        paint.set_text_align(align);
        paint.set_text_baseline(baseline);

        if let Ok(res) =
            cx.text_context.measure_text(origin_x + self.transform_x, origin_y, &text_string, paint)
        {
            let text_width = res.width();

            let mut caretx = origin_x;

            let mut selectx = caretx;

            if self.edit {
                let startx =
                    if let Some(first_glyph) = res.glyphs.first() { first_glyph.x } else { 0.0 };

                let endx = startx + text_width;

                if self.hitx != -1.0 {
                    selectx = if self.hitx < startx + text_width / 2.0 {
                        self.selection.anchor = 0;
                        startx
                    } else {
                        self.selection.anchor = self.text.len();
                        endx
                    };

                    caretx = if self.dragx < startx + text_width / 2.0 {
                        self.selection.active = 0;
                        startx
                    } else {
                        self.selection.active = self.text.len();
                        endx
                    };

                    let mut px = origin_x;

                    for (glyph, (index, _)) in
                        res.glyphs.iter().zip(text_string.grapheme_indices(true))
                    {
                        let left_edge = glyph.x;
                        let right_edge = left_edge + glyph.width;
                        let gx = left_edge * 0.3 + right_edge * 0.7;

                        if self.hitx >= px && self.hitx < gx {
                            selectx = left_edge;

                            self.selection.anchor = index;
                        }

                        if self.dragx >= px && self.dragx < gx {
                            caretx = left_edge;

                            self.selection.active = index;
                        }

                        px = gx;
                    }
                } else {
                    for (glyph, (index, _)) in
                        res.glyphs.iter().zip(text_string.grapheme_indices(true))
                    {
                        if index == self.selection.active {
                            caretx = glyph.x;
                        }

                        if index == self.selection.anchor {
                            selectx = glyph.x;
                        }
                    }

                    if self.selection.active as usize == self.text.len() && self.text.len() != 0 {
                        caretx = endx;
                    }

                    if self.selection.anchor as usize == self.text.len() && self.text.len() != 0 {
                        selectx = endx;
                    }
                }

                //Draw selection
                self.caret_x = caretx.floor() - res.x;
                self.line_height = font_metrics.height();
                self.selection_width = (caretx - selectx).abs();
                self.selection_x =
                    if selectx > caretx { self.caret_x } else { selectx.floor() - res.x };

                // adjust transform
                let res_right = res.x + res.width();
                if caretx < bounds_left {
                    self.transform_x += bounds_left - caretx;
                } else if caretx >= bounds_right {
                    self.transform_x -= caretx - bounds_right;
                } else if res.x > bounds_left && res_right > bounds_right {
                    self.transform_x -= (res.x - bounds_left).min(res_right - bounds_right);
                } else if res.x < bounds_left && res_right < bounds_right {
                    self.transform_x += (bounds_left - res.x).min(bounds_right - res_right);
                }
                self.transform_x = self.transform_x.round();
            }
        }
    }

    pub fn insert_text(&mut self, _cx: &mut Context, text: &str) {
        let text_length = text.len();
        self.text.edit(self.selection.range(), text);

        self.selection = Selection::caret(self.selection.min() + text_length);
    }

    pub fn delete_text(&mut self, _cx: &mut Context, movement: Movement) {
        if !self.selection.is_caret() {
            self.text.edit(self.selection.range(), "");

            self.selection = Selection::caret(self.selection.min());
        } else {
            match movement {
                Movement::Grapheme(Direction::Upstream) => {
                    if let Some(offset) = self.text.prev_grapheme_offset(self.selection.active) {
                        self.text.edit(offset..self.selection.active, "");
                        self.selection = Selection::caret(offset);
                    }
                }

                Movement::Grapheme(Direction::Downstream) => {
                    if let Some(offset) = self.text.next_grapheme_offset(self.selection.active) {
                        self.text.edit(self.selection.active..offset, "");
                        self.selection = Selection::caret(self.selection.active);
                    }
                }

                Movement::Word(Direction::Upstream) => {
                    if let Some(offset) = self.text.prev_word_offset(self.selection.active) {
                        self.text.edit(offset..self.selection.active, "");
                        self.selection = Selection::caret(offset);
                    }
                }

                Movement::Word(Direction::Downstream) => {
                    if let Some(offset) = self.text.next_word_offset(self.selection.active) {
                        self.text.edit(self.selection.active..offset, "");
                        self.selection = Selection::caret(self.selection.active);
                    }
                }

                _ => {}
            }
        }
    }

    pub fn move_cursor(&mut self, _: &mut Context, movement: Movement, selection: bool) {
        match movement {
            Movement::Grapheme(Direction::Upstream) => {
                if let Some(offset) = self.text.prev_grapheme_offset(self.selection.active) {
                    self.selection.active = offset;
                    offset
                } else {
                    self.selection.active
                };

                if !selection {
                    self.selection.anchor = self.selection.active;
                }
            }

            Movement::Grapheme(Direction::Downstream) => {
                if let Some(offset) = self.text.next_grapheme_offset(self.selection.active) {
                    self.selection.active = offset;
                    offset
                } else {
                    self.selection.active
                };

                if !selection {
                    self.selection.anchor = self.selection.active;
                }
            }

            Movement::Word(Direction::Upstream) => {
                if let Some(offset) = self.text.prev_word_offset(self.selection.active) {
                    self.selection.active = offset;
                    offset
                } else {
                    self.selection.active
                };

                if !selection {
                    self.selection.anchor = self.selection.active;
                }
            }

            Movement::Word(Direction::Downstream) => {
                if let Some(offset) = self.text.next_word_offset(self.selection.active) {
                    self.selection.active = offset;
                    offset
                } else {
                    self.selection.active
                };

                if !selection {
                    self.selection.anchor = self.selection.active;
                }
            }

            Movement::ParagraphStart => {
                if selection {
                    self.selection.active = 0;
                } else {
                    self.selection.active = 0;
                    self.selection.anchor = 0;
                }
            }

            Movement::ParagraphEnd => {
                if selection {
                    self.selection.active = self.text.len();
                } else {
                    self.selection.active = self.text.len();
                    self.selection.anchor = self.text.len();
                }
            }

            _ => {}
        }
    }

    pub fn select_all(&mut self, _: &mut Context) {
        self.selection = Selection::new(0, self.text.len());
    }
}

pub enum TextEvent {
    InsertText(String),
    DeleteText(Movement),
    MoveCursor(Movement, bool),
    SelectAll,
    StartEdit,
    EndEdit,
    Submit,
    SetHitX(f32),
    SetDragX(f32),
    Copy,
    Paste,

    // Helpers
    SetOnEdit(Option<Arc<dyn Fn(&mut Context, String) + Send + Sync>>),
    SetOnSubmit(Option<Arc<dyn Fn(&mut Context, String) + Send + Sync>>),
}

impl Model for TextboxData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(textbox_event) = event.message.downcast() {
            match textbox_event {
                TextEvent::InsertText(text) => {
                    if self.edit {
                        self.insert_text(cx, text);
                        self.set_caret(cx);

                        if let Some(callback) = self.on_edit.take() {
                            (callback)(cx, self.text.as_str().to_owned());

                            self.on_edit = Some(callback);
                        }
                    }
                }

                TextEvent::DeleteText(movement) => {
                    if self.edit {
                        self.delete_text(cx, *movement);
                        self.set_caret(cx);

                        if let Some(callback) = self.on_edit.take() {
                            (callback)(cx, self.text.as_str().to_owned());

                            self.on_edit = Some(callback);
                        }
                    }
                }

                TextEvent::MoveCursor(movement, selection) => {
                    if self.edit {
                        self.move_cursor(cx, *movement, *selection);
                        self.set_caret(cx);
                    }
                }

                TextEvent::StartEdit => {
                    if !cx.current.is_disabled(cx) {
                        self.edit = true;
                    }
                }

                TextEvent::EndEdit => {
                    self.edit = false;
                }

                TextEvent::Submit => {
                    cx.emit(TextEvent::EndEdit);
                    if let Some(callback) = self.on_submit.take() {
                        (callback)(cx, self.text.as_str().to_owned());

                        self.on_submit = Some(callback);
                    }
                }

                TextEvent::SelectAll => {
                    self.select_all(cx);
                    self.set_caret(cx);
                }

                TextEvent::SetHitX(val) => {
                    self.hitx = *val;
                    self.set_caret(cx);
                }

                TextEvent::SetDragX(val) => {
                    self.dragx = *val;
                    self.set_caret(cx);
                }

                TextEvent::Copy =>
                {
                    #[cfg(feature = "clipboard")]
                    if self.edit {
                        if cx.modifiers.contains(Modifiers::CTRL) {
                            let selected_text = &self.text.as_str()[self.selection.range()];
                            if selected_text.len() > 0 {
                                cx.clipboard
                                    .set_contents(selected_text.to_owned())
                                    .expect("Failed to add text to clipboard");
                            }
                        }
                    }
                }

                TextEvent::Paste =>
                {
                    #[cfg(feature = "clipboard")]
                    if self.edit {
                        if cx.modifiers.contains(Modifiers::CTRL) {
                            if let Ok(text) = cx.clipboard.get_contents() {
                                cx.emit(TextEvent::InsertText(text));
                            }
                        }
                    }
                }

                TextEvent::SetOnEdit(on_edit) => {
                    self.on_edit = on_edit.clone();
                }

                TextEvent::SetOnSubmit(on_submit) => {
                    self.on_submit = on_submit.clone();
                }
            }
        }
    }
}

pub struct Textbox<L: Lens> {
    lens: L,
}

impl<L: Lens> Textbox<L>
where
    <L as Lens>::Target: Data + Clone + ToString,
{
    pub fn new<'a>(cx: &'a mut Context, lens: L) -> Handle<'a, Self> {
        Self { lens: lens.clone() }.build2(cx, move |cx| {
            Binding::new(cx, lens.clone(), |cx, text| {
                let text =
                    text.get_fallible(cx).map(|x| x.to_string()).unwrap_or_else(|| "".to_owned());
                if let Some(text_data) = cx.data::<TextboxData>() {
                    if !text_data.edit {
                        let td = TextboxData {
                            text: text.clone(),
                            selection: text_data.selection,
                            edit: text_data.edit,
                            hitx: -1.0,
                            dragx: -1.0,
                            caret_x: text_data.caret_x,
                            selection_x: text_data.selection_x,
                            selection_width: text_data.selection_width,
                            transform_x: text_data.transform_x,
                            line_height: text_data.line_height,
                            on_edit: text_data.on_edit.clone(),
                            on_submit: text_data.on_submit.clone(),
                        };
                        let real_current = cx.current;
                        cx.current = cx.current.parent(&cx.tree).unwrap();
                        td.build(cx);
                        cx.current = real_current;
                    }
                } else {
                    let mut td = TextboxData::new(text.clone());
                    td.set_caret(cx);
                    let real_current = cx.current;
                    cx.current = cx.current.parent(&cx.tree).unwrap();
                    td.build(cx);
                    cx.current = real_current;
                }
            });
            TextboxContent {}
                .build2(cx, move |cx| {
                    TextboxLabel {}.build(cx).text(TextboxData::text).class("textbox_content");

                    // Selection
                    Element::new(cx)
                        .class("selection")
                        .position_type(PositionType::SelfDirected)
                        .visibility(TextboxData::edit)
                        .height(TextboxData::line_height.map(|x| Units::Pixels(*x)))
                        .left(TextboxData::selection_x.map(|x| Units::Pixels(*x)))
                        .width(TextboxData::selection_width.map(|x| Units::Pixels(*x)));

                    // Caret
                    Element::new(cx)
                        .class("caret")
                        .overflow(Overflow::Visible)
                        .position_type(PositionType::SelfDirected)
                        .width(Pixels(1.0))
                        .visibility(TextboxData::edit)
                        .height(TextboxData::line_height.map(|x| Units::Pixels(*x)))
                        .left(TextboxData::caret_x.map(|x| Units::Pixels(*x)));
                })
                .translate(TextboxData::transform_x.map(|x| (*x, 0.0)));
        })
    }
}

impl<'a, L: Lens> Handle<'a, Textbox<L>> {
    pub fn on_edit<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context, String) + Send + Sync,
    {
        self.cx.emit_to(self.entity, TextEvent::SetOnEdit(Some(Arc::new(callback))));

        self
    }

    pub fn on_submit<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context, String) + Send + Sync,
    {
        self.cx.emit_to(self.entity, TextEvent::SetOnSubmit(Some(Arc::new(callback))));

        self
    }
}

impl<L: Lens> View for Textbox<L>
where
    <L as Lens>::Target: Data + ToString,
{
    fn element(&self) -> Option<String> {
        Some("textbox".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        //let selection = cx.tree.get_child(cx.current, 0).unwrap();
        //let caret = cx.tree.get_child(cx.current, 1).unwrap();

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if cx.current.is_over(cx) {
                        //if !self.edit {
                        // self.edit = true;
                        cx.emit(TextEvent::StartEdit);

                        cx.focused = cx.current;
                        cx.capture();
                        cx.current.set_checked(cx, true);
                        //}

                        // Hit test
                        //if self.edit {
                        // self.hitx = cx.mouse.cursorx;
                        // self.dragx = cx.mouse.cursorx;
                        cx.emit(TextEvent::SetHitX(cx.mouse.cursorx));
                        cx.emit(TextEvent::SetDragX(cx.mouse.cursorx));
                        //}
                        //self.set_caret(cx, cx.current);
                    } else {
                        cx.release();
                        cx.current.set_checked(cx, false);
                        //self.edit = false;
                        cx.emit(TextEvent::EndEdit);
                        //cx.emit(TextEvent::SetEditing(false));
                        // Forward event to hovered
                        cx.event_queue.push_back(
                            Event::new(WindowEvent::MouseDown(MouseButton::Left))
                                .target(cx.hovered),
                        );
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    //self.hitx = -1.0;
                    //self.set_caret(cx, cx.current);
                    cx.emit(TextEvent::SetHitX(-1.0));
                }

                WindowEvent::MouseMove(x, _) => {
                    // if self.hitx != -1.0 {
                    //     self.dragx = *x;

                    //     self.set_caret(cx, cx.current);
                    // }
                    cx.emit(TextEvent::SetDragX(*x));
                }

                WindowEvent::MouseEnter => {
                    cx.emit(WindowEvent::SetCursor(CursorIcon::Text));
                }

                WindowEvent::MouseLeave => {
                    cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                }

                WindowEvent::CharInput(c) => {
                    //if self.edit {
                    if *c != '\u{1b}' && // Escape
                            *c != '\u{8}' && // Backspace
                            *c != '\u{7f}' && // Delete
                            *c != '\u{0d}' && // Return
                            !cx.modifiers.contains(Modifiers::CTRL)
                    {
                        //self.insert_text(cx, String::from(*c));
                        cx.emit(TextEvent::InsertText(String::from(*c)));
                        //cx.style.text.insert(cx.current, self.text_data.text.clone());
                    }
                    //self.set_caret(cx, cx.current);
                    //}
                }

                WindowEvent::KeyDown(code, _) => match code {
                    Code::Enter => {
                        // Finish editing
                        // self.edit = false;

                        //cx.emit(TextEvent::EndEdit);
                        cx.emit(TextEvent::Submit);

                        if let Some(source) = cx.data::<L::Source>() {
                            let text = self.lens.view(source, |t| {
                                if let Some(t) = t {
                                    t.to_string()
                                } else {
                                    "".to_owned()
                                }
                            });

                            cx.emit(TextEvent::SelectAll);
                            cx.emit(TextEvent::InsertText(text));
                        };

                        cx.current.set_checked(cx, false);
                    }

                    Code::ArrowLeft => {
                        //if self.edit {
                        let movement = if cx.modifiers.contains(Modifiers::CTRL) {
                            Movement::Word(Direction::Upstream)
                        } else {
                            Movement::Grapheme(Direction::Upstream)
                        };

                        cx.emit(TextEvent::MoveCursor(
                            movement,
                            cx.modifiers.contains(Modifiers::SHIFT),
                        ));

                        //self.move_cursor(cx, movement, cx.modifiers.contains(Modifiers::SHIFT));

                        //self.set_caret(cx, cx.current);
                        //}
                    }

                    Code::ArrowRight => {
                        //if self.edit {
                        let movement = if cx.modifiers.contains(Modifiers::CTRL) {
                            Movement::Word(Direction::Downstream)
                        } else {
                            Movement::Grapheme(Direction::Downstream)
                        };

                        cx.emit(TextEvent::MoveCursor(
                            movement,
                            cx.modifiers.contains(Modifiers::SHIFT),
                        ));

                        // self.move_cursor(cx, movement, cx.modifiers.contains(Modifiers::SHIFT));

                        // self.set_caret(cx, cx.current);
                        //}
                    }

                    // TODO
                    Code::ArrowUp => {}

                    // TODO
                    Code::ArrowDown => {}

                    Code::Backspace => {
                        if cx.modifiers.contains(Modifiers::CTRL) {
                            //self.delete_text(cx, Movement::Word(Direction::Upstream));
                            cx.emit(TextEvent::DeleteText(Movement::Word(Direction::Upstream)));
                        } else {
                            //self.delete_text(cx, Movement::Grapheme(Direction::Upstream));
                            cx.emit(TextEvent::DeleteText(Movement::Grapheme(Direction::Upstream)));
                        }

                        //self.set_caret(cx, cx.current);
                    }

                    Code::Delete => {
                        //if self.edit {
                        if cx.modifiers.contains(Modifiers::CTRL) {
                            //self.delete_text(cx, Movement::Word(Direction::Downstream));
                            cx.emit(TextEvent::DeleteText(Movement::Word(Direction::Downstream)));
                        } else {
                            //self.delete_text(cx, Movement::Grapheme(Direction::Downstream));
                            cx.emit(TextEvent::DeleteText(Movement::Grapheme(
                                Direction::Downstream,
                            )));
                        }
                        //self.set_caret(cx, cx.current);
                        //}
                    }

                    Code::Escape => {
                        //self.edit = false;
                        cx.emit(TextEvent::EndEdit);
                        cx.current.set_checked(cx, false);
                    }

                    Code::Home => {
                        cx.emit(TextEvent::MoveCursor(
                            Movement::ParagraphStart,
                            cx.modifiers.contains(Modifiers::SHIFT),
                        ));
                    }

                    Code::End => {
                        cx.emit(TextEvent::MoveCursor(
                            Movement::ParagraphEnd,
                            cx.modifiers.contains(Modifiers::SHIFT),
                        ));
                    }

                    // TODO
                    Code::PageUp => {}

                    // TODO
                    Code::PageDown => {}

                    Code::KeyA => {
                        //if self.edit {
                        if cx.modifiers.contains(Modifiers::CTRL) {
                            // self.select_all(cx);
                            cx.emit(TextEvent::SelectAll);
                        }
                        //}
                    }

                    Code::KeyC => {
                        cx.emit(TextEvent::Copy);
                    }

                    Code::KeyV => {
                        cx.emit(TextEvent::Paste);
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}

// can't just be a stack because what if you've styled stacks
pub struct TextboxContent {}
impl View for TextboxContent {}

// can't just be a label because above
pub struct TextboxLabel {}
impl View for TextboxLabel {}
