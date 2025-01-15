use crate::prelude::*;

/// A simple push button with a contained view.
///
/// # Examples
///
/// ## Button with an action
///
/// A button can be used to call an action when interacted with. Usually this is an
/// event that is being emitted when the button is [pressed](crate::modifiers::ActionModifiers::on_press).
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
/// Button::new(cx, |cx| Label::new(cx, "Text"))
///     .on_press(|ex| ex.emit(AppEvent::Action))
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
/// Button::new(cx, |cx| Label::new(cx, "Text"));
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
///     |cx| {
///         HStack::new(cx, |cx| {
///             Label::new(cx, "Hello");
///             Label::new(cx, "World");
///         })
///     },
/// );
/// ```
///
/// # Button Variants
///
/// The style of a button can be modified using the [`variant`](ButtonModifiers::variant) modifier from the [`ButtonModifiers`] trait
/// by specifying the [`ButtonVariant`].
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// Button::new(cx, |cx| Label::new(cx, "Text"))
///     .variant(ButtonVariant::Accent);
/// ```
pub struct Button {
    pub(crate) action: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl Button {
    /// Creates a new button with specified content.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Button::new(cx, |cx| Label::new(cx, "Press Me"));
    /// ```
    pub fn new<C, V>(cx: &mut Context, content: C) -> Handle<Self>
    where
        C: FnOnce(&mut Context) -> Handle<V>,
        V: View,
    {
        Self { action: None }
            .build(cx, move |cx| {
                (content)(cx).hoverable(false);
            })
            .role(Role::Button)
            .navigable(true)
    }
}

impl View for Button {
    fn element(&self) -> Option<&'static str> {
        Some("button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::PressDown { mouse: _ } => {
                if meta.target == cx.current {
                    cx.focus();
                }
            }

            WindowEvent::Press { .. } => {
                if meta.target == cx.current {
                    if let Some(action) = &self.action {
                        (action)(cx);
                    }
                }
            }

            WindowEvent::ActionRequest(action) => match action.action {
                Action::Click => {
                    if let Some(action) = &self.action {
                        (action)(cx);
                    }
                }

                _ => {}
            },

            _ => {}
        });
    }
}

/// Used in conjunction with the [`variant`](ButtonModifiers::variant) modifier for selecting the style variant of a button or button group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    /// A normal button.
    Normal,
    /// A button with an accent color.
    Accent,
    /// A button with just a border.
    Outline,
    /// A button with just text.
    Text,
}

/// Modifiers for changing the appearance of buttons.
pub trait ButtonModifiers {
    /// Selects the style variant to be used by the button or button group.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Button::new(cx, |cx| Label::new(cx, "Text"))
    ///     .variant(ButtonVariant::Accent);
    /// ```
    fn variant<U: Into<ButtonVariant>>(self, variant: impl Res<U>) -> Self;
}

impl ButtonModifiers for Handle<'_, Button> {
    fn variant<U: Into<ButtonVariant>>(self, variant: impl Res<U>) -> Self {
        self.bind(variant, |handle, val| {
            let var: ButtonVariant = val.get(&handle).into();
            match var {
                ButtonVariant::Normal => {
                    handle
                        .toggle_class("accent", false)
                        .toggle_class("outline", false)
                        .toggle_class("text", false);
                }

                ButtonVariant::Accent => {
                    handle
                        .toggle_class("accent", true)
                        .toggle_class("outline", false)
                        .toggle_class("text", false);
                }

                ButtonVariant::Outline => {
                    handle
                        .toggle_class("accent", false)
                        .toggle_class("outline", true)
                        .toggle_class("text", false);
                }

                ButtonVariant::Text => {
                    handle
                        .toggle_class("accent", false)
                        .toggle_class("outline", false)
                        .toggle_class("text", true);
                }
            }
        })
    }
}

/// A view which represents a group of buttons.
pub struct ButtonGroup {}

impl ButtonGroup {
    /// Creates a new button group.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    ///
    /// # let cx = &mut Context::default();
    ///
    /// ButtonGroup::new(cx, |cx| {
    ///     Button::new(cx, |cx| Label::new(cx, "ONE"));
    ///     Button::new(cx, |cx| Label::new(cx, "TWO"));
    ///     Button::new(cx, |cx| Label::new(cx, "THREE"));
    /// });
    /// ```
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

impl Handle<'_, ButtonGroup> {
    /// Sets whether the button group is in vertical orientation.
    pub fn vertical(self, is_vertical: impl Res<bool>) -> Self {
        self.toggle_class("vertical", is_vertical)
    }
}

impl ButtonModifiers for Handle<'_, ButtonGroup> {
    fn variant<U: Into<ButtonVariant>>(self, variant: impl Res<U>) -> Self {
        self.bind(variant, |handle, val| {
            let var: ButtonVariant = val.get(&handle).into();
            match var {
                ButtonVariant::Normal => {
                    handle
                        .toggle_class("accent", false)
                        .toggle_class("outline", false)
                        .toggle_class("text", false);
                }

                ButtonVariant::Accent => {
                    handle
                        .toggle_class("accent", true)
                        .toggle_class("outline", false)
                        .toggle_class("text", false);
                }

                ButtonVariant::Outline => {
                    handle
                        .toggle_class("accent", false)
                        .toggle_class("outline", true)
                        .toggle_class("text", false);
                }

                ButtonVariant::Text => {
                    handle
                        .toggle_class("accent", false)
                        .toggle_class("outline", false)
                        .toggle_class("text", true);
                }
            }
        })
    }
}
