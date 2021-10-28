use vizia::*;

fn main() {

    Application::new(|cx|{
        HStack::new(cx, |cx| {
            let hello = "hello".to_string().build(cx);
            let world = "world".to_string().build(cx);
            Label::new(cx, &hello.get(cx).to_owned());
            Label::new(cx, &format!("{} {}", hello.get(cx), world.get(cx)));
            Button::new(cx, move |cx| hello.set(cx, |v| *v = "goodbye".to_string()), |cx| {});
        });
    }).run();
}