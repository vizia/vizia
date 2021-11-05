


use std::rc::Rc;

use morphorm::LayoutType;
use vizia::*;

fn main() {
    Application::new(|cx| {

        TableData {
            table_data: (0..25).collect(),
        }.build(cx);

        //Label::new(cx, "Test");

        Table::new(cx, 5, TableData::table_data, |cx, width, item|{
            println!("{} {}", item.row(), item.col());
            VStack::new(cx, move |cx|{
                Label::new(cx, &item.value(cx).to_string()).width(Stretch(1.0)).height(Stretch(1.0));
                Label::new(cx, &item.index().to_string()).width(Stretch(1.0)).height(Stretch(1.0));
            }).width(Stretch(1.0)).height(Stretch(1.0));
        }).width(Pixels(300.0)).height(Pixels(300.0)).space(Stretch(1.0));
    }).run();
}

#[derive(Lens)]
pub struct TableData {
    table_data: Vec<u32>,
}

impl Model for TableData {

}