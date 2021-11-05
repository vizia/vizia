


use std::rc::Rc;

use morphorm::LayoutType;
use vizia::*;

fn main() {
    Application::new(|cx| {

        TableData {
            table_data: (0..100).collect(),
        }.build(cx);

        Table::new(cx, 10, TableData::table_data, |cx, width, item|{
            // VStack::new(cx, move |cx|{
            //     Label::new(cx, &format!("{}, {}", item.row(), item.col())).width(Stretch(1.0)).height(Stretch(1.0));
            // }).width(Stretch(1.0)).height(Stretch(1.0));
            Label::new(cx, &item.index().to_string()).width(Stretch(1.0)).height(Stretch(1.0)).background_color(Color::rgb(120, 120, 120));
        }).width(Pixels(300.0)).height(Pixels(300.0)).space(Stretch(1.0));
    }).run();
}

#[derive(Lens)]
pub struct TableData {
    table_data: Vec<u32>,
}

impl Model for TableData {

}