use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Clone, PartialEq)]
struct TreeRow {
    id: u32,
    parent_id: Option<u32>,
    name: String,
    category: String,
    status: String,
}

pub fn tree_table(cx: &mut Context) {
    let rows = Signal::new(vec![
        TreeRow {
            id: 1,
            parent_id: None,
            name: "Data".into(),
            category: "Folder".into(),
            status: "Open".into(),
        },
        TreeRow {
            id: 2,
            parent_id: Some(1),
            name: "Table".into(),
            category: "Widget".into(),
            status: "Stable".into(),
        },
        TreeRow {
            id: 3,
            parent_id: Some(1),
            name: "VirtualTable".into(),
            category: "Widget".into(),
            status: "Stable".into(),
        },
        TreeRow {
            id: 4,
            parent_id: None,
            name: "Input".into(),
            category: "Folder".into(),
            status: "Open".into(),
        },
        TreeRow {
            id: 5,
            parent_id: Some(4),
            name: "Textbox".into(),
            category: "Widget".into(),
            status: "Stable".into(),
        },
        TreeRow {
            id: 6,
            parent_id: Some(4),
            name: "Slider".into(),
            category: "Widget".into(),
            status: "Stable".into(),
        },
    ]);

    let sort_state: Signal<Option<TableSortState>> = Signal::new(None);
    let selected_rows: Signal<Vec<u32>> = Signal::new(vec![]);
    let expanded_rows: Signal<Vec<u32>> = Signal::new(vec![1, 4]);

    let columns: Signal<Vec<TableColumn<TreeRow, TableHeader>>> = Signal::new(vec![
        TableColumn::new(
            "name",
            |cx, sort_dir| TableHeader::new(cx, "Name", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TreeRow| r.name.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(220.0)
        .min_width(140.0)
        .resizable(true),
        TableColumn::new(
            "category",
            |cx, sort_dir| TableHeader::new(cx, "Category", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TreeRow| r.category.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(140.0)
        .min_width(100.0)
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
        .min_width(90.0)
        .resizable(true),
    ]);

    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# TreeTable
A TreeTable combines table columns with expandable hierarchical rows.",
        );

        Divider::new(cx);

        DemoRegion::new(cx, "TreeTable", move |cx| {
            TreeTable::new(
                cx,
                rows,
                columns,
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
            .height(Pixels(300.0));
        });
    })
    .class("panel");
}
