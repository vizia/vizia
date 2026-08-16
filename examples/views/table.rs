mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, PartialEq)]
struct RowData {
    id: u32,
    name: String,
    group: String,
    status: String,
    notes: String,
}

struct DynamicTableDemo {
    columns: Signal<Vec<TableColumn<RowData, TableHeader<String>>>>,
    sort_state: Signal<Option<TableSortState>>,
    selected_rows: Signal<Vec<u32>>,
    filter_text: Signal<String>,
    show_group: Signal<bool>,
    show_notes: Signal<bool>,
    emphasize_status: Signal<bool>,
    status_text: Signal<String>,
}

enum DynamicTableEvent {
    ToggleGroup,
    ToggleNotes,
    ToggleOrder,
    SetSortState(String, TableSortDirection),
    SelectRow(u32),
    SetFilterText(String),
}

impl DynamicTableDemo {
    fn update_column_hidden(&self, key: &str, hidden: bool) {
        if let Some(column) = self.columns.get().iter().find(|column| column.key == key) {
            column.hidden.set(hidden);
        }
    }

    fn reorder_columns(&self) {
        let current_columns = self.columns.get();

        let find_column = |key: &str| {
            current_columns
                .iter()
                .find(|column| column.key == key)
                .cloned()
                .expect("column key should exist")
        };

        let reordered = if self.emphasize_status.get() {
            vec![
                find_column("status"),
                find_column("Name"),
                find_column("Group"),
                find_column("notes"),
            ]
        } else {
            vec![
                find_column("Name"),
                find_column("Group"),
                find_column("status"),
                find_column("notes"),
            ]
        };

        self.columns.set(reordered);
    }
}

impl Model for DynamicTableDemo {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            DynamicTableEvent::ToggleGroup => {
                self.show_group.update(|value| *value ^= true);
                self.update_column_hidden("Group", !self.show_group.get());
                self.status_text.set("Toggled Group column visibility.".to_string());
            }

            DynamicTableEvent::ToggleNotes => {
                self.show_notes.update(|value| *value ^= true);
                self.update_column_hidden("notes", !self.show_notes.get());
                self.status_text.set("Toggled Notes column visibility.".to_string());
            }

            DynamicTableEvent::ToggleOrder => {
                self.emphasize_status.update(|value| *value ^= true);
                self.reorder_columns();
                self.status_text.set(
                    "Changed column order. Resized widths and sort state should follow keyed columns."
                        .to_string(),
                );
            }

            DynamicTableEvent::SetSortState(key, direction) => {
                self.sort_state
                    .set(Some(TableSortState { key: key.clone(), direction: *direction }));
            }

            DynamicTableEvent::SelectRow(id) => {
                self.selected_rows.set(vec![*id]);
                self.status_text.set(format!("Selected row {id}."));
            }

            DynamicTableEvent::SetFilterText(text) => {
                self.filter_text.set(text.clone());

                if text.trim().is_empty() {
                    self.status_text.set(
                        "Filter cleared. Showing all rows matching current sort.".to_string(),
                    );
                } else {
                    self.status_text.set(format!("Filtering rows by \"{}\".", text.trim()));
                }
            }
        });
    }
}

fn make_rows() -> Vec<RowData> {
    vec![
        RowData {
            id: 1,
            name: "Alice".to_string(),
            group: "Core".to_string(),
            status: "Ready".to_string(),
            notes: "Maintains the main workflow and documentation.".to_string(),
        },
        RowData {
            id: 2,
            name: "Bob".to_string(),
            group: "Ops".to_string(),
            status: "Blocked".to_string(),
            notes: "Waiting on deployment credentials for staging.".to_string(),
        },
        RowData {
            id: 3,
            name: "Charlie".to_string(),
            group: "Core".to_string(),
            status: "In Review".to_string(),
            notes: "Reviewing the reactive table implementation.".to_string(),
        },
        RowData {
            id: 4,
            name: "Diana".to_string(),
            group: "Design".to_string(),
            status: "Ready".to_string(),
            notes: "Prepared updated layout variants for narrow screens.".to_string(),
        },
        RowData {
            id: 5,
            name: "Eve".to_string(),
            group: "Ops".to_string(),
            status: "Paused".to_string(),
            notes: "Paused until audit findings are resolved.".to_string(),
        },
    ]
}

fn build_columns() -> Vec<TableColumn<RowData, TableHeader<String>>> {
    let name_column = TableColumn::new(
        "Name",
        |cx, sort_direction| TableHeader::new(cx, "Name", "Name", sort_direction),
        |cx, row| {
            let text = row.map(|row: &RowData| row.name.clone());
            Label::new(cx, text).class("table-cell-text").text_wrap(true);
        },
    )
    .width(170.0)
    .min_width(120.0)
    .resizable(true);

    let group_column = TableColumn::new(
        "Group",
        |cx, sort_direction| TableHeader::new(cx, "Group", "Group", sort_direction),
        |cx, row| {
            let text = row.map(|row: &RowData| row.group.clone());
            Label::new(cx, text).class("table-cell-text").text_wrap(true);
        },
    )
    .width(140.0)
    .min_width(100.0)
    .resizable(true);

    let status_column = TableColumn::new(
        "status",
        |cx, sort_direction| TableHeader::new(cx, "status", "Status", sort_direction),
        |cx, row| {
            let status = row.map(|row: &RowData| row.status.clone());
            let tone = row.map(|row: &RowData| match row.status.as_str() {
                "Ready" => "Stable".to_string(),
                "In Review" => "Pending".to_string(),
                "Blocked" => "Attention".to_string(),
                _ => "Hold".to_string(),
            });

            Label::new(cx, status).class("table-cell-text").text_wrap(false);
            Label::new(cx, tone).class("table-cell-meta").text_wrap(false);
        },
    )
    .width(180.0)
    .min_width(130.0)
    .resizable(true);

    let notes_column = TableColumn::new(
        "notes",
        |cx, sort_direction| TableHeader::new(cx, "notes", "Notes", sort_direction),
        |cx, row| {
            let notes = row.map(|row: &RowData| row.notes.clone());
            Label::new(cx, notes).class("table-cell-text").text_wrap(true);
        },
    )
    .width(360.0)
    .min_width(220.0)
    .resizable(true);

    vec![name_column, group_column, status_column, notes_column]
}

fn sort_rows(mut rows: Vec<RowData>, sort_state: Option<TableSortState>) -> Vec<RowData> {
    if let Some(sort_state) = sort_state {
        match sort_state.key.as_str() {
            "Name" => rows.sort_by(|a, b| a.name.cmp(&b.name)),
            "Group" => rows.sort_by(|a, b| a.group.cmp(&b.group)),
            "status" => rows.sort_by(|a, b| a.status.cmp(&b.status)),
            "notes" => rows.sort_by(|a, b| a.notes.cmp(&b.notes)),
            _ => {}
        }

        if sort_state.direction == TableSortDirection::Descending {
            rows.reverse();
        }
    }

    rows
}

fn row_matches_query(row: &RowData, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    row.name.to_lowercase().contains(query)
        || row.group.to_lowercase().contains(query)
        || row.status.to_lowercase().contains(query)
        || row.notes.to_lowercase().contains(query)
}

fn filter_rows(rows: Vec<RowData>, filter_text: &str) -> Vec<RowData> {
    let query = filter_text.trim().to_lowercase();
    rows.into_iter().filter(|row| row_matches_query(row, &query)).collect()
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let rows = Signal::new(make_rows());
        let columns = Signal::new(build_columns());
        let sort_state = Signal::new(None);
        let selected_rows = Signal::new(Vec::<u32>::new());
        let filter_text = Signal::new(String::new());
        let show_group = Signal::new(true);
        let show_notes = Signal::new(true);
        let emphasize_status = Signal::new(false);
        let status_text = Signal::new(
            "Resize columns, then toggle visibility or reorder to verify keyed state preservation."
                .to_string(),
        );

        DynamicTableDemo {
            columns,
            sort_state,
            selected_rows,
            filter_text,
            show_group,
            show_notes,
            emphasize_status,
            status_text,
        }
        .build(cx);

        let sorted_rows = Memo::new(move |_| {
            let filtered_rows = filter_rows(rows.get(), &filter_text.get());
            sort_rows(filtered_rows, sort_state.get())
        });

        ExamplePage::vertical(cx, |cx| {
            Textbox::new(cx, filter_text).width(Pixels(320.0)).placeholder("Filter rows").on_edit(
                |cx, text| {
                    cx.emit(DynamicTableEvent::SetFilterText(text));
                },
            );

            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Switch::new(cx, show_group)
                        .on_toggle(|cx| cx.emit(DynamicTableEvent::ToggleGroup))
                        .id("show_group");
                    Label::new(cx, Localized::new("table-show-group")).describing("show_group");
                })
                .size(Auto)
                .gap(Pixels(6.0))
                .alignment(Alignment::Center);

                HStack::new(cx, |cx| {
                    Switch::new(cx, show_notes)
                        .on_toggle(|cx| cx.emit(DynamicTableEvent::ToggleNotes))
                        .id("show_notes");
                    Label::new(cx, Localized::new("table-show-notes")).describing("show_notes");
                })
                .size(Auto)
                .gap(Pixels(6.0))
                .alignment(Alignment::Center);

                HStack::new(cx, |cx| {
                    Switch::new(cx, emphasize_status)
                        .on_toggle(|cx| cx.emit(DynamicTableEvent::ToggleOrder))
                        .id("status_first");
                    Label::new(cx, Localized::new("table-prioritize-status"))
                        .describing("status_first");
                })
                .size(Auto)
                .gap(Pixels(6.0))
                .alignment(Alignment::Center);
            })
            .height(Auto)
            .gap(Pixels(14.0));

            Label::new(cx, status_text).class("table-cell-meta").height(Auto);

            Table::new(cx, sorted_rows, columns, |row: &RowData| row.id)
                .sort_state(sort_state)
                .sort_cycle(TableSortCycle::TriState)
                .resizable_columns(true)
                .selectable(Selectable::Single)
                .selected_row_ids(selected_rows)
                .on_sort(|cx, key, direction| {
                    cx.emit(DynamicTableEvent::SetSortState(key, direction));
                })
                .on_row_select(|cx, id| cx.emit(DynamicTableEvent::SelectRow(id)))
                .width(Stretch(1.0))
                .height(Stretch(1.0));
        })
        .size(Stretch(1.0))
        .padding(Pixels(12.0))
        .gap(Pixels(8.0));
    })
    .title(Localized::new("view-title-table"))
    .inner_size((1100, 720))
    .run()
}
