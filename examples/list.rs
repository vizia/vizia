use vizia::Lens;
use vizia::*;

fn main() {
    Application::new(|cx| {
        let list: Vec<u32> = (10..22u32).collect();
        Data { list }.build(cx);

        // List of 12 items
        List::new(cx, Data::list, |cx, item| {
            HStack::new(cx, move |cx| {
                Label::new(cx, "Hello");
                Label::new(cx, "World").background_color(if item.index() == 5 {
                    Color::green()
                } else {
                    Color::blue()
                });
                let label = item.value(&cx).to_string();
                Label::new(cx, &label);
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
