use vizia::*;

fn main() {
    Application::new(|cx|{
        List::new(cx, 10, |cx, index|{
            //Binding::new(TodoData::selected).build(cx, |cx, selected|{
                HStack::new(cx, move |cx|{
                    Label::new(cx, &index.to_string());
                    //Checkbox::new(item.completed).build(cx);
                });
            //});
        });

        //Binding::new(TodoData::items).build(cx, |cx, items|{
            HStack::new(cx, |cx|{
                Button::new(cx, |_| {}, |cx| {
                    Label::new(cx, "Add");
                });
            });
        //});        
    }).run();

}

// struct TodoItem {
//     text: String,
//     completed: bool,
// }

// struct TodoData {
//     items: ObservableList<TodoItem>,
//     selected: usize,
// }