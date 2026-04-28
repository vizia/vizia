mod helpers;
use ego_tree::{NodeId, Tree};
use helpers::*;
use std::{fs, path::Path, time::SystemTime};
use vizia::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CheckState {
    Unchecked,
    Checked,
    Intermediate,
}

#[derive(Clone, PartialEq)]
struct TreeRow {
    id: NodeId,
    parent_id: Option<NodeId>,
    name: String,
    kind: String,
    size: String,
    modified: String,
    check_state: CheckState,
}

#[derive(Clone)]
struct FsNode {
    name: String,
    kind: String,
    size: String,
    modified: String,
    check_state: CheckState,
}

struct AppData {
    tree: Signal<Tree<FsNode>>,
    data: Signal<Vec<TreeRow>>,
    sort_state: Signal<Option<TableSortState>>,
    selected_rows: Signal<Vec<NodeId>>,
    expanded_rows: Signal<Vec<NodeId>>,
}

enum AppEvent {
    ToggleCheckState(NodeId),
    SetSort(String, TableSortDirection),
    SelectRow(NodeId),
    ToggleRow(NodeId, bool),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleCheckState(id) => {
                self.tree.update(|tree| {
                    toggle_check_state(tree, *id);
                    self.data.set(flatten_tree_rows(tree));
                });
            }

            AppEvent::SetSort(key, direction) => {
                self.sort_state
                    .set(Some(TableSortState { key: key.clone(), direction: *direction }));
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
        });
    }
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

fn add_dir_to_tree(tree: &mut Tree<FsNode>, parent_node: NodeId, root: &Path) {
    let Ok(mut entries) = fs::read_dir(root) else { return };

    // Collect and sort: directories first, then files, both alphabetically
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    while let Some(Ok(entry)) = entries.next() {
        let path = entry.path();
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

        let modified = meta.modified().map(format_modified).unwrap_or_else(|_| "—".to_string());

        let child_id = {
            let mut parent = tree.get_mut(parent_node).expect("tree parent node should exist");
            parent
                .append(FsNode { name, kind, size, modified, check_state: CheckState::Unchecked })
                .id()
        };

        if meta.is_dir() {
            add_dir_to_tree(tree, child_id, &path);
        }
    }
}

fn flatten_tree_rows(tree: &Tree<FsNode>) -> Vec<TreeRow> {
    fn visit(
        tree: &Tree<FsNode>,
        node_id: NodeId,
        parent_id: Option<NodeId>,
        out: &mut Vec<TreeRow>,
    ) {
        let node = tree.get(node_id).expect("tree node should exist");
        let value = node.value();
        let current_id = node.id();

        out.push(TreeRow {
            id: current_id,
            parent_id,
            name: value.name.clone(),
            kind: value.kind.clone(),
            size: value.size.clone(),
            modified: value.modified.clone(),
            check_state: value.check_state,
        });

        let child_ids = node.children().map(|child| child.id()).collect::<Vec<_>>();
        for child_id in child_ids {
            visit(tree, child_id, Some(current_id), out);
        }
    }

    let mut rows = Vec::new();
    let root = tree.root();
    let top_level_ids = root.children().map(|child| child.id()).collect::<Vec<_>>();
    for child_id in top_level_ids {
        visit(tree, child_id, None, &mut rows);
    }

    rows
}

fn build_fs_tree() -> Tree<FsNode> {
    // Walk the vizia workspace root (one directory up from examples/)
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut tree = Tree::new(FsNode {
        name: String::new(),
        kind: "Folder".to_string(),
        size: "—".to_string(),
        modified: "—".to_string(),
        check_state: CheckState::Unchecked,
    });

    let root_id = tree.root().id();
    add_dir_to_tree(&mut tree, root_id, root);
    tree
}

fn columns() -> Vec<TreeTableColumn<TreeRow, NodeId, TableHeader>> {
    vec![
        TreeTableColumn::new(
            "name",
            |cx, sort_dir| TableHeader::new(cx, "Name", sort_dir),
            move |cx, row| {
                let row_for_first_cell: TreeTableRow<TreeRow, NodeId> = row.get();

                TreeTableFirstCell::new(cx, row_for_first_cell, move |cx, row| {
                    let checked = row.row.check_state == CheckState::Checked;
                    let intermediate = row.row.check_state == CheckState::Intermediate;
                    let row_id = row.id;
                    let text = row.row.name.clone();

                    HStack::new(cx, move |cx| {
                        Checkbox::intermediate(cx, checked, intermediate).on_toggle(move |cx| {
                            cx.emit(AppEvent::ToggleCheckState(row_id));
                        });
                        Label::new(cx, text).class("table-cell-text");
                    })
                    .height(Auto)
                    .alignment(Alignment::Left)
                    .gap(Pixels(8.0));
                });
            },
        )
        .width(260.0)
        .min_width(140.0)
        .resizable(true),
        TreeTableColumn::new(
            "kind",
            |cx, sort_dir| TableHeader::new(cx, "Kind", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TreeTableRow<TreeRow, NodeId>| r.row.kind.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(80.0)
        .min_width(60.0)
        .resizable(true),
        TreeTableColumn::new(
            "size",
            |cx, sort_dir| TableHeader::new(cx, "Size", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TreeTableRow<TreeRow, NodeId>| r.row.size.clone());
                Label::new(cx, text).class("table-cell-text").text_wrap(false);
            },
        )
        .width(90.0)
        .min_width(60.0)
        .resizable(true),
        TreeTableColumn::new(
            "modified",
            |cx, sort_dir| TableHeader::new(cx, "Modified", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TreeTableRow<TreeRow, NodeId>| r.row.modified.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(110.0)
        .min_width(80.0)
        .resizable(true),
    ]
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let tree = Signal::new(build_fs_tree());
        let data = Signal::new(flatten_tree_rows(&tree.get()));
        let cols = Signal::new(columns());
        let sort_state: Signal<Option<TableSortState>> = Signal::new(None);
        let selected_rows: Signal<Vec<NodeId>> = Signal::new(vec![]);
        let expanded_rows: Signal<Vec<NodeId>> = Signal::new(vec![]);

        AppData { tree, data, sort_state, selected_rows, expanded_rows }.build(cx);

        ExamplePage::vertical(cx, move |cx| {
            TreeTable::new(cx, data, cols, |row: &TreeRow| row.id, |row: &TreeRow| row.parent_id)
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
                .height(Pixels(360.0));
        });
    })
    .run()
}
