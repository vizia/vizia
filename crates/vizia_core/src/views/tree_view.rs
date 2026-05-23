use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

use crate::{
    icons::{ICON_CHEVRON_DOWN, ICON_CHEVRON_RIGHT},
    prelude::*,
};

use super::tree_table::{TreeNodeRow, TreeTableRow};

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

enum TreeViewEvent<Id> {
    SelectRow(usize),
    ExpandSelected,
    CollapseSelected,
    ToggleRow(Id, bool),
}

type TreeViewItemContent<T, Id> = dyn Fn(&mut Context, Memo<TreeTableRow<T, Id>>);

pub struct TreeView<T, V, Id>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
{
    rows: Signal<V>,
    row_id: Rc<dyn Fn(&T) -> Id>,
    parent_id: Rc<dyn Fn(&T) -> Option<Id>>,
    selectable: Signal<Selectable>,
    selection_follows_focus: Signal<bool>,
    selected_row_ids: Signal<Vec<Id>>,
    expanded_row_ids: Signal<Vec<Id>>,
    on_row_select: Option<Box<dyn Fn(&mut EventContext, Id)>>,
    on_row_toggle: Option<Box<dyn Fn(&mut EventContext, Id, bool)>>,
}

impl<T, V, Id> TreeView<T, V, Id>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
{
    pub fn new<S, U, F>(
        cx: &mut Context,
        tree: S,
        flatten_rows: F,
        row_id: impl Fn(&T) -> Id + 'static,
        parent_id: impl Fn(&T) -> Option<Id> + 'static,
        item_content: impl Fn(&mut Context, Memo<TreeTableRow<T, Id>>) + 'static,
    ) -> Handle<Self>
    where
        S: Res<U> + 'static,
        U: Clone + 'static,
        F: Fn(&U) -> V + 'static,
    {
        let tree_signal = tree.to_signal(cx);
        let flatten_rows: Rc<dyn Fn(&U) -> V> = Rc::new(flatten_rows);
        let row_signal = Signal::new(tree_signal.with(|tree| flatten_rows(tree)));
        let row_id: Rc<dyn Fn(&T) -> Id> = Rc::new(row_id);
        let parent_id: Rc<dyn Fn(&T) -> Option<Id>> = Rc::new(parent_id);
        let item_content: Rc<TreeViewItemContent<T, Id>> = Rc::new(item_content);

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

        let selected_indices = Memo::new(move |_| {
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

        let handle = Self {
            rows: row_signal,
            row_id,
            parent_id,
            selectable,
            selection_follows_focus,
            selected_row_ids,
            expanded_row_ids,
            on_row_select: None,
            on_row_toggle: None,
        }
        .build(cx, move |cx| {
            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowRight),
                    KeymapEntry::new("Expand Selected", |cx| {
                        cx.emit(TreeViewEvent::<Id>::ExpandSelected)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowLeft),
                    KeymapEntry::new("Collapse Selected", |cx| {
                        cx.emit(TreeViewEvent::<Id>::CollapseSelected)
                    }),
                ),
            ])
            .build(cx);

            List::new(cx, visible_rows, move |cx, row_index, row| {
                let row: Memo<TreeTableRow<T, Id>> = Memo::new(move |_| row.get());
                let row_data = row.get();
                let has_children = row_data.has_children;
                let expanded = row_data.expanded;
                let row_id = row_data.id.clone();
                let indent = row_data.depth as f32 * 16.0;
                let item_content = item_content.clone();

                HStack::new(cx, move |cx| {
                    Element::new(cx)
                        .class("tree-view-indent")
                        .width(Pixels(indent))
                        .height(Stretch(1.0));

                    if has_children {
                        let icon = if expanded { ICON_CHEVRON_DOWN } else { ICON_CHEVRON_RIGHT };
                        Button::new(cx, move |cx| Svg::new(cx, icon).text_wrap(false))
                            .variant(ButtonVariant::Text)
                            .class("tree-view-disclosure")
                            .on_press({
                                let row_id = row_id.clone();
                                move |cx| {
                                    cx.emit(TreeViewEvent::ToggleRow(row_id.clone(), !expanded));
                                }
                            });
                    } else {
                        Element::new(cx)
                            .class("tree-view-disclosure-placeholder");
                    }

                    HStack::new(cx, move |cx| {
                        item_content(cx, row);
                    })
                    .class("tree-view-row-content")
                    .width(Stretch(1.0))
                    .min_width(Auto)
                    .height(Auto);
                })
                .class("tree-view-row")
                .toggle_class("odd", row_index % 2 == 1)
                .toggle_class("even", row_index % 2 == 0)
                .toggle_class("expanded", row.map(|value| value.expanded))
                .toggle_class("collapsible", row.map(|value| value.has_children))
                .alignment(Alignment::Left)
                .height(Auto)
                .width(Stretch(1.0))
                .min_width(Auto);

                Divider::new(cx).width(Stretch(1.0));
            })
            .width(Stretch(1.0))
            .height(Stretch(1.0))
            .class("tree-view-body")
            .selection(selected_indices)
            .selectable(selectable)
            .selection_follows_focus(selection_follows_focus)
            .on_select(move |cx, index| cx.emit(TreeViewEvent::<Id>::SelectRow(index)));
        })
        .class("tree-view")
        .navigable(true);

        let flatten_rows_for_bind = flatten_rows.clone();
        handle.bind(tree_signal, move |handle| {
            let rows = tree_signal.with(|tree| flatten_rows_for_bind(tree));
            handle.modify(|tree_view: &mut TreeView<T, V, Id>| tree_view.rows.set(rows));
        })
    }

    pub fn from_rows<S>(
        cx: &mut Context,
        rows: S,
        row_id: impl Fn(&T) -> Id + 'static,
        parent_id: impl Fn(&T) -> Option<Id> + 'static,
        item_content: impl Fn(&mut Context, Memo<TreeTableRow<T, Id>>) + 'static,
    ) -> Handle<Self>
    where
        S: Res<V> + 'static,
    {
        Self::new(cx, rows, |rows: &V| rows.clone(), row_id, parent_id, item_content)
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

impl<Id> TreeView<TreeNodeRow<Id>, Vec<TreeNodeRow<Id>>, Id>
where
    Id: Eq + Hash + Clone + Send + Sync + 'static,
{
    pub fn from_hierarchy<S, U>(
        cx: &mut Context,
        tree: S,
        root_ids: impl Fn(&U) -> Vec<Id> + 'static,
        child_ids: impl Fn(&U, &Id) -> Vec<Id> + 'static,
        is_visible: impl Fn(&U, &Id) -> bool + 'static,
        item_content: impl Fn(&mut Context, Memo<TreeTableRow<TreeNodeRow<Id>, Id>>) + 'static,
    ) -> Handle<Self>
    where
        S: Res<U> + 'static,
        U: Clone + 'static,
    {
        let root_ids: Rc<dyn Fn(&U) -> Vec<Id>> = Rc::new(root_ids);
        let child_ids: Rc<dyn Fn(&U, &Id) -> Vec<Id>> = Rc::new(child_ids);
        let is_visible: Rc<dyn Fn(&U, &Id) -> bool> = Rc::new(is_visible);

        Self::new(
            cx,
            tree,
            move |tree: &U| flatten_hierarchy_rows(tree, &*root_ids, &*child_ids, &*is_visible),
            |row: &TreeNodeRow<Id>| row.id.clone(),
            |row: &TreeNodeRow<Id>| row.parent_id.clone(),
            item_content,
        )
    }
}

impl<T, V, Id> View for TreeView<T, V, Id>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
{
    fn element(&self) -> Option<&'static str> {
        Some("tree-view")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|tree_event: &TreeViewEvent<Id>, _| match tree_event {
            TreeViewEvent::SelectRow(index) => {
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

            TreeViewEvent::ExpandSelected => {
                if let Some(row) = self.selected_visible_row() {
                    if row.has_children && !row.expanded {
                        self.emit_toggle(cx, row.id, true);
                    }
                }
            }

            TreeViewEvent::CollapseSelected => {
                if let Some(row) = self.selected_visible_row() {
                    if row.has_children && row.expanded {
                        self.emit_toggle(cx, row.id, false);
                    } else if let Some(parent_id) = row.parent_id {
                        self.emit_toggle(cx, parent_id, false);
                    }
                }
            }

            TreeViewEvent::ToggleRow(row_id, next) => {
                self.emit_toggle(cx, row_id.clone(), *next);
            }
        });
    }
}

pub trait TreeViewModifiers<Id>: Sized
where
    Id: Eq + Hash + Clone + Send + Sync + 'static,
{
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

    fn on_row_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id);

    fn on_row_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id, bool);
}

impl<T, V, Id> TreeViewModifiers<Id> for Handle<'_, TreeView<T, V, Id>>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
{
    fn selectable<U: Into<Selectable> + Clone + 'static>(
        self,
        selectable: impl Res<U> + 'static,
    ) -> Self {
        let selectable = selectable.to_signal(self.cx);
        self.bind(selectable, move |handle| {
            let selectable = selectable.get().into();
            handle.modify(|tree_view: &mut TreeView<T, V, Id>| tree_view.selectable.set(selectable));
        })
    }

    fn selection_follows_focus<U: Into<bool> + Clone + 'static>(
        self,
        flag: impl Res<U> + 'static,
    ) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let flag = flag.get().into();
            handle.modify(|tree_view: &mut TreeView<T, V, Id>| {
                tree_view.selection_follows_focus.set(flag)
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
            handle.modify(|tree_view: &mut TreeView<T, V, Id>| tree_view.selected_row_ids.set(ids));
        })
    }

    fn expanded_row_ids<R>(self, expanded_row_ids: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [Id]> + Clone + 'static,
    {
        let expanded_row_ids = expanded_row_ids.to_signal(self.cx);
        self.bind(expanded_row_ids, move |handle| {
            let ids = expanded_row_ids.with(|ids| ids.deref().to_vec());
            handle.modify(|tree_view: &mut TreeView<T, V, Id>| tree_view.expanded_row_ids.set(ids));
        })
    }

    fn on_row_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id),
    {
        self.modify(|tree_view: &mut TreeView<T, V, Id>| {
            tree_view.on_row_select = Some(Box::new(callback))
        })
    }

    fn on_row_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id, bool),
    {
        self.modify(|tree_view: &mut TreeView<T, V, Id>| {
            tree_view.on_row_toggle = Some(Box::new(callback))
        })
    }
}
