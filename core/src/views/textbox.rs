use std::sync::Arc;

#[cfg(feature = "clipboard")]
use copypasta::ClipboardProvider;

use keyboard_types::Code;

use crate::style::PropGet;
use crate::{
    idx_to_pos, measure_text_lines, pos_to_idx, text_layout, text_paint, Actions, Binding,
    BoundingBox, Context, CursorIcon, Data, EditableText, Entity, Event, Handle, Lens, LensExt,
    Model, Modifiers, MouseButton, MouseButtonState, Movement, PropSet, Selection, TreeExt, View,
    WindowEvent,
};

use crate::text::Direction;

#[derive(Lens)]
pub struct TextboxData {
    text: String,
    selection: Selection,
    sel_x: f32,
    re_sel_x: bool,
    edit: bool,
    transform: (f32, f32),
    line_height: f32,
    on_edit: Option<Arc<dyn Fn(&mut Context, String) + Send + Sync>>,
    content_entity: Entity,
    kind: TextboxKind,
    on_submit: Option<Arc<dyn Fn(&mut Context, String) + Send + Sync>>,
}

impl TextboxData {
    pub fn new(text: String) -> Self {
        let text_length = text.as_str().len();
        Self {
            text: text.clone(),
            selection: Selection::new(0, text_length),
            sel_x: -1.0,
            re_sel_x: false,
            edit: false,
            transform: (0.0, 0.0),
            line_height: 0.0,
            on_edit: None,
            content_entity: Entity::null(),
            kind: TextboxKind::SingleLine,
            on_submit: None,
        }
    }

    fn set_caret(&mut self, cx: &mut Context) {
        let entity = self.content_entity;
        if entity == Entity::null() {
            return;
        }
        let parent = entity.parent(&cx.tree).unwrap();

        // calculate visible area for content and container
        let bounds = cx.cache.bounds.get(entity).unwrap().clone();
        let mut parent_bounds = cx.cache.bounds.get(parent).unwrap().clone();

        // calculate line height - we'll need this
        let paint = text_paint(&cx.style, &cx.resource_manager, entity);
        let font_metrics = cx.text_context.measure_font(paint).unwrap();
        let line_height = font_metrics.height();

        // we can't just access cache.text_lines because the text could be just-updated
        let render_width = match self.kind {
            TextboxKind::MultiLineWrapped => parent_bounds.w,
            _ => f32::MAX,
        };
        let ranges = text_layout(render_width, &self.text, paint, &cx.text_context).unwrap();
        let metrics =
            measure_text_lines(&self.text, paint, &ranges, bounds.x, bounds.y, &cx.text_context);
        let ranges_metrics = ranges.into_iter().zip(metrics.into_iter()).collect::<Vec<_>>();
        let (line, (x, _)) = idx_to_pos(self.selection.active, ranges_metrics.iter());
        if self.re_sel_x {
            self.re_sel_x = false;
            self.sel_x = x;
        }

        // do the computation
        let (mut tx, mut ty) = self.transform;
        let text_box = BoundingBox { x: bounds.x + tx, y: bounds.y + ty, w: bounds.w, h: bounds.h };
        if text_box.x < parent_bounds.x
            && text_box.x + text_box.w < parent_bounds.x + parent_bounds.w
        {
            tx += parent_bounds.x - text_box.x;
        }
        if text_box.x > parent_bounds.x
            && text_box.x + text_box.w > parent_bounds.x + parent_bounds.w
        {
            tx -= (text_box.x + text_box.w) - (parent_bounds.x + parent_bounds.w);
        }
        if text_box.w < parent_bounds.w {
            tx = 0.0;
        }
        if text_box.y < parent_bounds.y
            && text_box.y + text_box.h < parent_bounds.y + parent_bounds.h
        {
            ty += parent_bounds.y - text_box.y;
        }
        if text_box.y > parent_bounds.y
            && text_box.y + text_box.h > parent_bounds.y + parent_bounds.h
        {
            ty -= (text_box.y + text_box.h) - (parent_bounds.y + parent_bounds.h);
        }
        if text_box.h < parent_bounds.h {
            ty = 0.0;
        }
        let caret_box = BoundingBox {
            x: x.round() + tx,
            y: bounds.y + line as f32 * line_height + ty,
            w: 1.0,
            h: line_height,
        };
        parent_bounds.x -= 1.0;
        parent_bounds.w += 2.0;
        if caret_box.x < parent_bounds.x {
            tx += parent_bounds.x - caret_box.x;
        }
        if caret_box.x + caret_box.w >= parent_bounds.x + parent_bounds.w {
            tx -= caret_box.x + caret_box.w - (parent_bounds.x + parent_bounds.w);
        }
        if caret_box.y < parent_bounds.y {
            ty += parent_bounds.y - caret_box.y;
        }
        if caret_box.y + caret_box.h >= parent_bounds.y + parent_bounds.h {
            ty -= caret_box.y + caret_box.h - (parent_bounds.y + parent_bounds.h);
        }
        self.transform = (tx.round(), ty.round());
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

    pub fn move_cursor(&mut self, cx: &mut Context, movement: Movement, selection: bool) {
        match movement {
            Movement::Grapheme(Direction::Upstream) => {
                self.re_sel_x = true;
                if let Some(offset) = self.text.prev_grapheme_offset(self.selection.active) {
                    self.selection.active = offset;
                    offset
                } else {
                    self.selection.active
                };
            }

            Movement::Grapheme(Direction::Downstream) => {
                self.re_sel_x = true;
                if let Some(offset) = self.text.next_grapheme_offset(self.selection.active) {
                    self.selection.active = offset;
                    offset
                } else {
                    self.selection.active
                };
            }

            Movement::Word(Direction::Upstream) => {
                self.re_sel_x = true;
                if let Some(offset) = self.text.prev_word_offset(self.selection.active) {
                    self.selection.active = offset;
                    offset
                } else {
                    self.selection.active
                };
            }

            Movement::Word(Direction::Downstream) => {
                self.re_sel_x = true;
                if let Some(offset) = self.text.next_word_offset(self.selection.active) {
                    self.selection.active = offset;
                    offset
                } else {
                    self.selection.active
                };
            }

            Movement::Line(dir) => {
                let entity = self.content_entity;
                let paint = text_paint(&cx.style, &cx.resource_manager, entity);
                let font_metrics = cx.text_context.measure_font(paint).unwrap();
                let line_height = font_metrics.height();

                let default = vec![];
                let lines = cx.cache.text_lines.get(entity).unwrap_or(&default);
                let (line, (_, y)) = idx_to_pos(self.selection.active, lines.iter());

                if line == 0 && matches!(dir, Direction::Upstream) {
                    self.selection.active = 0;
                } else {
                    let new_y = y + line_height
                        * match dir {
                            Direction::Upstream => -1.0,
                            Direction::Downstream => 1.0,
                            Direction::Left => 0.0,
                            Direction::Right => 0.0,
                        };

                    self.selection.active = pos_to_idx(self.sel_x, new_y, lines.iter());
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

        if !selection {
            self.selection.anchor = self.selection.active;
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
    Hit(f32, f32),
    Drag(f32, f32),
    Copy,
    Paste,

    // Helpers
    SetOnEdit(Option<Arc<dyn Fn(&mut Context, String) + Send + Sync>>),
    SetOnSubmit(Option<Arc<dyn Fn(&mut Context, String) + Send + Sync>>),
    InitContent(Entity, TextboxKind),
    GeometryChanged,
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

                TextEvent::Hit(posx, posy) => {
                    let posx = *posx - self.transform.0;
                    let posy = *posy - self.transform.1;
                    let idx = pos_to_idx(
                        posx,
                        posy,
                        cx.cache.text_lines.get(self.content_entity).unwrap().iter(),
                    );
                    self.selection = Selection::new(idx, idx);
                    self.sel_x = posx;
                    self.set_caret(cx);
                }

                TextEvent::Drag(posx, posy) => {
                    let posx = *posx - self.transform.0;
                    let posy = *posy - self.transform.1;
                    let idx = pos_to_idx(
                        posx,
                        posy,
                        cx.cache.text_lines.get(self.content_entity).unwrap().iter(),
                    );
                    self.selection = Selection::new(self.selection.anchor, idx);
                    self.sel_x = posx;
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

                TextEvent::InitContent(content, kind) => {
                    self.content_entity = *content;
                    self.kind = *kind;
                }

                TextEvent::GeometryChanged => {
                    self.set_caret(cx);
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
    kind: TextboxKind,
}

#[derive(Copy, Clone)]
pub enum TextboxKind {
    SingleLine,
    MultiLineUnwrapped,
    MultiLineWrapped,
}

impl<L: Lens> Textbox<L>
where
    <L as Lens>::Target: Data + Clone + ToString,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self::new_core(cx, lens, TextboxKind::SingleLine)
    }

    pub fn new_multiline(cx: &mut Context, lens: L, wrap: bool) -> Handle<Self> {
        Self::new_core(
            cx,
            lens,
            if wrap { TextboxKind::MultiLineWrapped } else { TextboxKind::MultiLineUnwrapped },
        )
    }

    fn new_core(cx: &mut Context, lens: L, kind: TextboxKind) -> Handle<Self> {
        let result = Self { lens: lens.clone(), kind }.build(cx, move |cx| {
            Binding::new(cx, lens.clone(), |cx, text| {
                let text =
                    text.get_fallible(cx).map(|x| x.to_string()).unwrap_or_else(|| "".to_owned());
                if let Some(text_data) = cx.data::<TextboxData>() {
                    if !text_data.edit {
                        let td = TextboxData {
                            text: text.clone(),
                            selection: text_data.selection,
                            edit: text_data.edit,
                            sel_x: text_data.sel_x,
                            re_sel_x: text_data.re_sel_x,
                            transform: text_data.transform,
                            line_height: text_data.line_height,
                            on_edit: text_data.on_edit.clone(),
                            content_entity: text_data.content_entity,
                            kind: text_data.kind,
                            on_submit: text_data.on_submit.clone(),
                        };
                        let real_current = cx.current;
                        cx.current = cx.current.parent(&cx.tree).unwrap();
                        td.build(cx);
                        cx.current = real_current;
                        // push an event into the queue to force an update because the textbox data
                        // may have already been observed this update cycle
                        cx.emit_to(cx.current, ());
                    }
                } else {
                    let mut td = TextboxData::new(text.clone());
                    td.set_caret(cx);
                    let real_current = cx.current;
                    cx.current = cx.current.parent(&cx.tree).unwrap();
                    td.build(cx);
                    cx.current = real_current;
                    cx.emit_to(cx.current, ());
                }
            });
            TextboxContainer {}
                .build(cx, move |cx| {
                    let lbl = TextboxLabel {}
                        .build(cx, |_| {})
                        .hoverable(false)
                        .class("textbox_content")
                        .text(TextboxData::text)
                        .text_selection(TextboxData::selection)
                        .translate(TextboxData::transform)
                        .on_geo_changed(|cx, _| cx.emit(TextEvent::GeometryChanged))
                        .entity;

                    cx.emit(TextEvent::InitContent(lbl, kind));
                })
                .hoverable(false)
                .class("textbox_container");
        });

        result.class(match kind {
            TextboxKind::SingleLine => "single_line",
            TextboxKind::MultiLineUnwrapped => "multi_line_unwrapped",
            TextboxKind::MultiLineWrapped => "multi_line_wrapped",
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
                        cx.emit(TextEvent::StartEdit);

                        cx.focused = cx.current;
                        cx.capture();
                        cx.current.set_checked(cx, true);

                        cx.emit(TextEvent::Hit(cx.mouse.cursorx, cx.mouse.cursory));
                    } else {
                        cx.release();
                        cx.current.set_checked(cx, false);
                        cx.emit(TextEvent::EndEdit);

                        // Forward event to hovered
                        cx.event_queue.push_back(
                            Event::new(WindowEvent::MouseDown(MouseButton::Left))
                                .target(cx.hovered),
                        );
                    }
                }

                WindowEvent::MouseMove(_, _) => {
                    cx.emit(WindowEvent::SetCursor(CursorIcon::Text));
                    if cx.mouse.left.state == MouseButtonState::Pressed {
                        cx.emit(TextEvent::Drag(cx.mouse.cursorx, cx.mouse.cursory));
                    }
                }

                WindowEvent::MouseLeave => {
                    cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                }

                WindowEvent::CharInput(c) => {
                    if *c != '\u{1b}' && // Escape
                            *c != '\u{8}' && // Backspace
                            *c != '\u{7f}' && // Delete
                            *c != '\u{0d}' && // Carriage return
                            !cx.modifiers.contains(Modifiers::CTRL)
                    {
                        cx.emit(TextEvent::InsertText(String::from(*c)));
                    }
                }

                WindowEvent::KeyDown(code, _) => match code {
                    Code::Enter => {
                        // Finish editing
                        // self.edit = false;

                        //cx.emit(TextEvent::EndEdit);

                        if matches!(self.kind, TextboxKind::SingleLine) {
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
                            cx.emit(TextEvent::EndEdit);
                            cx.emit(TextEvent::Submit);

                            cx.current.set_checked(cx, false);
                        } else {
                            cx.emit(TextEvent::InsertText("\n".to_owned()));
                        }
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

                    Code::ArrowUp => {
                        cx.emit(TextEvent::MoveCursor(
                            Movement::Line(Direction::Upstream),
                            cx.modifiers.contains(Modifiers::SHIFT),
                        ));
                    }

                    Code::ArrowDown => {
                        cx.emit(TextEvent::MoveCursor(
                            Movement::Line(Direction::Downstream),
                            cx.modifiers.contains(Modifiers::SHIFT),
                        ));
                    }

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
pub struct TextboxContainer {}
impl View for TextboxContainer {}

// can't just be a label because what if you've styled labels
pub struct TextboxLabel {}
impl View for TextboxLabel {}
