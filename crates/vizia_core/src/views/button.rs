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
/// let text = cx.state("Text");
/// Button::new(cx, |cx| Label::new(cx, text))
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
/// let text = cx.state("Text");
/// Button::new(cx, |cx| Label::new(cx, text));
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
/// let hello = cx.state("Hello");
/// let world = cx.state("World");
/// Button::new(cx, |cx| {
///     HStack::new(cx, |cx| {
///         Label::new(cx, hello);
///         Label::new(cx, world);
///     })
/// });
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
/// # let accent = cx.state(ButtonVariant::Accent);
/// #
/// let text = cx.state("Text");
/// Button::new(cx, |cx| Label::new(cx, text))
///     .variant(accent);
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
    /// let text = cx.state("Press Me");
    /// Button::new(cx, |cx| Label::new(cx, text));
    /// ```
    pub fn new<C, V>(cx: &mut Context, content: C) -> Handle<Self>
    where
        C: FnOnce(&mut Context) -> Handle<V>,
        V: View,
    {
        let false_signal = cx.state(false);
        let true_signal = cx.state(true);
        Self { action: None }
            .build(cx, move |cx| {
                (content)(cx).hoverable(false_signal);
            })
            .role(Role::Button)
            .navigable(true_signal)
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

crate::impl_res_simple!(ButtonVariant);

/// Modifiers for changing the appearance of buttons.
pub trait ButtonModifiers {
    /// Selects the style variant to be used by the button or button group.
    /// Accepts a `ButtonVariant` or `Signal<ButtonVariant>`.
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// let text = cx.state("Text");
    /// Button::new(cx, |cx| Label::new(cx, text))
    ///     .variant(cx.state(ButtonVariant::Accent));
    /// ```
    fn variant(self, variant: impl Res<ButtonVariant> + 'static) -> Self;
}

impl ButtonModifiers for Handle<'_, Button> {
    fn variant(mut self, variant: impl Res<ButtonVariant> + 'static) -> Self {
        let variant = variant.into_signal(self.context());
        let is_accent = self.context().derived({
            let variant = variant;
            move |store| *variant.get(store) == ButtonVariant::Accent
        });
        let is_outline = self.context().derived({
            let variant = variant;
            move |store| *variant.get(store) == ButtonVariant::Outline
        });
        let is_text = self.context().derived({
            let variant = variant;
            move |store| *variant.get(store) == ButtonVariant::Text
        });

        self.toggle_class("accent", is_accent)
            .toggle_class("outline", is_outline)
            .toggle_class("text", is_text)
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
    /// let one = cx.state("ONE");
    /// let two = cx.state("TWO");
    /// let three = cx.state("THREE");
    ///
    /// ButtonGroup::new(cx, |cx| {
    ///     Button::new(cx, |cx| Label::new(cx, one));
    ///     Button::new(cx, |cx| Label::new(cx, two));
    ///     Button::new(cx, |cx| Label::new(cx, three));
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
    pub fn vertical(self, is_vertical: impl Res<bool> + 'static) -> Self {
        self.toggle_class("vertical", is_vertical)
    }
}

impl ButtonModifiers for Handle<'_, ButtonGroup> {
    fn variant(mut self, variant: impl Res<ButtonVariant> + 'static) -> Self {
        let variant = variant.into_signal(self.context());
        let is_accent = self.context().derived({
            let variant = variant;
            move |store| *variant.get(store) == ButtonVariant::Accent
        });
        let is_outline = self.context().derived({
            let variant = variant;
            move |store| *variant.get(store) == ButtonVariant::Outline
        });
        let is_text = self.context().derived({
            let variant = variant;
            move |store| *variant.get(store) == ButtonVariant::Text
        });

        self.toggle_class("accent", is_accent)
            .toggle_class("outline", is_outline)
            .toggle_class("text", is_text)
    }
}
