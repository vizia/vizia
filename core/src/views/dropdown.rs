use crate::prelude::*;
use crate::views::popup::PopupData;

/// A dropdown is used to display some state with the ability to open a popup with options to change that state.
///
/// Usually a dropdown is used in the context of a "combo box" or "list picker" to allow the user to select
/// from one of several discrete options. The dropdown takes two closures, one which shows the current state
/// regardless of whether the dropdown is open or closed, and one which shows the contents while it is open.
///
/// ## Basic dropdown
///
/// A basic dropdown displaying five options that the user can choose from.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # #[derive(Lens)]
/// # struct AppData {
/// #     value: u8,
/// # }
/// #
/// # impl Model for AppData {}
/// #
/// # enum AppEvent {
/// #     SetValue(u8),
/// # }
/// #
/// # let cx = &mut Context::new();
/// #
/// # AppData { value: 0 }.build(cx);
/// #
/// Dropdown::new(
///     cx,
///     |cx| Label::new(cx, AppData::value),
///     |cx| {
///         for i in 0..5 {
///             Label::new(cx, i)
///                 .on_press(move |cx| {
///                     cx.emit(AppEvent::SetValue(i));
///                     cx.emit(PopupEvent::Close); // close the popup
///                 })
///                 .width(Stretch(1.0));
///         }
///     },
/// )
/// .width(Pixels(100.0));
/// ```
///
/// The line marked "close the popop" is not required for anything other than closing the popup -
/// if you leave it out, the popup will simply not close until the user clicks out of the dropdown.
pub struct Dropdown;

impl Dropdown {
    /// Creates a new dropdown.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::new();
    /// #
    /// Dropdown::new(cx, |cx| Label::new(cx, "Text"), |_| {});
    /// ```
    pub fn new<F, L, V>(cx: &mut Context, label: L, content: F) -> Handle<Self>
    where
        L: 'static + Fn(&mut Context) -> Handle<V>,
        F: 'static + Fn(&mut Context),
        V: 'static + View,
    {
        Self {}
            .build(cx, move |cx| {
                PopupData::default().build(cx);

                (label)(cx)
                    .class("title")
                    .width(Stretch(1.0))
                    .on_press(|cx| cx.emit(PopupEvent::Switch));

                Popup::new(cx, PopupData::is_open, move |cx| {
                    (content)(cx);
                })
                .on_blur(|cx| cx.emit(PopupEvent::Close))
                .top(Percentage(100.0))
                .height(Auto);
            })
            .size(Auto)
    }
}

impl View for Dropdown {
    fn element(&self) -> Option<&'static str> {
        Some("dropdown")
    }
}
