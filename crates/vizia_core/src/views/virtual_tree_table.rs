use accesskit::SortDirection as AccessSortDirection;
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
    Cell, TableSelectionMode, TableSortCycle, TableSortDirection, TableSortState, TreeNodeRow,
    TreeTableColumn, TreeTableEvent, TreeTableFirstCellEvent, TreeTableRow,
    table::next_sort_direction, table::sort_direction_for_column,
};

#[derive(Clone, PartialEq)]
enum TableFocus<Id, K>
where
    Id: Eq + Hash + Clone + Send + Sync + 'static,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    Row(Id),
    Cell(Id, K),
}

fn flatten_hierarchy_rows<U, Id>(
    tree: &U,
    root_ids: &dyn Fn(&U) -> Vec<Id>,
    child_ids: &dyn Fn(&U, &Id) -> Vec<Id>,
    is_visible: &dyn Fn(&U, &Id) -> bool,
) -> Vec<TreeNodeRow<Id>>
where
    Id: Clone + Eq + Hash + 'static,
{
    fn visit<U, Id>(
        tree: &U,
        node_id: Id,
        parent_id: Option<Id>,
        child_ids: &dyn Fn(&U, &Id) -> Vec<Id>,
        is_visible: &dyn Fn(&U, &Id) -> bool,
        out: &mut Vec<TreeNodeRow<Id>>,
    ) where
        Id: Clone + Eq + Hash + 'static,
    {
        if !is_visible(tree, &node_id) {
            return;
        }

        out.push(TreeNodeRow { id: node_id.clone(), parent_id });

        for child_id in child_ids(tree, &node_id) {
            visit(tree, child_id, Some(node_id.clone()), child_ids, is_visible, out);
        }
    }

    let mut rows = Vec::new();
    for root_id in root_ids(tree) {
        visit(tree, root_id, None, child_ids, is_visible, &mut rows);
    }

    rows
}

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
            let child_rows =
                rows_by_parent.get(&Some(id.clone())).map(Vec::as_slice).unwrap_or(&[]);
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
                visit(child_rows, depth + 1, rows_by_parent, expanded_set, row_id, out);
            }
        }
    }

    let roots = rows_by_parent.get(&None).map(Vec::as_slice).unwrap_or(&[]);
    visit(roots, 0, &rows_by_parent, &expanded_set, row_id, &mut visible_rows);

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
    K: Eq + Hash + Clone + PartialEq + Send + Sync + 'static,
{
    rows: Signal<V>,
    visible_rows: Memo<Vec<TreeTableRow<T, Id>>>,
    _header: PhantomData<H>,
    row_id: Rc<dyn Fn(&T) -> Id>,
    parent_id: Rc<dyn Fn(&T) -> Option<Id>>,
    sort_state: Signal<Option<TableSortState<K>>>,
    sort_cycle: Signal<TableSortCycle>,
    resizable_columns: Signal<bool>,
    selectable: Signal<Selectable>,
    selection_follows_focus: Signal<bool>,
    selection: Signal<HashSet<Cell<Id, K>>>,
    selection_mode: Signal<TableSelectionMode>,
    expanded_row_ids: Signal<Vec<Id>>,
    focused: Signal<Option<TableFocus<Id, K>>>,
    treegrid_label: Signal<Option<String>>,
    on_sort: Option<Arc<dyn Fn(&mut EventContext, K, TableSortDirection) + Send + Sync>>,
    on_select: Option<Box<dyn Fn(&mut EventContext, HashSet<Cell<Id, K>>) + Send + Sync>>,
    on_row_toggle: Option<Box<dyn Fn(&mut EventContext, Id, bool)>>,
    columns: Memo<Vec<K>>,
}

impl<T, V, Id, H, K> VirtualTreeTable<T, V, Id, H, K>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
    H: Clone + View,
    K: Eq + Hash + Clone + PartialEq + Send + Sync + 'static,
{
    pub fn new<S, U, C, R, F>(
        cx: &mut Context,
        tree: S,
        columns: C,
        item_height: f32,
        flatten_rows: F,
        row_id: impl Fn(&T) -> Id + 'static,
        parent_id: impl Fn(&T) -> Option<Id> + 'static,
    ) -> Handle<Self>
    where
        S: Res<U> + 'static,
        U: Clone + 'static,
        C: Res<R> + 'static,
        R: Deref<Target = [TreeTableColumn<T, Id, H, K>]> + Clone + 'static,
        F: Fn(&U) -> V + 'static,
    {
        let tree_signal = tree.to_signal(cx);
        let flatten_rows: Rc<dyn Fn(&U) -> V> = Rc::new(flatten_rows);
        let row_signal = Signal::new(tree_signal.with(|tree| flatten_rows(tree)));
        let column_signal = columns.to_signal(cx);
        let row_id: Rc<dyn Fn(&T) -> Id> = Rc::new(row_id);
        let parent_id: Rc<dyn Fn(&T) -> Option<Id>> = Rc::new(parent_id);

        let sort_state = Signal::new(None);
        let sort_cycle = Signal::new(TableSortCycle::BiState);
        let resizable_columns = Signal::new(false);
        let selectable = Signal::new(Selectable::None);
        let selection_follows_focus = Signal::new(false);
        let selection = Signal::new(HashSet::new());
        let selection_mode = Signal::new(TableSelectionMode::Cell);
        let expanded_row_ids = Signal::new(Vec::new());
        let focused = Signal::new(None);
        let treegrid_label = Signal::new(None::<String>);

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

        let focused_index = Memo::new(move |_| {
            visible_rows.with(|rows| {
                let focused_row_id =
                    focused.with(|focused: &Option<TableFocus<Id, K>>| match focused {
                        Some(TableFocus::Row(id)) => Some(id.clone()),
                        Some(TableFocus::Cell(_id, _)) => None,
                        None => None,
                    });

                rows.iter().position(|row| Some(&row.id) == focused_row_id.as_ref())
            })
        });

        let columns = Memo::new(move |_| {
            column_signal.with(|columns| {
                columns
                    .deref()
                    .iter()
                    .filter(|column| !column.hidden.get())
                    .map(|column| column.key.clone())
                    .collect::<Vec<_>>()
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

        let handle = Self {
            rows: row_signal,
            visible_rows,
            _header: PhantomData,
            row_id,
            parent_id,
            sort_state,
            sort_cycle,
            resizable_columns,
            selectable,
            selection_follows_focus,
            selection,
            selection_mode,
            expanded_row_ids,
            focused,
            treegrid_label,
            on_sort: None,
            on_select: None,
            on_row_toggle: None,
            columns,
        }
        .build(cx, move |cx| {
            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowRight),
                    KeymapEntry::new("Focus Right", |cx| {
                        cx.emit(TreeTableEvent::<K, Id>::FocusRight)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowLeft),
                    KeymapEntry::new("Focus Left", |cx| {
                        cx.emit(TreeTableEvent::<K, Id>::FocusLeft)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                    KeymapEntry::new("Focus Up", |cx| cx.emit(TreeTableEvent::<K, Id>::FocusUp)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                    KeymapEntry::new("Focus Down", |cx| {
                        cx.emit(TreeTableEvent::<K, Id>::FocusDown)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Home),
                    KeymapEntry::new("Focus Home", |cx| {
                        cx.emit(TreeTableEvent::<K, Id>::FocusHome)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::End),
                    KeymapEntry::new("Focus End", |cx| cx.emit(TreeTableEvent::<K, Id>::FocusEnd)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::PageUp),
                    KeymapEntry::new("Focus Page Up", |cx| {
                        cx.emit(TreeTableEvent::<K, Id>::PageUp)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::PageDown),
                    KeymapEntry::new("Focus Page Down", |cx| {
                        cx.emit(TreeTableEvent::<K, Id>::PageDown)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::CTRL, Code::Home),
                    KeymapEntry::new("Focus Control Home", |cx| {
                        cx.emit(TreeTableEvent::<K, Id>::CtrlHome)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::CTRL, Code::End),
                    KeymapEntry::new("Focus Control End", |cx| {
                        cx.emit(TreeTableEvent::<K, Id>::CtrlEnd)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Enter),
                    KeymapEntry::new("Select Focused", |cx| {
                        cx.emit(TreeTableEvent::<K, Id>::SelectFocused)
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
                            let header = header_content(cx, sort_direction);
                            let on_press_column_key = column_key.clone();
                            header
                                .class("table-header-cell")
                                .role(Role::ColumnHeader)
                                .sort_direction(sort_direction.map(|direction| match direction {
                                    TableSortDirection::Ascending => {
                                        Some(AccessSortDirection::Ascending)
                                    }
                                    TableSortDirection::Descending => {
                                        Some(AccessSortDirection::Descending)
                                    }
                                    TableSortDirection::None => None,
                                }))
                                .toggle_class("sortable", sortable)
                                .toggle_class("not-sortable", sortable.map(|value| !*value))
                                .toggle_class("resizable", false)
                                .toggle_class("not-resizable", true)
                                .width(Stretch(1.0))
                                .min_width(Auto)
                                .on_press(move |cx| {
                                    cx.emit(TreeTableEvent::<K, Id>::SelectColumn(
                                        on_press_column_key.clone(),
                                    ));
                                });
                        } else {
                            Resizable::new(
                                cx,
                                width_signal,
                                ResizeStackDirection::Right,
                                move |_cx, new_size| {
                                    if resizable_columns.get() && resizable.get() {
                                        width_signal.set(Pixels(new_size.max(min_width.get())));
                                    }
                                },
                                move |cx| {
                                    let header = header_content(cx, sort_direction);
                                    let on_press_column_key = column_key.clone();
                                    header.on_press(move |cx| {
                                        cx.emit(TreeTableEvent::<K, Id>::SelectColumn(
                                            on_press_column_key.clone(),
                                        ));
                                    });
                                },
                            )
                            .class("table-header-cell")
                            .role(Role::ColumnHeader)
                            .sort_direction(sort_direction.map(|direction| match direction {
                                TableSortDirection::Ascending => {
                                    Some(AccessSortDirection::Ascending)
                                }
                                TableSortDirection::Descending => {
                                    Some(AccessSortDirection::Descending)
                                }
                                TableSortDirection::None => None,
                            }))
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
                            .width(width_signal)
                            .min_width(min_width.map(|value| Pixels(*value)));
                        }
                    }
                })
                .class("table-header-row")
                .height(Auto)
                .width(Stretch(1.0))
                .min_width(Auto);

                VirtualList::new_custom_items_with_selection(
                    cx,
                    visible_rows,
                    item_height,
                    move |cx, row_index, row, _selected| {
                        let row: Memo<TreeTableRow<T, Id>> = Memo::new(move |_| row.get());
                        let mut row_handle =
                            HStack::new(cx, |cx| {
                                column_signal.with(|columns| {
                                    let visible_columns = columns
                                        .deref()
                                        .iter()
                                        .filter(|column| !column.hidden.get())
                                        .collect::<Vec<_>>();

                                    for (column_index, column) in visible_columns.iter().enumerate()
                                    {
                                        let width_signal = column.width;
                                        let min_width = column.min_width;
                                        let cell_content = column.cell_content.clone();
                                        let is_last_column =
                                            column_index + 1 == visible_columns.len();

                                        let col_key = column.key.clone();
                                        let cell_row_id = row.map(|value| value.id.clone());
                                        let cell_col_key = col_key.clone();

                                        let col_key_for_focus = col_key.clone();
                                        let row_id_for_focus = row.map(|value| value.id.clone());
                                        let cell_is_focused =
                                            focused.map(move |focused| match focused {
                                                Some(TableFocus::Cell(id, col)) => {
                                                    *col == col_key_for_focus
                                                        && *id == row_id_for_focus.get()
                                                }
                                                _ => false,
                                            });

                                        let col_key_for_select = col_key.clone();
                                        let row_id_for_select = row.map(|value| value.id.clone());
                                        let cell_is_selected = selection.map(move |selection| {
                                            selection.contains(&Cell(
                                                row_id_for_select.get(),
                                                col_key_for_select.clone(),
                                            ))
                                        });

                                        let cell_col_key_1 = cell_col_key.clone();
                                        let cell_col_key_2 = cell_col_key.clone();

                                        if is_last_column {
                                            VStack::new(cx, move |cx| {
                                                cell_content(cx, row);
                                            })
                                            .class("table-cell")
                                            .role(Role::GridCell)
                                            .toggle_class("selected", cell_is_selected)
                                            .selected(cell_is_selected)
                                            .focusable(true)
                                            .navigable(false)
                                            .focused_with_visibility(cell_is_focused, true)
                                            .width(Stretch(1.0))
                                            .min_width(Auto)
                                            .height(Stretch(1.0))
                                            .min_height(Auto)
                                            .on_press(move |cx| {
                                                cx.emit(TreeTableEvent::<K, Id>::SelectCell(
                                                    cell_row_id.get(),
                                                    cell_col_key_1.clone(),
                                                ));
                                            });
                                        } else {
                                            VStack::new(cx, move |cx| {
                                                cell_content(cx, row);
                                            })
                                            .class("table-cell")
                                            .role(Role::GridCell)
                                            .toggle_class("selected", cell_is_selected)
                                            .selected(cell_is_selected)
                                            .focusable(true)
                                            .navigable(false)
                                            .focused_with_visibility(cell_is_focused, true)
                                            .width(width_signal)
                                            .min_width(min_width.map(|value| Pixels(*value)))
                                            .height(Stretch(1.0))
                                            .min_height(Auto)
                                            .on_press(move |cx| {
                                                cx.emit(TreeTableEvent::<K, Id>::SelectCell(
                                                    cell_row_id.get(),
                                                    cell_col_key_2.clone(),
                                                ));
                                            });
                                        }
                                    }
                                });
                            })
                            .class("table-row")
                            .toggle_class("odd", row_index % 2 == 1)
                            .toggle_class("even", row_index % 2 == 0)
                            .toggle_class("expanded", row.map(|value| value.expanded))
                            .toggle_class("collapsible", row.map(|value| value.has_children))
                            .alignment(Alignment::Left)
                            .height(Auto)
                            .width(Stretch(1.0))
                            .min_width(Auto)
                            .role(Role::Row)
                            .level(row.map(|value| value.depth + 1))
                            .on_press(move |cx| cx.emit(ListEvent::Select(row_index)));

                        if row.get().has_children {
                            row_handle = row_handle.expanded(row.map(|value| value.expanded));
                        }

                        row_handle
                    },
                )
                .width(Stretch(1.0))
                .min_width(Auto)
                .height(Stretch(1.0))
                .min_height(Auto)
                .class("table-body")
                .class("tree-table-body")
                .focused_index(focused_index)
                .on_focus(move |_cx, index| {
                    visible_rows.with(|rows| {
                        if let Some(row) = rows.get(index) {
                            focused.set(Some(TableFocus::Row(row.id.clone())));
                        }
                    });
                })
                .on_build(|cx| {
                    cx.emit_to(
                        cx.current(),
                        KeymapEvent::RemoveAction(
                            KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                            "Focus Next",
                        ),
                    );

                    cx.emit_to(
                        cx.current(),
                        KeymapEvent::RemoveAction(
                            KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                            "Focus Previous",
                        ),
                    );
                });
            });
        })
        .class("table")
        .class("virtual-table")
        .class("tree-table")
        .class("virtual-tree-table")
        .navigable(true)
        .name(treegrid_label.map(|label| label.clone().unwrap_or_else(|| "Tree table".to_string())))
        .multiselectable(selectable.map(|mode| *mode == Selectable::Multi))
        .role(Role::TreeGrid);

        let flatten_rows_for_bind = flatten_rows.clone();
        handle.bind(tree_signal, move |handle| {
            let rows = tree_signal.with(|tree| flatten_rows_for_bind(tree));
            handle.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| table.rows.set(rows));
        })
    }

    pub fn from_rows<S, C, R>(
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
        Self::new(cx, rows, columns, item_height, |rows: &V| rows.clone(), row_id, parent_id)
    }

    fn emit_toggle(&self, cx: &mut EventContext, row_id: Id, next_expanded: bool) {
        if let Some(callback) = &self.on_row_toggle {
            (callback)(cx, row_id, next_expanded);
        }
    }

    fn focused_visible_row(&self) -> Option<TreeTableRow<T, Id>> {
        let focused_id = self.focused.get().and_then(|focused| match focused {
            TableFocus::Row(id) => Some(id.clone()),
            TableFocus::Cell(id, _) => Some(id.clone()),
        })?;

        self.visible_rows.with(|rows| rows.clone()).into_iter().find(|row| row.id == focused_id)
    }

    fn focus_row_id(&self, row_id: Id) {
        let next_focus = match self.focused.get() {
            Some(TableFocus::Cell(_, column_key)) => TableFocus::Cell(row_id, column_key),
            _ => TableFocus::Row(row_id),
        };

        self.focused.set(Some(next_focus));
    }
}

impl<Id, H, K> VirtualTreeTable<TreeNodeRow<Id>, Vec<TreeNodeRow<Id>>, Id, H, K>
where
    Id: Eq + Hash + Clone + Send + Sync + 'static,
    H: Clone + View,
    K: Eq + Hash + Clone + PartialEq + Send + Sync + 'static,
{
    /// Creates a [`VirtualTreeTable`] directly from hierarchical data.
    ///
    /// Provide closures to enumerate root node IDs and child node IDs for a given parent.
    /// The order returned by those closures is preserved; filtering only removes hidden
    /// nodes and their descendants.
    /// The table manages expand/collapse state and internally projects IDs into
    /// [`TreeNodeRow`] values.
    pub fn from_hierarchy<S, U, C, R>(
        cx: &mut Context,
        tree: S,
        columns: C,
        item_height: f32,
        root_ids: impl Fn(&U) -> Vec<Id> + 'static,
        child_ids: impl Fn(&U, &Id) -> Vec<Id> + 'static,
        is_visible: impl Fn(&U, &Id) -> bool + 'static,
    ) -> Handle<Self>
    where
        S: Res<U> + 'static,
        U: Clone + 'static,
        C: Res<R> + 'static,
        R: Deref<Target = [TreeTableColumn<TreeNodeRow<Id>, Id, H, K>]> + Clone + 'static,
    {
        let root_ids: Rc<dyn Fn(&U) -> Vec<Id>> = Rc::new(root_ids);
        let child_ids: Rc<dyn Fn(&U, &Id) -> Vec<Id>> = Rc::new(child_ids);
        let is_visible: Rc<dyn Fn(&U, &Id) -> bool> = Rc::new(is_visible);

        Self::new(
            cx,
            tree,
            columns,
            item_height,
            move |tree: &U| flatten_hierarchy_rows(tree, &*root_ids, &*child_ids, &*is_visible),
            |row: &TreeNodeRow<Id>| row.id.clone(),
            |row: &TreeNodeRow<Id>| row.parent_id.clone(),
        )
    }
}

impl<T, V, Id, H, K> View for VirtualTreeTable<T, V, Id, H, K>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
    H: Clone + View,
    K: Eq + Hash + Clone + PartialEq + Send + Sync + 'static,
{
    fn element(&self) -> Option<&'static str> {
        Some("virtual-tree-table")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|table_event: &TreeTableEvent<K, Id>, _| match table_event {
            TreeTableEvent::RequestSort(key, direction) => {
                if let Some(callback) = &self.on_sort {
                    (callback)(cx, key.clone(), *direction);
                }
            }

            TreeTableEvent::SelectColumn(column_key) => {
                let visible_rows = flatten_visible_rows(
                    &self.rows.get(),
                    &*self.row_id,
                    &*self.parent_id,
                    &self.expanded_row_ids.get(),
                );

                let column_cells = HashSet::from_iter(
                    visible_rows.iter().map(|row| Cell(row.id.clone(), column_key.clone())),
                );

                let current_selection = self.selection.get();
                let next_selection =
                    if current_selection == column_cells { HashSet::new() } else { column_cells };

                self.selection.set(next_selection.clone());

                if let Some(callback) = &self.on_select {
                    (callback)(cx, next_selection);
                }
            }

            TreeTableEvent::SelectRow(index) => {
                if self.selectable.get() == Selectable::None {
                    return;
                }

                let visible_rows = flatten_visible_rows(
                    &self.rows.get(),
                    &*self.row_id,
                    &*self.parent_id,
                    &self.expanded_row_ids.get(),
                );

                if let Some(row) = visible_rows.get(*index) {
                    let cols = self.columns.get();
                    self.selection.set(HashSet::from_iter(
                        cols.iter().map(|col| Cell(row.id.clone(), col.clone())),
                    ));

                    if let Some(callback) = &self.on_select {
                        (callback)(cx, self.selection.get());
                    }
                }
            }

            TreeTableEvent::SelectCell(row_id, column_key) => {
                let selectable = self.selectable.get();
                if selectable == Selectable::None {
                    return;
                }

                let selection_mode = self.selection_mode.get();

                let next_selection = match selection_mode {
                    TableSelectionMode::Cell => {
                        let cell = Cell(row_id.clone(), column_key.clone());
                        if selectable == Selectable::Single {
                            let current = self.selection.get();
                            if current.contains(&cell) {
                                HashSet::new()
                            } else {
                                HashSet::from([cell])
                            }
                        } else {
                            let current = self.selection.get();
                            let mut next = current;
                            if next.contains(&cell) {
                                next.remove(&cell);
                            } else {
                                next.insert(cell);
                            }
                            next
                        }
                    }
                    TableSelectionMode::Row => {
                        let cols = self.columns.get();
                        let row_cells: HashSet<Cell<Id, K>> = HashSet::from_iter(
                            cols.iter().map(|col| Cell(row_id.clone(), col.clone())),
                        );

                        if selectable == Selectable::Single {
                            let current = self.selection.get();
                            if current == row_cells { HashSet::new() } else { row_cells }
                        } else {
                            let mut next = self.selection.get();
                            let row_selected = row_cells.iter().all(|c| next.contains(c));
                            if row_selected {
                                for c in &row_cells {
                                    next.remove(c);
                                }
                            } else {
                                next.extend(row_cells);
                            }
                            next
                        }
                    }
                };

                self.selection.set(next_selection.clone());

                if let Some(callback) = &self.on_select {
                    (callback)(cx, next_selection);
                }
            }

            TreeTableEvent::SelectFocused => self.focused.with(|focused| match focused {
                Some(TableFocus::Row(id)) => {
                    if self.selectable.get() == Selectable::None {
                        return;
                    }

                    let cols = self.columns.get();
                    self.selection.set(HashSet::from_iter(
                        cols.iter().map(|col| Cell(id.clone(), col.clone())),
                    ));

                    if let Some(callback) = &self.on_select {
                        (callback)(cx, self.selection.get());
                    }
                }
                Some(TableFocus::Cell(id, col)) => {
                    self.selection.set(HashSet::from([Cell(id.clone(), col.clone())]));

                    if let Some(callback) = &self.on_select {
                        (callback)(cx, self.selection.get());
                    }
                }
                None => {}
            }),

            TreeTableEvent::ExpandSelected => {
                if let Some(row) = self.focused_visible_row() {
                    if row.has_children && !row.expanded {
                        self.emit_toggle(cx, row.id, true);
                    } else if row.has_children {
                        let child_id = self.visible_rows.with(|rows| {
                            rows.iter()
                                .find(|candidate| candidate.parent_id.as_ref() == Some(&row.id))
                                .map(|candidate| candidate.id.clone())
                        });

                        if let Some(child_id) = child_id {
                            self.focus_row_id(child_id);
                        }
                    }
                }
            }

            TreeTableEvent::FocusRight => {
                let next_focus = match self.focused.get() {
                    Some(TableFocus::Row(id)) => {
                        if let Some(row) = self.focused_visible_row() {
                            if row.has_children && !row.expanded {
                                cx.emit(TreeTableEvent::<K, Id>::ExpandSelected);
                                return;
                            }
                        }

                        let first_column_key =
                            self.columns.with(|columns| columns.first().cloned()).unwrap();
                        Some(TableFocus::Cell(id.clone(), first_column_key))
                    }
                    Some(TableFocus::Cell(id, col)) => {
                        let next_col = self.columns.with(|columns| {
                            columns
                                .iter()
                                .position(|column| column == &col)
                                .and_then(|index| index.checked_add(1))
                                .and_then(|index| columns.get(index))
                                .cloned()
                        });

                        next_col.map(|next_col| TableFocus::Cell(id.clone(), next_col))
                    }
                    None => None,
                };

                if next_focus.is_some() {
                    self.focused.set(next_focus);
                }
            }

            TreeTableEvent::FocusLeft => {
                let next_focus = match self.focused.get() {
                    Some(TableFocus::Row(_)) => {
                        if let Some(row) = self.focused_visible_row() {
                            if row.has_children && row.expanded {
                                self.emit_toggle(cx, row.id, false);
                            }
                        }

                        None
                    }
                    Some(TableFocus::Cell(id, col)) => {
                        let prev_col = self.columns.with(|columns| {
                            columns
                                .iter()
                                .position(|column| column == &col)
                                .and_then(|index| index.checked_sub(1))
                                .and_then(|index| columns.get(index))
                                .cloned()
                        });

                        prev_col
                            .map(|prev_col| TableFocus::Cell(id.clone(), prev_col))
                            .or_else(|| Some(TableFocus::Row(id.clone())))
                    }
                    None => None,
                };

                if next_focus.is_some() {
                    self.focused.set(next_focus);
                }
            }

            TreeTableEvent::FocusHome => {
                let next_focus = self.focused.with(|focused| match focused {
                    Some(TableFocus::Row(_)) => self
                        .visible_rows
                        .with(|rows| rows.first().map(|row| TableFocus::Row(row.id.clone()))),
                    Some(TableFocus::Cell(_, col)) => self.visible_rows.with(|rows| {
                        rows.first().map(|row| TableFocus::Cell(row.id.clone(), col.clone()))
                    }),
                    None => None,
                });

                if next_focus.is_some() {
                    self.focused.set(next_focus);
                }
            }

            TreeTableEvent::FocusEnd => {
                let next_focus = self.focused.with(|focused| match focused {
                    Some(TableFocus::Row(_)) => self
                        .visible_rows
                        .with(|rows| rows.last().map(|row| TableFocus::Row(row.id.clone()))),
                    Some(TableFocus::Cell(_, col)) => self.visible_rows.with(|rows| {
                        rows.last().map(|row| TableFocus::Cell(row.id.clone(), col.clone()))
                    }),
                    None => None,
                });

                if next_focus.is_some() {
                    self.focused.set(next_focus);
                }
            }

            TreeTableEvent::PageUp => {
                let page_size = self.visible_rows.with(|rows| rows.len().saturating_div(2).max(1));
                let next_focus = self.focused.with(|focused| {
                    self.visible_rows.with(|rows| {
                        let current_index = focused.as_ref().and_then(|focused| match focused {
                            TableFocus::Row(id) | TableFocus::Cell(id, _) => {
                                rows.iter().position(|row| row.id == id.clone())
                            }
                        });

                        current_index.and_then(|index| {
                            let next_index = index.saturating_sub(page_size);
                            rows.get(next_index).map(|row| match focused.as_ref() {
                                Some(TableFocus::Cell(_, col)) => {
                                    TableFocus::Cell(row.id.clone(), col.clone())
                                }
                                _ => TableFocus::Row(row.id.clone()),
                            })
                        })
                    })
                });

                if next_focus.is_some() {
                    self.focused.set(next_focus);
                }
            }

            TreeTableEvent::PageDown => {
                let page_size = self.visible_rows.with(|rows| rows.len().saturating_div(2).max(1));
                let next_focus = self.focused.with(|focused| {
                    self.visible_rows.with(|rows| {
                        let current_index = focused.as_ref().and_then(|focused| match focused {
                            TableFocus::Row(id) | TableFocus::Cell(id, _) => {
                                rows.iter().position(|row| row.id == id.clone())
                            }
                        });

                        current_index.and_then(|index| {
                            let next_index = (index + page_size).min(rows.len().saturating_sub(1));
                            rows.get(next_index).map(|row| match focused.as_ref() {
                                Some(TableFocus::Cell(_, col)) => {
                                    TableFocus::Cell(row.id.clone(), col.clone())
                                }
                                _ => TableFocus::Row(row.id.clone()),
                            })
                        })
                    })
                });

                if next_focus.is_some() {
                    self.focused.set(next_focus);
                }
            }

            TreeTableEvent::CtrlHome => {
                let next_focus = self.focused.with(|focused| {
                    self.visible_rows.with(|rows| {
                        rows.first().map(|row| match focused {
                            Some(TableFocus::Cell(_, col)) => {
                                TableFocus::Cell(row.id.clone(), col.clone())
                            }
                            _ => TableFocus::Row(row.id.clone()),
                        })
                    })
                });

                if next_focus.is_some() {
                    self.focused.set(next_focus);
                }
            }

            TreeTableEvent::CtrlEnd => {
                let next_focus = self.focused.with(|focused| {
                    self.visible_rows.with(|rows| {
                        rows.last().map(|row| match focused {
                            Some(TableFocus::Cell(_, col)) => {
                                TableFocus::Cell(row.id.clone(), col.clone())
                            }
                            _ => TableFocus::Row(row.id.clone()),
                        })
                    })
                });

                if next_focus.is_some() {
                    self.focused.set(next_focus);
                }
            }

            TreeTableEvent::FocusUp => {
                let next_focus = self.focused.with(|focused| match focused {
                    Some(TableFocus::Row(id)) => self.visible_rows.with(|rows| {
                        rows.iter()
                            .take_while(|row| row.id != *id)
                            .last()
                            .map(|row| TableFocus::Row(row.id.clone()))
                    }),
                    Some(TableFocus::Cell(id, col)) => self.visible_rows.with(|rows| {
                        rows.iter()
                            .take_while(|row| row.id != *id)
                            .last()
                            .map(|row| TableFocus::Cell(row.id.clone(), col.clone()))
                    }),
                    None => None,
                });

                if next_focus.is_some() {
                    self.focused.set(next_focus);
                }
            }

            TreeTableEvent::FocusDown => {
                let next_focus = self.focused.with(|focused| match focused {
                    Some(TableFocus::Row(id)) => self.visible_rows.with(|rows| {
                        rows.iter()
                            .skip_while(|row| row.id != *id)
                            .nth(1)
                            .map(|row| TableFocus::Row(row.id.clone()))
                    }),
                    Some(TableFocus::Cell(id, col)) => self.visible_rows.with(|rows| {
                        rows.iter()
                            .skip_while(|row| row.id != *id)
                            .nth(1)
                            .map(|row| TableFocus::Cell(row.id.clone(), col.clone()))
                    }),
                    None => None,
                });

                if next_focus.is_some() {
                    self.focused.set(next_focus);
                }
            }
        });

        event.map(|cell_event: &TreeTableFirstCellEvent<Id>, _| {
            let TreeTableFirstCellEvent::Toggle(id, next) = cell_event;
            self.emit_toggle(cx, id.clone(), *next);
        });

        event.map(|table_event: &TableEvent<K>, _| match table_event {
            TableEvent::ToggleSort(col) => {
                let visible_rows = flatten_visible_rows(
                    &self.rows.get(),
                    &*self.row_id,
                    &*self.parent_id,
                    &self.expanded_row_ids.get(),
                );

                let column_cells = HashSet::from_iter(
                    visible_rows.iter().map(|row| Cell(row.id.clone(), col.clone())),
                );

                let current_selection = self.selection.get();
                let next_selection =
                    if current_selection == column_cells { HashSet::new() } else { column_cells };

                self.selection.set(next_selection.clone());

                if let Some(callback) = &self.on_select {
                    (callback)(cx, next_selection);
                }

                if let Some(callback) = &self.on_sort {
                    let current_direction =
                        sort_direction_for_column(self.sort_state.get().as_ref(), col);
                    let next_direction =
                        next_sort_direction(self.sort_cycle.get(), current_direction);
                    (callback)(cx, col.clone(), next_direction);
                }
            }

            _ => {}
        });
    }
}

pub trait VirtualTreeTableModifiers<Id, K = String>: Sized
where
    Id: Eq + Hash + Clone + Send + Sync + 'static,
    K: Eq + Hash + Clone + PartialEq + Send + Sync + 'static,
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

    fn selection_mode<U: Into<TableSelectionMode> + Clone + 'static>(
        self,
        mode: impl Res<U> + 'static,
    ) -> Self;

    fn treegrid_label<U: Into<Option<String>> + Clone + 'static>(
        self,
        label: impl Res<U> + 'static,
    ) -> Self;

    fn selection<R>(self, selection: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [Cell<Id, K>]> + Clone + 'static;

    fn expanded_row_ids<R>(self, expanded_row_ids: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [Id]> + Clone + 'static;

    fn on_sort<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, K, TableSortDirection) + Send + Sync;

    fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, HashSet<Cell<Id, K>>) + Send + Sync;

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
    K: Eq + Hash + Clone + PartialEq + Send + Sync + 'static,
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

    fn selection_mode<U: Into<TableSelectionMode> + Clone + 'static>(
        self,
        mode: impl Res<U> + 'static,
    ) -> Self {
        let mode = mode.to_signal(self.cx);
        self.bind(mode, move |handle| {
            let mode = mode.get().into();
            handle.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
                table.selection_mode.set(mode)
            });
        })
    }

    fn treegrid_label<U: Into<Option<String>> + Clone + 'static>(
        self,
        label: impl Res<U> + 'static,
    ) -> Self {
        let label = label.to_signal(self.cx);
        self.bind(label, move |handle| {
            let label = label.get().into();
            handle.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
                table.treegrid_label.set(label)
            });
        })
    }

    fn selection<R>(self, selection: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [Cell<Id, K>]> + Clone + 'static,
    {
        let selection = selection.to_signal(self.cx);
        self.bind(selection, move |handle| {
            let ids = selection.with(|ids| ids.deref().to_vec());
            handle.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
                table.selection.set(HashSet::from_iter(ids))
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

    fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, HashSet<Cell<Id, K>>) + Send + Sync,
    {
        self.modify(|table: &mut VirtualTreeTable<T, V, Id, H, K>| {
            table.on_select = Some(Box::new(callback))
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
