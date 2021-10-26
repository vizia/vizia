use vizia::*;

fn main() {

Application::new(|cx|{
        VStack::new().build(cx, |cx| {
            HelloView::new("Hello").build(cx, |cx| {
                WorldView::new("World").build(cx, |cx|{
                    Binding::new(HelloView::value).build(cx, |cx, hello|{
                        Binding::new(WorldView::value).build(cx, |cx, world|{
                            Label::new(&format!("{} {}", hello.get(cx), world.get(cx))).build(cx);
                            Button::new(move |cx| hello.set(cx, |v| *v = "Goodbye".to_string())).build(cx, |cx| {});
                        })
                    });
                })
            });
        });
}).run();
}

// Mutating the data in a binding causes all bound view to update and rebuild their bodies


pub struct CustomView {
    value: String,
}

impl CustomView {
    pub fn new() -> Self {
        Self {
            value: "one".to_string(),
        }
    }
}

impl Model for CustomView {

}
