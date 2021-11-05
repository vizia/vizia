use vizia::*;

// INCOMPLETE!

fn main() {
    Application::new(|cx|{

        TodoData {
            items: vec![
                    TodoItem {
                        text: "Item 1".to_string(),
                        completed: false,
                    },

                    TodoItem {
                        text: "Item 2".to_string(),
                        completed: true,
                    }
                ],
            selected: 0,
        }.build(cx);

        VStack::new(cx, |cx|{
            HStack::new(cx, |cx|{
                Label::new(cx, "Enter a todo item...");
                Button::new(cx, |_|{}, |_|{});
            });

            List::new(cx, TodoData::items, |cx, item|{
                
                //let item = item.clone();
                Binding::new(cx, TodoData::selected, move |cx, selected|{
                    let item_clone = item.clone();
                    let selected = *selected.get(cx);
                    HStack::new(cx, move |cx|{
                        Label::new(cx, &item_clone.value(cx).text.to_owned()).width(Stretch(1.0));
                        Checkbox::new(cx, item_clone.value(cx).completed);
                        //Label::new(cx, &item_clone.value(cx).completed.to_string());
                    }).background_color(
                        if selected == item.index() {
                            Color::green()
                        } else {
                            Color::blue()
                        }
                    ).width(Stretch(1.0));
                }).width(Stretch(1.0));
            }).size(Stretch(1.0));

        }).width(Stretch(1.0)).height(Stretch(1.0));

    }).run();
}

#[derive(Lens, Clone)]
pub struct TodoItem {
    text: String,
    completed: bool,
}

#[derive(Lens)]
pub struct TodoData {
    items: Vec<TodoItem>,
    selected: usize,
}

impl Model for TodoData {

}