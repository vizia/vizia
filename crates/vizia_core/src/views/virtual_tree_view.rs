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

fn focused_visible_index<T, Id>(
    visible_rows: &[TreeTableRow<T, Id>],
    focused_row_id: Option<&Id>,
    selected_row_ids: &[Id],
) -> Option<usize>
where
    T: PartialEq + Clone + 'static,
    Id: Clone + Eq + Hash + 'static,
{
    if let Some(focused_row_id) = focused_row_id {
        if let Some(index) = visible_rows.iter().position(|row| &row.id == focused_row_id) {
            return Some(index);
        }
    }

    visible_rows
        .iter()
        .position(|row| selected_row_ids.iter().any(|selected_id| selected_id == &row.id))
}

enum VirtualTreeViewEvent<Id> {
    SelectRow(usize),
    FocusRow(Id),
    SelectFocused,
    ToggleCheckedFocused,
    ExpandFocused,
    CollapseFocused,
    ToggleRow(Id, bool),
}

type VirtualTreeViewItemContent<T, Id> = dyn Fn(&mut Context, Memo<TreeTableRow<T, Id>>);
type VirtualTreeViewTypeAheadText<T> = dyn Fn(&T) -> Option<String>;
type VirtualTreeViewCheckedRow<T> = dyn Fn(&T) -> bool;

pub struct VirtualTreeView<T, V, Id>
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
    focused_row_id: Signal<Option<Id>>,
    list_entity: Signal<Entity>,
    checked_row: Signal<Option<Rc<VirtualTreeViewCheckedRow<T>>>>,
    type_ahead_text: Signal<Option<Rc<VirtualTreeViewTypeAheadText<T>>>>,
    on_row_select: Option<Box<dyn Fn(&mut EventContext, Id)>>,
    on_row_focus: Option<Box<dyn Fn(&mut EventContext, Id)>>,
    on_row_check_toggle: Option<Box<dyn Fn(&mut EventContext, Id)>>,
    on_row_toggle: Option<Box<dyn Fn(&mut EventContext, Id, bool)>>,
}

impl<T, V, Id> VirtualTreeView<T, V, Id>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
{
    pub fn new<S, U, F>(
        cx: &mut Context,
        tree: S,
        item_height: f32,
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
        let item_content: Rc<VirtualTreeViewItemContent<T, Id>> = Rc::new(item_content);
        let item_content_signal = Signal::new(item_content.clone());

        let selectable = Signal::new(Selectable::None);
        let selection_follows_focus = Signal::new(false);
        let selected_row_ids = Signal::new(Vec::new());
        let expanded_row_ids = Signal::new(Vec::new());
        let focused_row_id = Signal::new(None);
        let list_entity = Signal::new(Entity::null());
        let checked_row = Signal::new(None);
        let type_ahead_text = Signal::new(None);

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

        let focused_index = Memo::new(move |_| {
            visible_rows.with(|rows| {
                let focused_row_id = focused_row_id.get();
                selected_row_ids.with(|selected_ids| {
                    focused_visible_index(rows, focused_row_id.as_ref(), selected_ids)
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
            focused_row_id,
            list_entity,
            checked_row,
            type_ahead_text,
            on_row_select: None,
            on_row_focus: None,
            on_row_check_toggle: None,
            on_row_toggle: None,
        }
        .build(cx, move |cx| {
            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::Enter),
                    KeymapEntry::new("Select Focused", |cx| {
                        cx.emit(VirtualTreeViewEvent::<Id>::SelectFocused)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Space),
                    KeymapEntry::new("Toggle Checked Focused", |cx| {
                        cx.emit(VirtualTreeViewEvent::<Id>::ToggleCheckedFocused)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowRight),
                    KeymapEntry::new("Expand Focused", |cx| {
                        cx.emit(VirtualTreeViewEvent::<Id>::ExpandFocused)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowLeft),
                    KeymapEntry::new("Collapse Focused", |cx| {
                        cx.emit(VirtualTreeViewEvent::<Id>::CollapseFocused)
                    }),
                ),
            ])
            .build(cx);

            let list = VirtualList::new_custom_items_with_selection(
                cx,
                visible_rows,
                item_height,
                move |cx, row_index, row, is_selected| {
                    let row: Memo<TreeTableRow<T, Id>> = Memo::new(move |_| row.get());
                    let row_data = row.get();
                    let has_children = row_data.has_children;
                    let expanded = row_data.expanded;
                    let row_id = row_data.id.clone();
                    let checked_row_callback = checked_row.get();
                    let indent = row_data.depth as f32 * 16.0;
                    let item_content = item_content_signal.get();
                    let row_for_size_of_set = row;
                    let size_of_set = Memo::new(move |_| {
                        let row_data = row_for_size_of_set.get();
                        visible_rows.with(|rows| {
                            rows.iter()
                                .filter(|candidate| candidate.parent_id == row_data.parent_id)
                                .count()
                        })
                    });
                    let row_for_position = row;
                    let position_in_set = Memo::new(move |_| {
                        let row_data = row_for_position.get();
                        visible_rows.with(|rows| {
                            rows.iter()
                                .filter(|candidate| candidate.parent_id == row_data.parent_id)
                                .position(|candidate| candidate.id == row_data.id)
                                .map(|position| position + 1)
                                .unwrap_or(1)
                        })
                    });

                    let is_checked = if let Some(checked_row_callback) = checked_row_callback {
                        row.map(move |value| checked_row_callback(&value.row))
                    } else {
                        is_selected
                    };

                    let row_handle = HStack::new(cx, move |cx| {
                        Element::new(cx)
                            .class("tree-view-indent")
                            .width(Pixels(indent))
                            .height(Stretch(1.0))
                            .pointer_events(PointerEvents::None);

                        if has_children {
                            let icon =
                                if expanded { ICON_CHEVRON_DOWN } else { ICON_CHEVRON_RIGHT };
                            Button::new(cx, move |cx| Svg::new(cx, icon).text_wrap(false))
                                .variant(ButtonVariant::Text)
                                .class("tree-view-disclosure")
                                .navigable(false)
                                .on_press({
                                    let row_id = row_id.clone();
                                    move |cx| {
                                        cx.emit(VirtualTreeViewEvent::ToggleRow(
                                            row_id.clone(),
                                            !expanded,
                                        ));
                                    }
                                });
                        } else {
                            Element::new(cx)
                                .class("tree-view-disclosure-placeholder")
                                .pointer_events(PointerEvents::None);
                        }

                        HStack::new(cx, move |cx| {
                            item_content(cx, row);
                        })
                        .class("tree-view-row-content")
                        .width(Stretch(1.0))
                        .min_width(Auto)
                        .height(Auto)
                        .pointer_events(PointerEvents::None);
                    })
                    .class("tree-view-row")
                    .toggle_class("odd", row_index % 2 == 1)
                    .toggle_class("even", row_index % 2 == 0)
                    .toggle_class("selected", is_selected)
                    .toggle_class("expanded", row.map(|value| value.expanded))
                    .toggle_class("collapsible", row.map(|value| value.has_children))
                    .alignment(Alignment::Left)
                    .height(Auto)
                    .width(Stretch(1.0))
                    .min_width(Auto)
                    .role(Role::TreeItem)
                    .accessibility_selected(is_selected)
                    .checked(is_checked)
                    .level(row.map(|value| value.depth + 1))
                    .size_of_set(size_of_set)
                    .position_in_set(position_in_set)
                    .on_mouse_down(move |cx, button| {
                        if button == MouseButton::Left {
                            cx.emit(ListEvent::Select(row_index));
                        }
                    });

                    if has_children {
                        row_handle.expanded(row.map(|value| value.expanded))
                    } else {
                        row_handle
                    }
                },
            )
            .width(Stretch(1.0))
            .height(Stretch(1.0))
            .class("tree-view-body")
            .selection(selected_indices)
            .focused_index(focused_index)
            .on_focus(move |cx, index| {
                visible_rows.with(|rows| {
                    if let Some(row) = rows.get(index) {
                        focused_row_id.set(Some(row.id.clone()));
                        cx.emit(VirtualTreeViewEvent::FocusRow(row.id.clone()));
                    }
                });
            })
            .selectable(selectable)
            .selection_follows_focus(selection_follows_focus)
            .space_selects_focused(false)
            .type_ahead_text(move |_cx, index| {
                let text_fn = type_ahead_text.get()?;
                visible_rows.with(|rows| rows.get(index).and_then(|row| text_fn(&row.row)))
            })
            .on_select(move |cx, index| cx.emit(VirtualTreeViewEvent::<Id>::SelectRow(index)))
            .role(Role::Tree);

            list_entity.set(list.entity());
        })
        .navigable(false);

        let flatten_rows_for_bind = flatten_rows.clone();
        handle.bind(tree_signal, move |handle| {
            let rows = tree_signal.with(|tree| flatten_rows_for_bind(tree));
            handle.modify(|tree_view: &mut VirtualTreeView<T, V, Id>| tree_view.rows.set(rows));
        })
    }

    pub fn from_rows<S>(
        cx: &mut Context,
        rows: S,
        item_height: f32,
        row_id: impl Fn(&T) -> Id + 'static,
        parent_id: impl Fn(&T) -> Option<Id> + 'static,
        item_content: impl Fn(&mut Context, Memo<TreeTableRow<T, Id>>) + 'static,
    ) -> Handle<Self>
    where
        S: Res<V> + 'static,
    {
        Self::new(cx, rows, item_height, |rows: &V| rows.clone(), row_id, parent_id, item_content)
    }

    fn emit_toggle(&self, cx: &mut EventContext, row_id: Id, next_expanded: bool) {
        if let Some(callback) = &self.on_row_toggle {
            (callback)(cx, row_id, next_expanded);
        }
    }

    fn emit_select(&self, cx: &mut EventContext, row_id: Id) {
        if let Some(callback) = &self.on_row_select {
            (callback)(cx, row_id);
        }
    }

    fn emit_focus(&self, cx: &mut EventContext, row_id: Id) {
        if let Some(callback) = &self.on_row_focus {
            (callback)(cx, row_id);
        }
    }

    fn emit_check_toggle(&self, cx: &mut EventContext, row_id: Id) {
        if let Some(callback) = &self.on_row_check_toggle {
            (callback)(cx, row_id);
        }
    }

    fn visible_rows(&self) -> Vec<TreeTableRow<T, Id>> {
        flatten_visible_rows(
            &self.rows.get(),
            &*self.row_id,
            &*self.parent_id,
            &self.expanded_row_ids.get(),
        )
    }

    fn focused_or_selected_visible_row(&self) -> Option<TreeTableRow<T, Id>> {
        let visible_rows = self.visible_rows();

        if let Some(focused_id) = self.focused_row_id.get() {
            if let Some(row) = visible_rows.iter().find(|row| row.id == focused_id) {
                return Some(row.clone());
            }
        }

        let selected_id = self.selected_row_ids.get().first().cloned()?;
        visible_rows.into_iter().find(|row| row.id == selected_id)
    }

    fn focus_row_id(&self, cx: &mut EventContext, row_id: Id) {
        let visible_rows = self.visible_rows();
        if let Some(index) = visible_rows.iter().position(|row| row.id == row_id) {
            let list_entity = self.list_entity.get();
            if list_entity != Entity::null() {
                cx.emit_to(list_entity, ListEvent::Focus(index));
                self.focused_row_id.set(Some(row_id));
            }
        }
    }
}

impl<Id> VirtualTreeView<TreeNodeRow<Id>, Vec<TreeNodeRow<Id>>, Id>
where
    Id: Eq + Hash + Clone + Send + Sync + 'static,
{
    pub fn from_hierarchy<S, U>(
        cx: &mut Context,
        tree: S,
        item_height: f32,
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
            item_height,
            move |tree: &U| flatten_hierarchy_rows(tree, &*root_ids, &*child_ids, &*is_visible),
            |row: &TreeNodeRow<Id>| row.id.clone(),
            |row: &TreeNodeRow<Id>| row.parent_id.clone(),
            item_content,
        )
    }
}

impl<T, V, Id> View for VirtualTreeView<T, V, Id>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
{
    fn element(&self) -> Option<&'static str> {
        Some("virtual-tree-view")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|tree_event: &VirtualTreeViewEvent<Id>, _| match tree_event {
            VirtualTreeViewEvent::SelectRow(index) => {
                let visible_rows = self.visible_rows();

                if let Some(row) = visible_rows.get(*index) {
                    self.emit_select(cx, row.id.clone());
                }
            }

            VirtualTreeViewEvent::SelectFocused => {
                if let Some(row) = self.focused_or_selected_visible_row() {
                    self.emit_select(cx, row.id);
                }
            }

            VirtualTreeViewEvent::FocusRow(row_id) => {
                self.emit_focus(cx, row_id.clone());
            }

            VirtualTreeViewEvent::ToggleCheckedFocused => {
                if let Some(row) = self.focused_or_selected_visible_row() {
                    self.emit_check_toggle(cx, row.id);
                }
            }

            VirtualTreeViewEvent::ExpandFocused => {
                if let Some(row) = self.focused_or_selected_visible_row() {
                    if row.has_children && !row.expanded {
                        self.emit_toggle(cx, row.id, true);
                    } else if row.has_children {
                        let child_id = self
                            .visible_rows()
                            .into_iter()
                            .find(|candidate| candidate.parent_id.as_ref() == Some(&row.id))
                            .map(|candidate| candidate.id);

                        if let Some(child_id) = child_id {
                            self.focus_row_id(cx, child_id);
                        }
                    }
                }
            }

            VirtualTreeViewEvent::CollapseFocused => {
                if let Some(row) = self.focused_or_selected_visible_row() {
                    if row.has_children && row.expanded {
                        self.emit_toggle(cx, row.id, false);
                    } else if let Some(parent_id) = row.parent_id {
                        self.focus_row_id(cx, parent_id);
                    }
                }
            }

            VirtualTreeViewEvent::ToggleRow(row_id, next) => {
                self.emit_toggle(cx, row_id.clone(), *next);
            }
        });
    }
}

pub trait VirtualTreeViewModifiers<Id>: Sized
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

    fn on_row_focus<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id);

    fn on_row_check_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id);

    fn on_row_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id, bool);
}

impl<T, V, Id> VirtualTreeViewModifiers<Id> for Handle<'_, VirtualTreeView<T, V, Id>>
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
            handle.modify(|tree_view: &mut VirtualTreeView<T, V, Id>| {
                tree_view.selectable.set(selectable)
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
            handle.modify(|tree_view: &mut VirtualTreeView<T, V, Id>| {
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
            handle.modify(|tree_view: &mut VirtualTreeView<T, V, Id>| {
                tree_view.selected_row_ids.set(ids)
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
            handle.modify(|tree_view: &mut VirtualTreeView<T, V, Id>| {
                tree_view.expanded_row_ids.set(ids)
            });
        })
    }

    fn on_row_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id),
    {
        self.modify(|tree_view: &mut VirtualTreeView<T, V, Id>| {
            tree_view.on_row_select = Some(Box::new(callback))
        })
    }

    fn on_row_focus<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id),
    {
        self.modify(|tree_view: &mut VirtualTreeView<T, V, Id>| {
            tree_view.on_row_focus = Some(Box::new(callback))
        })
    }

    fn on_row_check_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id),
    {
        self.modify(|tree_view: &mut VirtualTreeView<T, V, Id>| {
            tree_view.on_row_check_toggle = Some(Box::new(callback))
        })
    }

    fn on_row_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, Id, bool),
    {
        self.modify(|tree_view: &mut VirtualTreeView<T, V, Id>| {
            tree_view.on_row_toggle = Some(Box::new(callback))
        })
    }
}

impl<T, V, Id> Handle<'_, VirtualTreeView<T, V, Id>>
where
    V: Deref<Target = [T]> + Clone + 'static,
    T: PartialEq + Clone + 'static,
    Id: Eq + Hash + Clone + Send + Sync + 'static,
{
    pub fn type_ahead_text<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&T) -> Option<String>,
    {
        let callback: Rc<VirtualTreeViewTypeAheadText<T>> = Rc::new(callback);
        self.modify(|tree_view: &mut VirtualTreeView<T, V, Id>| {
            tree_view.type_ahead_text.set(Some(callback.clone()))
        })
    }

    pub fn checked_row<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&T) -> bool,
    {
        let callback: Rc<VirtualTreeViewCheckedRow<T>> = Rc::new(callback);
        self.modify(|tree_view: &mut VirtualTreeView<T, V, Id>| {
            tree_view.checked_row.set(Some(callback.clone()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_row(id: i32) -> TreeTableRow<i32, i32> {
        TreeTableRow {
            row: id,
            id,
            parent_id: None,
            depth: 0,
            has_children: false,
            expanded: false,
        }
    }

    #[test]
    fn focused_visible_index_prefers_focused_row() {
        let visible_rows = vec![make_row(1), make_row(2), make_row(3)];

        assert_eq!(focused_visible_index(&visible_rows, Some(&2), &[1, 3]), Some(1));
    }

    #[test]
    fn focused_visible_index_falls_back_to_first_visible_selected_row() {
        let visible_rows = vec![make_row(4), make_row(5), make_row(6)];

        assert_eq!(focused_visible_index(&visible_rows, Some(&9), &[6, 4]), Some(0));
    }

    #[test]
    fn focused_visible_index_returns_none_when_nothing_is_visible() {
        let visible_rows = vec![make_row(7), make_row(8)];

        assert_eq!(focused_visible_index(&visible_rows, None, &[9]), None);
    }
}
