use std::{ops::Deref, rc::Rc};

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DropTarget {
    Before(usize),
    Onto(usize),
    After(usize),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DropSource {
    Internal(usize),
    External(Entity, usize),
}

/// Events used internally by [`ReorderableList`].
pub enum ReorderableListEvent {
    /// Handles a completed drop operation.
    Drop(DropData),
    /// Clears drag-over visual state.
    ClearDropState,
    /// Updates scroll position.
    Scroll(f32, f32),
}

/// A list view with drag-and-drop reordering support.
pub struct ReorderableList {
    num_items: Signal<usize>,
    drop_target: Signal<Option<DropTarget>>,
    dragged_item: Signal<Option<Entity>>,
    on_row_drop: Option<Box<dyn Fn(&mut EventContext, DropSource, DropTarget)>>,
    accept_external_reorder: Signal<bool>,
    scroll_to_cursor: Signal<bool>,
    scroll_x: Signal<f32>,
    scroll_y: Signal<f32>,
    show_horizontal_scrollbar: Signal<bool>,
    show_vertical_scrollbar: Signal<bool>,
    on_scroll: Option<Box<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,
}

impl ReorderableList {
    /// Creates a new [`ReorderableList`] from a reactive or static list of values.
    pub fn new<S, V, T>(
        cx: &mut Context,
        list: S,
        item_content: impl 'static + Fn(&mut Context, usize, Signal<T>),
    ) -> Handle<Self>
    where
        S: Res<V> + 'static,
        V: Deref<Target = [T]> + Clone + 'static,
        T: Clone + 'static,
    {
        let content: Rc<dyn Fn(&mut Context, usize, Signal<T>)> = Rc::new(item_content);

        let num_items = Signal::new(0usize);
        let drop_target = Signal::new(None);
        let indicator_y = Signal::new(0.0f32);
        let dragged_item = Signal::new(None);
        let accept_external_reorder = Signal::new(false);
        let scroll_to_cursor = Signal::new(false);
        let scroll_x = Signal::new(0.0f32);
        let scroll_y = Signal::new(0.0f32);
        let show_horizontal_scrollbar = Signal::new(false);
        let show_vertical_scrollbar = Signal::new(true);

        Self {
            num_items,
            drop_target,
            dragged_item,
            on_row_drop: None,
            accept_external_reorder,
            scroll_to_cursor,
            scroll_x,
            scroll_y,
            show_horizontal_scrollbar,
            show_vertical_scrollbar,
            on_scroll: None,
        }
        .build(cx, move |cx| {
            let list_entity = cx.current();
            let list_signal = list.to_signal(cx);
            let content_for_binding = content.clone();

            ScrollView::new(cx, move |cx| {
                Binding::new(cx, list_signal, move |cx| {
                    let content = content_for_binding.clone();
                    let items = list_signal.with(|list| list.deref().to_vec());
                    let len = items.len();
                    num_items.set(len);

                    VStack::new(cx, move |cx| {
                        for (index, value) in items.into_iter().enumerate() {
                            let item_signal = Signal::new(value.clone());
                            let preview_signal = Signal::new(value);
                            let preview_width = Signal::new(0.0f32);
                            let preview_height = Signal::new(0.0f32);
                            let drop_target_signal = drop_target;
                            let dragged_item_signal = dragged_item;
                            let item_content = content.clone();
                            let preview_content = content.clone();

                            let row = VStack::new(cx, move |cx| {
                                (item_content)(cx, index, item_signal);
                            })
                            .class("reorderable-list-item");
                            let row_entity = row.entity();

                            row.on_drag(move |cx| {
                                dragged_item_signal.set(Some(row_entity));
                                cx.with_current(list_entity, |cx| cx.capture());
                                cx.set_drop_data(DropData::Reorder {
                                    source_list: list_entity,
                                    source_item: row_entity,
                                    source_index: index,
                                });
                            })
                            .on_drag_view(move |cx| {
                                let preview_content = preview_content.clone();
                                VStack::new(cx, move |cx| {
                                    (preview_content)(cx, index, preview_signal);
                                })
                                .class("reorderable-list-item")
                                .width(preview_width.map(|value| Pixels(*value)))
                                .height(preview_height.map(|value| Pixels(*value)))
                            })
                            .on_geo_changed(move |cx, geo| {
                                if geo.intersects(GeoChanged::WIDTH_CHANGED | GeoChanged::HEIGHT_CHANGED) {
                                    let bounds = cx.bounds();
                                    preview_width.set(cx.physical_to_logical(bounds.width()));
                                    preview_height.set(cx.physical_to_logical(bounds.height()));
                                }
                            })
                            .on_drag_leave(move |cx| {
                                if cx.drop_data().is_some() {
                                    drop_target_signal.set(None);
                                }
                            })
                            .on_drag_move(move |cx, _x, _y| {
                                let (source_list, source_item) = match cx.drop_data() {
                                    Some(DropData::Reorder {
                                        source_list, source_item, ..
                                    }) => (*source_list, *source_item),
                                    _ => return,
                                };

                                let allow_external = accept_external_reorder.get();
                                if source_list != list_entity && !allow_external {
                                    return;
                                }

                                dragged_item_signal.set(Some(source_item));

                                let (bounds, transform) = (cx.bounds(), cx.transform());
                                let rect: skia_safe::Rect = bounds.into();
                                let phys: BoundingBox = transform.map_rect(rect).0.into();
                                let y = cx.mouse().cursor_y;

                                if y < phys.y || y > phys.y + phys.h {
                                    return;
                                }

                                // Get the list's Y position to make indicator relative to list
                                let list_phys: BoundingBox = cx.with_current(list_entity, |cx| {
                                    let (bounds, transform) = (cx.bounds(), cx.transform());
                                    let rect: skia_safe::Rect = bounds.into();
                                    transform.map_rect(rect).0.into()
                                });

                                let top_threshold = phys.y + phys.h * 0.25;
                                let bottom_threshold = phys.y + phys.h * 0.75;

                                if y < top_threshold {
                                    indicator_y.set(cx.physical_to_logical(phys.y - list_phys.y));
                                    drop_target_signal.set(Some(DropTarget::Before(index)));
                                } else if y > bottom_threshold {
                                    indicator_y.set(cx.physical_to_logical(phys.y + phys.h - list_phys.y));
                                    drop_target_signal.set(Some(DropTarget::After(index)));
                                } else if source_item != row_entity {
                                    indicator_y.set(cx.physical_to_logical(phys.y + phys.h * 0.5 - list_phys.y));
                                    drop_target_signal.set(Some(DropTarget::Onto(index)));
                                } else {
                                    let midpoint = phys.y + phys.h * 0.5;
                                    let target = if y < midpoint {
                                        DropTarget::Before(index)
                                    } else {
                                        DropTarget::After(index)
                                    };
                                    indicator_y.set(cx.physical_to_logical(if y < midpoint { phys.y } else { phys.y + phys.h } - list_phys.y));
                                    drop_target_signal.set(Some(target));
                                }
                            })
                            .on_drop(move |cx, data| {
                                let (source_list, source_item) = match &data {
                                    DropData::Reorder { source_list, source_item, .. } => {
                                        (*source_list, *source_item)
                                    }
                                    _ => return,
                                };

                                let allow_external = accept_external_reorder.get();
                                if source_list != list_entity && !allow_external {
                                    return;
                                }

                                dragged_item_signal.set(Some(source_item));

                                let (bounds, transform) = (cx.bounds(), cx.transform());
                                let rect: skia_safe::Rect = bounds.into();
                                let phys: BoundingBox = transform.map_rect(rect).0.into();
                                let y = cx.mouse().cursor_y;

                                // Get the list's Y position to make indicator relative to list
                                let list_phys: BoundingBox = cx.with_current(list_entity, |cx| {
                                    let (bounds, transform) = (cx.bounds(), cx.transform());
                                    let rect: skia_safe::Rect = bounds.into();
                                    transform.map_rect(rect).0.into()
                                });

                                let top_threshold = phys.y + phys.h * 0.25;
                                let bottom_threshold = phys.y + phys.h * 0.75;

                                if y < top_threshold {
                                    indicator_y.set(cx.physical_to_logical(phys.y - list_phys.y));
                                    drop_target_signal.set(Some(DropTarget::Before(index)));
                                } else if y > bottom_threshold {
                                    indicator_y.set(cx.physical_to_logical(phys.y + phys.h - list_phys.y));
                                    drop_target_signal.set(Some(DropTarget::After(index)));
                                } else if source_item != row_entity {
                                    indicator_y.set(cx.physical_to_logical(phys.y + phys.h * 0.5 - list_phys.y));
                                    drop_target_signal.set(Some(DropTarget::Onto(index)));
                                } else {
                                    indicator_y.set(cx.physical_to_logical(phys.y - list_phys.y));
                                    drop_target_signal.set(Some(DropTarget::Before(index)));
                                }

                                cx.emit(ReorderableListEvent::Drop(data));
                            })
                            .bind(drop_target, move |handle| {
                                let is_target_row = matches!(drop_target.get(), Some(DropTarget::Before(i)) if i == index)
                                    || matches!(drop_target.get(), Some(DropTarget::Onto(i)) if i == index)
                                    || matches!(drop_target.get(), Some(DropTarget::After(i)) if i == index);
                                handle.toggle_class("reorder-drop-target", is_target_row);
                            })
                            .bind(drop_target, move |handle| {
                                let is_drop_onto =
                                    matches!(drop_target.get(), Some(DropTarget::Onto(i)) if i == index);
                                handle.toggle_class("reorder-drop-onto", is_drop_onto);
                            })
                            .bind(dragged_item, move |handle| {
                                let is_drag_source = dragged_item.get() == Some(row_entity);
                                handle.toggle_class("reorder-drag-source", is_drag_source);
                            });
                        }

                        // Single moving drop indicator.
                        Element::new(cx)
                            .class("reorder-indicator")
                            .height(Pixels(2.0))
                            .pointer_events(PointerEvents::None)
                            .position_type(PositionType::Absolute)
                            .display(Display::None)
                            .bind(indicator_y, move |handle| {
                                handle.top(Pixels(indicator_y.get()));
                            })
                            .bind(drop_target, move |handle| {
                                let visible = drop_target.get().is_some();
                                let handle = handle.display(if visible {
                                    Display::Flex
                                } else {
                                    Display::None
                                });
                                handle.toggle_class("reorder-indicator-active", visible);
                            });
                    });
                });
            })
            .show_horizontal_scrollbar(show_horizontal_scrollbar)
            .show_vertical_scrollbar(show_vertical_scrollbar)
            .scroll_to_cursor(scroll_to_cursor)
            .scroll_x(scroll_x)
            .scroll_y(scroll_y)
            .on_scroll(|cx, x, y| {
                if y.is_finite() {
                    cx.emit(ReorderableListEvent::Scroll(x, y));
                }
            });
        })
        .navigable(true)
        .toggle_class(
            "reorder-drop-active",
            Memo::new(move |_| drop_target.get().is_some()),
        )
        .role(Role::ListBox)
        .on_mouse_up(|cx, _| {
            if !cx.has_drop_data() {
                cx.emit(ReorderableListEvent::ClearDropState);
            }
        })
    }
}

impl View for ReorderableList {
    fn element(&self) -> Option<&'static str> {
        Some("reorderable-list")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|reorder_event, meta| match reorder_event {
            ReorderableListEvent::ClearDropState => {
                cx.release();
                self.drop_target.set(None);
                self.dragged_item.set(None);
                meta.consume();
            }

            ReorderableListEvent::Drop(data) => {
                if let DropData::Reorder { source_list, source_item: _, source_index } = data {
                    let target_list = cx.current();
                    let same_list = source_list == target_list;
                    let allow_external = self.accept_external_reorder.get();

                    if !same_list && !allow_external {
                        self.drop_target.set(None);
                        self.dragged_item.set(None);
                        cx.release();
                        meta.consume();
                        return;
                    }

                    if let Some(drop_target) = self.drop_target.get() {
                        let effective_source_index = source_index;
                        let len = self.num_items.get();

                        if same_list && effective_source_index >= len {
                            self.drop_target.set(None);
                            self.dragged_item.set(None);
                            cx.release();
                            meta.consume();
                            return;
                        }

                        // Construct DropSource based on whether it's internal or external
                        let drop_source = if same_list {
                            DropSource::Internal(effective_source_index)
                        } else {
                            DropSource::External(source_list, effective_source_index)
                        };

                        // Call the callback with the drop source and target information
                        if let Some(on_row_drop) = &self.on_row_drop {
                            (on_row_drop)(cx, drop_source, drop_target);
                        }
                    }
                }

                self.drop_target.set(None);
                self.dragged_item.set(None);
                cx.release();
                meta.consume();
            }

            ReorderableListEvent::Scroll(x, y) => {
                self.scroll_x.set(x);
                self.scroll_y.set(y);
                if let Some(callback) = &self.on_scroll {
                    (callback)(cx, x, y);
                }

                meta.consume();
            }
        });
    }
}

/// Modifiers for configuring a [`ReorderableList`].
pub trait ReorderableListModifiers: Sized {
    /// Called when a row drop operation is completed. The `DropTarget` indicates whether the item
    /// was dropped Before, After, or Onto another item. The `DropSource` indicates whether
    /// the drop was Internal (from same list) or External (from another list).
    fn on_row_drop<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, DropSource, DropTarget);

    /// Allows drops from other [`ReorderableList`] views.
    fn accept_external_reorder(self, flag: impl Res<bool> + 'static) -> Self;

    /// Sets whether the scrollbar should move to the cursor when pressed.
    fn scroll_to_cursor(self, flag: bool) -> Self;

    /// Sets a callback which will be called when the scrollview is scrolled.
    fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self;

    /// Set the horizontal scroll position of the [`ScrollView`].
    fn scroll_x(self, scrollx: impl Res<f32> + 'static) -> Self;

    /// Set the vertical scroll position of the [`ScrollView`].
    fn scroll_y(self, scrollx: impl Res<f32> + 'static) -> Self;

    /// Sets whether the horizontal scrollbar should be visible.
    fn show_horizontal_scrollbar(self, flag: impl Res<bool> + 'static) -> Self;

    /// Sets whether the vertical scrollbar should be visible.
    fn show_vertical_scrollbar(self, flag: impl Res<bool> + 'static) -> Self;
}

impl ReorderableListModifiers for Handle<'_, ReorderableList> {
    fn on_row_drop<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, DropSource, DropTarget),
    {
        self.modify(|list: &mut ReorderableList| list.on_row_drop = Some(Box::new(callback)))
    }

    fn accept_external_reorder(self, flag: impl Res<bool> + 'static) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let accept_external = flag.get();
            handle.modify(|list: &mut ReorderableList| {
                list.accept_external_reorder.set(accept_external);
            });
        })
    }

    fn scroll_to_cursor(self, flag: bool) -> Self {
        self.modify(|list| {
            list.scroll_to_cursor.set(flag);
        })
    }

    fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.modify(|list: &mut ReorderableList| list.on_scroll = Some(Box::new(callback)))
    }

    fn scroll_x(self, scrollx: impl Res<f32> + 'static) -> Self {
        let scrollx = scrollx.to_signal(self.cx);
        self.bind(scrollx, move |handle| {
            let scrollx = scrollx.get();
            handle.modify(|list| {
                list.scroll_x.set(scrollx);
            });
        })
    }

    fn scroll_y(self, scrollx: impl Res<f32> + 'static) -> Self {
        let scrollx = scrollx.to_signal(self.cx);
        self.bind(scrollx, move |handle| {
            let scrolly = scrollx.get();
            handle.modify(|list| {
                list.scroll_y.set(scrolly);
            });
        })
    }

    fn show_horizontal_scrollbar(self, flag: impl Res<bool> + 'static) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let show_scrollbar = flag.get();
            handle.modify(|list| {
                list.show_horizontal_scrollbar.set(show_scrollbar);
            });
        })
    }

    fn show_vertical_scrollbar(self, flag: impl Res<bool> + 'static) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let show_scrollbar = flag.get();
            handle.modify(|list| {
                list.show_vertical_scrollbar.set(show_scrollbar);
            });
        })
    }
}
