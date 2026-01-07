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
/// # let cx = &mut Context::default();
/// #
/// let value = cx.state(0u8);
/// let width_100 = cx.state(Pixels(100.0));
/// let stretch_one = cx.state(Stretch(1.0));
///
/// Dropdown::new(
///     cx,
///     move |cx| {
///         let label = cx.derived({
///             let value = value;
///             move |s| format!("Value: {}", value.get(s))
///         });
///         Label::new(cx, label)
///     },
///     move |cx| {
///         for i in 0..5u8 {
///             let option = cx.state(i);
///             Label::new(cx, option)
///                 .on_press(move |cx| {
///                     value.set(cx, i);
///                     cx.emit(PopupEvent::Close); // close the popup
///                 })
///                 .width(stretch_one);
///         }
///     },
/// )
/// .width(width_100);
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
/// # const LABELS: [&str; 6] = ["Bees", "Butterflies", "Dragonflies", "Crickets", "Moths", "Ladybugs"];
/// #
/// let filter = cx.state(String::new());
/// let values = cx.state([true, false, true, false, true, false]);
/// let width_100 = cx.state(Pixels(100.0));
/// let height_30 = cx.state(Pixels(30.0));
/// let gap_4 = cx.state(Pixels(4.0));
/// let stretch_one = cx.state(Stretch(1.0));
///
/// let view_state = cx.derived({
///     let filter = filter;
///     let values = values;
///     move |s| (filter.get(s).to_lowercase(), *values.get(s))
/// });
///
/// Dropdown::new(cx, move |cx| {
///     Textbox::new(cx, filter)
///         .two_way()
///         .width(width_100)
///         .height(height_30);
/// }, move |cx| {
///     Binding::new(cx, view_state, move |cx| {
///         let (filter_text, _) = view_state.get(cx).clone();
///         for i in 0..LABELS.len() {
///             if LABELS[i].to_lowercase().contains(&filter_text) {
///                 HStack::new(cx, move |cx| {
///                     let checked = cx.derived({
///                         let values = values;
///                         move |s| values.get(s)[i]
///                     });
///                     Checkbox::new(cx, checked)
///                         .on_toggle(move |cx| {
///                             values.upd(cx, |vals| vals[i] = !vals[i]);
///                         });
///                     let label = cx.state(LABELS[i]);
///                     Label::new(cx, label);
///                 })
///                 .width(stretch_one)
///                 .horizontal_gap(gap_4);
///             }
///         }
///     });
/// }).width(width_100);
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
    /// let text = cx.state("Text");
    /// Dropdown::new(cx, |cx| Label::new(cx, text), |_| {});
    /// ```
    pub fn new<F, L>(cx: &mut Context, trigger: L, content: F) -> Handle<Self>
    where
        L: 'static + Fn(&mut Context),
        F: 'static + Fn(&mut Context),
    {
        let is_open = cx.state(false);
        let placement = cx.state(Placement::Bottom);
        let show_arrow = cx.state(true);
        let arrow_size = cx.state(Length::Value(LengthValue::Px(4.0)));
        let should_reposition = cx.state(true);
        Self { is_open, placement, show_arrow, arrow_size, should_reposition }.build(
            cx,
            move |cx| {
                (trigger)(cx);

                Binding::new(cx, is_open, move |cx| {
                    if *is_open.get(cx) {
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

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|popup_event, meta| match popup_event {
            PopupEvent::Open => {
                self.is_open.set(cx, true);
                meta.consume();
            }

            PopupEvent::Close => {
                self.is_open.set(cx, false);
                meta.consume();
            }

            PopupEvent::Switch => {
                let is_open = *self.is_open.get(cx);
                self.is_open.set(cx, !is_open);
                meta.consume();
            }
        });
    }
}

impl Handle<'_, Dropdown> {
    /// Sets the position where the tooltip should appear relative to its parent element.
    /// Defaults to `Placement::Bottom`.
    pub fn placement(self, placement: Signal<Placement>) -> Self {
        self.bind(placement, |handle, placement| {
            let placement = *placement.get(&handle);
            handle.modify2(|dropdown, cx| dropdown.placement.set(cx, placement));
        })
    }

    /// Sets whether the popup should include an arrow. Defaults to true.
    pub fn show_arrow(self, show_arrow: Signal<bool>) -> Self {
        self.bind(show_arrow, |handle, show_arrow| {
            let show_arrow = *show_arrow.get(&handle);
            handle.modify2(|dropdown, cx| dropdown.show_arrow.set(cx, show_arrow));
        })
    }

    /// Sets the size of the popup arrow, or gap if the arrow is hidden.
    pub fn arrow_size(self, size: Signal<Length>) -> Self {
        self.bind(size, |handle, size| {
            let size = size.get(&handle).clone();
            handle.modify2(|dropdown, cx| dropdown.arrow_size.set(cx, size));
        })
    }

    /// Set to whether the popup should reposition to always be visible.
    pub fn should_reposition(self, flag: Signal<bool>) -> Self {
        self.bind(flag, |handle, flag| {
            let flag = *flag.get(&handle);
            handle.modify2(|dropdown, cx| dropdown.should_reposition.set(cx, flag));
        })
    }
}
