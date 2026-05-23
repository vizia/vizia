mod helpers;
use ego_tree::{NodeId, Tree};
use helpers::*;
use std::{cmp::Ordering, collections::HashSet, fs, path::Path, time::SystemTime};
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
    size_bytes: Option<u64>,
    modified: String,
    check_state: CheckState,
    visible: bool,
}

struct AppData {
    tree: Signal<Tree<FsNode>>,
    sort_state: Signal<Option<TableSortState>>,
    selected_rows: Signal<Vec<NodeId>>,
    expanded_rows: Signal<Vec<NodeId>>,
    filter_text: Signal<String>,
}

enum AppEvent {
    ToggleCheckState(NodeId),
    SetSort(String, TableSortDirection),
    SelectRow(NodeId),
    ToggleRow(NodeId, bool),
    SetFilterText(String),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleCheckState(id) => {
                self.tree.update(|tree| {
                    toggle_check_state(tree, *id);
                });
            }

            AppEvent::SetSort(key, direction) => {
                let next_sort_state =
                    Some(TableSortState { key: key.clone(), direction: *direction });
                self.sort_state.set(next_sort_state.clone());

                let tree = self.tree.get();
                self.tree.set(tree);
            }

            AppEvent::SelectRow(id) => {
                self.selected_rows.set(vec![*id]);
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
                let (include_set, expanded_ids) = self
                    .tree
                    .try_update(|tree| apply_filter_state(tree, &query))
                    .unwrap_or_else(|| (HashSet::new(), Vec::new()));

                self.selected_rows.update(|rows| rows.retain(|id| include_set.contains(id)));

                if query.is_empty() {
                    self.expanded_rows.update(|rows| rows.retain(|id| include_set.contains(id)));
                } else {
                    self.expanded_rows.set(expanded_ids);
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

fn compare_nodes(left: &FsNode, right: &FsNode, sort_state: Option<&TableSortState>) -> Ordering {
    let Some(sort_state) = sort_state else {
        return Ordering::Equal;
    };

    let key_order = match sort_state.key.as_str() {
        "name" => left.name.cmp(&right.name),
        "kind" => left.kind.cmp(&right.kind),
        "size" => left.size_bytes.cmp(&right.size_bytes),
        "modified" => left.modified.cmp(&right.modified),
        _ => Ordering::Equal,
    };

    match sort_state.direction {
        TableSortDirection::Ascending => key_order,
        TableSortDirection::Descending => key_order.reverse(),
        TableSortDirection::None => Ordering::Equal,
    }
}

fn sorted_child_ids(
    tree: &Tree<FsNode>,
    parent_id: NodeId,
    sort_state: Option<&TableSortState>,
) -> Vec<NodeId> {
    let mut child_ids = tree
        .get(parent_id)
        .map(|node| node.children().map(|child| child.id()).collect::<Vec<_>>())
        .unwrap_or_default();

    child_ids.sort_by(|left_id, right_id| {
        let left = tree.get(*left_id).map(|node| node.value());
        let right = tree.get(*right_id).map(|node| node.value());
        match (left, right) {
            (Some(left), Some(right)) => compare_nodes(left, right, sort_state),
            _ => Ordering::Equal,
        }
    });

    child_ids
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
    // Simple YYYY-MM-DD from seconds since epoch (no external crate)
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

    // Collect and sort: directories first, then files, both alphabetically
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
        let (kind, size) = if meta.is_dir() {
            ("Folder".to_string(), "—".to_string())
        } else {
            ("File".to_string(), format_size(meta.len()))
        };
        let size_bytes = if meta.is_dir() { None } else { Some(meta.len()) };

        let modified = meta.modified().map(format_modified).unwrap_or_else(|_| "—".to_string());

        let child_id = {
            let mut parent = tree.get_mut(parent_node).expect("tree parent node should exist");
            parent
                .append(FsNode {
                    name,
                    kind,
                    size,
                    size_bytes,
                    modified,
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
    // Walk the vizia workspace root (one directory up from examples/)
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root_name =
        root.file_name().and_then(|name| name.to_str()).unwrap_or("workspace").to_string();
    let mut tree = Tree::new(FsNode {
        name: root_name,
        kind: "Folder".to_string(),
        size: "—".to_string(),
        size_bytes: None,
        modified: "—".to_string(),
        check_state: CheckState::Unchecked,
        visible: true,
    });

    let root_id = tree.root().id();
    add_dir_to_tree(&mut tree, root_id, root, 0);
    tree
}

fn columns(
    tree: Signal<Tree<FsNode>>,
) -> Vec<TreeTableColumn<TreeNodeRow<NodeId>, NodeId, TableHeader>> {
    vec![
        TreeTableColumn::new(
            "name",
            |cx, sort_dir| TableHeader::new(cx, "Name", sort_dir),
            move |cx, row| {
                let row_for_first_cell: TreeTableRow<TreeNodeRow<NodeId>, NodeId> = row.get();

                TreeTableFirstCell::new(cx, row_for_first_cell, move |cx, row| {
                    let row_id = row.id;
                    let text = tree.map({
                        let node_id = row_id.clone();
                        move |tree| {
                            tree.get(node_id)
                                .map(|node| node.value().name.clone())
                                .unwrap_or_default()
                        }
                    });
                    let checked = tree.map({
                        let node_id = row_id.clone();
                        move |tree| node_check_state(tree, node_id) == CheckState::Checked
                    });
                    let intermediate = tree.map({
                        let node_id = row_id.clone();
                        move |tree| node_check_state(tree, node_id) == CheckState::Intermediate
                    });

                    HStack::new(cx, move |cx| {
                        Checkbox::intermediate(cx, checked, intermediate).on_toggle({
                            let node_id = row_id.clone();
                            move |cx| {
                                cx.emit(AppEvent::ToggleCheckState(node_id));
                            }
                        });
                        Label::new(cx, text).class("table-cell-text");
                    })
                    .height(Auto)
                    .alignment(Alignment::Left)
                    .gap(Pixels(8.0));
                });
            },
        )
        .width(Percentage(50.0))
        .min_width(140.0)
        .resizable(true),
        TreeTableColumn::new(
            "kind",
            |cx, sort_dir| TableHeader::new(cx, "Kind", sort_dir),
            move |cx, row| {
                let row_id = row.get().id;
                let text = tree.map(move |tree| {
                    tree.get(row_id).map(|node| node.value().kind.clone()).unwrap_or_default()
                });
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(Pixels(80.0))
        .min_width(60.0)
        .resizable(true),
        TreeTableColumn::new(
            "size",
            |cx, sort_dir| TableHeader::new(cx, "Size", sort_dir),
            move |cx, row| {
                let row_id = row.get().id;
                let text = tree.map(move |tree| {
                    tree.get(row_id).map(|node| node.value().size.clone()).unwrap_or_default()
                });
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(Pixels(90.0))
        .min_width(60.0)
        .resizable(true),
        TreeTableColumn::new(
            "modified",
            |cx, sort_dir| TableHeader::new(cx, "Modified", sort_dir),
            move |cx, row| {
                let row_id = row.get().id;
                let text = tree.map(move |tree| {
                    tree.get(row_id).map(|node| node.value().modified.clone()).unwrap_or_default()
                });
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(Pixels(110.0))
        .min_width(80.0)
        .resizable(true),
    ]
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let tree = Signal::new(build_fs_tree());
        let root_id = tree.with(|tree| tree.root().id());
        let cols = Signal::new(columns(tree));
        let sort_state: Signal<Option<TableSortState>> = Signal::new(None);
        let selected_rows: Signal<Vec<NodeId>> = Signal::new(vec![]);
        let expanded_rows: Signal<Vec<NodeId>> = Signal::new(vec![root_id]);
        let filter_text = Signal::new(String::new());

        AppData { tree, sort_state, selected_rows, expanded_rows, filter_text }.build(cx);

        ExamplePage::vertical(cx, move |cx| {
            Textbox::new(cx, filter_text).width(Pixels(320.0)).placeholder("Filter rows").on_edit(
                move |cx, text| {
                    cx.emit(AppEvent::SetFilterText(text));
                },
            );

            VirtualTreeTable::from_hierarchy(
                cx,
                tree,
                cols,
                34.0,
                { move |tree: &Tree<FsNode>| vec![tree.root().id()] },
                {
                    let sort_state = sort_state;
                    move |tree: &Tree<FsNode>, parent_id: &NodeId| {
                        sorted_child_ids(tree, *parent_id, sort_state.get().as_ref())
                    }
                },
                |tree: &Tree<FsNode>, node_id: &NodeId| {
                    if *node_id == tree.root().id() {
                        true
                    } else {
                        tree.get(*node_id).map(|node| node.value().visible).unwrap_or(false)
                    }
                },
            )
            .sort_state(sort_state)
            .sort_cycle(TableSortCycle::TriState)
            .resizable_columns(true)
            .selectable(Selectable::Single)
            .selected_row_ids(selected_rows)
            .expanded_row_ids(expanded_rows)
            .on_sort(move |cx, key, direction| {
                cx.emit(AppEvent::SetSort(key, direction));
            })
            .on_row_select(move |cx, id| {
                cx.emit(AppEvent::SelectRow(id));
            })
            .on_row_toggle(move |cx, id, expanded| {
                cx.emit(AppEvent::ToggleRow(id, expanded));
            })
            .width(Stretch(1.0))
            .height(Stretch(1.0));
        });
    })
    .run()
}
