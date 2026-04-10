use std::{ops::Deref, rc::Rc, sync::Arc};

use crate::prelude::*;

/// Sort direction for a table column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableSortDirection {
    /// No sort direction.
    None,
    /// Sort in ascending order.
    Ascending,
    /// Sort in descending order.
    Descending,
}

impl_res_simple!(TableSortDirection);

/// Controls how sortable columns cycle through sort directions when clicked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableSortCycle {
    /// Cycles between ascending and descending.
    BiState,
    /// Cycles ascending -> descending -> unsorted.
    TriState,
}

impl_res_simple!(TableSortCycle);

pub(super) fn sort_direction_for_column<K: PartialEq>(
    sort_state: Option<&TableSortState<K>>,
    column_key: &K,
) -> TableSortDirection {
    match sort_state {
        Some(state) if &state.key == column_key => state.direction,
        _ => TableSortDirection::None,
    }
}

pub(super) fn next_sort_direction(
    sort_cycle: TableSortCycle,
    current_direction: TableSortDirection,
) -> TableSortDirection {
    match (sort_cycle, current_direction) {
        (TableSortCycle::BiState, TableSortDirection::Ascending) => TableSortDirection::Descending,
        (TableSortCycle::BiState, _) => TableSortDirection::Ascending,
        (TableSortCycle::TriState, TableSortDirection::None) => TableSortDirection::Ascending,
        (TableSortCycle::TriState, TableSortDirection::Ascending) => TableSortDirection::Descending,
        (TableSortCycle::TriState, TableSortDirection::Descending) => TableSortDirection::None,
    }
}

type TableHeaderContent<S> = dyn Fn(&mut Context, Memo<TableSortDirection>) -> Handle<S>;
type TableCellContent<T> = dyn Fn(&mut Context, Memo<T>);

impl<T: PartialEq + 'static, S: View, K: Clone + PartialEq + Send + Sync + 'static>
    Res<Vec<TableColumn<T, S, K>>> for Vec<TableColumn<T, S, K>>
{
    fn get_value(&self, _: &impl DataContext) -> Vec<TableColumn<T, S, K>> {
        self.clone()
    }
}

/// Reusable helpers for building table header content.
#[derive(Clone)]
pub struct TableHeader;

impl TableHeader {
    pub fn new(
        cx: &mut Context,
        title: impl Into<String>,
        sort_direction: Memo<TableSortDirection>,
    ) -> Handle<'_, TableHeader> {
        Self.build(cx, move |cx| {
            let title = title.into();
            Label::new(cx, title).class("table-header-title").width(Stretch(1.0)).min_width(Auto);
            let sort_indicator = Memo::new(move |_| match sort_direction.get() {
                TableSortDirection::Ascending => "^".to_string(),
                TableSortDirection::Descending => "v".to_string(),
                TableSortDirection::None => "·".to_string(),
            });

            Label::new(cx, sort_indicator).class("table-sort-indicator").text_wrap(false);
        })
        .layout_type(LayoutType::Row)
        .width(Stretch(1.0))
        .min_width(Auto)
    }
}

impl View for TableHeader {
    fn element(&self) -> Option<&'static str> {
        Some("table-header")
    }
}

/// Externally controlled sort state for a table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableSortState<K = String> {
    /// Stable column key.
    pub key: K,
    /// Current sort direction.
    pub direction: TableSortDirection,
}

/// Describes a table column.
pub struct TableColumn<T: PartialEq + 'static, S: View, K = String>
where
    K: Clone + PartialEq + Send + Sync + 'static,
{
    /// Stable identity used to preserve state across reactive column updates.
    pub key: K,
    /// Initial width in logical pixels.
    pub width: Signal<f32>,
    /// Minimum width in logical pixels when resized.
    pub min_width: Signal<f32>,
    /// Whether this column can trigger sorting.
    pub sortable: Signal<bool>,
    /// Whether this column can be resized when table resizing is enabled.
    pub resizable: Signal<bool>,
    /// Whether this column is hidden from layout and rendering.
    pub hidden: Signal<bool>,
    /// Custom cell content builder.
    pub cell_content: Rc<TableCellContent<T>>,
    /// Custom header content builder.
    pub header_content: Rc<TableHeaderContent<S>>,
}

impl<T: PartialEq + 'static, S: View, K: Clone + PartialEq + Send + Sync + 'static> Clone for TableColumn<T, S, K> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            width: self.width,
            min_width: self.min_width,
            sortable: self.sortable,
            resizable: self.resizable,
            hidden: self.hidden,
            cell_content: self.cell_content.clone(),
            header_content: self.header_content.clone(),
        }
    }
}

impl<T: PartialEq + 'static, S: View, K: Clone + PartialEq + Send + Sync + 'static> TableColumn<T, S, K> {
    /// Creates a new table column from explicit header and cell builders.
    ///
    /// Use this when you need full control over header and cell rendering.
    ///
    /// ```ignore
    /// TableColumn::new(
    ///     "status",
    ///     |cx, sort_direction| TableHeader::new(cx, "Status", sort_direction),
    ///     |cx, row| {
    ///         let status = row.map(|row: &RowData| row.status.clone());
    ///         Label::new(cx, status).class("table-cell-text");
    ///     },
    /// )
    /// .resizable(true)
    /// .sortable(true);
    /// ```
    pub fn new(
        key: impl Into<K>,
        header_content: impl Fn(&mut Context, Memo<TableSortDirection>) -> Handle<S> + 'static,
        cell_content: impl Fn(&mut Context, Memo<T>) + 'static,
    ) -> Self {
        Self {
            key: key.into(),
            width: Signal::new(180.0),
            min_width: Signal::new(80.0),
            sortable: Signal::new(true),
            resizable: Signal::new(false),
            hidden: Signal::new(false),
            cell_content: Rc::new(cell_content),
            header_content: Rc::new(header_content),
        }
    }

    /// Sets the initial width.
    pub fn width(self, width: f32) -> Self {
        self.width.set(width.max(self.min_width.get_untracked()));
        self
    }

    /// Sets the minimum width.
    pub fn min_width(self, min_width: f32) -> Self {
        self.min_width.set(min_width);
        self.width.set(self.width.get_untracked().max(min_width));
        self
    }

    /// Sets whether this column can trigger sorting.
    pub fn sortable(self, sortable: bool) -> Self {
        self.sortable.set(sortable);
        self
    }

    /// Sets whether this column can be resized when table resizing is enabled.
    pub fn resizable(self, resizable: bool) -> Self {
        self.resizable.set(resizable);
        self
    }

    /// Sets whether this column is hidden from layout and rendering.
    pub fn hidden(self, hidden: bool) -> Self {
        self.hidden.set(hidden);
        self
    }

    /// Binds hidden state to an external resource.
    pub fn hidden_res<U: Into<bool> + Clone + 'static>(
        self,
        cx: &mut Context,
        hidden: impl Res<U> + 'static,
    ) -> Self {
        let hidden_signal = self.hidden;
        hidden.set_or_bind(cx, move |cx, res| {
            hidden_signal.set(res.get_value(cx).into());
        });
        self
    }
}

/// A table-like view backed by [`List`] for variable row heights.
///
/// This implementation prioritizes flexible row layout over viewport virtualization.
/// For large datasets, prefer filtering, pagination, or incremental loading at the model layer.
pub struct Table<T, V, Id, K = String>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: PartialEq + Clone + 'static,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    rows: Signal<V>,
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

enum TableEvent<K> {
    RequestSort(K, TableSortDirection),
    SelectRow(usize),
}

impl<T, V, Id, K> Table<T, V, Id, K>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: PartialEq + Clone + 'static,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    /// Creates a new table view.
    ///
    /// Sorting is emit-only: header presses call `on_sort`, while sorted data should be provided
    /// by the caller (for example via `Memo<Vec<T>>`).
    ///
    /// ```ignore
    /// Table::new(cx, sorted_rows, columns, |row: &RowData| row.id)
    ///     .sort_state(sort_state)
    ///     .resizable_columns(true)
    ///     .selectable(Selectable::Single)
    ///     .selected_row_ids(selected_ids)
    ///     .on_sort(|cx, column, direction| {
    ///         cx.emit(AppEvent::SetSort(column, direction));
    ///     })
    ///     .on_row_select(|cx, id| {
    ///         cx.emit(AppEvent::SelectRow(id));
    ///     });
    /// ```
    pub fn new<S, C, R, H>(
        cx: &mut Context,
        rows: S,
        columns: C,
        row_id: impl Fn(&T) -> Id + 'static,
    ) -> Handle<Self>
    where
        S: Res<V> + 'static,
        C: Res<R> + 'static,
        R: Deref<Target = [TableColumn<T, H, K>]> + Clone + 'static,
        H: Clone + View,
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
                let body_columns = header_columns.clone();

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
                                        cx.emit(TableEvent::RequestSort(
                                            column_key.clone(),
                                            next_direction,
                                        ));
                                    }
                                });
                            })
                            .class("table-header-cell")
                            .toggle_class("sortable", sortable)
                            .toggle_class("resizable", false)
                            .width(Stretch(1.0))
                            .min_width(Auto);
                        } else {
                            ResizableStack::new(
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
                                            cx.emit(TableEvent::RequestSort(
                                                column_key.clone(),
                                                next_direction,
                                            ));
                                        }
                                    });
                                },
                            )
                            .class("table-header-cell")
                            .toggle_class("sortable", sortable)
                            .toggle_class(
                                "resizable",
                                resizable_columns.map(move |enabled| *enabled && resizable.get()),
                            )
                            .min_width(min_width.map(|value| Pixels(*value)));
                        }
                    }
                })
                .class("table-header-row")
                .height(Auto)
                .width(Stretch(1.0))
                .min_width(Auto);

                List::new(cx, row_signal, move |cx, row_index, row| {
                    HStack::new(cx, |cx| {
                        for (column_index, column) in body_columns.iter().enumerate() {
                            let width_signal = column.width;
                            let min_width = column.min_width;
                            let cell_content = column.cell_content.clone();
                            let is_last_column = column_index + 1 == body_columns.len();

                            if is_last_column {
                                VStack::new(cx, move |cx| {
                                    cell_content(cx, row.map(|value| value.clone()));
                                })
                                .class("table-cell")
                                .width(Stretch(1.0))
                                .min_width(Auto)
                                .height(Auto);
                            } else {
                                VStack::new(cx, move |cx| {
                                    cell_content(cx, row.map(|value| value.clone()));
                                })
                                .class("table-cell")
                                .width(width_signal.map(|value| Pixels(*value)))
                                .min_width(min_width.map(|value| Pixels(*value)))
                                .height(Auto);
                            }
                        }
                    })
                    .class("table-row")
                    .toggle_class("odd", row_index % 2 == 1)
                    .toggle_class("even", row_index % 2 == 0)
                    .alignment(Alignment::Left)
                    .height(Auto)
                    .width(Stretch(1.0))
                    .min_width(Auto);
                })
                .width(Stretch(1.0))
                .min_width(Auto)
                .height(Stretch(1.0))
                .min_height(Auto)
                .class("table-body")
                .selected(selected_indices)
                .selectable(selectable)
                .selection_follows_focus(selection_follows_focus)
                .on_select(move |cx, index| cx.emit(TableEvent::<K>::SelectRow(index)));
            });
        })
    }
}

impl<T, V, Id, K> View for Table<T, V, Id, K>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: PartialEq + Clone + 'static,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    fn element(&self) -> Option<&'static str> {
        Some("table")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|table_event: &TableEvent<K>, _| match table_event {
            TableEvent::RequestSort(key, direction) => {
                if let Some(callback) = &self.on_sort {
                    (callback)(cx, key.clone(), *direction);
                }
            }

            TableEvent::SelectRow(index) => {
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

/// Modifiers for configuring controlled table state and callbacks.
pub trait TableModifiers<Id, K = String>: Sized
where
    K: Clone + PartialEq + Send + Sync + 'static,
{
    /// Sets the current sort state.
    fn sort_state(self, sort_state: impl Res<Option<TableSortState<K>>> + 'static) -> Self;

    /// Enables or disables column resizing for all columns.
    fn resizable_columns<U: Into<bool> + Clone + 'static>(
        self,
        flag: impl Res<U> + 'static,
    ) -> Self;

    /// Sets the sort cycle behavior for sortable columns.
    fn sort_cycle<U: Into<TableSortCycle> + Clone + 'static>(
        self,
        cycle: impl Res<U> + 'static,
    ) -> Self;

    /// Sets the selectable state of the table rows.
    fn selectable<U: Into<Selectable> + Clone + 'static>(
        self,
        selectable: impl Res<U> + 'static,
    ) -> Self;

    /// Sets whether selection follows focus.
    fn selection_follows_focus<U: Into<bool> + Clone + 'static>(
        self,
        flag: impl Res<U> + 'static,
    ) -> Self;

    /// Sets externally controlled selected row ids.
    fn selected_row_ids<R>(self, selected_row_ids: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [Id]> + Clone + 'static;

    /// Sets the callback triggered when a header requests sorting.
    fn on_sort<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, K, TableSortDirection) + Send + Sync;

    /// Sets the callback triggered when a row is selected.
    fn on_row_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id);
}

impl<T, V, Id, K> TableModifiers<Id, K> for Handle<'_, Table<T, V, Id, K>>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: PartialEq + Clone + 'static,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    fn sort_state(self, sort_state: impl Res<Option<TableSortState<K>>> + 'static) -> Self {
        let sort_state = sort_state.to_signal(self.cx);
        self.bind(sort_state, move |handle| {
            let sort_state = sort_state.get();
            handle.modify(|table: &mut Table<T, V, Id, K>| table.sort_state.set(sort_state));
        })
    }

    fn resizable_columns<U: Into<bool> + Clone + 'static>(
        self,
        flag: impl Res<U> + 'static,
    ) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let flag = flag.get().into();
            handle.modify(|table: &mut Table<T, V, Id, K>| table.resizable_columns.set(flag));
        })
    }

    fn sort_cycle<U: Into<TableSortCycle> + Clone + 'static>(
        self,
        cycle: impl Res<U> + 'static,
    ) -> Self {
        let cycle = cycle.to_signal(self.cx);
        self.bind(cycle, move |handle| {
            let cycle = cycle.get().into();
            handle.modify(|table: &mut Table<T, V, Id, K>| table.sort_cycle.set(cycle));
        })
    }

    fn selectable<U: Into<Selectable> + Clone + 'static>(
        self,
        selectable: impl Res<U> + 'static,
    ) -> Self {
        let selectable = selectable.to_signal(self.cx);
        self.bind(selectable, move |handle| {
            let selectable = selectable.get().into();
            handle.modify(|table: &mut Table<T, V, Id, K>| table.selectable.set(selectable));
        })
    }

    fn selection_follows_focus<U: Into<bool> + Clone + 'static>(
        self,
        flag: impl Res<U> + 'static,
    ) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let flag = flag.get().into();
            handle.modify(|table: &mut Table<T, V, Id, K>| table.selection_follows_focus.set(flag));
        })
    }

    fn selected_row_ids<R>(self, selected_row_ids: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [Id]> + Clone + 'static,
    {
        let selected_row_ids = selected_row_ids.to_signal(self.cx);
        self.bind(selected_row_ids, move |handle| {
            let ids = selected_row_ids.with(|ids| ids.deref().to_vec());
            handle.modify(|table: &mut Table<T, V, Id, K>| table.selected_row_ids.set(ids));
        })
    }

    fn on_sort<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, K, TableSortDirection) + Send + Sync,
    {
        self.modify(|table: &mut Table<T, V, Id, K>| table.on_sort = Some(Arc::new(callback)))
    }

    fn on_row_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id),
    {
        self.modify(|table: &mut Table<T, V, Id, K>| table.on_row_select = Some(Box::new(callback)))
    }
}
