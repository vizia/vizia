mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, PartialEq)]
struct NodeRow {
    id: u32,
    parent_id: Option<u32>,
    name: String,
    kind: String,
    notes: String,
}

struct DemoData {
    selected_rows: Signal<Vec<u32>>,
    expanded_rows: Signal<Vec<u32>>,
}

enum DemoEvent {
    SelectRow(u32),
    ToggleRow(u32, bool),
}

impl Model for DemoData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            DemoEvent::SelectRow(id) => {
                self.selected_rows.set(vec![*id]);
            }
            DemoEvent::ToggleRow(id, expanded) => {
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

fn make_rows() -> Vec<NodeRow> {
    let mut rows = Vec::new();
    let mut next_id = 1u32;

    for module in 0..120 {
        let module_id = next_id;
        next_id += 1;

        rows.push(NodeRow {
            id: module_id,
            parent_id: None,
            name: format!("module_{module:03}"),
            kind: "Folder".to_string(),
            notes: format!("Module group {module}"),
        });

        for section in 0..12 {
            let section_id = next_id;
            next_id += 1;

            rows.push(NodeRow {
                id: section_id,
                parent_id: Some(module_id),
                name: format!("section_{section:02}"),
                kind: "Folder".to_string(),
                notes: format!("Section {section} in module {module}"),
            });

            for file_index in 0..10 {
                rows.push(NodeRow {
                    id: next_id,
                    parent_id: Some(section_id),
                    name: format!("item_{file_index:02}.rs"),
                    kind: "File".to_string(),
                    notes: format!("Leaf entry {file_index} under section {section}"),
                });
                next_id += 1;
            }
        }
    }

    rows
}

fn columns() -> Vec<TreeTableColumn<NodeRow, u32, TableHeader>> {
    vec![
        TreeTableColumn::new(
            "name",
            |cx, sort_dir| TableHeader::new(cx, "Name", sort_dir),
            |cx, row| {
                let first_cell_row: TreeTableRow<NodeRow, u32> = row.get();
                TreeTableFirstCell::new(cx, first_cell_row, |cx, row| {
                    Label::new(cx, row.row.name).class("table-cell-text");
                });
            },
        )
        .width(260.0)
        .min_width(160.0)
        .resizable(true)
        .sortable(false),
        TreeTableColumn::new(
            "kind",
            |cx, sort_dir| TableHeader::new(cx, "Kind", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TreeTableRow<NodeRow, u32>| r.row.kind.clone());
                Label::new(cx, text).class("table-cell-text");
            },
        )
        .width(110.0)
        .min_width(80.0)
        .resizable(true)
        .sortable(false),
        TreeTableColumn::new(
            "notes",
            |cx, sort_dir| TableHeader::new(cx, "Notes", sort_dir),
            |cx, row| {
                let text = row.map(|r: &TreeTableRow<NodeRow, u32>| r.row.notes.clone());
                Label::new(cx, text).class("table-cell-text").text_wrap(true);
            },
        )
        .width(420.0)
        .min_width(220.0)
        .resizable(true)
        .sortable(false),
    ]
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let rows = Signal::new(make_rows());
        let columns = Signal::new(columns());
        let selected_rows = Signal::new(Vec::<u32>::new());
        let expanded_rows = Signal::new(vec![1]);

        DemoData { selected_rows, expanded_rows }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Label::new(cx, "VirtualTreeTable")
                .font_size(18.0)
                .height(Auto);

            Label::new(cx, "Virtualized hierarchical table with expandable rows.")
                .class("table-cell-meta")
                .height(Auto);

            VirtualTreeTable::new(
                cx,
                rows,
                columns,
                34.0,
                |row: &NodeRow| row.id,
                |row: &NodeRow| row.parent_id,
            )
            .resizable_columns(true)
            .selectable(Selectable::Single)
            .selected_row_ids(selected_rows)
            .expanded_row_ids(expanded_rows)
            .on_row_select(|cx, id| {
                cx.emit(DemoEvent::SelectRow(id));
            })
            .on_row_toggle(|cx, id, expanded| {
                cx.emit(DemoEvent::ToggleRow(id, expanded));
            })
            .width(Stretch(1.0))
            .height(Stretch(1.0));
        })
        .size(Stretch(1.0))
        .padding(Pixels(12.0))
        .gap(Pixels(6.0));
    })
    .title("Virtual Tree Table")
    .inner_size((1100, 760))
    .run()
}
