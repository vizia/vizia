use crate::{
    style::PropSet, Context, Handle, MouseButton, PseudoClass, Units::*, View, WindowEvent,
};

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
/// ```no_run
/// # use vizia_core::*;
/// # use vizia_glutin::application::*;
/// # Application::new(WindowDescription::new(), |cx|{
/// Checkbox::new(cx, false);
/// # }).run();
/// ```
///
/// To add a label, wrap the checkbox within a `HStack` view with a `Label` component:
/// ```no_run
/// # use vizia_core::*;
/// # use vizia_glutin::application::*;
/// # Application::new(WindowDescription::new(), |cx|{
/// HStack::new(cx, |cx|{
///     Checkbox::new(cx, false);
///     Label::new(cx, "Press me");
/// }).col_between(Pixels(5.0));
/// # }).run();
/// ```
///
/// To use the checkbox, bind its value to some app data and use the `on_toggle` callback to mutate the data:
/// ```no_run
/// # use vizia_core::*;
/// # use vizia_glutin::application::*;
/// # #[derive(Lens)]
/// # pub struct AppData {value: bool};
/// # impl Model for AppData {};
/// # #[derive(Debug)]
/// # pub enum AppEvent{ToggleValue};
/// # Application::new(WindowDescription::new(), |cx|{
/// # AppData{value: false}.build(cx);
/// Binding::new(cx, AppData::value, |cx, value|{
///     Checkbox::new(cx, *value.get(cx))
///         .on_toggle(cx, |cx| cx.emit(AppEvent::ToggleValue));
/// })
/// # }).run();
/// ```
///
pub struct Checkbox {
    on_toggle: Option<Box<dyn Fn(&mut Context)>>,
}

impl Checkbox {
    pub fn new(cx: &mut Context, checked: bool) -> Handle<Self> {
        Self { on_toggle: None }
            .build2(cx, |_| {})
            .width(Pixels(20.0))
            .height(Pixels(20.0))
            .text(if checked { ICON_CHECK } else { "" })
            .checked(checked)
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

                        // if self.checked {
                        //     self.checked = false;
                        //     if let Some(pseudo_classes) =
                        //         cx.style.borrow_mut().pseudo_classes.get_mut(cx.current)
                        //     {
                        //         pseudo_classes.set(PseudoClass::CHECKED, false);
                        //     }

                        //     cx.current.set_text(cx, "");

                        //     if let Some(callback) = self.on_unchecked.take() {
                        //         (callback)(cx);

                        //         self.on_unchecked = Some(callback);
                        //     }

                        // } else {
                        //     self.checked = true;
                        //     if let Some(pseudo_classes) =
                        //         cx.style.borrow_mut().pseudo_classes.get_mut(cx.current)
                        //     {
                        //         pseudo_classes.set(PseudoClass::CHECKED, true);
                        //     }

                        //     cx.current.set_text(cx, ICON_CHECK);

                        //     if let Some(callback) = self.on_checked.take() {
                        //         (callback)(cx);

                        //         self.on_checked = Some(callback);
                        //     }

                        //     if let Some(callback) = self.on_change.take() {
                        //         (callback)(cx, self.checked);

                        //         self.on_change = Some(callback);
                        //     }
                        // }
                    }
                }

                _ => {}
            }
        }
    }
}
