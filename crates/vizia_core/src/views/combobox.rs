use std::marker::PhantomData;

use crate::prelude::*;

/// A ComboBox view which combines a textbox with a picklist, allowing users to filter to only the options matching a query.
pub struct ComboBox<
    L1: Lens<Target = Vec<T>>,
    L2: Lens<Target = usize>,
    T: 'static + Data + ToString,
> {
    // Text to filter the list.
    filter_text: Signal<String>,
    // Text to display when the combobox is unfocused.
    placeholder: Signal<String>,
    // Callback triggered when an item is selected.
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
    // Lens to a list of values.
    list_lens: L1,
    // Lens to the selected value.
    selected: L2,
    // Whether the popup list is visible.
    is_open: Signal<bool>,

    p: PhantomData<T>,
}

pub(crate) enum ComboBoxEvent {
    SetOption(usize),
    SetFilterText(String),
}

impl<L1, L2, T> ComboBox<L1, L2, T>
where
    L1: Copy + Lens<Target = Vec<T>>,
    T: 'static + Data + ToString,
    L2: Copy + Lens<Target = usize>,
{
    /// Creates a new [ComboBox] view.
    pub fn new(cx: &mut Context, list_lens: L1, selected: L2) -> Handle<Self> {
        let filter_text = cx.state(String::from(""));
        let placeholder = cx.state(String::from("One"));
        let is_open = cx.state(false);

        Self {
            filter_text,
            on_select: None,
            list_lens,
            selected,
            p: PhantomData,
            is_open,
            placeholder,
        }
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

            Textbox::new(cx, filter_text)
                .on_edit(|cx, txt| cx.emit(ComboBoxEvent::SetFilterText(txt)))
                // Prevent the textbox from losing focus on blur. We control that with the listener instead.
                .on_blur(|_| {})
                // Prevent the textbox from losing focus on cancel (escape key press).
                .on_cancel(|_| {})
                .width(Stretch(1.0))
                .height(Pixels(32.0))
                .placeholder(placeholder)
                .class("title");

            Binding::new(cx, is_open, move |cx| {
                if *is_open.get(cx) {
                    Popup::new(cx, move |cx: &mut Context| {
                        // Binding to the filter text.
                        Binding::new(cx, filter_text, move |cx| {
                            let f = filter_text.get(cx).clone();
                            List::new_filtered(
                                cx,
                                list_lens,
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
                            .selectable(Selectable::Single)
                            .selected(selected.map(|s| vec![*s]))
                            .on_select(|cx, index| {
                                cx.emit(ComboBoxEvent::SetOption(index));
                                cx.emit(PopupEvent::Close);
                            });
                        });
                    })
                    .should_reposition(false)
                    .arrow_size(Pixels(4.0));
                }
            });
        })
        .bind(selected, move |handle, selected| {
            let selected_item = list_lens.idx(selected.get(&handle)).get(&handle);
            handle.modify2(|combobox, cx| combobox.placeholder.set(cx, selected_item.to_string()));
        })
    }
}

impl<L1, L2, T> View for ComboBox<L1, L2, T>
where
    L1: Lens<Target = Vec<T>>,
    T: 'static + Data + ToString,
    L2: Lens<Target = usize>,
{
    fn element(&self) -> Option<&'static str> {
        Some("combobox")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|combobox_event, _| match combobox_event {
            ComboBoxEvent::SetOption(index) => {
                // Set the placeholder text to the selected item.
                let selected_item = self.list_lens.idx(*index).get(cx);
                self.placeholder.set(cx, selected_item.to_string());

                // Call the on_select callback.
                if let Some(callback) = &self.on_select {
                    (callback)(cx, *index);
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
                let selected = self.selected.get(cx);
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

impl<L1, L2, T> Handle<'_, ComboBox<L1, L2, T>>
where
    L1: Lens<Target = Vec<T>>,
    T: 'static + Data + ToString,
    L2: Lens<Target = usize>,
{
    /// Set the callback triggered when an item is selected from the [ComboBox] dropdown list.
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|combobox: &mut ComboBox<L1, L2, T>| {
            combobox.on_select = Some(Box::new(callback))
        })
    }
}
