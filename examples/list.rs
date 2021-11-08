
use vizia::*;

const STYLE: &str = r#"
    .list {
        width: 1s;
        height: 1s;
        background-color: #CCCCCC;
        row-between: 1px;
    }
"#;



fn main() {
    Application::new(|cx| {

        cx.add_theme(STYLE);

        let list: Vec<u32> = (10..22u32).collect();
        Data { 
            list,
        }.build(cx);

        // List of 12 items
        List::new(cx, Data::list, |cx, item| {
            Binding::new(cx, ListData::selected, move |cx, selected|{
                let item = item.clone();
                HStack::new(cx, move |cx| {
                    Label::new(cx, "Hello").width(Stretch(1.0));
                    Label::new(cx, "World");
                    Label::new(cx, &item.value(cx).to_string());
                    //Label::new(cx, &item.index().to_string());
                })
                .background_color(
                    if item.index() == *selected.get(cx) {
                        Color::rgb(50, 200, 50)
                    } else {
                        Color::rgb(255,255,255)
                    }
                )
                .height(Auto)
                .width(Stretch(1.0))
                .on_press(cx, move |cx| cx.emit(ListEvent::SetSelected(item.index())));
            }).height(Auto).width(Stretch(1.0));
        }).class("list");
    })
    .run();
}

#[derive(Lens)]
pub struct Data {
    list: Vec<u32>,
}
impl Model for Data {}


