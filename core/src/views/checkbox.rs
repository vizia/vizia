use crate::{Context, Handle, Lens, MouseButton, Res, View, WindowEvent};

const ICON_CHECK: &str = "\u{2713}";

/// A checkbox widget.
///
/// The checkbox widget can be used to represent data which can be in a true or false state.
///
/// Clicking on the checkbox with the left mouse button triggers the `on_toggle` callback.
/// The checkbox itself does not store its state, and instead must be bound to some app data.
///
///
/// # Example
/// The following creates a simple checkbox with an initial state of false.
/// ```compile_fail
/// Checkbox::new(cx, false);
/// ```
///
/// To add a label, wrap the checkbox within a `HStack` view with a `Label`:
/// ```compile_fail
/// HStack::new(cx, |cx|{
///     Checkbox::new(cx, false);
///     Label::new(cx, "Press me");
/// }).col_between(Pixels(5.0));
/// ```
///
/// To use the checkbox, bind its value to some app data and use the `on_toggle` callback to mutate the data:
/// ```compile_fail
/// Binding::new(cx, AppData::value, |cx, value|{
///     Checkbox::new(cx, *value.get(cx))
///         .on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
/// })
/// ```
///
pub struct Checkbox {
    on_toggle: Option<Box<dyn Fn(&mut Context)>>,
}

impl Checkbox {
    pub fn new(cx: &mut Context, checked: impl Lens<Target = bool>) -> Handle<Self> {
        //let checked = checked.get_val_fallible(cx).unwrap_or(false);
        Self { on_toggle: None }.build2(cx, |_| {}).bind(checked, move |handle, lens| {
            let flag = lens.get_val_fallible(handle.cx).unwrap_or(false);
            handle.text(if flag { ICON_CHECK } else { "" }).checked(flag);
        })
        //.text(if checked { ICON_CHECK } else { "" })
        //.checked(checked)
    }
}

impl<'a> Handle<'a, Checkbox> {
    /// Set the callback triggered when the checkbox is pressed.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// Checkbox::new(cx, false)
    ///     .on_toggle(cx, |cx| {
    ///         cx.emit(WindowEvent::Debug(format!("Checkbox pressed!")));
    ///     });
    /// ```
    pub fn on_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        if let Some(view) = self.cx.views.get_mut(&self.entity) {
            if let Some(checkbox) = view.downcast_mut::<Checkbox>() {
                checkbox.on_toggle = Some(Box::new(callback));
            }
        }

        self
    }
}

impl View for Checkbox {
    fn element(&self) -> Option<String> {
        Some("checkbox".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if event.target == cx.current {
                        if let Some(callback) = self.on_toggle.take() {
                            (callback)(cx);

                            self.on_toggle = Some(callback);
                        }
                    }
                }

                _ => {}
            }
        }
    }
}
