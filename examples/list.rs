use vizia::*;

fn main() {
    Application::new(|cx|{

        Data {
            list: vec![0; 10],
        }.build(cx);

        // List of 10 items 
        List::new(cx, Data::list, |cx, index, item|{
            /// Due to lifetime issues the only way I can get this to work is by cloning the item
            /// This is because the item comes from context which is 'static so the ref to item needs to be static too
            /// however, making it explicitely static then results in a borrow error because context is borrowe
            /// immutably for the item and mutably for the build closures.
            let item = item.clone();
            HStack::new(cx, move |cx|{
                let item = item;
                Label::new(cx, "Hello");
                Label::new(cx, "World").background_color(
                    if index == 5 {
                        Color::green()
                    } else {
                        Color::blue()
                    }
                );
                Label::new(cx, &item.to_string());
            });
        });
    }).run();
}

#[derive(Lens)]
pub struct Data {
    list: Vec<u32>,
}

impl Model for Data {

}