use crate::prelude::*;
use crate::systems::text_selection::{
    begin_selection, end_selection, extend_selection, select_label, select_word,
};
use crate::text::resolved_text_direction;
use skia_safe::{Paint, PaintStyle, Rect};

/// A label used to display text.
///
/// # Examples
///
/// ## Basic label
///
/// A label can be used to simply display some text on the screen.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// Label::new(cx, "Text");
/// ```
///
/// ## Label from a signal source
///
/// A label can read from any signal, which automatically updates the text whenever the underlying data changes.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// let text = Signal::new(String::from("Text"));
/// Label::new(cx, text);
/// ```
///
/// ## Label with text wrapping
///
/// A label automatically wraps the text if it doesn't fit inside of the width of the label.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let mut cx = &mut Context::default();
/// #
/// Label::new(
///     cx,
///     "This is a really long text to showcase the text wrapping support of a label.",
/// )
/// .width(Pixels(100.0));
/// ```
///
/// ## Label without text wrapping
///
/// A label can also be configured to never wrap the text by using the [`text_wrap`](crate::prelude::Handle::text_wrap) method.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let mut cx = &mut Context::default();
/// #
/// Label::new(
///     cx,
///     "This is a really long text to showcase disabled text wrapping of a label.",
/// )
/// .width(Pixels(100.0))
/// .text_wrap(false);
/// ```
///
/// ## Label for a button
///
/// A label can also be used inside of a button to be able to add text to it.
///
/// ```
/// # use vizia_core::prelude::*;
/// # let cx = &mut Context::default();
/// #
/// Button::new(cx, |cx| Label::new(cx, "Text"));
/// ```
pub struct Label {
    describing: Option<String>,
}

impl Label {
    /// Creates a new [Label] view.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Label::new(cx, "Text");
    /// ```
    pub fn new<T>(cx: &mut Context, text: impl Res<T> + Clone + 'static) -> Handle<Self>
    where
        T: ToStringLocalized + 'static,
    {
        Self { describing: None }.build(cx, |_| {}).text(text.clone()).role(Role::Label).name(text)
    }

    /// Creates a new rich [Label] view.
    pub fn rich<T>(
        cx: &mut Context,
        text: impl Res<T> + Clone + 'static,
        children: impl Fn(&mut Context),
    ) -> Handle<Self>
    where
        T: ToStringLocalized + 'static,
    {
        Self { describing: None }
            .build(cx, |cx| {
                children(cx);
            })
            .text(text.clone())
            .role(Role::Label)
            .name(text)
    }
}

impl Handle<'_, Label> {
    /// Sets whether this label's text can be selected.
    pub fn text_selectable(self, selectable: impl Res<bool>) -> Self {
        let entity = self.entity;
        selectable.set_or_bind(self.cx, move |cx, selectable| {
            let selectable = selectable.get_value(cx);
            if selectable {
                cx.text_context.selectable_labels.insert(entity, true);
            } else {
                let window = cx.tree.get_parent_window(entity).unwrap_or(Entity::root());
                let selection_uses_entity = cx
                    .text_context
                    .selections
                    .get(&window)
                    .and_then(|selection| selection.points())
                    .map(|(anchor, focus)| anchor.entity == entity || focus.entity == entity)
                    .unwrap_or(false);
                cx.text_context.selectable_labels.remove(entity);
                cx.text_context.selected_ranges.remove(entity);
                if selection_uses_entity {
                    if let Some(selection) = cx.text_context.selections.get_mut(&window) {
                        selection.clear();
                    }
                    let labels = window
                        .branch_iter(&cx.tree)
                        .filter(|candidate| cx.text_context.selectable_labels.contains(*candidate))
                        .collect::<Vec<_>>();
                    for label in labels {
                        cx.text_context.selected_ranges.remove(label);
                        cx.needs_redraw(label);
                    }
                }
            }
            cx.with_current(entity, |cx| cx.toggle_class("text-selectable", selectable));
            cx.needs_redraw(entity);
        });
        self
    }

    /// Which form element does this label describe.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// #
    /// # struct AppData {
    /// #     value: bool,
    /// # }
    /// #
    /// # impl Model for AppData {}
    /// #
    /// # enum AppEvent {
    /// #     ToggleValue,
    /// # }
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// # let value = Signal::new(false);
    /// #
    /// Checkbox::new(cx, value).on_toggle(|cx| cx.emit(AppEvent::ToggleValue)).id("checkbox_identifier");
    /// Label::new(cx, "hello").describing("checkbox_identifier");
    /// ```
    pub fn describing(self, entity_identifier: impl Into<String>) -> Self {
        let identifier = entity_identifier.into();
        if let Some(id) = self.cx.resolve_entity_identifier(&identifier) {
            let label_identifier = format!("{}", self.entity);
            self.cx.entity_identifiers.insert(label_identifier.clone(), self.entity);
            self.cx.style.labelled_by.insert(id, label_identifier);
        }
        self.modify(|label| label.describing = Some(identifier)).class("describing").hidden(true)
    }
}

impl View for Label {
    fn element(&self) -> Option<&'static str> {
        Some("label")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(MouseButton::Left)
                if cx.text_context.selectable_labels.contains(cx.current()) =>
            {
                begin_selection(cx, cx.modifiers.shift());
                cx.capture();
                meta.consume();
            }
            WindowEvent::MouseMove(_, _)
                if cx.text_context.selectable_labels.contains(cx.current())
                    && *cx.captured == cx.current() =>
            {
                extend_selection(cx);
                meta.consume();
            }
            WindowEvent::MouseUp(MouseButton::Left)
                if cx.text_context.selectable_labels.contains(cx.current())
                    && *cx.captured == cx.current() =>
            {
                end_selection(cx);
                cx.release();
                meta.consume();
            }
            WindowEvent::MouseDoubleClick(MouseButton::Left)
                if cx.text_context.selectable_labels.contains(cx.current()) =>
            {
                select_word(cx);
                meta.consume();
            }
            WindowEvent::MouseTripleClick(MouseButton::Left)
                if cx.text_context.selectable_labels.contains(cx.current()) =>
            {
                select_label(cx);
                meta.consume();
            }
            WindowEvent::Press { .. } | WindowEvent::PressDown { .. }
                if cx.text_context.selectable_labels.contains(cx.current()) =>
            {
                meta.consume();
            }
            WindowEvent::Press { .. } | WindowEvent::PressDown { .. } => {
                if cx.current() == cx.mouse.left.pressed && meta.target == cx.current() {
                    if let Some(describing) = self
                        .describing
                        .as_ref()
                        .and_then(|identity| cx.resolve_entity_identifier(identity))
                    {
                        let old = cx.current;
                        cx.current = describing;
                        cx.focus_with_visibility(false);
                        let message = if matches!(window_event, WindowEvent::Press { .. }) {
                            WindowEvent::Press { mouse: false }
                        } else {
                            WindowEvent::PressDown { mouse: false }
                        };
                        cx.emit_to(describing, message);
                        cx.current = old;
                    }
                }
            }
            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let bounds = cx.bounds();
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        cx.draw_background(canvas);
        cx.draw_shadows(canvas);
        cx.draw_border(canvas);

        if let Some(range) = cx.text_context.selected_ranges.get(cx.current).cloned()
            && let Some(shaped) = cx.text_context.text_shaped.get(cx.current)
        {
            let alignment = cx.alignment();
            let mut top = match alignment {
                Alignment::TopLeft | Alignment::TopCenter | Alignment::TopRight => 0.0,
                Alignment::Left | Alignment::Center | Alignment::Right => 0.5,
                Alignment::BottomLeft | Alignment::BottomCenter | Alignment::BottomRight => 1.0,
            };
            let padding_top = match cx.padding_top() {
                Units::Pixels(value) => value,
                _ => 0.0,
            };
            let padding_bottom = match cx.padding_bottom() {
                Units::Pixels(value) => value,
                _ => 0.0,
            };
            top *= bounds.height() - padding_top - padding_bottom - shaped.height();

            let mut padding_left = match cx.padding_left() {
                Units::Pixels(value) => value,
                _ => 0.0,
            };
            let mut padding_right = match cx.padding_right() {
                Units::Pixels(value) => value,
                _ => 0.0,
            };
            if resolved_text_direction(cx.style, cx.current) == Direction::RightToLeft {
                std::mem::swap(&mut padding_left, &mut padding_right);
            }

            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_style(PaintStyle::Fill);
            paint.set_color(cx.selection_color());
            for text_box in shaped.get_rects_for_range(range) {
                canvas.draw_rect(
                    Rect::new(
                        bounds.x + padding_left + text_box.rect.x(),
                        bounds.y + padding_top + top + text_box.rect.y(),
                        bounds.x + padding_left + text_box.rect.right(),
                        bounds.y + padding_top + top + text_box.rect.bottom(),
                    ),
                    &paint,
                );
            }
        }

        cx.draw_text(canvas);
    }
}

/// A view which represents a span of text within a label.
pub struct TextSpan {}

impl TextSpan {
    /// Create a new [TextSpan] view.
    pub fn new<'a>(
        cx: &'a mut Context,
        text: &str,
        children: impl Fn(&mut Context),
    ) -> Handle<'a, Self> {
        Self {}
            .build(cx, |cx| {
                cx.style.text_span.insert(cx.current(), true);
                children(cx);
            })
            .text(text.to_string())
            .display(Display::None)
            .pointer_events(PointerEvents::None)
    }
}

impl View for TextSpan {
    fn element(&self) -> Option<&'static str> {
        Some("text-span")
    }
}
