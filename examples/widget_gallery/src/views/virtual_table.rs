use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Clone, PartialEq)]
struct VTableRow {
    id: u32,
    name: String,
    category: String,
    notes: String,
}

pub fn virtual_table(cx: &mut Context) {
    let rows = Signal::new(
        (0u32..500)
            .map(|i| VTableRow {
                id: i,
                name: format!("Widget {:04}", i),
                category: ["Layout", "Input", "Display", "Data", "Feedback"][(i % 5) as usize]
                    .to_string(),
                notes: format!("Auto-generated description for row {}.", i),
            })
            .collect::<Vec<_>>(),
    );

    let sort_state: Signal<Option<TableSortState>> = Signal::new(None);
    let selected_rows: Signal<Vec<u32>> = Signal::new(vec![]);

    let sorted_rows = Memo::new(move |_| {
        let mut r = rows.get();
        if let Some(state) = sort_state.get() {
            match state.key.as_str() {
                "name" => r.sort_by(|a, b| a.name.cmp(&b.name)),
                "category" => r.sort_by(|a, b| a.category.cmp(&b.category)),
                "notes" => r.sort_by(|a, b| a.notes.cmp(&b.notes)),
                _ => {}
            }
            if state.direction == TableSortDirection::Descending {
                r.reverse();
            }
        }
        r
    });

    let columns: Signal<Vec<TableColumn<VTableRow, TableHeader>>> = Signal::new(vec![
        TableColumn::new(
            "name",
            |cx, sort_dir| TableHeader::new(cx, "Name", sort_dir),
            |cx, row| {
                let text = row.map(|r: &VTableRow| r.name.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(180.0)
        .min_width(120.0)
        .resizable(true),
        TableColumn::new(
            "category",
            |cx, sort_dir| TableHeader::new(cx, "Category", sort_dir),
            |cx, row| {
                let text = row.map(|r: &VTableRow| r.category.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(140.0)
        .min_width(100.0)
        .resizable(true),
        TableColumn::new(
            "notes",
            |cx, sort_dir| TableHeader::new(cx, "Notes", sort_dir),
            |cx, row| {
                let text = row.map(|r: &VTableRow| r.notes.clone());
                Label::new(cx, text).class("table-cell-text").text_wrap(true);
            },
        )
        .width(300.0)
        .min_width(150.0)
        .resizable(true),
    ]);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("virtual-table")).class("panel-title");

        Divider::new(cx);

        Label::new(cx, Localized::new("virtual-table-500-row")).class("section-title");

        DemoRegion::new(cx, "VirtualTable (500 rows)", move |cx| {
            VirtualTable::new(cx, sorted_rows, columns, 34.0, |row: &VTableRow| row.id)
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
                .height(Pixels(360.0));
        });
    })
    .class("panel");
}
