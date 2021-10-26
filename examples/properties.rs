use vizia::*;

fn main() {

    Application::new(|cx|{
        HStack::new().build(cx, |cx| {
            let hello = "hello".to_string();
            let world = "world".to_string();
            Label::new(&hello).build(cx);
            Label::new(&world).build(cx);

            HStack::new().build(cx, move |cx| {
                Label::new(&hello).build(cx);
                Label::new(&world).build(cx);
            });
        });
    }).run();
}