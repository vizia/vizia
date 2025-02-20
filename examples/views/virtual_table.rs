mod helpers;
use std::vec;

use fake::{Fake, Faker};
use helpers::*;
use vizia::prelude::*;

#[derive(Lens, Clone, Data)]
pub struct RowData {
    id: usize,
    name: String,
    price: f32,
    market_cap: f32,
    volume: f32,
}

fn random_data(size: usize) -> Vec<RowData> {
    (0..size)
        .map(|id| RowData {
            id,
            name: Faker.fake::<String>(),
            price: (0.0..1000.0).fake(),
            market_cap: (0.0..1000.0).fake(),
            volume: (0.0..1000.0).fake(),
        })
        .collect()
}

pub enum AppEvent {
    SortColumn(usize, bool),
    ReorderColumn(usize, usize),
    SelectRow(usize),
}

#[derive(Lens)]
pub struct AppData {
    columns: Vec<String>,
    rows: Vec<RowData>,
    selected: usize,
}

impl AppData {
    pub fn new(size: usize) -> Self {
        Self {
            columns: vec![
                String::from("ID"),
                String::from("Name"),
                String::from("Price"),
                String::from("Market Cap"),
                String::from("Volume"),
            ],
            rows: random_data(size),
            selected: 0,
        }
    }
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|app_event, _| match app_event {
            AppEvent::SortColumn(index, asc) => {
                self.rows.sort_by(|a, b| match index {
                    0 => {
                        if asc {
                            a.id.cmp(&b.id)
                        } else {
                            b.id.cmp(&a.id)
                        }
                    }
                    1 => {
                        if asc {
                            a.name.cmp(&b.name)
                        } else {
                            b.name.cmp(&a.name)
                        }
                    }
                    2 => {
                        if asc {
                            a.price.partial_cmp(&b.price).unwrap()
                        } else {
                            b.price.partial_cmp(&a.price).unwrap()
                        }
                    }
                    3 => {
                        if asc {
                            a.market_cap.partial_cmp(&b.market_cap).unwrap()
                        } else {
                            b.market_cap.partial_cmp(&a.market_cap).unwrap()
                        }
                    }
                    4 => {
                        if asc {
                            a.volume.partial_cmp(&b.volume).unwrap()
                        } else {
                            b.volume.partial_cmp(&a.volume).unwrap()
                        }
                    }
                    _ => std::cmp::Ordering::Equal,
                });
            }

            AppEvent::ReorderColumn(from, to) => {
                let column = self.columns.remove(from);
                self.columns.insert(to, column);
            }

            AppEvent::SelectRow(index) => {
                self.selected = index;
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData::new(10000).build(cx);

        ExamplePage::new(cx, |cx| {
            VirtualTable::new(
                cx,
                AppData::columns,
                AppData::rows,
                40.0,
                |cx, _, item| {
                    Label::new(cx, item);
                },
                |cx, index, item| match index {
                    0 => {
                        Label::new(cx, item.then(RowData::id));
                    }
                    1 => {
                        Label::new(cx, item.then(RowData::name));
                    }
                    2 => {
                        Label::new(cx, item.then(RowData::price));
                    }
                    3 => {
                        Label::new(cx, item.then(RowData::market_cap));
                    }
                    4 => {
                        Label::new(cx, item.then(RowData::volume));
                    }
                    _ => {}
                },
            )
            .selectable(Selectable::Single)
            .selected(AppData::selected.map(|s| vec![*s]))
            .on_select(|cx, index| cx.emit(AppEvent::SelectRow(index)))
            .selection_follows_focus(true)
            .on_sort_column(|cx, column, asc| {
                cx.emit(AppEvent::SortColumn(column, asc));
            })
            .size(Pixels(400.0))
            .border_color(Color::gray())
            .border_width(Pixels(1.0))
            .corner_radius(Pixels(4.0));
        });
    })
    .title("Virtual Table")
    .run()
}
