mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, PartialEq)]
struct TreeRow {
    id: u32,
    parent_id: Option<u32>,
    name: String,
    kind: String,
    status: String,
}

fn rows() -> Vec<TreeRow> {
    vec![
        TreeRow {
            id: 1,
            parent_id: None,
            name: "src".to_string(),
            kind: "Folder".to_string(),
            status: "Open".to_string(),
        },
        TreeRow {
            id: 2,
            parent_id: Some(1),
            name: "views".to_string(),
            kind: "Folder".to_string(),
            status: "Open".to_string(),
        },
        TreeRow {
            id: 3,
            parent_id: Some(2),
            name: "table.rs".to_string(),
            kind: "File".to_string(),
            status: "Tracked".to_string(),
        },
        TreeRow {
            id: 4,
            parent_id: Some(2),
            name: "list.rs".to_string(),
            kind: "File".to_string(),
            status: "Tracked".to_string(),
        },
        TreeRow {
            id: 5,
            parent_id: Some(1),
            name: "models".to_string(),
            kind: "Folder".to_string(),
            status: "Closed".to_string(),
        },
        TreeRow {
            id: 6,
            parent_id: Some(5),
            name: "state.rs".to_string(),
            kind: "File".to_string(),
            status: "Tracked".to_string(),
        },
        TreeRow {
            id: 7,
            parent_id: None,
            name: "README.md".to_string(),
            kind: "File".to_string(),
            status: "Tracked".to_string(),
        },
    ]
}

fn columns() -> Vec<TableColumn<TreeRow, TableHeader>> {
    vec![
        TableColumn::new(
            "name",
            |cx, sort_dir| TableHeader::new(cx, "Name", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TreeRow| r.name.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(260.0)
        .min_width(140.0)
        .resizable(true),
        TableColumn::new(
            "kind",
            |cx, sort_dir| TableHeader::new(cx, "Kind", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TreeRow| r.kind.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(120.0)
        .min_width(80.0)
        .resizable(true),
        TableColumn::new(
            "status",
            |cx, sort_dir| TableHeader::new(cx, "Status", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TreeRow| r.status.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(120.0)
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
        let expanded_rows: Signal<Vec<u32>> = Signal::new(vec![1, 2]);

        ExamplePage::vertical(cx, move |cx| {
            TreeTable::new(
                cx,
                data,
                cols,
                |row: &TreeRow| row.id,
                |row: &TreeRow| row.parent_id,
            )
            .sort_state(sort_state)
            .sort_cycle(TableSortCycle::TriState)
            .resizable_columns(true)
            .selectable(Selectable::Single)
            .selected_row_ids(selected_rows)
            .expanded_row_ids(expanded_rows)
            .on_sort(move |_cx, key, direction| {
                sort_state.set(Some(TableSortState { key, direction }));
            })
            .on_row_select(move |_cx, id| {
                selected_rows.set(vec![id]);
            })
            .on_row_toggle(move |_cx, id, expanded| {
                expanded_rows.update(|rows| {
                    if expanded {
                        if !rows.contains(&id) {
                            rows.push(id);
                        }
                    } else {
                        rows.retain(|current| *current != id);
                    }
                });
            })
            .width(Stretch(1.0))
            .height(Pixels(360.0));
        });
    })
    .run()
}
