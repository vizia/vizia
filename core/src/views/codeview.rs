use std::ops::Range;

#[cfg(feature = "clipboard")]
use copypasta::ClipboardProvider;

use femtovg::{Align, Baseline, Paint, Path, PixelFormat, ImageFlags, RenderTarget};
use keyboard_types::Code;
use morphorm::{PositionType, Units, Cache};
use unicode_segmentation::UnicodeSegmentation;

use crate::style::{PropGet, GradientDirection};
use crate::{
    Binding, Context, CursorIcon, Data, EditableText, Element, Entity, Event, FontOrId, Handle,
    Lens, Model, Modifiers, MouseButton, Movement, PropSet, Selection, Units::*, View, WindowEvent, Visibility, BorderCornerShape, Overflow,
};

use crate::text::Direction;


pub struct CodeView<T>
where
    T: EditableText,
{
    //text_data: TextData,
    text: T,
    selection: Selection,
    caret_entity: Entity,
    selection_entity: Entity,
    edit: bool,
    hitx: f32,
    dragx: f32,
    on_edit: Option<Box<dyn Fn(&mut Context, String)>>,
    //on_submit: Option<Box<dyn Fn(&mut Context, &Self)>>,
}

impl<T> CodeView<T>
where
    T: 'static + EditableText,
{
    pub fn new<'a>(cx: &'a mut Context, text: T) -> Handle<'a, Self> {
        // let selection = if let Some(source) = cx.data::<L::Source>() {
        //     let text = lens.view(source);
        //     Selection::new(0, text.len())
        // } else {
        //     Selection::caret(0)
        // };

        //let text_length = cx.data::<L::Source>().and_then(|source| Some(lens.view(source))).unwrap().len();

        let text_length = text.len();
        Self {
            // text_data: TextData {
            //     //text: placeholder.to_string(),
            //     // selection: Selection::new(0, placeholder.len()),
            //     selection: Selection::new(0, placeholder.len()),
            // },
            text: text.clone(),
            selection: Selection::new(0, text_length),
            caret_entity: Entity::null(),
            selection_entity: Entity::null(),
            edit: false,
            hitx: -1.0,
            dragx: -1.0,
            on_edit: None,
            //on_submit: None,
        }
        .build3(cx, move |codeview, cx| {

            if !codeview.edit {
                codeview.text = text;
            }
            cx.current.set_text(cx, &codeview.text.as_str());

            // Selection
            codeview.selection_entity = Element::new(cx)
                .left(Pixels(0.0))
                .width(Pixels(0.0))
                .class("selection")
                //.background_color(Color::rgba(100, 100, 200, 120))
                .position_type(PositionType::SelfDirected)
                .visibility(false)
                .entity();

            // Caret
            codeview.caret_entity = Element::new(cx)
                .left(Pixels(0.0))
                .top(Pixels(0.0))
                .class("caret")
                //.background_color(Color::rgba(255, 0, 0, 255))
                .position_type(PositionType::SelfDirected)
                .width(Pixels(1.0))
                .visibility(true)
                .overflow(Overflow::Visible)
                .entity();
        })
        //.text(text.as_str())
    }

    fn set_caret(&mut self, cx: &mut Context, entity: Entity) {
        // TODO - replace this with something better
        //let selection = cx.tree.get_child(entity, 0).unwrap();
        //let caret = cx.tree.get_child(entity, 1).unwrap();

        let bounds = cx.cache.get_bounds(entity);

        let border_width = match cx.style.border_width.get(entity).cloned().unwrap_or_default() {
            Units::Pixels(val) => val,
            Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
            _ => 0.0,
        };

        let font_color =
            cx.style.font_color.get(entity).cloned().unwrap_or(crate::Color::rgb(0, 0, 0));

        let opacity = cx.cache.get_opacity(entity);
        // let posx = cx.cache.get_posx(entity);
        // let posy = cx.cache.get_posy(entity);
        // let width = cx.cache.get_width(entity);
        // let height = cx.cache.get_height(entity);

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

            // let mut x = posx + (border_width / 2.0);
            // let mut y = posy + (border_width / 2.0);

            let mut x = bounds.x;
            let mut y = bounds.y;

            let text_string = text.to_owned();

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
                        x += bounds.w - val - border_width;
                        Align::Right
                    }

                    Units::Stretch(_) => {
                        x += 0.5 * bounds.w;
                        Align::Center
                    }

                    _ => Align::Right,
                },

                _ => Align::Left,
            };

            let baseline = match child_top {
                Units::Pixels(val) => match child_bottom {
                    Units::Stretch(_) | Units::Auto => {
                        y += val + border_width;
                        Baseline::Top
                    }

                    _ => Baseline::Top,
                },

                Units::Stretch(_) => match child_bottom {
                    Units::Pixels(val) => {
                        y += bounds.h - val - border_width;
                        Baseline::Bottom
                    }

                    Units::Stretch(_) => {
                        y += 0.5 * bounds.h;
                        Baseline::Middle
                    }

                    _ => Baseline::Bottom,
                },

                _ => Baseline::Top,
            };

            let mut font_color: femtovg::Color = font_color.into();
            font_color.set_alphaf(font_color.a * opacity);

            let font_size = cx.style.font_size.get(entity).cloned().unwrap_or(16.0);

            

            let mut paint = Paint::color(font_color);
            paint.set_font_size(font_size);
            paint.set_font(&[font_id.clone()]);
            paint.set_text_align(align);
            paint.set_text_baseline(baseline);
            paint.set_anti_alias(false);

            let font_metrics = cx.text_context.measure_font(paint).expect("Error measuring font");

            

            if let Ok(lines) = cx.text_context.break_text_vec(std::f32::MAX, text, paint) {

                let mut count = 0;

                let mut caretx = x;
                let mut carety = y;
                //let mut selectx = caretx;

                for (line, line_range) in lines.into_iter().enumerate() {
                    let text_str = &text_string[line_range];
                    if let Ok(res) = cx.text_context.measure_text(x, y, text_str, paint) {
                        let text_width = res.width();
                    
                        if self.edit {
                            let startx= if let Some(first_glyph) = res.glyphs.first() {
                                first_glyph.x
                            } else {
                                0.0
                            };
                            //let startx = x - text_width / 2.0;
                            let endx = startx + text_width;
        
                            // if self.hitx != -1.0 {
                            //     //let endx = res.glyphs.last().unwrap().x + res.glyphs.last().unwrap().w;
        
                            //     selectx = if self.hitx < startx + text_width / 2.0 {
                            //         self.selection.anchor = 0;
                            //         //cx.emit(TextEvent::SetAnchor(0));
                            //         startx
                            //     } else {
                            //         self.selection.anchor = text_str.len();
                            //         //cx.emit(TextEvent::SetAnchor(text.len()));
                            //         endx
                            //     };
        
                            //     caretx = if self.dragx < startx + text_width / 2.0 {
                            //         self.selection.active = 0;
                            //         //cx.emit(TextEvent::SetActive(0));
                            //         startx
                            //     } else {
                            //         self.selection.active = text_str.len();
                            //         //cx.emit(TextEvent::SetActive(text.len()));
                            //         endx
                            //     };

                            //     let mut px = x;
        
                            //     for (glyph, (index, _)) in
                            //         res.glyphs.iter().zip(text_string.grapheme_indices(true))
                            //     {
                            //         let left_edge = glyph.x;
                            //         let right_edge = left_edge + glyph.width;
                            //         let gx = left_edge * 0.3 + right_edge * 0.7;
        
                            //         //println!("{} {} {}", self.hitx, left_edge, right_edge);
        
                            //         // if n == 0 && self.hitx <= glyph.x {
                            //         //     selectx = left_edge;
                            //         //     self.select_pos = 0;
                            //         // }
        
                            //         // if n == res.glyphs.len() as u32 && self.hitx >= glyph.x + glyph.width {
                            //         //     selectx = right_edge;
                            //         //     self.select_pos = n;
                            //         // }
        
                            //         // if n == 0 && self.dragx <= glyph.x {
                            //         //     caretx = left_edge;
                            //         //     self.cursor_pos = 0;
                            //         // }
        
                            //         // if n == res.glyphs.len() as u32 && self.hitx >= glyph.x + glyph.width {
                            //         //     caretx = right_edge;
                            //         //     self.cursor_pos = n;
                            //         // }
        
                            //         if self.hitx >= px && self.hitx < gx {
                            //             selectx = left_edge;
        
                            //             self.selection.anchor = index;
                            //             //cx.emit(TextEvent::SetAnchor(index));
                            //         }
        
                            //         if self.dragx >= px && self.dragx < gx {
                            //             caretx = left_edge;
        
                            //             self.selection.active = index;
                            //             //cx.emit(TextEvent::SetActive(index));
                            //         }
        
                            //         px = gx;
                            //     }
                            // } else {
                                   
                                    for (glyph, (index, _)) in
                                        res.glyphs.iter().zip(text_str.grapheme_indices(true))
                                    {

                                        if index + count == self.selection.active {
                                            caretx = glyph.x;
                                            carety += font_metrics.height() * line as f32;
                                            break;
                                        }                                        

                                    }
                                    
                                    if self.selection.active == text_str.len() + count && text_str.len() != 0
                                    {
                                        caretx = endx;
                                    }

                                

                                // if self.selection.anchor >= count {
                                //     let anchor = self.selection.anchor - count;
                                //     for (glyph, (index, _)) in
                                //         res.glyphs.iter().zip(text_str.grapheme_indices(true))
                                //     {
                                //         if index == anchor {
                                //             selectx = glyph.x;
                                //             break;
                                //         }
                                //     }

                                //     if anchor as usize == text_str.len() && text_str.len() != 0
                                //     {
                                //         selectx = endx;
                                //     }
                                        
                                // }
                        }
                    }
                    y += font_metrics.height();
                    count += text_str.len();
                }

                // let select_width = (caretx - selectx).abs();
                // if selectx > caretx {
                //     self.selection_entity.set_left(cx, Pixels(caretx.floor() - bounds.x - 1.0));
                // } else if caretx > selectx {
                //     //path.rect(selectx, sy, select_width, font_metrics.height());
                //     self.selection_entity.set_left(cx, Pixels(selectx.floor() - bounds.x - 1.0));
                // }

                // self.selection_entity.set_width(cx, Pixels(select_width));
                // self.selection_entity.set_height(cx, Pixels(font_metrics.height()));
                // self.selection_entity.set_top(cx, Stretch(1.0));
                // self.selection_entity.set_bottom(cx, Stretch(1.0));

                let caret_left = (caretx.floor() - bounds.x - 1.0).max(0.0);
                let caret_top = carety.floor() - bounds.y - font_metrics.height() / 2.0;

                self.caret_entity.set_left(cx, Pixels(caret_left));
                self.caret_entity.set_top(cx, Pixels(caret_top));
                self.caret_entity.set_height(cx, Pixels(font_metrics.height()));
            }
            
        }
    }

    pub fn insert_text(&mut self, cx: &mut Context, text: String) {
        let text_length = text.len();
        self.text.edit(self.selection.range(), text);
        // Send event to edit text
        if let Some(callback) = self.on_edit.take() {
            (callback)(cx, self.text.as_str().to_owned());

            self.on_edit = Some(callback);
        }
        
        //cx.emit(TextEvent::SetCaret(text_data.selection.min() + text_length));
        self.selection = Selection::caret(self.selection.min() + text_length);

        cx.current.set_text(cx, self.text.as_str());
    }

    pub fn delete_text(&mut self, cx: &mut Context, movement: Movement) {
        // If selection is a range - delete the selection
        if !self.selection.is_caret() {
            self.text.edit(self.selection.range(), "");
            if let Some(callback) = self.on_edit.take() {
                (callback)(cx, self.text.as_str().to_owned());
    
                self.on_edit = Some(callback);
            }
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
                        if let Some(callback) = self.on_edit.take() {
                            (callback)(cx, self.text.as_str().to_owned());
                
                            self.on_edit = Some(callback);
                        }
                        self.selection = Selection::caret(offset);
                        //cx.emit(TextEvent::SetCaret(offset));
                    }
                }

                Movement::Grapheme(Direction::Downstream) => {
                    if let Some(offset) =
                        self.text.next_grapheme_offset(self.selection.active)
                    {
                        self.text.edit(self.selection.active..offset, "");
                        if let Some(callback) = self.on_edit.take() {
                            (callback)(cx, self.text.as_str().to_owned());
                
                            self.on_edit = Some(callback);
                        }
                        self.selection = Selection::caret(self.selection.active);
                        //cx.emit(TextEvent::SetCaret(text_data.selection.active));
                    }
                }

                Movement::Word(Direction::Upstream) => {
                    if let Some(offset) = self.text.prev_word_offset(self.selection.active)
                    {
                        self.text.edit(offset..self.selection.active, "");
                        if let Some(callback) = self.on_edit.take() {
                            (callback)(cx, self.text.as_str().to_owned());
                
                            self.on_edit = Some(callback);
                        }
                        self.selection = Selection::caret(offset);
                        //cx.emit(TextEvent::SetCaret(offset));
                    }
                }

                Movement::Word(Direction::Downstream) => {
                    if let Some(offset) = self.text.next_word_offset(self.selection.active)
                    {
                        self.text.edit(self.selection.active..offset, "");
                        if let Some(callback) = self.on_edit.take() {
                            (callback)(cx, self.text.as_str().to_owned());
                
                            self.on_edit = Some(callback);
                        }
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

impl<'a, T> Handle<'a, CodeView<T>>
where
    T: 'static + EditableText,
{
    pub fn on_edit<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context, String),
    {
        if let Some(view) = self.cx.views.get_mut(&self.entity) {
            if let Some(codeview) = view.downcast_mut::<CodeView<T>>() {
                codeview.on_edit = Some(Box::new(callback));
            }
        }

        self
    }
}

impl<T: 'static> View for CodeView<T>
where
    T: EditableText,
{
    fn element(&self) -> Option<String> {
        Some("codeview".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {

        //let selection = cx.tree.get_child(cx.current, 0).unwrap();
        //let caret = cx.tree.get_child(cx.current, 1).unwrap();

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if cx.current.is_over(cx) {
                        if !self.edit {
                            self.edit = true;
                            self.selection_entity.set_visibility(cx, Visibility::Visible);
                            self.caret_entity.set_visibility(cx, Visibility::Visible);
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
                        //cx.emit(TextEvent::SetEditing(false));
                        // Forward event to hovered
                        cx.event_queue.push_back(
                            Event::new(WindowEvent::MouseDown(MouseButton::Left))
                                .target(cx.hovered),
                        );
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
                        if *c != '\u{1b}' && // Escape
                            *c != '\u{8}' && // Backspace
                            *c != '\u{7f}' && // Delete
                            !cx.modifiers.contains(Modifiers::CTRL)
                        {
                            println!("char: {:?}", c);
                            self.insert_text(cx, String::from(*c));
                            //cx.style.text.insert(cx.current, self.text_data.text.clone());
                        }

                        self.set_caret(cx, cx.current);
                    }
                }

                WindowEvent::KeyDown(code, _) => match code {
                    Code::Enter => {
                        // Finish editing
                        //self.edit = false;
                        //cx.emit(TextEvent::SetEditing(false));
                        //self.selection_entity.set_visibility(cx, Visibility::Invisible);
                        //self.caret_entity.set_visibility(cx, Visibility::Invisible);
                        //cx.current.set_checked(cx, false);
                        //self.insert_text(cx, String::from("\n"))
                    }

                    Code::ArrowLeft => {
                        if self.edit {
                            let movement = if cx.modifiers.contains(Modifiers::CTRL) {
                                Movement::Word(Direction::Upstream)
                            } else {
                                Movement::Grapheme(Direction::Upstream)
                            };

                            self.move_cursor(cx, movement, cx.modifiers.contains(Modifiers::SHIFT));

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

                            self.move_cursor(cx, movement, cx.modifiers.contains(Modifiers::SHIFT));

                            self.set_caret(cx, cx.current);
                        }
                    }

                    // TODO
                    Code::ArrowUp => {}

                    // TODO
                    Code::ArrowDown => {}

                    Code::Backspace => {
                        if self.edit {
                            if cx.modifiers.contains(Modifiers::CTRL) {
                                self.delete_text(cx, Movement::Word(Direction::Upstream));
                            } else {
                                self.delete_text(cx, Movement::Grapheme(Direction::Upstream));
                            }

                            self.set_caret(cx, cx.current);
                        }
                    }

                    Code::Delete => {
                        if self.edit {
                            if cx.modifiers.contains(Modifiers::CTRL) {
                                self.delete_text(cx, Movement::Word(Direction::Downstream));
                            } else {
                                self.delete_text(cx, Movement::Grapheme(Direction::Downstream));
                            }
                            self.set_caret(cx, cx.current);
                        }
                    }

                    Code::Escape => {
                        self.edit = false;
                        //cx.emit(TextEvent::SetEditing(false));
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
                        if self.edit {
                            if cx.modifiers.contains(Modifiers::CTRL) {
                                self.select_all(cx);
                            }
                        }
                    }

                    Code::KeyC =>
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

                    Code::KeyV =>
                    {
                        #[cfg(feature = "clipboard")]
                        if self.edit {
                            if cx.modifiers.contains(Modifiers::CTRL) {
                                if let Ok(text) = cx.clipboard.get_contents() {
                                    self.insert_text(cx, text);
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

    fn draw(&self, cx: &mut Context, canvas: &mut crate::Canvas) {
        let entity = cx.current;

        let bounds = cx.cache.get_bounds(entity);

        //Skip widgets with no width or no height
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let _padding_left = match cx.style.child_left.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let _padding_right = match cx.style.child_right.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let _padding_top = match cx.style.child_top.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let _padding_bottom = match cx.style.child_bottom.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let background_color = cx.style.background_color.get(entity).cloned().unwrap_or_default();

        let font_color =
            cx.style.font_color.get(entity).cloned().unwrap_or(crate::Color::rgb(0, 0, 0));

        let border_color = cx.style.border_color.get(entity).cloned().unwrap_or_default();

        let parent = cx
            .tree
            .get_parent(entity)
            .expect(&format!("Failed to find parent somehow: {}", entity));

        let parent_width = cx.cache.get_width(parent);
        let parent_height = cx.cache.get_height(parent);

        let border_shape_top_left =
            cx.style.border_shape_top_left.get(entity).cloned().unwrap_or_default();

        let border_shape_top_right =
            cx.style.border_shape_top_right.get(entity).cloned().unwrap_or_default();

        let border_shape_bottom_left =
            cx.style.border_shape_bottom_left.get(entity).cloned().unwrap_or_default();

        let border_shape_bottom_right =
            cx.style.border_shape_bottom_right.get(entity).cloned().unwrap_or_default();

        let border_radius_top_left =
            match cx.style.border_radius_top_left.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
                _ => 0.0,
            };

        let border_radius_top_right =
            match cx.style.border_radius_top_right.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
                _ => 0.0,
            };

        let border_radius_bottom_left =
            match cx.style.border_radius_bottom_left.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
                _ => 0.0,
            };

        let border_radius_bottom_right =
            match cx.style.border_radius_bottom_right.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
                _ => 0.0,
            };

        let opacity = cx.cache.get_opacity(entity);

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut border_color: femtovg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);

        let border_width = match cx.style.border_width.get(entity).cloned().unwrap_or_default() {
            Units::Pixels(val) => val,
            Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
            _ => 0.0,
        };

        let outer_shadow_h_offset =
            match cx.style.outer_shadow_h_offset.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let outer_shadow_v_offset =
            match cx.style.outer_shadow_v_offset.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let outer_shadow_blur =
            match cx.style.outer_shadow_blur.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let outer_shadow_color =
            cx.style.outer_shadow_color.get(entity).cloned().unwrap_or_default();

        let mut outer_shadow_color: femtovg::Color = outer_shadow_color.into();
        outer_shadow_color.set_alphaf(outer_shadow_color.a * opacity);

        let _inner_shadow_h_offset =
            match cx.style.inner_shadow_h_offset.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let _inner_shadow_v_offset =
            match cx.style.inner_shadow_v_offset.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let _inner_shadow_blur =
            match cx.style.inner_shadow_blur.get(entity).cloned().unwrap_or_default() {
                Units::Pixels(val) => val,
                Units::Percentage(val) => bounds.w * (val / 100.0),
                _ => 0.0,
            };

        let inner_shadow_color =
            cx.style.inner_shadow_color.get(entity).cloned().unwrap_or_default();

        let mut inner_shadow_color: femtovg::Color = inner_shadow_color.into();
        inner_shadow_color.set_alphaf(inner_shadow_color.a * opacity);

        // // Draw outer shadow
        // let mut path = Path::new();
        // path.rounded_rect_varying(
        //     bounds.x - outer_shadow_blur + outer_shadow_h_offset,
        //     bounds.y - outer_shadow_blur + outer_shadow_v_offset,
        //     bounds.w + 2.0 * outer_shadow_blur,
        //     bounds.h + 2.0 * outer_shadow_blur,
        //     border_radius_top_left,
        //     border_radius_top_right,
        //     border_radius_bottom_right,
        //     border_radius_bottom_left,
        // );
        // path.rounded_rect_varying(
        //     bounds.x,
        //     bounds.y,
        //     bounds.w,
        //     bounds.h,
        //     border_radius_top_left,
        //     border_radius_top_right,
        //     border_radius_bottom_right,
        //     border_radius_bottom_left,
        // );
        // path.solidity(Solidity::Hole);

        // let mut paint = Paint::box_gradient(
        //     bounds.x + outer_shadow_h_offset,
        //     bounds.y + outer_shadow_v_offset,
        //     bounds.w,
        //     bounds.h,
        //     border_radius_top_left
        //         .max(border_radius_top_right)
        //         .max(border_radius_bottom_left)
        //         .max(border_radius_bottom_right),
        //     outer_shadow_blur,
        //     outer_shadow_color,
        //     femtovg::Color::rgba(0, 0, 0, 0),
        // );

        // canvas.fill_path(&mut path, paint);

        //let start = std::time::Instant::now();
        let mut path = Path::new();

        if border_radius_bottom_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_bottom_right == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_right == (bounds.w - 2.0 * border_width) / 2.0
        {
            path.circle(
                bounds.x + (border_width / 2.0) + (bounds.w - border_width) / 2.0,
                bounds.y + (border_width / 2.0) + (bounds.h - border_width) / 2.0,
                bounds.w / 2.0,
            );
        } else {
            let x = bounds.x + border_width / 2.0;
            let y = bounds.y + border_width / 2.0;
            let w = bounds.w - border_width;
            let h = bounds.h - border_width;
            let halfw = w.abs() * 0.5;
            let halfh = h.abs() * 0.5;

            let rx_bl = border_radius_bottom_left.min(halfw) * w.signum();
            let ry_bl = border_radius_bottom_left.min(halfh) * h.signum();

            let rx_br = border_radius_bottom_right.min(halfw) * w.signum();
            let ry_br = border_radius_bottom_right.min(halfh) * h.signum();

            let rx_tr = border_radius_top_right.min(halfw) * w.signum();
            let ry_tr = border_radius_top_right.min(halfh) * h.signum();

            let rx_tl = border_radius_top_left.min(halfw) * w.signum();
            let ry_tl = border_radius_top_left.min(halfh) * h.signum();

            path.move_to(x, y + ry_tl);
            path.line_to(x, y + h - ry_bl);
            if border_radius_bottom_left != 0.0 {
                if border_shape_bottom_left == BorderCornerShape::Round {
                    path.bezier_to(
                        x,
                        y + h - ry_bl * (1.0 - KAPPA90),
                        x + rx_bl * (1.0 - KAPPA90),
                        y + h,
                        x + rx_bl,
                        y + h,
                    );
                } else {
                    path.line_to(x + rx_bl, y + h);
                }
            }

            path.line_to(x + w - rx_br, y + h);

            if border_radius_bottom_right != 0.0 {
                if border_shape_bottom_right == BorderCornerShape::Round {
                    path.bezier_to(
                        x + w - rx_br * (1.0 - KAPPA90),
                        y + h,
                        x + w,
                        y + h - ry_br * (1.0 - KAPPA90),
                        x + w,
                        y + h - ry_br,
                    );
                } else {
                    path.line_to(x + w, y + h - ry_br);
                }
            }

            path.line_to(x + w, y + ry_tr);

            if border_radius_top_right != 0.0 {
                if border_shape_top_right == BorderCornerShape::Round {
                    path.bezier_to(
                        x + w,
                        y + ry_tr * (1.0 - KAPPA90),
                        x + w - rx_tr * (1.0 - KAPPA90),
                        y,
                        x + w - rx_tr,
                        y,
                    );
                } else {
                    path.line_to(x + w - rx_tr, y);
                }
            }

            path.line_to(x + rx_tl, y);

            if border_radius_top_left != 0.0 {
                if border_shape_top_left == BorderCornerShape::Round {
                    path.bezier_to(
                        x + rx_tl * (1.0 - KAPPA90),
                        y,
                        x,
                        y + ry_tl * (1.0 - KAPPA90),
                        x,
                        y + ry_tl,
                    );
                } else {
                    path.line_to(x, y + ry_tl);
                }
            }

            path.close();
        }

        // Draw outer shadow

        if cx.style.outer_shadow_color.get(entity).is_some() {
            let sigma = outer_shadow_blur / 2.0;
            let d = (sigma * 5.0).ceil();

            let shadow_image = cx.cache.shadow_image.get(&entity).cloned().unwrap_or_else(|| {
                (
                    canvas
                        .create_image_empty(
                            (bounds.w + d) as usize,
                            (bounds.h + d) as usize,
                            PixelFormat::Rgba8,
                            ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                        )
                        .expect("Failed to create image"),
                    canvas
                        .create_image_empty(
                            (bounds.w + d) as usize,
                            (bounds.h + d) as usize,
                            PixelFormat::Rgba8,
                            ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                        )
                        .expect("Failed to create image"),
                )
            });

            canvas.save();

            let size = canvas.image_size(shadow_image.0).expect("Failed to get image");

            let (source, target) =
                if size.0 != (bounds.w + d) as usize || size.1 != (bounds.h + d) as usize {
                    canvas.delete_image(shadow_image.0);
                    canvas.delete_image(shadow_image.1);

                    (
                        canvas
                            .create_image_empty(
                                (bounds.w + d) as usize,
                                (bounds.h + d) as usize,
                                PixelFormat::Rgba8,
                                ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                            )
                            .expect("Failed to create image"),
                        canvas
                            .create_image_empty(
                                (bounds.w + d) as usize,
                                (bounds.h + d) as usize,
                                PixelFormat::Rgba8,
                                ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                            )
                            .expect("Failed to create image"),
                    )
                } else {
                    (shadow_image.0, shadow_image.1)
                };

            cx.cache.shadow_image.insert(entity, (source, target));

            canvas.set_render_target(RenderTarget::Image(source));
            canvas.clear_rect(0, 0, size.0 as u32, size.1 as u32, femtovg::Color::rgba(0, 0, 0, 0));
            canvas.translate(-bounds.x + d / 2.0, -bounds.y + d / 2.0);
            let mut outer_shadow = path.clone();
            let paint = Paint::color(outer_shadow_color);
            canvas.fill_path(&mut outer_shadow, paint);

            canvas.restore();

            let target_image = if outer_shadow_blur > 0.0 {
                canvas.filter_image(target, femtovg::ImageFilter::GaussianBlur { sigma }, source);
                target
            } else {
                source
            };

            canvas.set_render_target(RenderTarget::Screen);

            canvas.save();
            canvas.translate(outer_shadow_h_offset, outer_shadow_v_offset);
            let mut path = Path::new();
            path.rect(bounds.x - d / 2.0, bounds.y - d / 2.0, bounds.w + d, bounds.h + d);

            canvas.fill_path(
                &mut path,
                Paint::image(
                    target_image,
                    bounds.x - d / 2.0,
                    bounds.y - d / 2.0,
                    bounds.w + d,
                    bounds.h + d,
                    0f32,
                    1f32,
                ),
            );
            //canvas.fill_path(&mut path, Paint::color(femtovg::Color::rgb(0,0,0)));
            canvas.restore();
        }

        // Fill with background color
        let mut paint = Paint::color(background_color);

        // if let Some(background_image) = cx.style.background_image.get(entity) {
        //     if let Some(image_id) = cx.resource_manager.image_ids.get(background_image) {
        //         match image_id {
        //             crate::ImageOrId::Id(id) => {
        //                 paint = Paint::image(*id, 0.0, 0.0, 100.0, 100.0, 0.0, 1.0);
        //             }

        //             _ => {}
        //         }
        //     }
        // }

        // Gradient overrides background color
        if let Some(background_gradient) = cx.style.background_gradient.get(entity) {
            let (_, _, end_x, end_y, parent_length) = match background_gradient.direction {
                GradientDirection::LeftToRight => (0.0, 0.0, bounds.w, 0.0, parent_width),
                GradientDirection::TopToBottom => (0.0, 0.0, 0.0, bounds.h, parent_height),
                _ => (0.0, 0.0, bounds.w, 0.0, parent_width),
            };

            paint = Paint::linear_gradient_stops(
                bounds.x,
                bounds.y,
                bounds.x + end_x,
                bounds.y + end_y,
                background_gradient
                    .get_stops(parent_length)
                    .iter()
                    .map(|stop| {
                        let col: femtovg::Color = stop.1.into();
                        (stop.0, col)
                    })
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
        }

        //canvas.global_composite_blend_func(BlendFactor::DstColor, BlendFactor::OneMinusSrcAlpha);

        // Fill the quad
        canvas.fill_path(&mut path, paint);

        //println!("{:.2?} seconds for whatever you did.", start.elapsed());

        // Draw border
        let mut paint = Paint::color(border_color);
        paint.set_line_width(border_width);
        canvas.stroke_path(&mut path, paint);

        // // Draw inner shadow
        // let mut path = Path::new();
        // path.rounded_rect_varying(
        //     0.0 + border_width,
        //     0.0 + border_width,
        //     bounds.w - border_width * 2.0,
        //     bounds.h - border_width * 2.0,
        //     border_radius_top_left,
        //     border_radius_top_right,
        //     border_radius_bottom_right,
        //     border_radius_bottom_left,
        // );

        // let mut paint = Paint::box_gradient(
        //     0.0 + inner_shadow_h_offset + border_width,
        //     0.0 + inner_shadow_v_offset + border_width,
        //     bounds.w - border_width * 2.0,
        //     bounds.h - border_width * 2.0,
        //     border_radius_top_left
        //         .max(border_radius_top_right)
        //         .max(border_radius_bottom_left)
        //         .max(border_radius_bottom_right),
        //     inner_shadow_blur,
        //     femtovg::Color::rgba(0, 0, 0, 0),
        //     inner_shadow_color,
        // );
        // canvas.fill_path(&mut path, paint);

        // Draw text
        if let Some(text) = cx.style.text.get(entity) {

            
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

            // let mut x = posx + (border_width / 2.0);
            // let mut y = posy + (border_width / 2.0);

            let mut x = bounds.x;
            let mut y = bounds.y;

            let text_string = text.to_owned();

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
                        x += bounds.w - val - border_width;
                        Align::Right
                    }

                    Units::Stretch(_) => {
                        x += 0.5 * bounds.w;
                        Align::Center
                    }

                    _ => Align::Right,
                },

                _ => Align::Left,
            };

            let baseline = match child_top {
                Units::Pixels(val) => match child_bottom {
                    Units::Stretch(_) | Units::Auto => {
                        y += val + border_width;
                        Baseline::Top
                    }

                    _ => Baseline::Top,
                },

                Units::Stretch(_) => match child_bottom {
                    Units::Pixels(val) => {
                        y += bounds.h - val - border_width;
                        Baseline::Bottom
                    }

                    Units::Stretch(_) => {
                        y += 0.5 * bounds.h;
                        Baseline::Middle
                    }

                    _ => Baseline::Bottom,
                },

                _ => Baseline::Top,
            };

            let mut font_color: femtovg::Color = font_color.into();
            font_color.set_alphaf(font_color.a * opacity);

            let font_size = cx.style.font_size.get(entity).cloned().unwrap_or(16.0);

            

            let mut paint = Paint::color(font_color);
            paint.set_font_size(font_size);
            paint.set_font(&[font_id.clone()]);
            paint.set_text_align(align);
            paint.set_text_baseline(baseline);
            paint.set_anti_alias(false);

            let font_metrics = canvas.measure_font(paint).expect("Error measuring font");

            if let Ok(lines) = canvas.break_text_vec(std::f32::MAX, text, paint) {
                for line_range in lines {
                    if let Ok(_text_metrics) = canvas.fill_text(x, y, &text_string[line_range], paint) {
                        y += font_metrics.height();
                    }

                }
            }

            
        }
    }
}


const KAPPA90: f32 = 0.5522847493;