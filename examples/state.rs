use vizia::*;

fn main() {

    Application::new(|cx|{
        HStack::new().build(cx, |cx| {
            let hello = "hello".to_string().build(cx);
            let world = "world".to_string().build(cx);
            Label::new(&hello.get(cx)).build(cx);
            Label::new(&format!("{} {}", hello.get(cx), world.get(cx))).build(cx);
            Button::new(move |cx| hello.set(cx, |v| *v = "goodbye".to_string())).build(cx, |cx| {});
        });
    }).run();
}