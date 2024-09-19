use std::marker::PhantomData;

use crate::prelude::*;

#[derive(Lens)]
pub struct ComboBox<
    L1: Lens<Target = Vec<T>>,
    L2: Lens<Target = usize>,
    T: 'static + Data + ToString,
> {
    // Text to filter the list.
    filter_text: String,
    // Text to display when the combobox is unfocused.
    placeholder: String,
    // Callback triggered when an item is selected.
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
    // Lens to a list of values.
    list_lens: L1,
    // Lens to the selected value.
    selected: L2,
    // Whether the popup list is visible.
    is_open: bool,
    // Index of value hovered (or selected by arrow keys).
    hovered: usize,
    p: PhantomData<T>,
}

pub enum ComboBoxEvent {
    SetOption(usize),
    SetFilterText(String),
    SetHovered(usize),
    Close,
}

impl<L1, L2, T> ComboBox<L1, L2, T>
where
    L1: Copy + Lens<Target = Vec<T>>,
    T: 'static + Data + ToString,
    L2: Copy + Lens<Target = usize>,
{
    pub fn new(cx: &mut Context, list_lens: L1, selected: L2) -> Handle<Self> {
        Self {
            filter_text: String::new(),
            on_select: None,
            list_lens,
            selected,
            p: PhantomData,
            is_open: false,
            hovered: selected.get(cx),
            placeholder: String::from("One"),
        }
        .build(cx, |cx| {
            // Add listener to defocus when mouse is pressed outside the combobox.
            cx.add_listener(move |popup: &mut Self, cx, event| {
                let flag: bool = popup.is_open;
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

            Textbox::new(cx, Self::filter_text)
                .on_edit(|cx, txt| cx.emit(ComboBoxEvent::SetFilterText(txt)))
                // Prevent the textbox from losing focus on blur. We control that with the listener instead.
                .on_blur(|_| {})
                // Prevent the textbox from losing focus on cancel (escape key press).
                .on_cancel(|_| {})
                .width(Stretch(1.0))
                .height(Pixels(32.0))
                .space(Pixels(0.0))
                .placeholder(Self::placeholder)
                .class("title");

            Binding::new(cx, Self::is_open, move |cx, is_open| {
                if is_open.get(cx) {
                    Popup::new(cx, move |cx: &mut Context| {
                        // Binding to the filter text.
                        Binding::new(cx, Self::filter_text, move |cx, _filter_text| {
                            // Binding to the list of values.
                            Binding::new(cx, list_lens, move |cx, list| {
                                // Seems that the layout bugs out when rebuilding the contents of a scrollview that's been scrolled to 100%.
                                // So instead we just rebuild the whole scrollview.
                                ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                                    let f = Self::filter_text.get(cx);
                                    // List view doesn't have an option for filtering (yet) so we do it manually instead.
                                    VStack::new(cx, |cx| {
                                        let ll = list
                                            .get(cx)
                                            .iter()
                                            .enumerate()
                                            .filter(|(_, item)| {
                                                if f.is_empty() {
                                                    true
                                                } else {
                                                    item.to_string()
                                                        .to_ascii_lowercase()
                                                        .contains(&f.to_ascii_lowercase())
                                                }
                                            })
                                            .map(|(idx, _)| idx)
                                            .collect::<Vec<_>>();

                                        for index in ll.into_iter() {
                                            let item = list.idx(index);
                                            Label::new(cx, item)
                                                .child_top(Stretch(1.0))
                                                .child_bottom(Stretch(1.0))
                                                .checked(
                                                    selected
                                                        .map(move |selected| *selected == index),
                                                )
                                                .navigable(true)
                                                .toggle_class(
                                                    "nav",
                                                    Self::hovered.map(move |nav| *nav == index),
                                                )
                                                .on_hover(move |cx| {
                                                    cx.emit(ComboBoxEvent::SetHovered(index))
                                                })
                                                .on_press(move |cx| {
                                                    cx.emit(ComboBoxEvent::SetOption(index));
                                                });
                                        }
                                    })
                                    .height(Auto)
                                    .class("list");
                                })
                                .height(Auto);
                                //.min_height(Auto);
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
            handle.modify(|combobox| combobox.placeholder = selected_item.to_string());
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
                self.placeholder = selected_item.to_string();

                // Call the on_select callback.
                if let Some(callback) = &self.on_select {
                    (callback)(cx, *index);
                }

                // Close the popup.
                self.is_open = false;

                // Set the hovered index to the selected index.
                self.hovered = *index;

                // Reset the filter text.
                self.filter_text = String::new();

                // Set the textbox to non-edit state.
                // TODO: Add a modifier to textbox and bind to some state in combobox.
                cx.emit_custom(
                    Event::new(TextEvent::EndEdit)
                        .target(cx.current)
                        .propagate(Propagation::Subtree),
                );
            }

            ComboBoxEvent::SetHovered(index) => {
                self.hovered = *index;
            }

            ComboBoxEvent::SetFilterText(text) => {
                self.placeholder.clone_from(text);
                self.filter_text.clone_from(text);

                // Reopen the popup in case it was closed with the ESC key.
                self.is_open = true;

                let filter = |(_, txt): &(usize, &T)| {
                    if self.filter_text.is_empty() {
                        true
                    } else {
                        txt.to_string()
                            .to_ascii_lowercase()
                            .contains(&self.filter_text.to_ascii_lowercase())
                    }
                };

                let list = self.list_lens.get(cx);
                if let Some((next_index, _)) =
                    list.iter().enumerate().skip_while(|(idx, _)| *idx != self.hovered).find(filter)
                {
                    self.hovered = next_index;
                } else {
                    let list_len = list.len();
                    self.hovered = list
                        .iter()
                        .enumerate()
                        .find(filter)
                        .map(|(index, _)| index)
                        .unwrap_or(list_len);
                }
            }

            ComboBoxEvent::Close => {
                self.is_open = false;
            }
        });

        event.map(|textbox_event, _| match textbox_event {
            // User pressed on the textbox or focused it.
            TextEvent::StartEdit => {
                self.is_open = true;
                self.hovered = self.selected.get(cx);
            }

            TextEvent::Submit(enter) => {
                let selected = self.selected.get(cx);
                if *enter && self.hovered < self.list_lens.get(cx).len() {
                    // User pressed the enter key.
                    cx.emit(ComboBoxEvent::SetOption(self.hovered));
                } else {
                    // User clicked outside the textbox.
                    cx.emit(ComboBoxEvent::SetOption(selected));
                }
            }

            _ => {}
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                if !self.is_open {
                    self.is_open = true;
                    self.hovered = self.selected.get(cx);
                }
            }

            WindowEvent::KeyDown(code, _) => match code {
                Code::ArrowDown => {
                    if !self.is_open {
                        self.is_open = true;
                    } else {
                        let filter = |(_, txt): &(usize, &T)| {
                            if self.filter_text.is_empty() {
                                true
                            } else {
                                txt.to_string()
                                    .to_ascii_lowercase()
                                    .contains(&self.filter_text.to_ascii_lowercase())
                            }
                        };

                        let list = self.list_lens.get(cx);
                        if let Some((next_index, _)) = list
                            .iter()
                            .enumerate()
                            .filter(filter)
                            .skip_while(|(idx, _)| *idx != self.hovered)
                            .nth(1)
                        {
                            self.hovered = next_index;
                        } else {
                            self.hovered = list.iter().enumerate().find(filter).unwrap().0;
                        }
                    }
                }

                Code::ArrowUp => {
                    if !self.is_open {
                        self.is_open = true;
                    } else {
                        let filter = |(_, txt): &(usize, &T)| {
                            if self.filter_text.is_empty() {
                                true
                            } else {
                                txt.to_string()
                                    .to_ascii_lowercase()
                                    .contains(&self.filter_text.to_ascii_lowercase())
                            }
                        };

                        let list = self.list_lens.get(cx);
                        if let Some((next_index, _)) = list
                            .iter()
                            .enumerate()
                            .rev()
                            .filter(filter)
                            .skip_while(|(idx, _)| *idx != self.hovered)
                            .nth(1)
                        {
                            self.hovered = next_index;
                        } else {
                            self.hovered = list.iter().enumerate().rev().find(filter).unwrap().0;
                        }
                    }
                }

                Code::Escape => {
                    if self.is_open {
                        self.is_open = false;
                        self.hovered = self.selected.get(cx);
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

impl<'v, L1, L2, T> Handle<'v, ComboBox<L1, L2, T>>
where
    L1: Lens<Target = Vec<T>>,
    T: 'static + Data + ToString,
    L2: Lens<Target = usize>,
{
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|combobox: &mut ComboBox<L1, L2, T>| {
            combobox.on_select = Some(Box::new(callback))
        })
    }
}
