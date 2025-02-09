mod helpers;
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

#[derive(Lens)]
pub struct AppData {
    columns: Vec<String>,
    rows: Vec<RowData>,
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
        }
    }
}

impl Model for AppData {}

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
                        Label::new(cx, item.then(RowData::name))
                            .width(Stretch(1.0))
                            .text_overflow(TextOverflow::Ellipsis);
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
            .size(Pixels(400.0))
            .border_color(Color::black())
            .border_width(Pixels(1.0))
            .corner_radius(Pixels(4.0));
        });
    })
    .title("Virtual Table")
    .run()
}
