mod helpers;
use helpers::*;
use std::{collections::HashMap, fs, path::Path, time::SystemTime};
use vizia::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CheckState {
    Unchecked,
    Checked,
    Intermediate,
}

#[derive(Clone, PartialEq)]
struct TreeRow {
    id: u32,
    parent_id: Option<u32>,
    name: String,
    kind: String,
    size: String,
    modified: String,
    check_state: CheckState,
}

struct AppData {
    data: Signal<Vec<TreeRow>>,
    sort_state: Signal<Option<TableSortState>>,
    selected_rows: Signal<Vec<u32>>,
    expanded_rows: Signal<Vec<u32>>,
}

enum AppEvent {
    ToggleCheckState(u32),
    SetSort(String, TableSortDirection),
    SelectRow(u32),
    ToggleRow(u32, bool),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleCheckState(id) => {
                self.data.update(|rows| toggle_check_state(rows, *id));
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

fn build_indices(
    rows: &[TreeRow],
) -> (HashMap<u32, usize>, HashMap<u32, Vec<u32>>, HashMap<u32, Option<u32>>) {
    let id_to_index =
        rows.iter().enumerate().map(|(index, row)| (row.id, index)).collect::<HashMap<_, _>>();

    let mut children_by_parent: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut parent_by_id: HashMap<u32, Option<u32>> = HashMap::new();

    for row in rows {
        parent_by_id.insert(row.id, row.parent_id);
        if let Some(parent_id) = row.parent_id {
            children_by_parent.entry(parent_id).or_default().push(row.id);
        }
    }

    (id_to_index, children_by_parent, parent_by_id)
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

fn set_descendants(
    rows: &mut [TreeRow],
    root_id: u32,
    target_state: CheckState,
    id_to_index: &HashMap<u32, usize>,
    children_by_parent: &HashMap<u32, Vec<u32>>,
) {
    let mut stack = vec![root_id];

    while let Some(current_id) = stack.pop() {
        if let Some(&index) = id_to_index.get(&current_id) {
            rows[index].check_state = target_state;
        }

        if let Some(children) = children_by_parent.get(&current_id) {
            stack.extend(children.iter().copied());
        }
    }
}

fn recompute_ancestors(
    rows: &mut [TreeRow],
    start_id: u32,
    id_to_index: &HashMap<u32, usize>,
    children_by_parent: &HashMap<u32, Vec<u32>>,
    parent_by_id: &HashMap<u32, Option<u32>>,
) {
    let mut current = parent_by_id.get(&start_id).copied().flatten();

    while let Some(parent_id) = current {
        if let (Some(child_ids), Some(&parent_index)) =
            (children_by_parent.get(&parent_id), id_to_index.get(&parent_id))
        {
            rows[parent_index].check_state = aggregate_check_state(
                child_ids
                    .iter()
                    .filter_map(|id| id_to_index.get(id).map(|&index| rows[index].check_state)),
            );
        }

        current = parent_by_id.get(&parent_id).copied().flatten();
    }
}

fn toggle_check_state(rows: &mut [TreeRow], id: u32) {
    let (id_to_index, children_by_parent, parent_by_id) = build_indices(rows);

    let next_checked = id_to_index
        .get(&id)
        .map(|&index| rows[index].check_state != CheckState::Checked)
        .unwrap_or(false);

    let target_state = if next_checked { CheckState::Checked } else { CheckState::Unchecked };
    set_descendants(rows, id, target_state, &id_to_index, &children_by_parent);
    recompute_ancestors(rows, id, &id_to_index, &children_by_parent, &parent_by_id);
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

fn read_dir_flat(root: &Path, parent_id: Option<u32>, next_id: &mut u32, rows: &mut Vec<TreeRow>) {
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
        let id = *next_id;
        *next_id += 1;

        let (kind, size) = if meta.is_dir() {
            ("Folder".to_string(), "—".to_string())
        } else {
            ("File".to_string(), format_size(meta.len()))
        };

        let modified = meta.modified().map(format_modified).unwrap_or_else(|_| "—".to_string());

        rows.push(TreeRow {
            id,
            parent_id,
            name,
            kind,
            size,
            modified,
            check_state: CheckState::Unchecked,
        });

        if meta.is_dir() {
            read_dir_flat(&path, Some(id), next_id, rows);
        }
    }
}

fn rows() -> Vec<TreeRow> {
    // Walk the vizia workspace root (one directory up from examples/)
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut all = Vec::new();
    let mut next_id = 1u32;
    read_dir_flat(root, None, &mut next_id, &mut all);
    all
}

fn columns() -> Vec<TreeTableColumn<TreeRow, u32, TableHeader>> {
    vec![
        TreeTableColumn::new(
            "name",
            |cx, sort_dir| TableHeader::new(cx, "Name", sort_dir),
            move |cx, row| {
                let row_for_first_cell: TreeTableRow<TreeRow, u32> = row.get();

                TreeTableFirstCell::new(cx, row_for_first_cell, move |cx, row| {
                    let checked = row.row.check_state == CheckState::Checked;
                    let intermediate = row.row.check_state == CheckState::Intermediate;
                    let row_id = row.id;
                    let text = row.row.name.clone();

                    HStack::new(cx, move |cx| {
                        Checkbox::intermediate(cx, checked, intermediate).on_toggle(move |_cx| {
                            _cx.emit(AppEvent::ToggleCheckState(row_id));
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
                let text = row.map(|r: &TreeTableRow<TreeRow, u32>| r.row.kind.clone());
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
                let text = row.map(|r: &TreeTableRow<TreeRow, u32>| r.row.size.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(90.0)
        .min_width(60.0)
        .resizable(true),
        TreeTableColumn::new(
            "modified",
            |cx, sort_dir| TableHeader::new(cx, "Modified", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TreeTableRow<TreeRow, u32>| r.row.modified.clone());
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
        let data = Signal::new(rows());
        let cols = Signal::new(columns());
        let sort_state: Signal<Option<TableSortState>> = Signal::new(None);
        let selected_rows: Signal<Vec<u32>> = Signal::new(vec![]);
        let expanded_rows: Signal<Vec<u32>> = Signal::new(vec![]);

        AppData { data, sort_state, selected_rows, expanded_rows }.build(cx);

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
