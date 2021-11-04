use vizia::*;

// Example of read-only variables local to a view

fn main() {

    Application::new(|cx|{
        HStack::new(cx, |cx| {
            let hello = "hello".to_string();
            let world = "world".to_string();
            Label::new(cx, &hello);
            Label::new(cx, &world);

            HStack::new(cx, move |cx| {
                Label::new(cx, &hello);
                Label::new(cx, &world);
            });
        });
    }).run();
}