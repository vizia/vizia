use crate::prelude::*;
use crate::text::TextSpan;
use cosmic_text::Cursor;
use std::marker::PhantomData;

/// A label used to display text to the screen.
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
/// ## Label bound to data
///
/// A label can be bound to data using a lens which automatically updates the text whenever the underlying data changes.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// #[derive(Lens)]
/// struct AppData {
///     text: String,
/// }
///
/// impl Model for AppData {}
///
/// AppData {
///     text: String::from("Text"),
/// }
/// .build(cx);
///
/// Label::new(cx, AppData::text);
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
/// Button::new(cx, |_| {}, |cx| Label::new(cx, "Text"));
/// ```
pub struct Label {
    describing: Option<String>,
}

impl Label {
    /// Creates a new label.
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
    pub fn new<'a, T>(cx: &'a mut Context, text: impl Res<T>) -> Handle<'a, Self>
    where
        T: ToString,
    {
        Self::new_with_spans(cx, text, |_| {})
    }

    pub fn new_with_spans<'a, T, F>(
        cx: &'a mut Context,
        text: impl Res<T>,
        builder: F,
    ) -> Handle<'a, Self>
    where
        T: ToString,
        F: FnOnce(&mut Context),
    {
        Self { describing: None }.build(cx, builder).text(text)
    }

    pub fn new_rich_formatted<T>(cx: &mut Context, text: impl Res<T>) -> Handle<Self>
    where
        T: AsRef<str> + Data,
    {
        Self { describing: None }.build(cx, move |cx| {
            #[derive(Clone, Debug)]
            enum FmtCmd {
                Class(String),
                Id(String),
            }
            #[derive(Debug)]
            struct FmtSpan {
                cmds: Vec<FmtCmd>,
                begin: Cursor,
                end: Cursor,
            }
            text.set_or_bind(cx, cx.current, move |cx, entity, text_etc| {
                let mut text = String::new();
                let mut spans = Vec::new();
                let mut literal = true;
                let mut cmd_buf = String::new();
                let mut line = 0;
                let mut line_start = 0;
                let mut last_cursor = Cursor::new(0, 0);
                let mut stack = Vec::new();

                for ch in text_etc.as_ref().chars() {
                    if literal {
                        match ch {
                            '\0' => {
                                literal = false;
                            }
                            '\n' => {
                                text.push('\n');
                                line += 1;
                                line_start = text.len();
                            }
                            '\r' => {}
                            _ => {
                                text.push(ch);
                            }
                        }
                    } else {
                        match ch {
                            '\0' => {
                                literal = true;
                                if cmd_buf.is_empty() {
                                    text.push('\0');
                                } else {
                                    let new_cursor = Cursor::new(line, text.len() - line_start);
                                    if new_cursor != last_cursor && !stack.is_empty() {
                                        spans.push(FmtSpan {
                                            cmds: stack.clone(),
                                            begin: last_cursor,
                                            end: new_cursor,
                                        });
                                    }
                                    last_cursor = new_cursor;
                                    let (cmd, rest) = cmd_buf.as_str().split_at(1);
                                    match cmd {
                                        "." => stack.push(FmtCmd::Class(rest.to_owned())),
                                        "#" => stack.push(FmtCmd::Id(rest.to_owned())),
                                        "-" => {
                                            stack.pop().expect("Bad pop command - empty stack");
                                        }
                                        _ => panic!("Bad formatting command"),
                                    }
                                    cmd_buf.clear();
                                }
                            }
                            _ => {
                                cmd_buf.push(ch);
                            }
                        }
                    }
                }
                if !literal {
                    panic!("Bad formatting command - unclosed directive");
                }
                if !stack.is_empty() {
                    panic!("Bad formatting commands - stack not empty");
                }
                let new_cursor = Cursor::new(line, text.len() - line_start);
                if new_cursor != last_cursor && !stack.is_empty() {
                    spans.push(FmtSpan {
                        cmds: stack.clone(),
                        begin: last_cursor,
                        end: new_cursor,
                    });
                }
                Handle { entity, p: PhantomData::<Self>::default(), cx }.text(&text);
                println!("SO true bestie {:?}", entity);
                for FmtSpan { begin, end, cmds } in spans.into_iter() {
                    let mut handle = TextSpan::new(cx, begin, end);
                    for cmd in cmds {
                        match cmd {
                            FmtCmd::Class(name) => {
                                handle = handle.class(&name);
                            }
                            FmtCmd::Id(name) => {
                                handle = handle.id(&name);
                            }
                        }
                    }
                }
            });
        })
    }
}

impl Handle<'_, Label> {
    /// Which form element does this label describe.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # #[derive(Lens)]
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
    /// # AppData { value: false }.build(cx);
    /// #
    /// Checkbox::new(cx, AppData::value).on_toggle(|cx| cx.emit(AppEvent::ToggleValue)).id("checkbox_identifier");
    /// Label::new(cx, "hello").describing("checkbox_identifier");
    /// ```
    pub fn describing(self, entity_identifier: impl Into<String>) -> Self {
        self.modify(|label| label.describing = Some(entity_identifier.into())).class("describing")
    }
}

impl View for Label {
    fn element(&self) -> Option<&'static str> {
        Some("label")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::Press { .. } | WindowEvent::PressDown { .. } => {
                if cx.current() == cx.mouse.left.pressed && meta.target == cx.current() {
                    if let Some(describing) = self
                        .describing
                        .as_ref()
                        .and_then(|identity| cx.resolve_entity_identifier(&identity))
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
}
