use std::marker::PhantomData;

use crate::prelude::*;

/// A ComboBox view which combines a textbox with a list popup, allowing users to
/// filter options by typing.
pub struct ComboBox<L: SignalGet<Vec<T>>, S: SignalGet<usize>, T: 'static + Clone + ToString> {
    // Text used to filter the list.
    filter_text: Signal<String>,
    // Text displayed when the textbox is not actively editing.
    placeholder: Signal<String>,
    // Callback triggered when an item is selected.
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
    // Source list of values.
    list: L,
    // Selected item index in the source list.
    selected: S,
    // Whether the popup list is visible.
    is_open: Signal<bool>,
    // Highlighted source index in the filtered popup list.
    highlighted: Signal<Option<usize>>,

    p: PhantomData<T>,
}

pub(crate) enum ComboBoxEvent {
    SetOption(usize),
    SetFilterText(String),
}

impl<L, S, T> ComboBox<L, S, T>
where
    L: SignalGet<Vec<T>> + Copy + 'static,
    T: 'static + Clone + ToString,
    S: SignalGet<usize> + Copy + 'static,
{
    /// Creates a new [ComboBox] view.
    pub fn new(cx: &mut Context, list: L, selected: S) -> Handle<Self> {
        let filter_text = Signal::new(String::new());
        let placeholder = Signal::new(
            list.get()
                .get(selected.get())
                .map(ToString::to_string)
                .unwrap_or_else(|| String::from("Select")),
        );
        let is_open = Signal::new(false);
        let highlighted = Signal::new(Some(selected.get()));

        Self {
            filter_text,
            placeholder,
            on_select: None,
            list,
            selected,
            is_open,
            highlighted,
            p: PhantomData,
        }
        .build(cx, move |cx| {
            // Defocus when clicking outside the combobox while the popup is open.
            cx.add_listener(move |popup: &mut Self, cx, event| {
                let open = popup.is_open.get();
                event.map(|window_event, meta| match window_event {
                    WindowEvent::MouseDown(_) => {
                        if open
                            && meta.origin != cx.current()
                            && !cx.hovered.is_descendant_of(cx.tree, cx.current)
                        {
                            cx.emit(TextEvent::Submit(false));
                            cx.emit_custom(
                                Event::new(TextEvent::EndEdit)
                                    .target(cx.current)
                                    .propagate(Propagation::Subtree),
                            );
                            meta.consume();
                        }
                    }

                    _ => {}
                });
            });

            Textbox::new(cx, filter_text)
                .on_edit(|cx, txt| cx.emit(ComboBoxEvent::SetFilterText(txt)))
                // Prevent blur/cancel from ending edit; this view handles it explicitly.
                .on_blur(|_| {})
                .on_cancel(|_| {})
                .width(Stretch(1.0))
                .height(Pixels(32.0))
                .placeholder(placeholder)
                .class("title");

            Binding::new(cx, is_open, move |cx| {
                let open = is_open.get();
                if open {
                    Popup::new(cx, move |cx: &mut Context| {
                        let filtered_indices = Memo::new(move |_| {
                            let query = filter_text.get().to_ascii_lowercase();
                            list.get()
                                .iter()
                                .enumerate()
                                .filter_map(|(index, item)| {
                                    if query.is_empty()
                                        || item.to_string().to_ascii_lowercase().contains(&query)
                                    {
                                        Some(index)
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<usize>>()
                        });

                        Binding::new(cx, filtered_indices, move |cx| {
                            let indices = filtered_indices.get();
                            let values = list.get();
                            let options = indices
                                .into_iter()
                                .filter_map(|index| {
                                    values.get(index).map(|item| (index, item.to_string()))
                                })
                                .collect::<Vec<(usize, String)>>();

                            let options_state = Signal::new(
                                options.into_iter().map(Signal::new).collect::<Vec<_>>(),
                            );

                            let highlighted_row = Memo::new(move |_| {
                                let highlighted_source = highlighted.get();
                                let row = highlighted_source.and_then(|source_index| {
                                    options_state
                                        .get()
                                        .iter()
                                        .position(|item| item.get().0 == source_index)
                                });

                                row.map(|idx| vec![idx]).unwrap_or_default()
                            });

                            List::new(cx, options_state, move |cx, _row, item| {
                                Label::new(cx, item.map(|(_, text)| text.clone())).hoverable(false);
                            })
                            .width(Stretch(1.0))
                            .selectable(Selectable::Single)
                            .selected(highlighted_row)
                            .show_horizontal_scrollbar(false)
                            .on_select(move |cx, row| {
                                if let Some((source_index, _)) =
                                    options_state.get().get(row).map(|item| item.get())
                                {
                                    cx.emit(ComboBoxEvent::SetOption(source_index));
                                    cx.emit(PopupEvent::Close);
                                }
                            });
                        });
                    })
                    .should_reposition(false)
                    .arrow_size(Pixels(4.0));
                }
            });

            Binding::new(cx, selected, move |_cx| {
                let selected_index = selected.get();
                if let Some(selected_item) = list.get().get(selected_index).cloned() {
                    placeholder.set(selected_item.to_string());
                }
                highlighted.set(Some(selected_index));
            });
        })
    }
}

impl<L, S, T> ComboBox<L, S, T>
where
    L: SignalGet<Vec<T>>,
    T: 'static + Clone + ToString,
    S: SignalGet<usize>,
{
    fn filtered_indices(&self) -> Vec<usize> {
        let query = self.filter_text.get().to_ascii_lowercase();
        self.list
            .get()
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                if query.is_empty() || item.to_string().to_ascii_lowercase().contains(&query) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<L, S, T> View for ComboBox<L, S, T>
where
    L: SignalGet<Vec<T>> + 'static,
    T: 'static + Clone + ToString,
    S: SignalGet<usize> + 'static,
{
    fn element(&self) -> Option<&'static str> {
        Some("combobox")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|combobox_event, _| match combobox_event {
            ComboBoxEvent::SetOption(index) => {
                if let Some(selected_item) = self.list.get().get(*index).cloned() {
                    self.placeholder.set(selected_item.to_string());
                }
                self.highlighted.set(Some(*index));

                if let Some(callback) = &self.on_select {
                    (callback)(cx, *index);
                }

                self.is_open.set(false);
                self.filter_text.set(String::new());

                cx.emit_custom(
                    Event::new(TextEvent::EndEdit)
                        .target(cx.current)
                        .propagate(Propagation::Subtree),
                );
            }

            ComboBoxEvent::SetFilterText(text) => {
                self.placeholder.set(text.clone());
                self.filter_text.set(text.clone());
                self.highlighted.set(self.filtered_indices().first().copied());

                // Reopen in case it was closed with Escape.
                self.is_open.set(true);
            }
        });

        event.map(|textbox_event, _| match textbox_event {
            TextEvent::StartEdit => {
                self.is_open.set(true);
                self.highlighted.set(
                    self.filtered_indices().first().copied().or_else(|| Some(self.selected.get())),
                );
            }

            TextEvent::Submit(enter) => {
                let selected = self.selected.get();
                if *enter {
                    // Enter behavior can be enhanced to select current focused popup row.
                } else {
                    cx.emit(ComboBoxEvent::SetOption(selected));
                }
            }

            _ => {}
        });

        event.map(|window_event, meta| match window_event {
            WindowEvent::KeyDown(code, _) => match code {
                Code::ArrowDown => {
                    if self.is_open.get() {
                        let filtered = self.filtered_indices();
                        if !filtered.is_empty() {
                            let current_pos = self
                                .highlighted
                                .get()
                                .and_then(|h| filtered.iter().position(|index| *index == h))
                                .unwrap_or_else(|| {
                                    filtered
                                        .iter()
                                        .position(|index| *index == self.selected.get())
                                        .unwrap_or(0)
                                });

                            let next_pos = (current_pos + 1) % filtered.len();
                            self.highlighted.set(Some(filtered[next_pos]));
                            meta.consume();
                        }
                    }
                }

                Code::ArrowUp => {
                    if self.is_open.get() {
                        let filtered = self.filtered_indices();
                        if !filtered.is_empty() {
                            let current_pos = self
                                .highlighted
                                .get()
                                .and_then(|h| filtered.iter().position(|index| *index == h))
                                .unwrap_or_else(|| {
                                    filtered
                                        .iter()
                                        .position(|index| *index == self.selected.get())
                                        .unwrap_or(0)
                                });

                            let prev_pos =
                                if current_pos == 0 { filtered.len() - 1 } else { current_pos - 1 };

                            self.highlighted.set(Some(filtered[prev_pos]));
                            meta.consume();
                        }
                    }
                }

                Code::Enter => {
                    if self.is_open.get() {
                        if let Some(index) = self.highlighted.get() {
                            cx.emit(ComboBoxEvent::SetOption(index));
                            meta.consume();
                        }
                    }
                }

                Code::Escape => {
                    if self.is_open.get() {
                        self.is_open.set(false);
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

impl<L, S, T> Handle<'_, ComboBox<L, S, T>>
where
    L: SignalGet<Vec<T>> + 'static,
    T: 'static + Clone + ToString,
    S: SignalGet<usize> + 'static,
{
    /// Sets the callback triggered when an item is selected from the [ComboBox] list.
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|combobox: &mut ComboBox<L, S, T>| {
            combobox.on_select = Some(Box::new(callback));
        })
    }
}
