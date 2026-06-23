mod helpers;
use ego_tree::{NodeId, Tree};
use helpers::*;
use std::{collections::HashSet, fs, path::Path, time::SystemTime};
use vizia::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CheckState {
    Unchecked,
    Checked,
    Intermediate,
}

#[derive(Clone, PartialEq)]
struct FsNode {
    name: String,
    kind: String,
    size: String,
    check_state: CheckState,
    visible: bool,
}

struct AppData {
    tree: Signal<Tree<FsNode>>,
    selected_rows: Signal<Vec<NodeId>>,
    expanded_rows: Signal<Vec<NodeId>>,
    filter_text: Signal<String>,
    selectable: Signal<Selectable>,
    selection_follows_focus: Signal<bool>,
}

enum AppEvent {
    ToggleCheckState(NodeId),
    SelectRow(NodeId),
    ToggleRow(NodeId, bool),
    SetFilterText(String),
    ToggleMultiSelect,
    ToggleSelectionFollowsFocus,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleCheckState(id) => {
                self.tree.update(|tree| {
                    toggle_check_state(tree, *id);
                });
            }

            AppEvent::SelectRow(id) => {
                if self.selectable.get() == Selectable::Multi {
                    self.selected_rows.update(|rows| {
                        if rows.contains(id) {
                            rows.retain(|row_id| row_id != id);
                        } else {
                            rows.push(*id);
                        }
                    });
                } else {
                    self.selected_rows.set(vec![*id]);
                }
            }

            AppEvent::ToggleRow(id, expanded) => {
                self.expanded_rows.update(|rows| {
                    if *expanded {
                        if !rows.contains(id) {
                            rows.push(*id);
                        }
                    } else {
                        rows.retain(|current| *current != *id);
                    }
                });
            }

            AppEvent::SetFilterText(text) => {
                self.filter_text.set(text.clone());

                let query = text.trim().to_lowercase();
                let (include_set, expanded_rows) = self
                    .tree
                    .try_update(|tree| apply_filter_state(tree, &query))
                    .unwrap_or_else(|| (HashSet::new(), Vec::new()));

                self.selected_rows.update(|rows| rows.retain(|id| include_set.contains(id)));

                if query.is_empty() {
                    self.expanded_rows.update(|rows| rows.retain(|id| include_set.contains(id)));
                } else {
                    self.expanded_rows.set(expanded_rows);
                }
            }

            AppEvent::ToggleMultiSelect => {
                self.selectable.update(|selectable| match selectable {
                    Selectable::Single => {
                        *selectable = Selectable::Multi;
                        self.selection_follows_focus.set(false);
                    }

                    Selectable::Multi => {
                        *selectable = Selectable::Single;
                        self.selected_rows.update(|rows| rows.truncate(1));
                    }

                    Selectable::None => {
                        *selectable = Selectable::Single;
                    }
                });
            }

            AppEvent::ToggleSelectionFollowsFocus => {
                if self.selectable.get() == Selectable::Single {
                    self.selection_follows_focus.update(|value| *value ^= true);
                }
            }
        });
    }
}

fn node_matches_query(node: &FsNode, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    node.name.to_lowercase().contains(query)
}

fn apply_filter_state(tree: &mut Tree<FsNode>, query: &str) -> (HashSet<NodeId>, Vec<NodeId>) {
    let mut include_set = HashSet::new();
    let mut expanded_ids = Vec::new();
    let root_id = tree.root().id();

    fn visit(
        tree: &mut Tree<FsNode>,
        node_id: NodeId,
        query: &str,
        include_set: &mut HashSet<NodeId>,
        expanded_ids: &mut Vec<NodeId>,
    ) -> bool {
        let Some(node) = tree.get(node_id) else {
            return false;
        };

        let child_ids = node.children().map(|child| child.id()).collect::<Vec<_>>();
        let self_matches = node_matches_query(node.value(), query);
        let mut descendant_matches = false;

        for child_id in child_ids {
            descendant_matches |= visit(tree, child_id, query, include_set, expanded_ids);
        }

        let included = query.is_empty() || self_matches || descendant_matches;
        if included {
            include_set.insert(node_id);
            if descendant_matches {
                expanded_ids.push(node_id);
            }
        }

        if let Some(mut node) = tree.get_mut(node_id) {
            node.value().visible = included;
        }

        included
    }

    let child_ids = tree.root().children().map(|child| child.id()).collect::<Vec<_>>();
    let mut has_visible_descendant = false;
    for child_id in child_ids {
        has_visible_descendant |= visit(tree, child_id, query, &mut include_set, &mut expanded_ids);
    }

    include_set.insert(root_id);
    if !query.is_empty() && has_visible_descendant {
        expanded_ids.push(root_id);
    }

    if let Some(mut root) = tree.get_mut(root_id) {
        root.value().visible = true;
    }

    (include_set, expanded_ids)
}

fn aggregate_check_state(child_states: impl Iterator<Item = CheckState>) -> CheckState {
    let mut saw_checked = false;
    let mut saw_unchecked = false;

    for state in child_states {
        match state {
            CheckState::Checked => saw_checked = true,
            CheckState::Unchecked => saw_unchecked = true,
            CheckState::Intermediate => {
                saw_checked = true;
                saw_unchecked = true;
            }
        }

        if saw_checked && saw_unchecked {
            return CheckState::Intermediate;
        }
    }

    if saw_checked { CheckState::Checked } else { CheckState::Unchecked }
}

fn toggle_check_state(tree: &mut Tree<FsNode>, node_id: NodeId) {
    if tree.get(node_id).is_none() {
        return;
    }

    let next_checked = tree
        .get(node_id)
        .map(|node| node.value().check_state != CheckState::Checked)
        .unwrap_or(false);
    let target_state = if next_checked { CheckState::Checked } else { CheckState::Unchecked };

    let descendant_ids = tree
        .get(node_id)
        .map(|node| node.descendants().map(|descendant| descendant.id()).collect::<Vec<_>>())
        .unwrap_or_default();

    for descendant_id in descendant_ids {
        if let Some(mut descendant) = tree.get_mut(descendant_id) {
            descendant.value().check_state = target_state;
        }
    }

    let ancestor_ids = tree
        .get(node_id)
        .map(|node| node.ancestors().map(|ancestor| ancestor.id()).collect::<Vec<_>>())
        .unwrap_or_default();

    for ancestor_id in ancestor_ids {
        let next_state = match tree.get(ancestor_id) {
            Some(ancestor) => {
                aggregate_check_state(ancestor.children().map(|child| child.value().check_state))
            }
            None => continue,
        };

        if let Some(mut ancestor) = tree.get_mut(ancestor_id) {
            ancestor.value().check_state = next_state;
        }
    }
}

fn node_check_state(tree: &Tree<FsNode>, node_id: NodeId) -> CheckState {
    tree.get(node_id).map(|node| node.value().check_state).unwrap_or(CheckState::Unchecked)
}

fn format_size(bytes: u64) -> String {
    if bytes < 1_024 {
        format!("{} B", bytes)
    } else if bytes < 1_048_576 {
        format!("{:.1} KB", bytes as f64 / 1_024.0)
    } else if bytes < 1_073_741_824 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    }
}

fn format_modified(t: SystemTime) -> String {
    use std::time::UNIX_EPOCH;
    let secs = t.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let days = secs / 86400;
    let years_since_1970 = days / 365;
    let year = 1970 + years_since_1970;
    let remaining_days = days - years_since_1970 * 365;
    let month = (remaining_days / 30) + 1;
    let day = (remaining_days % 30) + 1;
    format!("{year}-{month:02}-{day:02}")
}

const MAX_SCAN_DEPTH: usize = 4;

fn should_skip_entry(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    matches!(name, ".git" | "target" | "node_modules") || name.starts_with("target")
}

fn add_dir_to_tree(tree: &mut Tree<FsNode>, parent_node: NodeId, root: &Path, depth: usize) {
    if depth >= MAX_SCAN_DEPTH {
        return;
    }

    let Ok(mut entries) = fs::read_dir(root) else { return };

    let mut dirs = Vec::new();
    let mut files = Vec::new();
    while let Some(Ok(entry)) = entries.next() {
        let path = entry.path();
        if should_skip_entry(&path) {
            continue;
        }

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => continue,
        };
        if file_type.is_symlink() {
            continue;
        }

        let meta = match fs::metadata(&path) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if meta.is_dir() {
            dirs.push((path, meta));
        } else {
            files.push((path, meta));
        }
    }
    dirs.sort_by(|a, b| a.0.file_name().cmp(&b.0.file_name()));
    files.sort_by(|a, b| a.0.file_name().cmp(&b.0.file_name()));

    for (path, meta) in dirs.into_iter().chain(files) {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?").to_string();
        let kind = if meta.is_dir() { "Folder".to_string() } else { "File".to_string() };
        let size = if meta.is_dir() { "—".to_string() } else { format_size(meta.len()) };
        let _ = meta.modified().map(format_modified).unwrap_or_else(|_| "—".to_string());

        let child_id = {
            let mut parent = tree.get_mut(parent_node).expect("tree parent node should exist");
            parent
                .append(FsNode {
                    name,
                    kind,
                    size,
                    check_state: CheckState::Unchecked,
                    visible: true,
                })
                .id()
        };

        if meta.is_dir() {
            add_dir_to_tree(tree, child_id, &path, depth + 1);
        }
    }
}

fn build_fs_tree() -> Tree<FsNode> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root_name =
        root.file_name().and_then(|name| name.to_str()).unwrap_or("workspace").to_string();
    let mut tree = Tree::new(FsNode {
        name: root_name,
        kind: "Folder".to_string(),
        size: "—".to_string(),
        check_state: CheckState::Unchecked,
        visible: true,
    });

    let root_id = tree.root().id();
    add_dir_to_tree(&mut tree, root_id, root, 0);
    tree
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let tree = Signal::new(build_fs_tree());
        let root_id = tree.with(|tree| tree.root().id());
        let selected_rows: Signal<Vec<NodeId>> = Signal::new(vec![]);
        let expanded_rows: Signal<Vec<NodeId>> = Signal::new(vec![root_id]);
        let filter_text = Signal::new(String::new());
        let selectable = Signal::new(Selectable::Single);
        let selection_follows_focus = Signal::new(false);
        let multiselect_enabled = Memo::new(move |_| selectable.get() == Selectable::Multi);
        let selection_follows_focus_disabled =
            Memo::new(move |_| selectable.get() != Selectable::Single);

        AppData {
            tree,
            selected_rows,
            expanded_rows,
            filter_text,
            selectable,
            selection_follows_focus,
        }
        .build(cx);

        ExamplePage::vertical(cx, move |cx| {
            Textbox::new(cx, filter_text).width(Pixels(320.0)).placeholder("Filter rows").on_edit(
                move |cx, text| {
                    cx.emit(AppEvent::SetFilterText(text));
                },
            );

            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Switch::new(cx, multiselect_enabled)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleMultiSelect))
                        .id("treeview_multiselect");
                    Label::new(cx, "Multi-select").describing("treeview_multiselect");
                })
                .size(Auto)
                .alignment(Alignment::Center)
                .gap(Pixels(6.0));

                HStack::new(cx, |cx| {
                    Switch::new(cx, selection_follows_focus)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleSelectionFollowsFocus))
                        .disabled(selection_follows_focus_disabled)
                        .id("treeview_selection_follows_focus");
                    Label::new(cx, "Selection follows focus (single-select)")
                        .describing("treeview_selection_follows_focus");
                })
                .size(Auto)
                .alignment(Alignment::Center)
                .gap(Pixels(6.0));
            })
            .size(Auto)
            .alignment(Alignment::Left)
            .gap(Pixels(16.0));

            TreeView::from_hierarchy(
                cx,
                tree,
                move |tree: &Tree<FsNode>| vec![tree.root().id()],
                move |tree: &Tree<FsNode>, parent_id: &NodeId| {
                    tree.get(*parent_id)
                        .map(|node| node.children().map(|child| child.id()).collect::<Vec<_>>())
                        .unwrap_or_default()
                },
                |tree: &Tree<FsNode>, node_id: &NodeId| {
                    if *node_id == tree.root().id() {
                        true
                    } else {
                        tree.get(*node_id).map(|node| node.value().visible).unwrap_or(false)
                    }
                },
                move |cx, row| {
                    let row_id = row.get().id;
                    let text = tree.map({
                        let node_id = row_id;
                        move |tree| {
                            tree.get(node_id)
                                .map(|node| node.value().name.clone())
                                .unwrap_or_default()
                        }
                    });
                    let checked = tree.map({
                        let node_id = row_id;
                        move |tree| node_check_state(tree, node_id) == CheckState::Checked
                    });
                    let intermediate = tree.map({
                        let node_id = row_id;
                        move |tree| node_check_state(tree, node_id) == CheckState::Intermediate
                    });

                    HStack::new(cx, move |cx| {
                        Checkbox::intermediate(cx, checked, intermediate)
                            .on_toggle({
                                let node_id = row_id;
                                move |cx| {
                                    cx.emit(AppEvent::ToggleCheckState(node_id));
                                }
                            })
                            .navigable(false)
                            .pointer_events(PointerEvents::Auto);
                        Label::new(cx, text).text_wrap(false).hoverable(false);
                    })
                    .pointer_events(PointerEvents::None)
                    .height(Auto)
                    .alignment(Alignment::Left)
                    .gap(Pixels(8.0));
                },
            )
            .selectable(selectable)
            .selection_follows_focus(selection_follows_focus)
            .selected_row_ids(selected_rows)
            .expanded_row_ids(expanded_rows)
            .checked_row(move |row| {
                tree.with(|tree| {
                    tree.get(row.id)
                        .map(|node| node.value().check_state != CheckState::Unchecked)
                        .unwrap_or(false)
                })
            })
            .type_ahead_text(move |row| {
                tree.with(|tree| tree.get(row.id).map(|node| node.value().name.clone()))
            })
            .on_row_select(move |cx, id| {
                cx.emit(AppEvent::SelectRow(id));
            })
            .on_row_check_toggle(move |cx, id| {
                cx.emit(AppEvent::ToggleCheckState(id));
            })
            .on_row_toggle(move |cx, id, expanded| {
                cx.emit(AppEvent::ToggleRow(id, expanded));
            })
            .width(Stretch(1.0))
            .height(Pixels(360.0));
        });
    })
    .run()
}
