use vizia::Lens;
use vizia::*;

fn main() {
    Application::new(|cx| {
        let list: Vec<u32> = (10..22u32).collect();
        Data { 
            list,
        }.build(cx);

        Label::new(cx, "Left click on list to go down, right click on list to go up")
            .background_color(Color::rgb(100, 100, 100))
            .width(Pixels(400.0))
            .height(Pixels(100.0));

        // List of 12 items
        List::new(cx, Data::list, |cx, item| {
            Binding::new(cx, ListData::selected, move |cx, selected|{
                HStack::new(cx, move |cx| {
                    Label::new(cx, "Hello");
                    Label::new(cx, "World");
                    Label::new(cx, &item.value(cx).to_string());
                }).background_color(
                    if item.index() == *selected.get(cx) {
                        Color::green()
                    } else {
                        Color::blue()
                    }
                );
            });
        });
    })
    .run();
}

#[derive(Lens)]
pub struct Data {
    list: Vec<u32>,
}
impl Model for Data {}
