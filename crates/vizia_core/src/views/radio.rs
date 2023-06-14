use crate::prelude::*;

/// A radio button used to display and toggle boolean state.
///
/// Clicking on the radio button with the left mouse button triggers the `on_select` callback.
///
/// # Examples
///
/// ## Basic radio button
///
/// The radio button must bound to some boolean data.
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
/// # let cx = &mut Context::default();
/// #
/// # AppData { value: false }.build(cx);
/// #
/// RadioButton::new(cx, AppData::value);
/// ```
///
/// ## Radio button with an action
///
/// A radio button can be used to trigger a callback when selected. Usually this is emitting an
/// event responsible for changing the data the radio button is bound to.
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
/// RadioButton::new(cx, AppData::value).on_select(|cx| cx.emit(AppEvent::ToggleValue));
/// ```
///
/// ## Radio button with a label
///
/// A radio button is usually used with a label next to it describing what data the radio button
/// is bound to or what the radio button does when pressed. This can be done, for example, by
/// wrapping the radio button in an [`HStack`](crate::prelude::HStack) and adding a [`Label`](crate::prelude::Label)
/// to it.
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
/// # let cx = &mut Context::default();
/// #
/// # AppData { value: false }.build(cx);
/// #
/// HStack::new(cx, |cx| {
///     RadioButton::new(cx, AppData::value);
///     Label::new(cx, "Press me");
/// });
/// ```
pub struct RadioButton {
    on_select: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl RadioButton {
    pub fn new(cx: &mut Context, checked: impl Lens<Target = bool>) -> Handle<Self> {
        Self { on_select: None }
            .build(cx, |cx| {
                Element::new(cx)
                    .class("inner")
                    .hoverable(false)
                    .position_type(PositionType::SelfDirected);
            })
            .checked(checked)
            .cursor(CursorIcon::Hand)
            .navigable(true)
            .checkable(true)
            .role(Role::RadioButton)
            .default_action_verb(DefaultActionVerb::Click)
    }
}

impl View for RadioButton {
    fn element(&self) -> Option<&'static str> {
        Some("radiobutton")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::Press { mouse } => {
                let over = if *mouse { cx.mouse.left.pressed } else { cx.focused() };
                if over == cx.current() && meta.target == cx.current() && !cx.is_disabled() {
                    if let Some(callback) = &self.on_select {
                        (callback)(cx);
                    }
                }
            }

            WindowEvent::ActionRequest(request) => match request.action {
                Action::Default => {
                    if let Some(callback) = &self.on_select {
                        (callback)(cx);
                    }
                }

                _ => {}
            },

            _ => {}
        });
    }
}

impl Handle<'_, RadioButton> {
    /// Set the callback triggered when the radio button is selected.
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
    /// RadioButton::new(cx, AppData::value).on_select(|cx| cx.emit(AppEvent::ToggleValue));
    /// ```
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext),
    {
        self.modify(|radiobutton| radiobutton.on_select = Some(Box::new(callback)))
    }
}
