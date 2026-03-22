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
/// ```ignore
/// # use vizia_core::prelude::*;
/// # let cx = &mut Context::default();
/// #
/// # let selected = Signal::new(0_u8);
/// #
/// Dropdown::new(
///     cx,
///     |cx| Label::new(cx, selected.map(|v| v.to_string())),
///     |cx| {
///         for i in 0..5 {
///             Label::new(cx, i)
///                 .on_press(move |cx| {
///                     selected.set(i);
///                     cx.emit(PopupEvent::Close); // close the popup
///                 })
///                 .width(Stretch(1.0));
///         }
///     },
/// )
/// .width(Pixels(100.0));
/// ```ignore
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
pub struct Dropdown {
    pub is_open: Signal<bool>,
    pub placement: Signal<Placement>,
    pub show_arrow: Signal<bool>,
    pub arrow_size: Signal<Length>,
    pub should_reposition: Signal<bool>,
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
    /// Dropdown::new(cx, |cx| { Label::new(cx, "Text"); }, |_| {});
    /// ```
    pub fn new<F, L>(cx: &mut Context, trigger: L, content: F) -> Handle<Self>
    where
        L: 'static + Fn(&mut Context),
        F: 'static + Fn(&mut Context),
    {
        let is_open = Signal::new(false);
        let placement = Signal::new(Placement::Bottom);
        let show_arrow = Signal::new(true);
        let arrow_size = Signal::new(Length::Value(LengthValue::Px(4.0)));
        let should_reposition = Signal::new(true);

        Self { is_open, placement, show_arrow, arrow_size, should_reposition }.build(
            cx,
            move |cx| {
                (trigger)(cx);

                Binding::new(cx, is_open, move |cx, is_open| {
                    if is_open {
                        Popup::new(cx, |cx| {
                            (content)(cx);
                        })
                        .on_blur(|cx| cx.emit(PopupEvent::Close))
                        .placement(placement)
                        .show_arrow(show_arrow)
                        .arrow_size(arrow_size)
                        .should_reposition(should_reposition);
                    }
                })
            },
        )
    }
}

impl View for Dropdown {
    fn element(&self) -> Option<&'static str> {
        Some("dropdown")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|popup_event, meta| match popup_event {
            PopupEvent::Open => {
                self.is_open.set(true);
                meta.consume();
            }

            PopupEvent::Close => {
                self.is_open.set(false);
                meta.consume();
            }

            PopupEvent::Switch => {
                self.is_open.set(!self.is_open.get());

                meta.consume();
            }
        });
    }
}

impl Handle<'_, Dropdown> {
    /// Sets the position where the tooltip should appear relative to its parent element.
    /// Defaults to `Placement::Bottom`.
    pub fn placement(self, placement: impl Res<Placement>) -> Self {
        self.bind(placement, |handle, placement| {
            handle.modify(|dropdown| {
                dropdown.placement.set(placement);
            });
        })
    }

    /// Sets whether the popup should include an arrow. Defaults to true.
    pub fn show_arrow(self, show_arrow: impl Res<bool>) -> Self {
        self.bind(show_arrow, |handle, show_arrow| {
            handle.modify(|dropdown| {
                dropdown.show_arrow.set(show_arrow);
            });
        })
    }

    /// Sets the size of the popup arrow, or gap if the arrow is hidden.
    pub fn arrow_size<U: Into<Length>>(self, size: impl Res<U>) -> Self {
        self.bind(size, |handle, size| {
            let size = size;
            handle.modify(|dropdown| {
                dropdown.arrow_size.set(size.into());
            });
        })
    }

    /// Set to whether the popup should reposition to always be visible.
    pub fn should_reposition(self, flag: impl Res<bool>) -> Self {
        self.bind(flag, |handle, flag| {
            handle.modify(|dropdown| {
                dropdown.should_reposition.set(flag);
            });
        })
    }
}
