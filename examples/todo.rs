use vizia::*;

fn main() {
    List::new(TodoData::items).build(cx, |cx, item|{
        Binding::new(TodoData::selected).build(cx, |cx, selected|{
            HStack::new().background_color(
                if selected {Color::red()} else {Color::blue()}
            ).build(cx, |cx|{
                Label::new(&item.text).build(cx);
                Checkbox::new(item.completed).build(cx);
            });
        });
    });

    Binding::new(TodoData::items).build(cx, |cx, items|{
        HStack::new().build(cx, |cx|{
            Button::new(|cx| items.get(cx))
        });      
    });
}

struct TodoItem {
    text: String,
    completed: bool,
}

struct TodoData {
    items: ObservableList<TodoItem>,
    selected: usize,
}