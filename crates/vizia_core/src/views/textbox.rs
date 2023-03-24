use crate::cache::BoundingBox;
use crate::prelude::*;
use crate::text::{enforce_text_bounds, ensure_visible, Direction, Movement};
use crate::views::scrollview::SCROLL_SENSITIVITY;
use cosmic_text::{Action, Attrs, Edit};
use vizia_input::Code;
use vizia_storage::TreeExt;

pub enum TextEvent {
    InsertText(String),
    ResetText(String),
    DeleteText(Movement),
    MoveCursor(Movement, bool),
    SelectAll,
    SelectWord,
    SelectParagraph,
    StartEdit,
    EndEdit,
    Submit(bool),
    Hit(f32, f32),
    Drag(f32, f32),
    Scroll(f32, f32),
    Copy,
    Paste,
    Cut,
    GeometryChanged,
}

pub struct Textbox<L: Lens> {
    lens: L,
    kind: TextboxKind,
    edit: bool,
    transform: (f32, f32),
    on_edit: Option<Box<dyn Fn(&mut EventContext, String) + Send + Sync>>,
    on_submit: Option<Box<dyn Fn(&mut EventContext, String, bool) + Send + Sync>>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
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
        Self {
            lens: lens.clone(),
            kind,
            edit: false,
            transform: (0.0, 0.0),
            on_edit: None,
            on_submit: None,
        }
        .build(cx, move |cx| {
            let parent = cx.current;

            Binding::new(cx, lens.clone(), move |cx, text| {
                let text_str = text.view(cx.data().unwrap(), |text| {
                    text.map(|x| x.to_string()).unwrap_or_else(|| "".to_owned())
                });

                cx.text_context.with_buffer(parent, |fs, buf| {
                    buf.set_text(fs, &text_str, Attrs::new());
                });
            });
        })
        .text_wrap(kind == TextboxKind::MultiLineWrapped)
        .cursor(CursorIcon::Text)
        .navigable(true)
    }

    fn set_caret(&mut self, cx: &mut EventContext) {
        let parent = cx.current().parent(cx.tree).unwrap();

        // this is a weird situation - layout and drawing must be done in physical space, but our
        // output (translate) must be in logical space.
        let scale = cx.style.dpi_factor as f32;

        // calculate visible area for content and container
        // let bounds = *cx.cache.bounds.get(cx.current).unwrap();
        let bounds = cx.bounds();
        let mut parent_bounds = *cx.cache.bounds.get(parent).unwrap();

        cx.text_context.sync_styles(cx.current, cx.style);

        // do the computation
        let (mut tx, mut ty) = self.transform;
        tx *= scale;
        ty *= scale;
        (tx, ty) = enforce_text_bounds(&bounds, &parent_bounds, (tx, ty));

        // TODO justify????
        if let Some((x, y, w, h)) =
            cx.text_context.layout_caret(cx.current, (bounds.x, bounds.y), (0., 0.), 1.0 * scale)
        {
            let caret_box = BoundingBox { x, y, w, h };

            parent_bounds.x -= 1.0;
            parent_bounds.w += 2.0;
            (tx, ty) = ensure_visible(&caret_box, &parent_bounds, (tx, ty));
        }

        self.transform = (tx.round() / scale, ty.round() / scale);
    }

    pub fn insert_text(&mut self, cx: &mut EventContext, text: &str) {
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.insert_string(text, None);
        });
        cx.needs_relayout();
        cx.needs_redraw();
    }

    pub fn delete_text(&mut self, cx: &mut EventContext, movement: Movement) {
        if cx.text_context.with_editor(cx.current, |fs, buf| !buf.delete_selection()) {
            self.move_cursor(cx, movement, true);
            cx.text_context.with_editor(cx.current, |fs, buf| {
                buf.delete_selection();
            });
        }
        cx.needs_relayout();
        cx.needs_redraw();
    }

    pub fn reset_text(&mut self, cx: &mut EventContext, text: &str) {
        cx.text_context.with_buffer(cx.current, |fs, buf| {
            buf.set_text(fs, text, Attrs::new());
        });
        cx.needs_relayout();
        cx.needs_redraw();
    }

    pub fn move_cursor(&mut self, cx: &mut EventContext, movement: Movement, selection: bool) {
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

    pub fn select_all(&mut self, cx: &mut EventContext) {
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.action(fs, Action::BufferStart);
            buf.set_select_opt(Some(buf.cursor()));
            buf.action(fs, Action::BufferEnd);
        });
        cx.needs_redraw();
    }

    pub fn select_word(&mut self, cx: &mut EventContext) {
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.action(fs, Action::PreviousWord);
            buf.set_select_opt(Some(buf.cursor()));
            buf.action(fs, Action::NextWord);
        });
        cx.needs_redraw();
    }

    pub fn select_paragraph(&mut self, cx: &mut EventContext) {
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.action(fs, Action::ParagraphStart);
            buf.set_select_opt(Some(buf.cursor()));
            buf.action(fs, Action::ParagraphEnd);
        });
        cx.needs_redraw();
    }

    pub fn deselect(&mut self, cx: &mut EventContext) {
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.set_select_opt(None);
        });
        cx.needs_redraw();
    }

    /// These input coordinates should be physical coordinates, i.e. what the mouse events provide.
    /// The output text coordinates will also be physical, but relative to the top of the text
    /// glyphs, appropriate for passage to cosmic.
    pub fn coordinates_global_to_text(&self, cx: &EventContext, x: f32, y: f32) -> (f32, f32) {
        let parent = cx.current.parent(cx.tree).unwrap();
        let parent_bounds = *cx.cache.bounds.get(parent).unwrap();

        let child_left = cx.style.child_left.get(cx.current).copied().unwrap_or_default();
        let child_right = cx.style.child_right.get(cx.current).copied().unwrap_or_default();
        let child_top = cx.style.child_top.get(cx.current).copied().unwrap_or_default();
        let child_bottom = cx.style.child_bottom.get(cx.current).copied().unwrap_or_default();

        let justify_x = match (child_left, child_right) {
            (Stretch(left), Stretch(right)) => {
                if left + right == 0.0 {
                    0.5
                } else {
                    left / (left + right)
                }
            }
            (Stretch(_), _) => 1.0,
            _ => 0.0,
        };
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

        let bounds = cx.bounds();

        let origin_x = bounds.w * justify_x;
        let origin_y = bounds.h * justify_y;

        // println!("{} {}", origin_x, origin_y);

        let x = x - self.transform.0 * cx.style.dpi_factor as f32 + origin_x;
        let y = y - self.transform.1 * cx.style.dpi_factor as f32 + origin_y;
        (x, y)
    }

    /// This function takes window-global physical coordinates.
    pub fn hit(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        let (x, y) = self.coordinates_global_to_text(cx, x, y);
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.action(fs, Action::Click { x: x as i32, y: y as i32 })
        });
        cx.needs_redraw();
    }

    /// This function takes window-global physical coordinates.
    pub fn drag(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        let (x, y) = self.coordinates_global_to_text(cx, x, y);
        cx.text_context.with_editor(cx.current, |fs, buf| {
            buf.action(fs, Action::Drag { x: x as i32, y: y as i32 })
        });
        cx.needs_redraw();
    }

    /// This function takes window-global physical dimensions.
    pub fn scroll(&mut self, cx: &mut EventContext, x: f32, y: f32) {
        let entity = cx.current;
        let parent = cx.tree.get_parent(entity).unwrap();
        let bounds = *cx.cache.bounds.get(entity).unwrap();
        let parent_bounds = *cx.cache.bounds.get(parent).unwrap();
        let (mut tx, mut ty) = self.transform;
        let scale = cx.style.dpi_factor as f32;
        tx *= scale;
        ty *= scale;
        tx += x * SCROLL_SENSITIVITY;
        ty += y * SCROLL_SENSITIVITY;
        (tx, ty) = enforce_text_bounds(&bounds, &parent_bounds, (tx, ty));
        self.transform = (tx / scale, ty / scale);
    }

    #[allow(dead_code)]
    pub fn clone_selected(&self, cx: &mut EventContext) -> Option<String> {
        cx.text_context.with_editor(cx.current, |fs, buf| buf.copy_selection())
    }

    pub fn clone_text(&self, cx: &mut EventContext) -> String {
        cx.text_context.with_buffer(cx.current, |fs, buf| {
            buf.lines.iter().map(|line| line.text()).collect::<Vec<_>>().join("\n")
        })
    }
}

impl<'a, L: Lens> Handle<'a, Textbox<L>> {
    pub fn on_edit<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, String) + Send + Sync,
    {
        self.modify(|textbox: &mut Textbox<L>| textbox.on_edit = Some(Box::new(callback)))
    }

    pub fn on_submit<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, String, bool) + Send + Sync,
    {
        self.modify(|textbox: &mut Textbox<L>| textbox.on_submit = Some(Box::new(callback)))
    }
}

impl<L: Lens> View for Textbox<L>
where
    <L as Lens>::Target: Data + ToString,
{
    fn element(&self) -> Option<&'static str> {
        Some("textbox")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        // Window Events
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(MouseButton::Left) => {
                if meta.origin == cx.current {
                    return;
                }

                if cx.is_over() {
                    cx.focus_with_visibility(false);
                    cx.capture();
                    cx.set_checked(true);
                    cx.lock_cursor_icon();

                    cx.emit(TextEvent::Hit(cx.mouse.cursorx, cx.mouse.cursory));
                } else {
                    cx.emit(TextEvent::Submit(false));
                    if let Some(source) = cx.data::<L::Source>() {
                        let text = self.lens.view(source, |t| {
                            if let Some(t) = t {
                                t.to_string()
                            } else {
                                "".to_owned()
                            }
                        });

                        cx.emit(TextEvent::ResetText(text));
                    };
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
                cx.unlock_cursor_icon();
                if cx.mouse.left.pressed == cx.current() {
                    cx.emit(TextEvent::StartEdit);
                }
            }

            WindowEvent::MouseMove(_, _) => {
                if cx.mouse.left.state == MouseButtonState::Pressed
                    && cx.mouse.left.pressed == cx.current
                {
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
                            !cx.modifiers.contains(Modifiers::CTRL)
                {
                    cx.emit(TextEvent::InsertText(String::from(*c)));
                }
            }

            WindowEvent::KeyDown(code, _) => match code {
                Code::Enter => {
                    // Finish editing
                    if matches!(self.kind, TextboxKind::SingleLine) {
                        cx.emit(TextEvent::Submit(true));
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

                        cx.set_checked(false);
                        cx.release();
                    } else {
                        cx.emit(TextEvent::InsertText("\n".to_owned()));
                    }
                }

                Code::ArrowLeft => {
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
                        cx.emit(TextEvent::DeleteText(Movement::Word(Direction::Upstream)));
                    } else {
                        cx.emit(TextEvent::DeleteText(Movement::Grapheme(Direction::Upstream)));
                    }
                }

                Code::Delete => {
                    if cx.modifiers.contains(Modifiers::CTRL) {
                        cx.emit(TextEvent::DeleteText(Movement::Word(Direction::Downstream)));
                    } else {
                        cx.emit(TextEvent::DeleteText(Movement::Grapheme(Direction::Downstream)));
                    }
                }

                Code::Escape => {
                    cx.emit(TextEvent::EndEdit);
                    cx.set_checked(false);
                }

                Code::Home => {
                    cx.emit(TextEvent::MoveCursor(
                        Movement::LineStart,
                        cx.modifiers.contains(Modifiers::SHIFT),
                    ));
                }

                Code::End => {
                    cx.emit(TextEvent::MoveCursor(
                        Movement::LineEnd,
                        cx.modifiers.contains(Modifiers::SHIFT),
                    ));
                }

                Code::PageUp | Code::PageDown => {
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
                    if cx.modifiers.contains(Modifiers::CTRL) {
                        cx.emit(TextEvent::SelectAll);
                    }
                }

                Code::KeyC if cx.modifiers == &Modifiers::CTRL => {
                    cx.emit(TextEvent::Copy);
                }

                Code::KeyV if cx.modifiers == &Modifiers::CTRL => {
                    cx.emit(TextEvent::Paste);
                }

                Code::KeyX if cx.modifiers == &Modifiers::CTRL => {
                    cx.emit(TextEvent::Cut);
                }

                _ => {}
            },

            _ => {}
        });

        // Textbox Events
        event.map(|text_event, _| match text_event {
            TextEvent::InsertText(text) => {
                if self.edit {
                    self.insert_text(cx, text);
                    self.set_caret(cx);

                    if let Some(callback) = &self.on_edit {
                        let text = self.clone_text(cx);
                        (callback)(cx, text);
                    }
                }
            }

            TextEvent::ResetText(text) => {
                self.reset_text(cx, text);
                self.scroll(cx, 0.0, 0.0); // ensure_visible
            }

            TextEvent::DeleteText(movement) => {
                if self.edit {
                    self.delete_text(cx, *movement);
                    self.set_caret(cx);

                    if let Some(callback) = &self.on_edit {
                        let text = self.clone_text(cx);
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

            TextEvent::StartEdit => {
                if !cx.is_disabled() && !self.edit {
                    self.edit = true;
                    cx.focus_with_visibility(false);
                    cx.capture();
                    cx.set_checked(true);
                }
            }

            TextEvent::EndEdit => {
                self.deselect(cx);
                self.edit = false;
                cx.set_checked(false);
                cx.release();
            }

            TextEvent::Submit(reason) => {
                if let Some(callback) = self.on_submit.take() {
                    let text = self.clone_text(cx);
                    (callback)(cx, text, *reason);

                    self.on_submit = Some(callback);
                }
                cx.emit(TextEvent::EndEdit);
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
                            if let Some(callback) = self.on_edit.take() {
                                let text = self.clone_text(cx);
                                (callback)(cx, text);

                                self.on_edit = Some(callback);
                            }
                        }
                    }
                }
            }

            TextEvent::GeometryChanged => {
                self.set_caret(cx);
            }
        });
    }
}
