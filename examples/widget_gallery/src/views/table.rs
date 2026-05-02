use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Clone, PartialEq)]
struct TableRow {
    id: u32,
    name: String,
    category: String,
    status: String,
}

pub fn table(cx: &mut Context) {
    let rows = Signal::new(vec![
        TableRow {
            id: 1,
            name: "Button".into(),
            category: "Input".into(),
            status: "Stable".into(),
        },
        TableRow {
            id: 2,
            name: "Label".into(),
            category: "Display".into(),
            status: "Stable".into(),
        },
        TableRow {
            id: 3,
            name: "Slider".into(),
            category: "Input".into(),
            status: "Stable".into(),
        },
        TableRow {
            id: 4,
            name: "VirtualList".into(),
            category: "Data".into(),
            status: "Stable".into(),
        },
        TableRow {
            id: 5,
            name: "Combobox".into(),
            category: "Input".into(),
            status: "Stable".into(),
        },
        TableRow {
            id: 6,
            name: "Accordion".into(),
            category: "Containers".into(),
            status: "Stable".into(),
        },
        TableRow {
            id: 7,
            name: "Tooltip".into(),
            category: "Feedback".into(),
            status: "Stable".into(),
        },
    ]);

    let sort_state: Signal<Option<TableSortState>> = Signal::new(None);
    let selected_rows: Signal<Vec<u32>> = Signal::new(vec![]);

    let sorted_rows = Memo::new(move |_| {
        let mut r = rows.get();
        if let Some(state) = sort_state.get() {
            match state.key.as_str() {
                "name" => r.sort_by(|a, b| a.name.cmp(&b.name)),
                "category" => r.sort_by(|a, b| a.category.cmp(&b.category)),
                "status" => r.sort_by(|a, b| a.status.cmp(&b.status)),
                _ => {}
            }
            if state.direction == TableSortDirection::Descending {
                r.reverse();
            }
        }
        r
    });

    let columns: Signal<Vec<TableColumn<TableRow, TableHeader>>> = Signal::new(vec![
        TableColumn::new(
            "name",
            |cx, sort_dir| TableHeader::new(cx, "Name", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TableRow| r.name.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(200.0)
        .min_width(120.0)
        .resizable(true),
        TableColumn::new(
            "category",
            |cx, sort_dir| TableHeader::new(cx, "Category", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TableRow| r.category.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(160.0)
        .min_width(100.0)
        .resizable(true),
        TableColumn::new(
            "status",
            |cx, sort_dir| TableHeader::new(cx, "Status", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TableRow| r.status.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(120.0)
        .min_width(80.0)
        .resizable(true),
    ]);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("table")).class("panel-title");

        Divider::new(cx);

        Label::new(cx, Localized::new("basic-table")).class("section-title");

        DemoRegion::new(cx, "Basic Table", move |cx| {
            Table::new(cx, sorted_rows, columns, |row: &TableRow| row.id)
                .sort_state(sort_state)
                .sort_cycle(TableSortCycle::TriState)
                .resizable_columns(true)
                .selectable(Selectable::Single)
                .selected_row_ids(selected_rows)
                .on_sort(move |_cx, key, direction| {
                    sort_state.set(Some(TableSortState { key, direction }));
                })
                .on_row_select(move |_cx, id| {
                    selected_rows.set(vec![id]);
                })
                .width(Stretch(1.0))
                .height(Pixels(280.0));
        });
    })
    .class("panel");
}
