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

    let columns: Signal<Vec<TreeTableColumn<TreeRow, u32, TableHeader<String>>>> =
        Signal::new(vec![
            TreeTableColumn::new(
                "name",
                |cx, sort_dir| TableHeader::new(cx, "name", "Name", sort_dir),
                |cx, row| {
                    let row_for_first_cell: TreeTableRow<TreeRow, u32> = row.get();
                    TreeTableFirstCell::new(cx, row_for_first_cell, move |cx, row| {
                        Label::new(cx, row.row.name.clone()).class("table-cell-text");
                    });
                },
            )
            .width(Pixels(220.0))
            .min_width(140.0)
            .resizable(true),
            TreeTableColumn::new(
                "category",
                |cx, sort_dir| TableHeader::new(cx, "category", "Category", sort_dir),
                |cx, row| {
                    let text = row.map(|r: &TreeTableRow<TreeRow, u32>| r.row.category.clone());
                    Label::new(cx, text).class("table-cell-text");
                },
            )
            .width(Pixels(140.0))
            .min_width(100.0)
            .resizable(true),
            TreeTableColumn::new(
                "status",
                |cx, sort_dir| TableHeader::new(cx, "status", "Status", sort_dir),
                |cx, row| {
                    let text = row.map(|r: &TreeTableRow<TreeRow, u32>| r.row.status.clone());
                    Label::new(cx, text).class("table-cell-text");
                },
            )
            .width(Pixels(120.0))
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
            TreeTable::from_rows(
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
            .expanded_row_ids(expanded_rows)
            .on_sort(move |_cx, key, direction| {
                sort_state.set(Some(TableSortState { key, direction }));
            })
            .on_select(move |_cx, ids| {
                selected_rows.set(ids.iter().map(|cell| cell.0).collect());
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
