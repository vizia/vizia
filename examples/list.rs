
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
    Application::new(WindowDescription::new().with_title("List"), |cx| {

        cx.add_theme(STYLE);

        let list: Vec<u32> = (10..14u32).collect();
        Data { 
            list,
        }.build(cx);

        HStack::new(cx, |cx|{
            Button::new(cx, |cx| cx.emit(AppEvent::Add(20)), |cx| {Label::new(cx, "Add")});
            Button::new(cx, |cx| cx.emit(AppEvent::Remove), |cx| {Label::new(cx, "Remove")});
        }).height(Auto);

        // List of 12 items
        List::new(cx, Data::list, |cx, item| {
            VStack::new(cx, move |cx|{
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
                });
            }).height(Auto).background_color(Color::red());
        }).class("list");
    })
    .run();
}

#[derive(Lens)]
pub struct Data {
    list: Vec<u32>,
}

#[derive(Debug)]
pub enum AppEvent {
    Add(u32),
    Remove,
}

impl Model for Data {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::Add(val) => {
                    self.list.push(*val);
                }

                AppEvent::Remove => {
                    self.list.pop();
                }
            }
        }
    }
}


