use std::marker::PhantomData;
use std::usize;

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
    ///
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
            filter_text: String::from(""),
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
                .width(Stretch(1.0))
                .height(Pixels(32.0))
                .space(Pixels(0.0))
                .placeholder(Self::placeholder);

            ComboPopup::new(cx, Self::is_open, false, move |cx: &mut Context| {
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
                                for (index, item) in
                                    list.get(cx).iter().enumerate().filter(|(_, item)| {
                                        if f.is_empty() {
                                            true
                                        } else {
                                            item.to_string()
                                                .to_ascii_lowercase()
                                                .contains(&f.to_ascii_lowercase())
                                        }
                                    })
                                {
                                    Label::new(cx, &item.to_string())
                                        .child_top(Stretch(1.0))
                                        .child_bottom(Stretch(1.0))
                                        .checked(selected.map(move |selected| *selected == index))
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
                    });
                });
            })
            .top(Percentage(100.0))
            .translate((Pixels(0.0), Pixels(4.0)))
            .width(Percentage(100.0))
            .height(Auto);
        })
        .bind(selected, move |handle, selected| {
            let selected_item = list_lens.index(selected.get(&handle)).get(&handle);
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
        Some("dropdown")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|combobox_event, _| match combobox_event {
            ComboBoxEvent::SetOption(index) => {
                // Set the placeholder text to the selected item.
                let selected_item = self.list_lens.index(*index).get(cx);
                self.placeholder = selected_item.to_string();

                // Call the on_select callback.
                if let Some(callback) = &self.on_select {
                    (callback)(cx, *index);
                }

                // Close the popup.
                self.is_open = false;

                // Reset the filter text.
                self.filter_text = String::new();

                // Set the textbox to non-edit state.
                cx.emit_custom(
                    Event::new(TextEvent::EndEdit)
                        .target(cx.current)
                        .propagate(Propagation::Subtree),
                );

                // cx.needs_redraw();
            }

            ComboBoxEvent::SetHovered(index) => {
                self.hovered = *index;
            }

            ComboBoxEvent::SetFilterText(text) => {
                self.filter_text = text.clone();

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
            // User pressed on the textbox or focused it
            TextEvent::StartEdit => {
                self.is_open = true;
                self.hovered = self.selected.get(cx);
            }

            TextEvent::Submit(enter) => {
                let selected = self.selected.get(cx);
                if *enter && self.hovered < self.list_lens.get(cx).len() {
                    // User pressed the enter key
                    cx.emit(ComboBoxEvent::SetOption(self.hovered));
                } else {
                    // User clicked outside the combobox
                    cx.emit(ComboBoxEvent::SetOption(selected));
                }
            }

            _ => {}
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::KeyDown(code, _) => match code {
                Code::ArrowDown => {
                    if self.is_open {
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
                    if self.is_open {
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
                        // The textbox will receive the key event first and defocus so send an event to refocus the textbox
                        cx.emit_custom(
                            Event::new(TextEvent::StartEdit)
                                .target(cx.current)
                                .propagate(Propagation::Subtree),
                        );

                        // Emit an event instead of setting is_open because the StartEdit will cause the popup to reopen
                        cx.emit(ComboBoxEvent::Close);
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

pub struct ComboPopup {}

impl ComboPopup {
    pub fn new<F, L: Lens<Target = bool>>(
        cx: &mut Context,
        lens: L,
        capture_focus: bool,
        content: F,
    ) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        Self {}
            .build(cx, |cx| {
                let parent = cx.current;
                Binding::new(cx, lens, move |cx, lens| {
                    if let Some(geo) = cx.cache.geo_changed.get_mut(parent) {
                        geo.set(GeoChanged::WIDTH_CHANGED, true);
                    }

                    if lens.get(cx) {
                        if capture_focus {
                            VStack::new(cx, &content).lock_focus_to_within();
                        } else {
                            (content)(cx);
                        }
                    }
                });
            })
            .role(Role::Dialog)
            .checked(lens)
            .position_type(PositionType::SelfDirected)
            .z_index(100)
    }
}

impl View for ComboPopup {
    fn element(&self) -> Option<&'static str> {
        Some("popup")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::GeometryChanged(_) => {
                let parent = cx.tree.get_parent(cx.current).unwrap();

                let bounds = cx.bounds();
                let parent_bounds = cx.cache.get_bounds(parent);
                let window_bounds = cx.cache.get_bounds(Entity::root());

                let space_below = window_bounds.bottom() - bounds.bottom();
                let space_above = parent_bounds.top() - window_bounds.top();

                let scale = cx.scale_factor();

                if space_below < 0.0 {
                    if space_above.abs() > bounds.h {
                        cx.set_translate((
                            Pixels(0.0),
                            Pixels(-(bounds.h + parent_bounds.h) / scale - 4.0),
                        ));
                    } else if let Some(first_child) = cx.tree.get_layout_first_child(cx.current) {
                        let mut child_bounds = cx.cache.get_bounds(first_child);
                        child_bounds.h = window_bounds.bottom() - bounds.top() - 4.0 * scale;

                        cx.style.max_height.insert(first_child, Pixels(child_bounds.h / scale));
                    }
                } else {
                    cx.set_translate((Pixels(0.0), Pixels(4.0)));
                }
            }

            _ => {}
        });
    }
}
