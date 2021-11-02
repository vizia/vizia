use vizia::*;

fn main() {

    Application::new(|cx|{
        
HStack::new(cx, move |cx| {
    let hello = "hello".to_string().build(cx);
    let world = "world".to_string().build(cx);
    VStack::new(cx, move |cx|{
        Label::new(cx, &hello.get(cx).to_owned()).background_color(
            if hello.get(cx) == "hello" {
                Color::red()
            } else {
                Color::blue()
            }
        );
        Label::new(cx, &format!("{} {}", hello.get(cx), world.get(cx)));
        let hello = hello.clone();
        Button::new(cx, move |cx| hello.set(cx, |v| *v = "goodbye".to_string()), |cx| {});
    });
});
    }).run();
}