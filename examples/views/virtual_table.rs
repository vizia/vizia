mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, PartialEq)]
struct PerfRow {
    id: u32,
    name: String,
    group: String,
    notes: String,
}

struct PerfTableDemo {
    sort_state: Signal<Option<TableSortState>>,
    selected_rows: Signal<Vec<u32>>,
}

enum PerfTableEvent {
    SetSortState(String, TableSortDirection),
    SelectRow(u32),
}

impl Model for PerfTableDemo {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            PerfTableEvent::SetSortState(key, direction) => {
                self.sort_state
                    .set(Some(TableSortState { key: key.clone(), direction: *direction }));
            }
            PerfTableEvent::SelectRow(id) => {
                self.selected_rows.set(vec![*id]);
            }
        });
    }
}

fn make_rows(count: usize) -> Vec<PerfRow> {
    (0..count)
        .map(|i| PerfRow {
            id: i as u32,
            name: format!("Row {i:05}"),
            group: format!("Group {}", i % 24),
            notes: format!("Compact note for row {i}."),
        })
        .collect()
}

fn sort_rows(mut rows: Vec<PerfRow>, state: Option<TableSortState>) -> Vec<PerfRow> {
    if let Some(state) = state {
        match state.key.as_str() {
            "Name" => rows.sort_by(|a, b| a.name.cmp(&b.name)),
            "Group" => rows.sort_by(|a, b| a.group.cmp(&b.group)),
            "notes" => rows.sort_by(|a, b| a.notes.cmp(&b.notes)),
            _ => {}
        }

        match state.direction {
            TableSortDirection::Descending => rows.reverse(),
            TableSortDirection::Ascending | TableSortDirection::None => {}
        }
    }

    rows
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let rows = Signal::new(make_rows(5000));
        let sort_state = Signal::new(None);
        let selected_rows = Signal::new(Vec::<u32>::new());

        PerfTableDemo { sort_state, selected_rows }.build(cx);

        let sorted_rows = Memo::new(move |_| sort_rows(rows.get(), sort_state.get()));

        let columns: Signal<Vec<TableColumn<PerfRow, TableHeader>>> = Signal::new(vec![
            TableColumn::new(
                "Name",
                |cx, sort_direction| TableHeader::new(cx, "Name", sort_direction),
                |cx, row| {
                    let text = row.map(|row: &PerfRow| row.name.clone());
                    Label::new(cx, text).class("table-cell-text").text_wrap(true);
                },
            )
            .width(180.0)
            .min_width(120.0)
            .resizable(true),
            TableColumn::new(
                "Group",
                |cx, sort_direction| TableHeader::new(cx, "Group", sort_direction),
                |cx, row| {
                    let text = row.map(|row: &PerfRow| row.group.clone());
                    Label::new(cx, text).class("table-cell-text").text_wrap(true);
                },
            )
            .width(160.0)
            .min_width(100.0)
            .resizable(true),
            TableColumn::new(
                "notes",
                |cx, sort_direction| TableHeader::new(cx, "Notes", sort_direction),
                |cx, row| {
                    let notes = row.map(|row: &PerfRow| row.notes.clone());
                    Label::new(cx, notes).class("table-cell-text").text_wrap(true);
                },
            )
            .width(520.0)
            .min_width(220.0)
            .resizable(true),
        ]);

        ExamplePage::vertical(cx, |cx| {
            Label::new(cx, "VirtualTable Large Dataset (5,000 Rows)")
                .font_size(18.0)
                .height(Auto);

            Label::new(cx, "Virtualized fixed-height rows for large datasets. Use sorting and resizing to profile interactivity.")
                .class("table-cell-meta")
                .height(Auto);

            VirtualTable::new(cx, sorted_rows, columns, 34.0, |row: &PerfRow| row.id)
                .sort_state(sort_state)
                .sort_cycle(TableSortCycle::TriState)
                .resizable_columns(true)
                .selectable(Selectable::Single)
                .selected_row_ids(selected_rows)
                .on_sort(|cx, key, direction| {
                    cx.emit(PerfTableEvent::SetSortState(key, direction));
                })
                .on_row_select(|cx, id| cx.emit(PerfTableEvent::SelectRow(id)))
                .width(Stretch(1.0))
                .height(Stretch(1.0));
        })
        .size(Stretch(1.0))
        .padding(Pixels(12.0))
        .gap(Pixels(6.0));
    })
    .title("VirtualTable Large Dataset")
    .inner_size((1100, 760))
    .run()
}
