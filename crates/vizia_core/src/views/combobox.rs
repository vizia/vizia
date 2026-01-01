use crate::prelude::*;

/// A ComboBox view which combines a textbox with a picklist, allowing users to filter to only the options matching a query.
pub struct ComboBox<T: 'static + Clone + ToString> {
    // Text to filter the list.
    filter_text: Signal<String>,
    // Text to display when the combobox is unfocused.
    placeholder: Signal<String>,
    // Callback triggered when an item is selected.
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
    // List of values.
    list: Signal<Vec<T>>,
    // Selected value index.
    selected: Signal<usize>,
    // Whether the popup list is visible.
    is_open: Signal<bool>,
}

pub(crate) enum ComboBoxEvent {
    SetOption(usize),
    SetFilterText(String),
}

impl<T> ComboBox<T>
where
    T: 'static + Clone + ToString,
{
    /// Creates a new [ComboBox] view.
    pub fn new(cx: &mut Context, list: Signal<Vec<T>>, selected: Signal<usize>) -> Handle<Self> {
        let filter_text = cx.state(String::from(""));
        let placeholder = cx.state(String::from("One"));
        let is_open = cx.state(false);

        Self { filter_text, on_select: None, list, selected, is_open, placeholder }
            .build(cx, |cx| {
                // Add listener to defocus when mouse is pressed outside the combobox.
                cx.add_listener(move |popup: &mut Self, cx, event| {
                    let flag: bool = *popup.is_open.get(cx);
                    event.map(|window_event, meta| match window_event {
                        WindowEvent::MouseDown(_) => {
                            if flag && meta.origin != cx.current() {
                                // Check if the mouse was pressed outside of any descendants.
                                // TODO: Replace with a check to is_over when that works correctly.
                                if !cx.hovered.is_descendant_of(cx.tree, cx.current) {
                                    cx.emit(TextEvent::Submit(false));
                                    cx.emit_custom(
                                        Event::new(TextEvent::EndEdit)
                                            .target(cx.current)
                                            .propagate(Propagation::Subtree),
                                    );
                                    meta.consume();
                                }
                            }
                        }

                        _ => {}
                    });
                });

                let stretch_one = cx.state(Stretch(1.0));
                let input_height = cx.state(Pixels(32.0));
                Textbox::new(cx, filter_text)
                    .on_edit(|cx, txt| cx.emit(ComboBoxEvent::SetFilterText(txt)))
                    // Prevent the textbox from losing focus on blur. We control that with the listener instead.
                    .on_blur(|_| {})
                    // Prevent the textbox from losing focus on cancel (escape key press).
                    .on_cancel(|_| {})
                    .width(stretch_one)
                    .height(input_height)
                    .placeholder(placeholder)
                    .class("title");

                let should_reposition = cx.state(false);
                let arrow_size = cx.state(Length::Value(LengthValue::Px(4.0)));
                let selectable_single = cx.state(Selectable::Single);
                let selected_indices = cx.derived({
                    let selected = selected;
                    move |s| vec![*selected.get(s)]
                });

                Binding::new(cx, is_open, move |cx| {
                    if *is_open.get(cx) {
                        Popup::new(cx, move |cx: &mut Context| {
                            // Binding to the filter text.
                            Binding::new(cx, filter_text, move |cx| {
                                let f = filter_text.get(cx).clone();
                                List::new_filtered(
                                    cx,
                                    list,
                                    move |item| {
                                        if f.is_empty() {
                                            true
                                        } else {
                                            item.to_string()
                                                .to_ascii_lowercase()
                                                .contains(&f.to_ascii_lowercase())
                                        }
                                    },
                                    |cx, _, item| {
                                        Label::new(cx, item);
                                    },
                                )
                                .selectable(selectable_single)
                                .selected(selected_indices)
                                .on_select(|cx, index| {
                                    cx.emit(ComboBoxEvent::SetOption(index));
                                    cx.emit(PopupEvent::Close);
                                });
                            });
                        })
                        .should_reposition(should_reposition)
                        .arrow_size(arrow_size);
                    }
                });
            })
            .bind(selected, move |handle, selected| {
                let selected_index = *selected.get(&handle);
                let selected_item = list.get(&handle).get(selected_index).cloned();
                if let Some(selected_item) = selected_item {
                    handle.modify2(|combobox, cx| {
                        combobox.placeholder.set(cx, selected_item.to_string())
                    });
                }
            })
    }
}

impl<T> View for ComboBox<T>
where
    T: 'static + Clone + ToString,
{
    fn element(&self) -> Option<&'static str> {
        Some("combobox")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|combobox_event, _| match combobox_event {
            ComboBoxEvent::SetOption(index) => {
                // Set the placeholder text to the selected item.
                let index = *index;
                if let Some(selected_item) = self.list.get(cx).get(index).cloned() {
                    self.placeholder.set(cx, selected_item.to_string());
                    self.selected.set(cx, index);
                }

                // Call the on_select callback.
                if let Some(callback) = &self.on_select {
                    (callback)(cx, index);
                }

                // Close the popup.
                self.is_open.set(cx, false);

                // Reset the filter text.
                self.filter_text.set(cx, String::new());

                // Set the textbox to non-edit state.
                // TODO: Add a modifier to textbox and bind to some state in combobox.
                cx.emit_custom(
                    Event::new(TextEvent::EndEdit)
                        .target(cx.current)
                        .propagate(Propagation::Subtree),
                );
            }

            ComboBoxEvent::SetFilterText(text) => {
                self.placeholder.set(cx, text.clone());
                self.filter_text.set(cx, text.clone());

                // Reopen the popup in case it was closed with the ESC key.
                self.is_open.set(cx, true);
            }
        });

        event.map(|textbox_event, _| match textbox_event {
            // User pressed on the textbox or focused it.
            TextEvent::StartEdit => {
                self.is_open.set(cx, true);
            }

            TextEvent::Submit(enter) => {
                let selected = *self.selected.get(cx);
                if *enter {
                    // User pressed the enter key.
                    //cx.emit(ComboBoxEvent::SetOption(self.hovered));
                } else {
                    // User clicked outside the textbox.
                    cx.emit(ComboBoxEvent::SetOption(selected));
                }
            }

            _ => {}
        });

        event.map(|window_event, meta| match window_event {
            WindowEvent::KeyDown(code, _) => match code {
                Code::ArrowDown => {
                    // Forward events to list
                    if meta.origin != cx.current() {
                        cx.emit_custom(
                            Event::new(window_event.clone())
                                .origin(cx.current())
                                .target(Entity::root())
                                .propagate(Propagation::Subtree),
                        );
                    }
                }

                Code::ArrowUp => {
                    // Forward events to list
                    if meta.origin != cx.current() {
                        cx.emit_custom(
                            Event::new(window_event.clone())
                                .origin(cx.current())
                                .target(Entity::root())
                                .propagate(Propagation::Subtree),
                        );
                    }
                }

                Code::Enter => {
                    if meta.origin != cx.current() {
                        cx.emit_custom(
                            Event::new(window_event.clone())
                                .origin(cx.current())
                                .target(Entity::root())
                                .propagate(Propagation::Subtree),
                        );
                    }
                }

                Code::Escape => {
                    if *self.is_open.get(cx) {
                        self.is_open.set(cx, false);
                    } else {
                        cx.emit(TextEvent::Submit(false));
                    }
                }

                _ => {}
            },

            _ => {}
        });
    }
}

impl<T> Handle<'_, ComboBox<T>>
where
    T: 'static + Clone + ToString,
{
    /// Set the callback triggered when an item is selected from the [ComboBox] dropdown list.
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|combobox: &mut ComboBox<T>| combobox.on_select = Some(Box::new(callback)))
    }
}
