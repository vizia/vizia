use vizia::*;

fn main() {

    Application::new(|cx|{
        HStack::new(cx, |cx| {
            Label::new(cx, "Hello");
            Label::new(cx, "World");
        });
    }).run();
}