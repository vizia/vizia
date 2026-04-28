use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    marker::PhantomData,
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

use crate::prelude::*;

use super::{
    TableSortCycle, TableSortDirection, TableSortState, TreeTableColumn, TreeTableFirstCellEvent,
    TreeTableRow, table::next_sort_direction, table::sort_direction_for_column,
};

fn flatten_visible_rows<T, V, Id>(
    rows: &V,
    row_id: &dyn Fn(&T) -> Id,
    parent_id: &dyn Fn(&T) -> Option<Id>,
    expanded_row_ids: &[Id],
) -> Vec<TreeTableRow<T, Id>>
where
    V: Deref<Target = [T]>,
    T: PartialEq + Clone + 'static,
    Id: Clone + Eq + Hash + 'static,
{
    let mut rows_by_parent: HashMap<Option<Id>, Vec<T>> = HashMap::new();
    for row in rows.deref().iter().cloned() {
        rows_by_parent.entry(parent_id(&row)).or_default().push(row);
    }

    let expanded_set: HashSet<Id> = expanded_row_ids.iter().cloned().collect();
    let mut visible_rows = Vec::new();

    fn visit<T, Id>(
        rows: &[T],
        depth: usize,
        rows_by_parent: &HashMap<Option<Id>, Vec<T>>,
        expanded_set: &HashSet<Id>,
        row_id: &dyn Fn(&T) -> Id,
        out: &mut Vec<TreeTableRow<T, Id>>,
    ) where
        T: PartialEq + Clone + 'static,
        Id: Clone + Eq + Hash + 'static,
    {
        for row in rows {
            let id = row_id(row);
            let child_rows = rows_by_parent.get(&Some(id.clone())).cloned().unwrap_or_default();
            let has_children = !child_rows.is_empty();
            let expanded = has_children && expanded_set.contains(&id);

            out.push(TreeTableRow {
                row: row.clone(),
                id: id.clone(),
                parent_id: None,
                depth,
                has_children,
                expanded,
            });

            if expanded {
                visit(&child_rows, depth + 1, rows_by_parent, expanded_set, row_id, out);
            }
        }
    }

    let roots = rows_by_parent.get(&None).cloned().unwrap_or_default();
    visit(&roots, 0, &rows_by_parent, &expanded_set, row_id, &mut visible_rows);

    for visible_row in &mut visible_rows {
        visible_row.parent_id = parent_id(&visible_row.row);
    }

    visible_rows
}

pub struct VirtualTreeTable<T, V, Id, H, K = String>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
    H: View,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    rows: Signal<V>,
    _header: PhantomData<H>,
    row_id: Rc<dyn Fn(&T) -> Id>,
    parent_id: Rc<dyn Fn(&T) -> Option<Id>>,
    sort_state: Signal<Option<TableSortState<K>>>,
    sort_cycle: Signal<TableSortCycle>,
    resizable_columns: Signal<bool>,
    selectable: Signal<Selectable>,
    selection_follows_focus: Signal<bool>,
    selected_row_ids: Signal<Vec<Id>>,
    expanded_row_ids: Signal<Vec<Id>>,
    on_sort: Option<Arc<dyn Fn(&mut EventContext, K, TableSortDirection) + Send + Sync>>,
    on_row_select: Option<Box<dyn Fn(&mut EventContext, Id)>>,
    on_row_toggle: Option<Box<dyn Fn(&mut EventContext, Id, bool)>>,
}

enum VirtualTreeTableEvent<K, Id> {
    RequestSort(K, TableSortDirection),
    SelectRow(usize),
    ExpandSelected,
    CollapseSelected,
    ToggleRow(Id, bool),
}

impl<T, V, Id, H, K> VirtualTreeTable<T, V, Id, H, K>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
    H: Clone + View,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    pub fn new<S, C, R>(
        cx: &mut Context,
        rows: S,
        columns: C,
        item_height: f32,
        row_id: impl Fn(&T) -> Id + 'static,
        parent_id: impl Fn(&T) -> Option<Id> + 'static,
    ) -> Handle<Self>
    where
        S: Res<V> + 'static,
        C: Res<R> + 'static,
        R: Deref<Target = [TreeTableColumn<T, Id, H, K>]> + Clone + 'static,
    {
        let row_signal = rows.to_signal(cx);
        let column_signal = columns.to_signal(cx);
        let row_id: Rc<dyn Fn(&T) -> Id> = Rc::new(row_id);
        let parent_id: Rc<dyn Fn(&T) -> Option<Id>> = Rc::new(parent_id);

        let sort_state = Signal::new(None);
        let sort_cycle = Signal::new(TableSortCycle::BiState);
        let resizable_columns = Signal::new(false);
        let selectable = Signal::new(Selectable::None);
        let selection_follows_focus = Signal::new(false);
        let selected_row_ids = Signal::new(Vec::new());
        let expanded_row_ids = Signal::new(Vec::new());

        let visible_rows = Memo::new({
            let row_id = row_id.clone();
            let parent_id = parent_id.clone();
            move |_| {
                row_signal.with(|rows| {
                    expanded_row_ids.with(|expanded| {
                        flatten_visible_rows(rows, &*row_id, &*parent_id, expanded)
                    })
                })
            }
        });

        let selected_indices =
            Memo::new(move |_| {
                visible_rows.with(|rows| {
                    selected_row_ids.with(|selected_ids| {
                        rows.iter()
                            .enumerate()
                            .filter_map(|(index, row)| {
                                if selected_ids.contains(&row.id) { Some(index) } else { None }
                            })
                            .collect::<Vec<usize>>()
                    })
                })
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
            parent_id,
            sort_state,
            sort_cycle,
            resizable_columns,
            selectable,
            selection_follows_focus,
            selected_row_ids,
            expanded_row_ids,
            on_sort: None,
            on_row_select: None,
            on_row_toggle: None,
        }
        .build(cx, move |cx| {
            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowRight),
                    KeymapEntry::new("Expand Selected", |cx| {
                        cx.emit(VirtualTreeTableEvent::<K, Id>::ExpandSelected)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowLeft),
                    KeymapEntry::new("Collapse Selected", |cx| {
                        cx.emit(VirtualTreeTableEvent::<K, Id>::CollapseSelected)
                    }),
                ),
            ])
            .build(cx);

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
                                        cx.emit(VirtualTreeTableEvent::<K, Id>::RequestSort(
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
                            .toggle_class("not-resizable", true)
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
                                            cx.emit(VirtualTreeTableEvent::<K, Id>::RequestSort(
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

                VirtualList::new(cx, visible_rows, item_height, move |cx, row_index, row| {
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
                                    .toggle_class("tree-table-cell", column_index == 0)
                                    .width(Stretch(1.0))
                                    .min_width(Auto)
                                    .height(Percentage(100.0));
                                } else {
                                    VStack::new(cx, move |cx| {
                                        cell_content(cx, row.map(|value| value.clone()));
                                    })
                                    .class("table-cell")
                                    .toggle_class("tree-table-cell", column_index == 0)
                                    .width(width_signal.map(|value| Pixels(*value)))
                                    .min_width(min_width.map(|value| Pixels(*value)))
                                    .height(Percentage(100.0));
                                }
                            }
                        });
                    })
                    .class("table-row")
                    .class("tree-table-row")
                    .toggle_class("odd", row_index % 2 == 1)
                    .toggle_class("even", row_index % 2 == 0)
                    .toggle_class("expanded", row.map(|value| value.expanded))
                    .toggle_class("collapsible", row.map(|value| value.has_children))
                    .alignment(Alignment::Left)
                    .height(Percentage(100.0))
                    .width(Stretch(1.0))
                    .min_width(Auto)
                    .role(Role::Row)
                    .expanded(row.map(|value| value.expanded))
                })
                .width(Stretch(1.0))
                .min_width(Auto)
                .height(Stretch(1.0))
                .min_height(Auto)
                .class("table-body")
                .class("tree-table-body")
                .selection(selected_indices)
                .selectable(selectable)
                .selection_follows_focus(selection_follows_focus)
                .on_select(move |cx, index| {
                    cx.emit(VirtualTreeTableEvent::<K, Id>::SelectRow(index))
                });
            });
        })
        .class("table")
        .class("virtual-table")
        .class("tree-table")
        .class("virtual-tree-table")
        .navigable(true)
        .role(Role::Table)
    }

    fn emit_toggle(&self, cx: &mut EventContext, row_id: Id, next_expanded: bool) {
        if let Some(callback) = &self.on_row_toggle {
            (callback)(cx, row_id, next_expanded);
        }
    }

    fn selected_visible_row(&self) -> Option<TreeTableRow<T, Id>> {
        let selected_id = self.selected_row_ids.get().first().cloned()?;
        let visible_rows = flatten_visible_rows(
            &self.rows.get(),
            &*self.row_id,
            &*self.parent_id,
            &self.expanded_row_ids.get(),
        );

        visible_rows.into_iter().find(|row| row.id == selected_id)
    }
}

impl<T, V, Id, H, K> View for VirtualTreeTable<T, V, Id, H, K>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
    H: Clone + View,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    fn element(&self) -> Option<&'static str> {
        Some("virtual-tree-table")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|table_event: &VirtualTreeTableEvent<K, Id>, _| match table_event {
            VirtualTreeTableEvent::RequestSort(column, direction) => {
                if let Some(callback) = &self.on_sort {
                    (callback)(cx, column.clone(), *direction);
                }
            }

            VirtualTreeTableEvent::SelectRow(index) => {
                let visible_rows = flatten_visible_rows(
                    &self.rows.get(),
                    &*self.row_id,
                    &*self.parent_id,
                    &self.expanded_row_ids.get(),
                );

                if let Some(row) = visible_rows.get(*index) {
                    if let Some(callback) = &self.on_row_select {
                        (callback)(cx, row.id.clone());
                    }
                }
            }

            VirtualTreeTableEvent::ExpandSelected => {
                if let Some(row) = self.selected_visible_row() {
                    if row.has_children && !row.expanded {
                        self.emit_toggle(cx, row.id, true);
                    }
                }
            }

            VirtualTreeTableEvent::CollapseSelected => {
                if let Some(row) = self.selected_visible_row() {
                    if row.has_children && row.expanded {
                        self.emit_toggle(cx, row.id, false);
                    } else if let Some(parent_id) = row.parent_id {
                        self.emit_toggle(cx, parent_id, false);
                    }
                }
            }

            VirtualTreeTableEvent::ToggleRow(row_id, next) => {
                self.emit_toggle(cx, row_id.clone(), *next);
            }
        });

        event.map(|cell_event: &TreeTableFirstCellEvent<Id>, _| {
            let TreeTableFirstCellEvent::Toggle(id, next) = cell_event;
            cx.emit(VirtualTreeTableEvent::<K, Id>::ToggleRow(id.clone(), *next));
        });
    }
}

pub trait VirtualTreeTableModifiers<Id, K = String>: Sized
where
    Id: Eq + Hash + Clone + Send + Sync + 'static,
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

    fn expanded_row_ids<R>(self, expanded_row_ids: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [Id]> + Clone + 'static;

    fn on_sort<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, K, TableSortDirection) + Send + Sync;

    fn on_row_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id);

    fn on_row_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id, bool);
}

impl<T, V, Id, H, K> VirtualTreeTableModifiers<Id, K>
    for Handle<'_, VirtualTreeTable<T, V, Id, H, K>>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
    H: Clone + View,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    fn sort_state(self, sort_state: impl Res<Option<TableSortState<K>>> + 'static) -> Self {
        let sort_state = sort_state.to_signal(self.cx);
        self.bind(sort_state, move |handle| {
            let sort_state = sort_state.get();
            handle.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
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
            handle.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
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
            handle
                .modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| table.sort_cycle.set(cycle));
        })
    }

    fn selectable<U: Into<Selectable> + Clone + 'static>(
        self,
        selectable: impl Res<U> + 'static,
    ) -> Self {
        let selectable = selectable.to_signal(self.cx);
        self.bind(selectable, move |handle| {
            let selectable = selectable.get().into();
            handle.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
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
            handle.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
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
            handle.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
                table.selected_row_ids.set(ids)
            });
        })
    }

    fn expanded_row_ids<R>(self, expanded_row_ids: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [Id]> + Clone + 'static,
    {
        let expanded_row_ids = expanded_row_ids.to_signal(self.cx);
        self.bind(expanded_row_ids, move |handle| {
            let ids = expanded_row_ids.with(|ids| ids.deref().to_vec());
            handle.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
                table.expanded_row_ids.set(ids)
            });
        })
    }

    fn on_sort<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, K, TableSortDirection) + Send + Sync,
    {
        self.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
            table.on_sort = Some(Arc::new(callback))
        })
    }

    fn on_row_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id),
    {
        self.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
            table.on_row_select = Some(Box::new(callback))
        })
    }

    fn on_row_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id, bool),
    {
        self.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
            table.on_row_toggle = Some(Box::new(callback))
        })
    }
}
