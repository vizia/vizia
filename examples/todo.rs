use vizia::*;

// NOT CURRENTLY WORKING!

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

        List::new(cx, TodoData::items, |cx, item|{
            
            //let item = item.clone();
            Binding::new(cx, TodoData::selected, move |cx, selected|{
                //let item_clone = item.clone();
                //let selected = *selected.get(cx);
                // HStack::new(cx, move |cx|{
                //     Label::new(cx, &item_clone.value(cx).text.to_owned());
                //     Label::new(cx, &item_clone.value(cx).completed.to_string());
                // }).background_color(
                //     if selected == item.index() {
                //         Color::green()
                //     } else {
                //         Color::blue()
                //     }
                // );
            });
        });
    
        // Binding::new(TodoData::items).build(cx, |cx, items|{
        //     HStack::new().build(cx, |cx|{
        //         Button::new(|cx| items.get(cx))
        //     });      
        // });
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