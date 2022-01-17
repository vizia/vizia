use copypasta::ClipboardProvider;
use femtovg::{Paint, Align, Baseline};
use keyboard_types::Code;
use morphorm::{Units, PositionType};
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

use crate::style::PropGet;
use crate::{Context, Handle, MouseButton, View, WindowEvent, Selection, Label, ZStack, Binding, Lens, Model, Element, Units::*, Color, Action, Movement, EditableText, Modifiers, FontOrId, Entity, PropSet, CursorIcon, Event};

use crate::text::Direction;

pub struct TextData {
    text: String,
    selection: Selection,
}

impl TextData {
    pub fn insert_text(&mut self, text: &str) {
        let range = self.selection.range();
        self.text.replace_range(range, text);
        self.selection = Selection::caret(self.selection.min() + text.len());
    }

    pub fn delete_text(&mut self, movement: Movement) {
        // If selection is a range - delete the selection
        if !self.selection.is_caret() {
            self.text.replace_range(self.selection.range(), "");
            self.selection = Selection::caret(self.selection.min());
        } else {
            let range = self.selection.range();
            match movement {
                Movement::Grapheme(Direction::Upstream) => {
                    if let Some(offset) = self.text.prev_grapheme_offset(self.selection.active) {
                        self.text.replace_range(offset..self.selection.active, "");
                        self.selection = Selection::caret(offset);
                    }
                }

                Movement::Grapheme(Direction::Downstream) => {
                    if let Some(offset) = self.text.next_grapheme_offset(self.selection.active) {
                        self.text.replace_range(self.selection.active..offset, "");
                        self.selection = Selection::caret(self.selection.active);
                    }
                }

                Movement::Word(Direction::Upstream) => {
                    if let Some(offset) = self.text.prev_word_offset(self.selection.active) {
                        self.text.replace_range(offset..self.selection.active, "");
                        self.selection = Selection::caret(offset);
                    }
                }

                Movement::Word(Direction::Downstream) => {
                    if let Some(offset) = self.text.next_word_offset(self.selection.active) {
                        self.text.replace_range(self.selection.active..offset, "");
                        self.selection = Selection::caret(self.selection.active);
                    }
                }

                _=> {}
            }
        }
    }

    pub fn move_cursor(&mut self, movement: Movement, selection: bool) {
        match movement {
            Movement::Grapheme(Direction::Upstream) => {
                if let Some(offset) = self.text.prev_grapheme_offset(self.selection.active) {
                    self.selection.active = offset;
                }

                if !selection {
                    self.selection.anchor = self.selection.active;
                }
            }

            Movement::Grapheme(Direction::Downstream) => {
                if let Some(offset) = self.text.next_grapheme_offset(self.selection.active) {
                    self.selection.active = offset;
                }

                if !selection {
                    self.selection.anchor = self.selection.active;
                }
            }

            Movement::Word(Direction::Upstream) => {
                if let Some(offset) = self.text.prev_word_offset(self.selection.active) {
                    self.selection.active = offset;
                }

                if !selection {
                    self.selection.anchor = self.selection.active;
                }
            }

            Movement::Word(Direction::Downstream) => {
                if let Some(offset) = self.text.next_word_offset(self.selection.active) {
                    self.selection.active = offset;
                }

                if !selection {
                    self.selection.anchor = self.selection.active;
                }
            }

            _=> {}
        }
    }

    pub fn select_all(&mut self) {
        self.selection.anchor = 0;
        self.selection.active = self.text.len();
    }
}

#[derive(Lens)]
pub struct TextboxData {
    editing: bool,
}

#[derive(Debug)]
pub enum TextEvent {
    SetEditing(bool),
}


impl Model for TextboxData {
    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(text_event) = event.message.downcast() {
            match text_event {
                TextEvent::SetEditing(flag) => {
                    self.editing = *flag;
                }
            }
        }
    }
}



pub struct Textbox {
    text_data: TextData,
    edit: bool,
    hitx: f32,
    dragx: f32,
    on_submit: Option<Box<dyn Fn(&mut Context, &Self)>>,
}

impl Textbox {
    pub fn new<'a>(cx: &'a mut Context, placeholder: &'static str) -> Handle<'a, Self> {
        Self {
            text_data: TextData {
                text: placeholder.to_string(),
                // selection: Selection::new(0, placeholder.len()),
                selection: Selection::new(0, placeholder.len()),
            },
            edit: false,
            hitx: -1.0,
            dragx: -1.0,
            on_submit: None 
        }
        .build2(cx, move |cx|{


            TextboxData {
                editing: false,
            }.build(cx);
            // TextData {
            //     text: placeholder.to_owned(),
            //     selection: Selection::new(0, placeholder.len()),
            // }.build(cx);
            Binding::new(cx, TextboxData::editing, |cx, editing|{
                // Selection
                Element::new(cx)
                    .background_color(Color::rgba(100,100,200,120))
                    .position_type(PositionType::SelfDirected)
                    .visibility(editing);
                
                // Caret
                Element::new(cx)
                    .background_color(Color::rgba(255,0,0,255))
                    .position_type(PositionType::SelfDirected).width(Pixels(1.0))
                    .visibility(editing);
            });

        }).text(placeholder)
    }

    fn set_caret(&mut self, cx: &mut Context, entity: Entity) {

        // TODO - replace this with something better
        let selection = cx.tree.get_child(entity, 1).unwrap();
        let caret = cx.tree.get_child(entity, 2).unwrap();
        
        let posx = cx.cache.get_posx(entity);
        let posy = cx.cache.get_posy(entity);
        let width = cx.cache.get_width(entity);
        let height = cx.cache.get_height(entity);
        
        if let Some(text) = cx.style.text.get(entity) {
            let font = cx.style.font.get(entity).cloned().unwrap_or_default();

            // TODO - This should probably be cached in cx to save look-up time
            let default_font = cx.resource_manager.fonts.get(&cx.style.default_font).and_then(|font|{
                match font {
                    FontOrId::Id(id) => Some(id),
                    _=> None,
                }
            }).expect("Failed to find default font");

            let font_id = cx.resource_manager.fonts.get(&font).and_then(|font|{
                match font {
                    FontOrId::Id(id) => Some(id),
                    _=> None,
                }
            }).unwrap_or(default_font);


            let mut x = posx;
            let mut y = posy;
            //let mut sx = posx;
            let mut sy = posy;

            let text_string = text.to_owned();

            let font_size = cx.style.font_size.get(entity).cloned().unwrap_or(16.0);

            let mut paint = Paint::default();
            paint.set_font_size(font_size);
            paint.set_font(&[font_id.clone()]);

            let font_metrics = cx.text_context
                .measure_font(paint)
                .expect("Failed to read font metrics");

            
            let parent = cx
                .tree
                .get_parent(entity)
                .expect("Failed to find parent somehow");

            let parent_posx = cx.cache.get_posx(parent);
            let parent_posy = cx.cache.get_posy(parent);
            let parent_width = cx.cache.get_width(parent);
            let parent_height = cx.cache.get_height(parent);
            
            let border_width = match cx
                .style
                .border_width
                .get(entity)
                .cloned()
                .unwrap_or_default()
            {
                Units::Pixels(val) => val,
                Units::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            // TODO - Move this to a text layout system and include constraints
            let child_left = cx
                .style
                .child_left
                .get(entity)
                .cloned()
                .unwrap_or_default();
            let child_right = cx
                .style
                .child_right
                .get(entity)
                .cloned()
                .unwrap_or_default();
            let child_top = cx
                .style
                .child_top
                .get(entity)
                .cloned()
                .unwrap_or_default();
            let child_bottom = cx
                .style
                .child_bottom
                .get(entity)
                .cloned()
                .unwrap_or_default();

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
                        sy = y - font_metrics.height();
                        Baseline::Bottom
                    }

                    Units::Stretch(_) => {
                        y += 0.5 * height;
                        sy = y - font_metrics.height() * 0.5;
                        Baseline::Middle
                    }

                    _ => Baseline::Top,
                },

                _ => Baseline::Top,
            };

            paint.set_text_align(align);
            paint.set_text_baseline(baseline);

            if let Ok(res) = cx.text_context.measure_text(x, y, &text_string, paint) {

                let padding_left = match cx.style.child_left.get(entity).unwrap_or(&Units::Auto) {
                    Units::Pixels(val) => *val,
                    _ => 0.0,
                };
        
                let padding_right = match cx.style.child_right.get(entity).unwrap_or(&Units::Auto) {
                    Units::Pixels(val) => *val,
                    _ => 0.0,
                };

                let text_width = res.width();
                //let mut glyph_positions = res.glyphs.iter().peekable();

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
                            self.text_data.selection.anchor = 0;
                            startx
                        } else {
                            self.text_data.selection.anchor = text.len();
                            endx
                        };

                        caretx = if self.dragx < startx + text_width / 2.0 {
                            self.text_data.selection.active = 0;
                            startx
                        } else {
                            self.text_data.selection.active = text.len();
                            endx
                        };

                        let mut px = x;

                        for (glyph, (index, _)) in res.glyphs.iter().zip(text_string.grapheme_indices(true)) {
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

                                self.text_data.selection.anchor = index;
                            }

                            if self.dragx >= px && self.dragx < gx {
                                caretx = left_edge;

                                self.text_data.selection.active = index;
                            }

                            px = gx;
                        }
                    } else {
                        let mut n = 0;
                        //println!("cursor: {}", self.cursor_pos);
                        //let mut start_x = 0.0;


                        for (glyph, (index, _)) in res.glyphs.iter().zip(text_string.grapheme_indices(true)) {
                            if index == self.text_data.selection.active {
                                caretx = glyph.x;
                            }

                            if index == self.text_data.selection.anchor {
                                selectx = glyph.x;
                            }
                        }

                        if self.text_data.selection.active as usize == text.len() && text.len() != 0 {
                            caretx = endx;
                        }

                        if self.text_data.selection.anchor as usize == text.len() && text.len() != 0 {
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
                        selection.set_left(cx, Pixels(caretx.floor() - posx - 1.0));
                    } else if caretx > selectx {
                        //path.rect(selectx, sy, select_width, font_metrics.height());
                        selection.set_left(cx, Pixels(selectx.floor() - posx - 1.0));
                        
                    }

                    selection.set_width(cx, Pixels(select_width));
                    selection.set_height(cx, Pixels(font_metrics.height()));
                    selection.set_top(cx, Stretch(1.0));
                    selection.set_bottom(cx, Stretch(1.0));

                    // // Draw Caret
                    // let mut path = Path::new();
                    // path.rect(caretx.floor(), sy, 1.0, font_metrics.height());
                    // canvas.fill_path(&mut path, Paint::color(Color::rgba(247, 76, 0, 255)));

                    let caret_left = (caretx.floor() - posx - 1.0).max(0.0);

                    caret.set_left(cx, Pixels(caret_left));
                    caret.set_top(cx, Stretch(1.0));
                    caret.set_bottom(cx, Stretch(1.0));
                    caret.set_height(cx, Pixels(font_metrics.height()));
                }
            }
        }
    }
}

impl<'a> Handle<'a, Textbox> {
    pub fn on_submit<F>(self, cx: &mut Context, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context, &Textbox),
    {
        if let Some(view) = cx.views.get_mut(&self.entity) {
            if let Some(textbox) = view.downcast_mut::<Textbox>() {
                textbox.on_submit = Some(Box::new(callback));
            }
        }

        self
    }
}

impl View for Textbox {


    fn element(&self) -> Option<String> {
        Some("textbox".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if cx.current.is_over(cx) {
                        if !self.edit {
                            self.edit = true;
                            cx.emit(TextEvent::SetEditing(true));
                            cx.focused = cx.current;
                            cx.captured = cx.current;
                            cx.current.set_checked(cx, true);
                        }
    
                        // Hit test
                        if self.edit {
                            self.hitx = cx.mouse.cursorx;
                            self.dragx = cx.mouse.cursorx;
                        }
                        self.set_caret(cx, cx.current);
                    } else {
                        cx.captured = Entity::null();
                        cx.current.set_checked(cx, false);
                        self.edit = false;
                        cx.emit(TextEvent::SetEditing(false));
                        // Forward event to hovered
                        cx.event_queue.push_back(Event::new(WindowEvent::MouseDown(MouseButton::Left)).target(cx.hovered));
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    self.hitx = -1.0;
                    self.set_caret(cx, cx.current);
                }

                WindowEvent::MouseMove(x, _) => {
                    if self.hitx != -1.0 {
                        self.dragx = *x;

                        self.set_caret(cx, cx.current);
                    }
                }

                WindowEvent::MouseOver => {
                    cx.emit(WindowEvent::SetCursor(CursorIcon::Text));
                }

                WindowEvent::MouseOut => {
                    cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                }

                WindowEvent::CharInput(c) => {
                    if self.edit {
                        if  *c != '\u{1b}' && // Escape
                            *c != '\u{8}' && // Backspace
                            *c != '\u{7f}' && // Delete
                            !cx.modifiers.contains(Modifiers::CTRL)
                        {
                            self.text_data.insert_text(&String::from(*c));
                            cx.style.text.insert(cx.current, self.text_data.text.clone());
                        }
    
                        self.set_caret(cx, cx.current);
                    }
                }

                WindowEvent::KeyDown(code, key) => match code {
                    Code::Enter => {
                        // Finish editing
                        self.edit = false;
                        cx.emit(TextEvent::SetEditing(false));
                        cx.current.set_checked(cx, false);
                    }

                    Code::ArrowLeft => {
                        if self.edit {
                            let movement = if cx.modifiers.contains(Modifiers::CTRL) {
                                Movement::Word(Direction::Upstream)
                            } else {
                                Movement::Grapheme(Direction::Upstream)
                            };
    
                            self.text_data.move_cursor(movement, cx.modifiers.contains(Modifiers::SHIFT));
    
                            println!("{:?}", self.text_data.selection);
                            self.set_caret(cx, cx.current);
                        }
                    }

                    Code::ArrowRight => {
                        if self.edit {
                            let movement = if cx.modifiers.contains(Modifiers::CTRL) {
                                Movement::Word(Direction::Downstream)
                            } else {
                                Movement::Grapheme(Direction::Downstream)
                            };
    
                            self.text_data.move_cursor(movement, cx.modifiers.contains(Modifiers::SHIFT));
                            
                            self.set_caret(cx, cx.current);
                        }
                    }

                    // TODO
                    Code::ArrowUp => {

                    }

                    // TODO
                    Code::ArrowDown => {

                    }

                    Code::Backspace => {
                        if self.edit {
                            if cx.modifiers.contains(Modifiers::CTRL) {
                                self.text_data.delete_text(Movement::Word(Direction::Upstream));
                            } else {
                                self.text_data.delete_text(Movement::Grapheme(Direction::Upstream));
                            }
    
                            cx.style.text.insert(cx.current, self.text_data.text.clone());
                            self.set_caret(cx, cx.current);
                        }
                    }

                    Code::Delete => {
                        if self.edit {
                            if cx.modifiers.contains(Modifiers::CTRL) {
                                self.text_data.delete_text(Movement::Word(Direction::Downstream));
                            } else {
                                self.text_data.delete_text(Movement::Grapheme(Direction::Downstream));
                            }
                            cx.style.text.insert(cx.current, self.text_data.text.clone());
                            self.set_caret(cx, cx.current);
                        }
                    }

                    Code::Escape => {
                        self.edit = false;
                        cx.emit(TextEvent::SetEditing(false));
                        cx.current.set_checked(cx, false);
                    }

                    // TODO
                    Code::Home => {

                    }

                    // TODO
                    Code::End => {

                    }

                    // TODO
                    Code::PageUp => {

                    }

                    // TODO
                    Code::PageDown => {

                    }

                    Code::KeyA => {
                        if self.edit {
                            if cx.modifiers.contains(Modifiers::CTRL) {
                                self.text_data.select_all();
                            }
                        }
                    }

                    Code::KeyC => {
                        if self.edit {
                            if cx.modifiers.contains(Modifiers::CTRL) {
                                cx.clipboard.set_contents(self.text_data.text[self.text_data.selection.range()].to_string());
                            }
                        }
                    }

                    Code::KeyV => {
                        if self.edit {
                            if cx.modifiers.contains(Modifiers::CTRL) {
                                if let Ok(text) = cx.clipboard.get_contents() {
                                    self.text_data.insert_text(&text);
                                    cx.style.text.insert(cx.current, self.text_data.text.clone());
                                    self.set_caret(cx, cx.current);
                                }
                            }
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}
