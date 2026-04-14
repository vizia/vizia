use std::{marker::PhantomData, ops::Deref, rc::Rc, sync::Arc};

use crate::prelude::*;

use super::{
    TableColumn, TableSortCycle, TableSortDirection, TableSortState, table::next_sort_direction,
    table::sort_direction_for_column,
};

/// A virtualized table view backed by [`VirtualList`] for large datasets.
///
/// Rows use a fixed `item_height` for virtualization performance.
pub struct VirtualTable<T, V, Id, H, K = String>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: PartialEq + Clone + 'static,
    H: View,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    rows: Signal<V>,
    _header: PhantomData<H>,
    row_id: Rc<dyn Fn(&T) -> Id>,
    sort_state: Signal<Option<TableSortState<K>>>,
    sort_cycle: Signal<TableSortCycle>,
    resizable_columns: Signal<bool>,
    selectable: Signal<Selectable>,
    selection_follows_focus: Signal<bool>,
    selected_row_ids: Signal<Vec<Id>>,
    on_sort: Option<Arc<dyn Fn(&mut EventContext, K, TableSortDirection) + Send + Sync>>,
    on_row_select: Option<Box<dyn Fn(&mut EventContext, Id)>>,
}

enum VirtualTableEvent<K> {
    RequestSort(K, TableSortDirection),
    SelectRow(usize),
}

impl<T, V, Id, H, K> VirtualTable<T, V, Id, H, K>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: PartialEq + Clone + 'static,
    H: Clone + View,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    /// Creates a new virtualized table view.
    pub fn new<S, C, R>(
        cx: &mut Context,
        rows: S,
        columns: C,
        item_height: f32,
        row_id: impl Fn(&T) -> Id + 'static,
    ) -> Handle<Self>
    where
        S: Res<V> + 'static,
        C: Res<R> + 'static,
        R: Deref<Target = [TableColumn<T, H, K>]> + Clone + 'static,
    {
        let row_signal = rows.to_signal(cx);
        let column_signal = columns.to_signal(cx);
        let row_id: Rc<dyn Fn(&T) -> Id> = Rc::new(row_id);

        let sort_state = Signal::new(None);
        let sort_cycle = Signal::new(TableSortCycle::BiState);
        let resizable_columns = Signal::new(false);
        let selectable = Signal::new(Selectable::None);
        let selection_follows_focus = Signal::new(false);
        let selected_row_ids = Signal::new(Vec::new());

        let selected_indices = Memo::new({
            let row_id = row_id.clone();
            move |_| {
                row_signal.with(|rows| {
                    selected_row_ids.with(|selected_ids| {
                        rows.deref()
                            .iter()
                            .enumerate()
                            .filter_map(|(index, row)| {
                                let id = (row_id)(row);
                                if selected_ids.contains(&id) { Some(index) } else { None }
                            })
                            .collect::<Vec<usize>>()
                    })
                })
            }
        });

        let column_layout = Memo::new(move |_| {
            column_signal.with(|columns| {
                columns
                    .deref()
                    .iter()
                    .map(|column| (column.key.clone(), column.hidden.get()))
                    .collect::<Vec<_>>()
            })
        });

        Self {
            rows: row_signal,
            _header: PhantomData,
            row_id,
            sort_state,
            sort_cycle,
            resizable_columns,
            selectable,
            selection_follows_focus,
            selected_row_ids,
            on_sort: None,
            on_row_select: None,
        }
        .build(cx, move |cx| {
            Binding::new(cx, column_layout, move |cx| {
                let visible_columns = column_signal.with(|columns| {
                    columns
                        .deref()
                        .iter()
                        .filter(|column| !column.hidden.get())
                        .cloned()
                        .collect::<Vec<_>>()
                });
                let last_header_index = visible_columns.len().saturating_sub(1);
                let header_columns = Rc::new(visible_columns);

                HStack::new(cx, move |cx| {
                    for (column_index, column) in header_columns.iter().cloned().enumerate() {
                        let width_signal = column.width;
                        let sort_state = sort_state;
                        let sort_cycle = sort_cycle;
                        let resizable_columns = resizable_columns;
                        let min_width = column.min_width;
                        let sortable = column.sortable;
                        let resizable = column.resizable;
                        let is_last_column = column_index == last_header_index;
                        let header_content = column.header_content.clone();
                        let column_key = column.key.clone();
                        let sort_direction = sort_state.map({
                            let column_key = column_key.clone();
                            move |state| sort_direction_for_column(state.as_ref(), &column_key)
                        });

                        if is_last_column {
                            HStack::new(cx, move |cx| {
                                let header = header_content(cx, sort_direction);

                                let column_key = column_key.clone();
                                header.on_press(move |cx| {
                                    if sortable.get() {
                                        let current_direction = sort_direction_for_column(
                                            sort_state.get().as_ref(),
                                            &column_key,
                                        );
                                        let next_direction = next_sort_direction(
                                            sort_cycle.get(),
                                            current_direction,
                                        );
                                        cx.emit(VirtualTableEvent::RequestSort(
                                            column_key.clone(),
                                            next_direction,
                                        ));
                                    }
                                });
                            })
                            .class("table-header-cell")
                            .toggle_class("sortable", sortable)
                            .toggle_class("not-sortable", sortable.map(|value| !*value))
                            .toggle_class("resizable", false)
                            .width(Stretch(1.0))
                            .min_width(Auto);
                        } else {
                            Resizable::new(
                                cx,
                                width_signal.map(|value| Pixels(*value)),
                                ResizeStackDirection::Right,
                                move |_cx, new_size| {
                                    if resizable_columns.get() && resizable.get() {
                                        width_signal.set(new_size.max(min_width.get()));
                                    }
                                },
                                move |cx| {
                                    let header = header_content(cx, sort_direction);

                                    let column_key = column_key.clone();
                                    header.on_press(move |cx| {
                                        if sortable.get() {
                                            let current_direction = sort_direction_for_column(
                                                sort_state.get().as_ref(),
                                                &column_key,
                                            );
                                            let next_direction = next_sort_direction(
                                                sort_cycle.get(),
                                                current_direction,
                                            );
                                            cx.emit(VirtualTableEvent::RequestSort(
                                                column_key.clone(),
                                                next_direction,
                                            ));
                                        }
                                    });
                                },
                            )
                            .class("table-header-cell")
                            .toggle_class("sortable", sortable)
                            .toggle_class("not-sortable", sortable.map(|value| !*value))
                            .toggle_class(
                                "resizable",
                                resizable_columns.map(move |enabled| *enabled && resizable.get()),
                            )
                            .toggle_class(
                                "not-resizable",
                                resizable_columns.map(move |enabled| !*enabled || !resizable.get()),
                            )
                            .min_width(min_width.map(|value| Pixels(*value)));
                        }
                    }
                })
                .class("table-header-row")
                .height(Auto)
                .width(Stretch(1.0))
                .min_width(Auto);

                VirtualList::new(cx, row_signal, item_height, move |cx, row_index, row| {
                    HStack::new(cx, |cx| {
                        column_signal.with(|columns| {
                            let visible_columns = columns
                                .deref()
                                .iter()
                                .filter(|column| !column.hidden.get())
                                .collect::<Vec<_>>();

                            for (column_index, column) in visible_columns.iter().enumerate() {
                                let width_signal = column.width;
                                let min_width = column.min_width;
                                let cell_content = column.cell_content.clone();
                                let is_last_column = column_index + 1 == visible_columns.len();

                                if is_last_column {
                                    VStack::new(cx, move |cx| {
                                        cell_content(cx, row.map(|value| value.clone()));
                                    })
                                    .class("table-cell")
                                    .width(Stretch(1.0))
                                    .min_width(Auto)
                                    .height(Percentage(100.0));
                                } else {
                                    VStack::new(cx, move |cx| {
                                        cell_content(cx, row.map(|value| value.clone()));
                                    })
                                    .class("table-cell")
                                    .width(width_signal.map(|value| Pixels(*value)))
                                    .min_width(min_width.map(|value| Pixels(*value)))
                                    .height(Percentage(100.0));
                                }
                            }
                        });
                    })
                    .class("table-row")
                    .toggle_class("odd", row_index % 2 == 1)
                    .toggle_class("even", row_index % 2 == 0)
                    .alignment(Alignment::Left)
                    .height(Percentage(100.0))
                    .width(Stretch(1.0))
                    .min_width(Auto)
                })
                .width(Stretch(1.0))
                .min_width(Auto)
                .height(Stretch(1.0))
                .min_height(Auto)
                .class("table-body")
                .selection(selected_indices)
                .selectable(selectable)
                .selection_follows_focus(selection_follows_focus)
                .on_select(move |cx, index| cx.emit(VirtualTableEvent::<K>::SelectRow(index)));
            });
        })
        .class("table")
        .role(Role::List)
    }
}

impl<T, V, Id, H, K> View for VirtualTable<T, V, Id, H, K>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: PartialEq + Clone + 'static,
    H: Clone + View,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    fn element(&self) -> Option<&'static str> {
        Some("virtual-table")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|table_event: &VirtualTableEvent<K>, _| match table_event {
            VirtualTableEvent::RequestSort(column, direction) => {
                if let Some(callback) = &self.on_sort {
                    (callback)(cx, column.clone(), *direction);
                }
            }

            VirtualTableEvent::SelectRow(index) => {
                let rows = self.rows.get();
                if let Some(row) = rows.deref().get(*index) {
                    if let Some(callback) = &self.on_row_select {
                        (callback)(cx, (self.row_id)(row));
                    }
                }
            }
        });
    }
}

/// Modifiers for configuring controlled virtual table state and callbacks.
pub trait VirtualTableModifiers<Id, K = String>: Sized
where
    K: Clone + PartialEq + Send + Sync + 'static,
{
    fn sort_state(self, sort_state: impl Res<Option<TableSortState<K>>> + 'static) -> Self;

    fn resizable_columns<U: Into<bool> + Clone + 'static>(
        self,
        flag: impl Res<U> + 'static,
    ) -> Self;

    fn sort_cycle<U: Into<TableSortCycle> + Clone + 'static>(
        self,
        cycle: impl Res<U> + 'static,
    ) -> Self;

    fn selectable<U: Into<Selectable> + Clone + 'static>(
        self,
        selectable: impl Res<U> + 'static,
    ) -> Self;

    fn selection_follows_focus<U: Into<bool> + Clone + 'static>(
        self,
        flag: impl Res<U> + 'static,
    ) -> Self;

    fn selected_row_ids<R>(self, selected_row_ids: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [Id]> + Clone + 'static;

    fn on_sort<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, K, TableSortDirection) + Send + Sync;

    fn on_row_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id);
}

impl<T, V, Id, H, K> VirtualTableModifiers<Id, K> for Handle<'_, VirtualTable<T, V, Id, H, K>>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: PartialEq + Clone + 'static,
    H: Clone + View,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    fn sort_state(self, sort_state: impl Res<Option<TableSortState<K>>> + 'static) -> Self {
        let sort_state = sort_state.to_signal(self.cx);
        self.bind(sort_state, move |handle| {
            let sort_state = sort_state.get();
            handle.modify(|table: &mut VirtualTable<T, V, Id, H, K>| {
                table.sort_state.set(sort_state)
            });
        })
    }

    fn resizable_columns<U: Into<bool> + Clone + 'static>(
        self,
        flag: impl Res<U> + 'static,
    ) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let flag = flag.get().into();
            handle.modify(|table: &mut VirtualTable<T, V, Id, H, K>| {
                table.resizable_columns.set(flag)
            });
        })
    }

    fn sort_cycle<U: Into<TableSortCycle> + Clone + 'static>(
        self,
        cycle: impl Res<U> + 'static,
    ) -> Self {
        let cycle = cycle.to_signal(self.cx);
        self.bind(cycle, move |handle| {
            let cycle = cycle.get().into();
            handle.modify(|table: &mut VirtualTable<T, V, Id, H, K>| table.sort_cycle.set(cycle));
        })
    }

    fn selectable<U: Into<Selectable> + Clone + 'static>(
        self,
        selectable: impl Res<U> + 'static,
    ) -> Self {
        let selectable = selectable.to_signal(self.cx);
        self.bind(selectable, move |handle| {
            let selectable = selectable.get().into();
            handle.modify(|table: &mut VirtualTable<T, V, Id, H, K>| {
                table.selectable.set(selectable)
            });
        })
    }

    fn selection_follows_focus<U: Into<bool> + Clone + 'static>(
        self,
        flag: impl Res<U> + 'static,
    ) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let flag = flag.get().into();
            handle.modify(|table: &mut VirtualTable<T, V, Id, H, K>| {
                table.selection_follows_focus.set(flag)
            });
        })
    }

    fn selected_row_ids<R>(self, selected_row_ids: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [Id]> + Clone + 'static,
    {
        let selected_row_ids = selected_row_ids.to_signal(self.cx);
        self.bind(selected_row_ids, move |handle| {
            let ids = selected_row_ids.with(|ids| ids.deref().to_vec());
            handle
                .modify(|table: &mut VirtualTable<T, V, Id, H, K>| table.selected_row_ids.set(ids));
        })
    }

    fn on_sort<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, K, TableSortDirection) + Send + Sync,
    {
        self.modify(|table: &mut VirtualTable<T, V, Id, H, K>| {
            table.on_sort = Some(Arc::new(callback))
        })
    }

    fn on_row_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id),
    {
        self.modify(|table: &mut VirtualTable<T, V, Id, H, K>| {
            table.on_row_select = Some(Box::new(callback))
        })
    }
}
