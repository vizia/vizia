use crate::prelude::*;

/// A dropdown is used to display some state with the ability to open a popup with options to change that state.
///
/// Usually a dropdown is used in the context of a "combobox" or "picklist" to allow the user to select
/// from one of several discrete options. The dropdown takes two closures, one which shows the current state
/// regardless of whether the dropdown is open or closed, and one which shows the contents while it is open.
///
/// ## Basic Dropdown
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
#[derive(Lens)]
pub struct Dropdown {
    /// Whether the dropdown is open.
    pub is_open: PopupData,
    /// The placement of the dropdown popup.
    pub placement: Placement,
    /// Whether the dropdown popup should show an arrow.
    pub show_arrow: bool,
    /// The size of the arrow.
    pub arrow_size: Length,
    /// Whether the dropdown popup should reposition to always be visible.
    pub should_reposition: bool,
}

impl Dropdown {
    /// Creates a new dropdown.
    ///
    /// # Example
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Dropdown::new(cx, |cx| Label::new(cx, "Text"), |_| {});
    /// ```
    pub fn new<F, L>(cx: &mut Context, trigger: L, content: F) -> Handle<Self>
    where
        L: 'static + Fn(&mut Context),
        F: 'static + Fn(&mut Context),
    {
        Self {
            is_open: PopupData::default(),
            placement: Placement::Bottom,
            show_arrow: true,
            arrow_size: Length::Value(LengthValue::Px(4.0)),
            should_reposition: true,
        }
        .build(cx, move |cx| {
            (trigger)(cx);

            Binding::new(cx, Self::is_open, move |cx, is_open| {
                if is_open.get(cx).into() {
                    Popup::new(cx, |cx| {
                        (content)(cx);
                    })
                    .on_blur(|cx| cx.emit(PopupEvent::Close))
                    .placement(Dropdown::placement)
                    .show_arrow(Dropdown::show_arrow)
                    .arrow_size(Dropdown::arrow_size)
                    .should_reposition(Dropdown::should_reposition);
                }
            })
        })
    }
}

impl View for Dropdown {
    fn element(&self) -> Option<&'static str> {
        Some("dropdown")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.is_open.event(cx, event);
    }
}

impl Handle<'_, Dropdown> {
    /// Sets the position where the tooltip should appear relative to its parent element.
    /// Defaults to `Placement::Bottom`.
    pub fn placement(self, placement: impl Res<Placement>) -> Self {
        self.bind(placement, |handle, placement| {
            let placement = placement.get(&handle);
            handle.modify(|dropdown| {
                dropdown.placement = placement;
            });
        })
    }

    /// Sets whether the popup should include an arrow. Defaults to true.
    pub fn show_arrow(self, show_arrow: bool) -> Self {
        self.modify(|dropdown| dropdown.show_arrow = show_arrow)
    }

    /// Sets the size of the popup arrow, or gap if the arrow is hidden.
    pub fn arrow_size(self, size: impl Into<Length>) -> Self {
        self.modify(|dropdown| dropdown.arrow_size = size.into())
    }

    /// Set to whether the popup should reposition to always be visible.
    pub fn should_reposition(self, flag: bool) -> Self {
        self.modify(|dropdown| dropdown.should_reposition = flag)
    }
}
