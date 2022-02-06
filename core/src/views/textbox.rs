use std::sync::Arc;

#[cfg(feature = "clipboard")]
use copypasta::ClipboardProvider;

use femtovg::{Align, Baseline, Paint};
use keyboard_types::Code;
use morphorm::{PositionType, Units};
use unicode_segmentation::UnicodeSegmentation;

use crate::style::PropGet;
use crate::{
    Binding, Context, CursorIcon, Data, EditableText, Element, Entity, Event, FontOrId, Handle,
    Lens, Model, Modifiers, MouseButton, Movement, PropSet, Selection, Units::*, View, WindowEvent, Visibility, Res,
};

use crate::text::Direction;



pub struct TextboxData<T: EditableText> {
    text: T,
    selection: Selection,
    caret_entity: Entity,
    selection_entity: Entity,
    edit: bool,
    hitx: f32,
    dragx: f32,
    on_edit: Option<Arc<dyn Fn(&mut Context, String) + Send + Sync>>,
}

impl<T: EditableText> TextboxData<T> {

    pub fn new(text: T) -> Self {
        let text_length = text.as_str().len();
        Self {
            text: text.clone(),
            selection: Selection::new(0, text_length),
            caret_entity: Entity::null(),
            selection_entity: Entity::null(),
            edit: false,
            hitx: -1.0,
            dragx: -1.0,
            on_edit: None,
        }
    }

    fn set_caret(&mut self, cx: &mut Context) {
        // TODO - replace this with something better
        //let selection = cx.tree.get_child(entity, 0).unwrap();
        //let caret = cx.tree.get_child(entity, 1).unwrap();

        let entity = cx.current;

        let posx = cx.cache.get_posx(entity);
        let posy = cx.cache.get_posy(entity);
        let width = cx.cache.get_width(entity);
        let height = cx.cache.get_height(entity);

        if let Some(text) = cx.style.text.get(entity).cloned() {
            let font = cx.style.font.get(entity).cloned().unwrap_or_default();

            // TODO - This should probably be cached in cx to save look-up time
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

            let mut x = posx;
            let mut y = posy;

            let text_string = text.to_owned();

            let font_size = cx.style.font_size.get(entity).cloned().unwrap_or(16.0);

            let mut paint = Paint::default();
            paint.set_font_size(font_size);
            paint.set_font(&[font_id.clone()]);

            let font_metrics =
                cx.text_context.measure_font(paint).expect("Failed to read font metrics");

            let parent = cx.tree.get_parent(entity).expect("Failed to find parent somehow");

            let parent_width = cx.cache.get_width(parent);

            let border_width =
                match cx.style.border_width.get(entity).cloned().unwrap_or_default() {
                    Units::Pixels(val) => val,
                    Units::Percentage(val) => parent_width * val,
                    _ => 0.0,
                };

            // TODO - Move this to a text layout system and include constraints
            let child_left = cx.style.child_left.get(entity).cloned().unwrap_or_default();
            let child_right = cx.style.child_right.get(entity).cloned().unwrap_or_default();
            let child_top = cx.style.child_top.get(entity).cloned().unwrap_or_default();
            let child_bottom = cx.style.child_bottom.get(entity).cloned().unwrap_or_default();

            let align = match child_left {
                Units::Pixels(val) => match child_right {
                    Units::Stretch(_) | Units::Auto => {
                        x += val + border_width;
                        Align::Left
                    }

                    _ => Align::Left,
                },

                Units::Stretch(_) => match child_right {
                    Units::Pixels(val) => {
                        x += width - val - border_width;
                        Align::Right
                    }

                    Units::Stretch(_) => {
                        x += 0.5 * width;
                        Align::Center
                    }

                    _ => Align::Right,
                },

                _ => Align::Left,
            };

            let baseline = match child_top {
                Units::Pixels(val) => match child_bottom {
                    Units::Stretch(_) => {
                        y += val + border_width;
                        Baseline::Top
                    }

                    _ => Baseline::Top,
                },

                Units::Stretch(_) => match child_bottom {
                    Units::Pixels(val) => {
                        y += height - val - border_width;
                        Baseline::Bottom
                    }

                    Units::Stretch(_) => {
                        y += 0.5 * height;
                        Baseline::Middle
                    }

                    _ => Baseline::Top,
                },

                _ => Baseline::Top,
            };

            paint.set_text_align(align);
            paint.set_text_baseline(baseline);

            if let Ok(res) = cx.text_context.measure_text(x, y, &text_string, paint) {
                let text_width = res.width();

                let mut caretx = x;

                let mut selectx = caretx;

                if self.edit {
                    let startx = if let Some(first_glyph) = res.glyphs.first() {
                        first_glyph.x
                    } else {
                        0.0
                    };
                    //let startx = x - text_width / 2.0;
                    let endx = startx + text_width;

                    if self.hitx != -1.0 {
                        //let endx = res.glyphs.last().unwrap().x + res.glyphs.last().unwrap().w;

                        selectx = if self.hitx < startx + text_width / 2.0 {
                            self.selection.anchor = 0;
                            //cx.emit(TextEvent::SetAnchor(0));
                            startx
                        } else {
                            self.selection.anchor = text.len();
                            //cx.emit(TextEvent::SetAnchor(text.len()));
                            endx
                        };

                        caretx = if self.dragx < startx + text_width / 2.0 {
                            self.selection.active = 0;
                            //cx.emit(TextEvent::SetActive(0));
                            startx
                        } else {
                            self.selection.active = text.len();
                            //cx.emit(TextEvent::SetActive(text.len()));
                            endx
                        };

                        let mut px = x;

                        for (glyph, (index, _)) in
                            res.glyphs.iter().zip(text_string.grapheme_indices(true))
                        {
                            let left_edge = glyph.x;
                            let right_edge = left_edge + glyph.width;
                            let gx = left_edge * 0.3 + right_edge * 0.7;

                            //println!("{} {} {}", self.hitx, left_edge, right_edge);

                            // if n == 0 && self.hitx <= glyph.x {
                            //     selectx = left_edge;
                            //     self.select_pos = 0;
                            // }

                            // if n == res.glyphs.len() as u32 && self.hitx >= glyph.x + glyph.width {
                            //     selectx = right_edge;
                            //     self.select_pos = n;
                            // }

                            // if n == 0 && self.dragx <= glyph.x {
                            //     caretx = left_edge;
                            //     self.cursor_pos = 0;
                            // }

                            // if n == res.glyphs.len() as u32 && self.hitx >= glyph.x + glyph.width {
                            //     caretx = right_edge;
                            //     self.cursor_pos = n;
                            // }

                            if self.hitx >= px && self.hitx < gx {
                                selectx = left_edge;

                                self.selection.anchor = index;
                                //cx.emit(TextEvent::SetAnchor(index));
                            }

                            if self.dragx >= px && self.dragx < gx {
                                caretx = left_edge;

                                self.selection.active = index;
                                //cx.emit(TextEvent::SetActive(index));
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

                        if self.selection.active as usize == text.len() && text.len() != 0
                        {
                            caretx = endx;
                        }

                        if self.selection.anchor as usize == text.len() && text.len() != 0
                        {
                            selectx = endx;
                        }
                    }

                    //Draw selection
                    // let select_width = (caretx - selectx).abs();
                    // if selectx > caretx {
                    //     let mut path = Path::new();
                    //     path.rect(caretx, sy, select_width, font_metrics.height());
                    //     canvas.fill_path(&mut path, Paint::color(Color::rgba(0, 0, 0, 64)));
                    // } else if caretx > selectx {
                    //     let mut path = Path::new();
                    //     path.rect(selectx, sy, select_width, font_metrics.height());
                    //     canvas.fill_path(&mut path, Paint::color(Color::rgba(0, 0, 0, 64)));
                    // }

                    //Draw selection
                    let select_width = (caretx - selectx).abs();
                    if selectx > caretx {
                        //path.rect(caretx, sy, select_width, font_metrics.height());
                        self.selection_entity.set_left(cx, Pixels(caretx.floor() - posx - 1.0));
                    } else if caretx > selectx {
                        //path.rect(selectx, sy, select_width, font_metrics.height());
                        self.selection_entity.set_left(cx, Pixels(selectx.floor() - posx - 1.0));
                    }

                    self.selection_entity.set_width(cx, Pixels(select_width));
                    self.selection_entity.set_height(cx, Pixels(font_metrics.height()));
                    self.selection_entity.set_top(cx, Stretch(1.0));
                    self.selection_entity.set_bottom(cx, Stretch(1.0));

                    // // Draw Caret
                    // let mut path = Path::new();
                    // path.rect(caretx.floor(), sy, 1.0, font_metrics.height());
                    // canvas.fill_path(&mut path, Paint::color(Color::rgba(247, 76, 0, 255)));

                    let caret_left = (caretx.floor() - posx - 1.0).max(0.0);

                    self.caret_entity.set_left(cx, Pixels(caret_left));
                    self.caret_entity.set_top(cx, Stretch(1.0));
                    self.caret_entity.set_bottom(cx, Stretch(1.0));
                    self.caret_entity.set_height(cx, Pixels(font_metrics.height()));
                }
            }
        }
    }

    pub fn insert_text(&mut self, cx: &mut Context, text: &str) {
        let text_length = text.len();
        self.text.edit(self.selection.range(), text);
        // Send event to edit text
        // if let Some(callback) = self.on_edit.take() {
        //     (callback)(cx, self.text.as_str().to_owned());

        //     self.on_edit = Some(callback);
        // }
        
        //cx.emit(TextEvent::SetCaret(text_data.selection.min() + text_length));
        self.selection = Selection::caret(self.selection.min() + text_length);

        cx.current.set_text(cx, self.text.as_str());
    }

    pub fn delete_text(&mut self, cx: &mut Context, movement: Movement) {
        // If selection is a range - delete the selection
        if !self.selection.is_caret() {
            self.text.edit(self.selection.range(), "");
            // if let Some(callback) = self.on_edit.take() {
            //     (callback)(cx, self.text.as_str().to_owned());
    
            //     self.on_edit = Some(callback);
            // }
            self.selection = Selection::caret(self.selection.min());
            //cx.emit(TextEvent::SetCaret(text_data.selection.min()))
            //println!("Selection: {:?}", self.selection);
        } else {
            match movement {
                Movement::Grapheme(Direction::Upstream) => {
                    if let Some(offset) =
                        self.text.prev_grapheme_offset(self.selection.active)
                    {
                        self.text.edit(offset..self.selection.active, "");
                        // if let Some(callback) = self.on_edit.take() {
                        //     (callback)(cx, self.text.as_str().to_owned());
                
                        //     self.on_edit = Some(callback);
                        // }
                        self.selection = Selection::caret(offset);
                        //cx.emit(TextEvent::SetCaret(offset));
                    }
                }

                Movement::Grapheme(Direction::Downstream) => {
                    if let Some(offset) =
                        self.text.next_grapheme_offset(self.selection.active)
                    {
                        self.text.edit(self.selection.active..offset, "");
                        // if let Some(callback) = self.on_edit.take() {
                        //     (callback)(cx, self.text.as_str().to_owned());
                
                        //     self.on_edit = Some(callback);
                        // }
                        self.selection = Selection::caret(self.selection.active);
                        //cx.emit(TextEvent::SetCaret(text_data.selection.active));
                    }
                }

                Movement::Word(Direction::Upstream) => {
                    if let Some(offset) = self.text.prev_word_offset(self.selection.active)
                    {
                        self.text.edit(offset..self.selection.active, "");
                        // if let Some(callback) = self.on_edit.take() {
                        //     (callback)(cx, self.text.as_str().to_owned());
                
                        //     self.on_edit = Some(callback);
                        // }
                        self.selection = Selection::caret(offset);
                        //cx.emit(TextEvent::SetCaret(offset));
                    }
                }

                Movement::Word(Direction::Downstream) => {
                    if let Some(offset) = self.text.next_word_offset(self.selection.active)
                    {
                        self.text.edit(self.selection.active..offset, "");
                        // if let Some(callback) = self.on_edit.take() {
                        //     (callback)(cx, self.text.as_str().to_owned());
                
                        //     self.on_edit = Some(callback);
                        // }
                        self.selection = Selection::caret(self.selection.active);
                        //cx.emit(TextEvent::SetCaret(text_data.selection.active));
                    }
                }

                _ => {}
            }
        }

        cx.current.set_text(cx, self.text.as_str());
    }

    pub fn move_cursor(&mut self, cx: &mut Context, movement: Movement, selection: bool) {
        match movement {
            Movement::Grapheme(Direction::Upstream) => {
                let active = if let Some(offset) =
                    self.text.prev_grapheme_offset(self.selection.active)
                {
                    self.selection.active = offset;
                    //cx.emit(TextEvent::SetActive(offset));
                    offset
                } else {
                    self.selection.active
                };

                if !selection {
                    self.selection.anchor = self.selection.active;
                    //cx.emit(TextEvent::SetAnchor(active));
                }
            }

            Movement::Grapheme(Direction::Downstream) => {
                let active = if let Some(offset) =
                    self.text.next_grapheme_offset(self.selection.active)
                {
                    self.selection.active = offset;
                    //cx.emit(TextEvent::SetActive(offset));
                    offset
                } else {
                    self.selection.active
                };

                if !selection {
                    self.selection.anchor = self.selection.active;
                    //cx.emit(TextEvent::SetAnchor(active));
                }
            }

            Movement::Word(Direction::Upstream) => {
                let active = if let Some(offset) =
                    self.text.prev_word_offset(self.selection.active)
                {
                    self.selection.active = offset;
                    //cx.emit(TextEvent::SetActive(offset));
                    offset
                } else {
                    self.selection.active
                };

                if !selection {
                    self.selection.anchor = self.selection.active;
                    //cx.emit(TextEvent::SetAnchor(active));
                }
            }

            Movement::Word(Direction::Downstream) => {
                let active = if let Some(offset) =
                    self.text.next_word_offset(self.selection.active)
                {
                    self.selection.active = offset;
                    //cx.emit(TextEvent::SetActive(offset));
                    offset
                } else {
                    self.selection.active
                };

                if !selection {
                    self.selection.anchor = self.selection.active;
                    //cx.emit(TextEvent::SetAnchor(active));
                }
            }

            _ => {}
        }
            
    }

    pub fn select_all(&mut self, cx: &mut Context) {
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
    SetHitX(f32),
    SetDragX(f32),
    Copy,
    Paste,

    // Helpers
    SetSelectionEntity(Entity),
    SetCaretEntity(Entity),
    SetOnEdit(Option<Arc<dyn Fn(&mut Context, String) + Send + Sync>>),
}


impl<T: 'static + EditableText> Model for TextboxData<T> {
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
                    self.edit = true;
                    self.selection_entity.set_visibility(cx, Visibility::Visible);
                    self.caret_entity.set_visibility(cx, Visibility::Visible);
                }

                TextEvent::EndEdit => {
                    self.edit = false;
                    self.selection_entity.set_visibility(cx, Visibility::Invisible);
                    self.caret_entity.set_visibility(cx, Visibility::Invisible);
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

                TextEvent::Copy => {
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

                TextEvent::Paste => {
                    #[cfg(feature = "clipboard")]
                    if self.edit {
                        if cx.modifiers.contains(Modifiers::CTRL) {
                            if let Ok(text) = cx.clipboard.get_contents() {
                                cx.emit(TextEvent::InsertText(text));
                            }
                        }
                    }
                }

                TextEvent::SetSelectionEntity(entity) => {
                    self.selection_entity = *entity;
                }

                TextEvent::SetCaretEntity(entity) => {
                    self.caret_entity = *entity;
                }

                TextEvent::SetOnEdit(on_edit) => {
                    self.on_edit = on_edit.clone();
                }
            }
        }
    }
}


pub struct Textbox<L: Lens> {
    //text_data: TextData,
    // text: T,
    // selection: Selection,
    // caret_entity: Entity,
    // selection_entity: Entity,
    // edit: bool,
    // hitx: f32,
    // dragx: f32,
    lens: L,
    on_edit: Option<Box<dyn Fn(&mut Context, String)>>,
    //on_submit: Option<Box<dyn Fn(&mut Context, &Self)>>,
}

impl<L: Lens> Textbox<L> 
where 
    <L as Lens>::Target: Data + EditableText,
{
    pub fn new<'a>(cx: &'a mut Context, lens: L) -> Handle<'a, Self> {
        // let selection = if let Some(source) = cx.data::<L::Source>() {
        //     let text = lens.view(source);
        //     Selection::new(0, text.len())
        // } else {
        //     Selection::caret(0)
        // };

        //let text_length = cx.data::<L::Source>().and_then(|source| Some(lens.view(source))).unwrap().len();

        //let text_length = text.len();
        Self {
            // text_data: TextData {
            //     //text: placeholder.to_string(),
            //     // selection: Selection::new(0, placeholder.len()),
            //     selection: Selection::new(0, placeholder.len()),
            // },
            // text: text.clone(),
            // selection: Selection::new(0, text_length),
            // caret_entity: Entity::null(),
            // selection_entity: Entity::null(),
            // edit: false,
            // hitx: -1.0,
            // dragx: -1.0,
            lens,
            on_edit: None,
            //on_submit: None,
        }
        .build2(cx, move |cx| {

            Binding::new(cx, lens.clone(), |cx, text|{
                if let Some(text_data) = cx.data::<TextboxData<L::Target>>() {
                    if !text_data.edit {
                        TextboxData {
                            text: text.get(cx).clone(),
                            selection: text_data.selection,
                            caret_entity: text_data.caret_entity,
                            selection_entity: text_data.selection_entity,
                            edit: text_data.edit,
                            hitx: -1.0,
                            dragx: -1.0,
                            on_edit: text_data.on_edit.clone(),
                        }.build(cx);
                        cx.current.set_text(cx, text.get(cx).clone().as_str());
                    }
                } else {
                    TextboxData::new(text.get(cx).clone()).build(cx);
                    cx.current.set_text(cx, text.get(cx).clone().as_str());
                }
            });
            
            

            // Selection
            let selection_entity = Element::new(cx)
                .left(Pixels(0.0))
                .width(Pixels(0.0))
                .class("selection")
                //.background_color(Color::rgba(100, 100, 200, 120))
                .position_type(PositionType::SelfDirected)
                .visibility(false)
                .entity();

            cx.emit(TextEvent::SetSelectionEntity(selection_entity));

            // Caret
            let caret_entity = Element::new(cx)
                .left(Pixels(0.0))
                .class("caret")
                //.background_color(Color::rgba(255, 0, 0, 255))
                .position_type(PositionType::SelfDirected)
                .width(Pixels(1.0))
                .visibility(false)
                .entity();

            cx.emit(TextEvent::SetCaretEntity(caret_entity));
        })
        //.text(text.as_str())
    }
}

impl<'a, L: Lens> Handle<'a, Textbox<L>> {
    pub fn on_edit<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context, String) + Send + Sync,
    {
        // if let Some(view) = self.cx.views.get_mut(&self.entity) {
        //     if let Some(textbox) = view.downcast_mut::<Textbox<L>>() {
        //         textbox.on_edit = Some(Box::new(callback));
        //     }
        // }

        self.cx.emit_to(self.entity, TextEvent::SetOnEdit(Some(Arc::new(callback))));

        self
    }
}

impl<L: Lens> View for Textbox<L> 
where 
    <L as Lens>::Target: Data + EditableText,
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
                            cx.captured = cx.current;
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
                        cx.captured = Entity::null();
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

                WindowEvent::MouseOver => {
                    cx.emit(WindowEvent::SetCursor(CursorIcon::Text));
                }

                WindowEvent::MouseOut => {
                    cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                }

                WindowEvent::CharInput(c) => {
                    //if self.edit {
                        if *c != '\u{1b}' && // Escape
                            *c != '\u{8}' && // Backspace
                            *c != '\u{7f}' && // Delete
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
                        
                        if let Some(source) = cx.data::<L::Source>() {
                            let text_data = self.lens.view(source);
                            let text = text_data.as_str().to_owned();
                            println!("Text: {}", text);

                            cx.emit(TextEvent::SelectAll);
                            cx.emit(TextEvent::InsertText(text));

                        };
                        cx.emit(TextEvent::EndEdit);

                        
                        //self.selection_entity.set_visibility(cx, Visibility::Invisible);
                        //self.caret_entity.set_visibility(cx, Visibility::Invisible);
                        cx.current.set_checked(cx, false);
                    }

                    Code::ArrowLeft => {
                        //if self.edit {
                            let movement = if cx.modifiers.contains(Modifiers::CTRL) {
                                Movement::Word(Direction::Upstream)
                            } else {
                                Movement::Grapheme(Direction::Upstream)
                            };

                            cx.emit(TextEvent::MoveCursor(movement, cx.modifiers.contains(Modifiers::SHIFT)));

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

                            cx.emit(TextEvent::MoveCursor(movement, cx.modifiers.contains(Modifiers::SHIFT)));


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
                                cx.emit(TextEvent::DeleteText(Movement::Grapheme(Direction::Downstream)));
                            }
                            //self.set_caret(cx, cx.current);
                        //}
                    }

                    Code::Escape => {
                        //self.edit = false;
                        cx.emit(TextEvent::StartEdit);
                        cx.current.set_checked(cx, false);
                    }

                    // TODO
                    Code::Home => {}

                    // TODO
                    Code::End => {}

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

                    Code::KeyC =>
                    {
                        cx.emit(TextEvent::Copy);
                    }

                    Code::KeyV =>
                    {
                        cx.emit(TextEvent::Paste);
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}
