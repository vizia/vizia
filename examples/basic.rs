use vizia::*;

fn main() {

    Application::new(|cx|{
            HStack::new().build(cx, |cx| {
                Label::new("Hello").build(cx);
                Label::new("World").build(cx);
            });
    }).run();
}