use crate::prelude::*;

/// A simple push button with an action and a contained view.
///
/// # Examples
///
/// ## Button with an action
///
/// A button can be used to call an action when pressed. Usually this is an
/// event that is being emitted.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # enum AppEvent {
/// #     Action,
/// # }
/// #
/// # let cx = &mut Context::default();
/// #
/// Button::new(cx, |cx| cx.emit(AppEvent::Action), |cx| Label::new(cx, "Text"));
/// ```
///
/// ## Button without an action
///
/// A button can be used without an action and therefore do nothing when pressed.
/// This is useful for prototyping and testing out the different styling options of
/// a button without having to add an action.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// Button::new(cx, |_| {}, |cx| Label::new(cx, "Text"));
/// ```
///
/// ## Button containing multiple views
///
/// A button can contain more than just a single view or label inside of it. This can
/// for example be done by using a [`HStack`](crate::prelude::HStack) or [`VStack`](crate::prelude::VStack).
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// Button::new(
///     cx,
///     |_| {},
///     |cx| {
///         HStack::new(cx, |cx| {
///             Label::new(cx, "Hello");
///             Label::new(cx, "World");
///         })
///     },
/// );
/// ```
pub struct Button {
    action: Box<dyn Fn(&mut EventContext)>,
}

impl Button {
    /// Creates a new button with a specified action and content.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Button::new(cx, |cx| cx.emit(AppEvent::TriggerAction), |cx| Label::new(cx, "Press Me"));
    /// ```
    pub fn new<A, C, V>(cx: &mut Context, action: A, content: C) -> Handle<Self>
    where
        A: 'static + Fn(&mut EventContext),
        C: FnOnce(&mut Context) -> Handle<V>,
        V: View,
    {
        Self { action: Box::new(action) }
            .build(cx, move |cx| {
                (content)(cx).hoverable(false).class("inner");
            })
            .role(Role::Button)
            .default_action_verb(DefaultActionVerb::Click)
            .navigable(true)
    }
}

impl View for Button {
    fn element(&self) -> Option<&'static str> {
        Some("button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::PressDown { mouse } => {
                if *mouse {
                    cx.capture()
                }
                cx.focus();
            }

            WindowEvent::Press { .. } => {
                if meta.target == cx.current() {
                    (self.action)(cx);
                }
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                cx.release();
            }

            WindowEvent::ActionRequest(action) => match action.action {
                Action::Default => {
                    (self.action)(cx);
                }

                _ => {}
            },

            _ => {}
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Normal,
    Accent,
    Outline,
    Text,
}

pub trait ButtonModifiers {
    fn variant<U: Into<ButtonVariant>>(self, variant: impl Res<U>) -> Self;
}

impl<'a> ButtonModifiers for Handle<'a, Button> {
    fn variant<U: Into<ButtonVariant>>(mut self, variant: impl Res<U>) -> Self {
        let entity = self.entity();
        variant.set_or_bind(self.context(), entity, |cx, val| {
            let var: ButtonVariant = val.into();
            match var {
                ButtonVariant::Normal => {
                    cx.toggle_class("accent", false);
                    cx.toggle_class("outline", false);
                    cx.toggle_class("text", false);
                }

                ButtonVariant::Accent => {
                    cx.toggle_class("accent", true);
                    cx.toggle_class("outline", false);
                    cx.toggle_class("text", false);
                }

                ButtonVariant::Outline => {
                    cx.toggle_class("accent", false);
                    cx.toggle_class("outline", true);
                    cx.toggle_class("text", false);
                }

                ButtonVariant::Text => {
                    cx.toggle_class("accent", false);
                    cx.toggle_class("outline", false);
                    cx.toggle_class("text", true);
                }
            }
        });

        self
    }
}

pub struct IconButton {
    action: Box<dyn Fn(&mut EventContext)>,
}

impl IconButton {
    pub fn new<A, S>(cx: &mut Context, action: A, icon: impl Res<S> + Clone) -> Handle<Self>
    where
        A: 'static + Fn(&mut EventContext),
        S: ToString,
    {
        Self { action: Box::new(action) }
            .build(cx, move |cx| {
                Icon::new(cx, icon).hoverable(false).class("inner");
            })
            .class("icon")
            .role(Role::Button)
            .default_action_verb(DefaultActionVerb::Click)
            .navigable(true)
    }
}

impl View for IconButton {
    fn element(&self) -> Option<&'static str> {
        Some("button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::PressDown { mouse } => {
                if *mouse {
                    cx.capture()
                }
                cx.focus();
            }

            WindowEvent::Press { .. } => {
                if meta.target == cx.current() {
                    (self.action)(cx);
                }
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                cx.release();
            }

            WindowEvent::ActionRequest(action) => match action.action {
                Action::Default => {
                    (self.action)(cx);
                }

                _ => {}
            },

            _ => {}
        });
    }
}

impl<'a> ButtonModifiers for Handle<'a, IconButton> {
    fn variant<U: Into<ButtonVariant>>(mut self, variant: impl Res<U>) -> Self {
        let entity = self.entity();
        variant.set_or_bind(self.context(), entity, |cx, val| {
            let var: ButtonVariant = val.into();
            match var {
                ButtonVariant::Normal => {
                    cx.toggle_class("accent", false);
                    cx.toggle_class("outline", false);
                    cx.toggle_class("text", false);
                }

                ButtonVariant::Accent => {
                    cx.toggle_class("accent", true);
                    cx.toggle_class("outline", false);
                    cx.toggle_class("text", false);
                }

                ButtonVariant::Outline => {
                    cx.toggle_class("accent", false);
                    cx.toggle_class("outline", true);
                    cx.toggle_class("text", false);
                }

                ButtonVariant::Text => {
                    cx.toggle_class("accent", false);
                    cx.toggle_class("outline", false);
                    cx.toggle_class("text", true);
                }
            }
        });

        self
    }
}

pub struct ButtonGroup {}

impl ButtonGroup {
    pub fn new<C>(cx: &mut Context, content: C) -> Handle<Self>
    where
        C: FnOnce(&mut Context),
    {
        Self {}.build(cx, |cx| {
            (content)(cx);
        })
    }
}

impl View for ButtonGroup {
    fn element(&self) -> Option<&'static str> {
        Some("button-group")
    }
}

impl<'a> ButtonModifiers for Handle<'a, ButtonGroup> {
    fn variant<U: Into<ButtonVariant>>(mut self, variant: impl Res<U>) -> Self {
        let entity = self.entity();
        variant.set_or_bind(self.context(), entity, |cx, val| {
            let var: ButtonVariant = val.into();
            match var {
                ButtonVariant::Normal => {
                    cx.toggle_class("accent", false);
                    cx.toggle_class("outline", false);
                    cx.toggle_class("text", false);
                }

                ButtonVariant::Accent => {
                    cx.toggle_class("accent", true);
                    cx.toggle_class("outline", false);
                    cx.toggle_class("text", false);
                }

                ButtonVariant::Outline => {
                    cx.toggle_class("accent", false);
                    cx.toggle_class("outline", true);
                    cx.toggle_class("text", false);
                }

                ButtonVariant::Text => {
                    cx.toggle_class("accent", false);
                    cx.toggle_class("outline", false);
                    cx.toggle_class("text", true);
                }
            }
        });

        self
    }
}

// pub struct ToggleButton {
//     on_toggle: Box<dyn Fn(&mut EventContext)>,
// }

// impl ToggleButton {
//     pub fn new<A, C, V>(cx: &mut Context, on_toggle: A, content: C) -> Handle<Self>
//     where
//         A: 'static + Fn(&mut EventContext),
//         C: FnOnce(&mut Context) -> Handle<V>,
//         V: View,
//     {
//         Self { on_toggle: Box::new(on_toggle) }.build(cx, |cx| {
//             (content)(cx);
//         })
//     }
// }

// impl View for ToggleButton {
//     fn element(&self) -> Option<&'static str> {
//         Some("toggle-button")
//     }
// }
