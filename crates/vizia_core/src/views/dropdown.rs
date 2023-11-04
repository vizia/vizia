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
/// # let cx = &mut Context::default();
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
/// The line marked "close the popup" is not required for anything other than closing the popup -
/// if you leave it out, the popup will simply not close until the user clicks out of the dropdown.
///
/// ## Custom Dropdown
///
/// The dropdown doesn't have to be the current state and then a set of options - it can contain any
/// set of views in either location. Here's an example where you can use a textbox to filter a list
/// of checkboxes which pop up when you click the textbox:
///
/// ```
/// # use vizia_core::prelude::*;
/// # let cx = &mut Context::default();
/// #
/// # cx.add_stylesheet(r#"
/// #     dropdown popup {
/// #         background-color: white;
/// #     }
/// # "#);
///
/// #[derive(Lens, Clone, PartialEq, Eq)]
/// struct AppData {
///     values: [bool; 6],
///     filter: String,
/// }
///
/// # impl Data for AppData {
/// #     fn same(&self, other: &Self) -> bool {
/// #         self == other
/// #     }
/// # }
/// #
/// # #[derive(Debug)]
/// # enum AppEvent {
/// #     SetFilter(String),
/// #     SetValue(usize, bool),
/// # }
/// #
/// # impl Model for AppData {
/// #     fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
/// #         event.map(|msg, _| {
/// #             match msg {
/// #                 AppEvent::SetFilter(s) => self.filter = s.clone(),
/// #                 AppEvent::SetValue(i, b) => self.values[*i] = *b,
/// #             }
/// #         });
/// #     }
/// # }
/// #
/// # const LABELS: [&str; 6] = ["Bees", "Butterflies", "Dragonflies", "Crickets", "Moths", "Ladybugs"];
/// #
/// # AppData {
/// #     values: [true, false, true, false, true, false],
/// #     filter: "".to_owned(),
/// # }.build(cx);
///
/// Dropdown::new(cx, |cx| {
///     Textbox::new(cx, AppData::filter).on_edit(|cx, text| {
///         cx.emit(AppEvent::SetFilter(text));
///     })
///     .width(Pixels(100.0))
///     .height(Pixels(30.0))
/// }, |cx| {
///     Binding::new(cx, AppData::root, |cx, lens| {
///         let current = lens.get(cx);
///         for i in 0..6 {
///             if LABELS[i].to_lowercase().contains(&current.filter.to_lowercase()) {
///                 HStack::new(cx, move |cx| {
///                     Checkbox::new(cx, AppData::values.map(move |x| x[i]))
///                         .on_toggle(move |cx| {
///                             cx.emit(AppEvent::SetValue(i, !current.values[i]));
///                         });
///                     Label::new(cx, LABELS[i]);
///                 });
///             }
///         }
///     });
/// }).width(Pixels(100.0));
/// ```
pub struct Dropdown;

impl Dropdown {
    /// Creates a new dropdown.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
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
                    .role(Role::PopupButton)
                    .width(Stretch(1.0))
                    .cursor(CursorIcon::Hand)
                    .checked(PopupData::is_open)
                    .navigable(true)
                    .on_press(|cx| cx.emit(PopupEvent::Switch));

                Popup::new(cx, PopupData::is_open, false, move |cx| {
                    (content)(cx);
                })
                .on_blur(|cx| cx.emit(PopupEvent::Close));
            })
            .cursor(CursorIcon::Hand)
    }
}

impl View for Dropdown {
    fn element(&self) -> Option<&'static str> {
        Some("dropdown")
    }
}
