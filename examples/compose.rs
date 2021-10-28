use vizia::*;

fn other_view(cx: &mut Context) {
    Label::new(cx, "One");
}

fn custom_view(cx: &mut Context) {
    VStack::new(cx, |cx| {
        other_view(cx);
        Label::new(cx, "Two");
        Label::new(cx, "Three");
    });
}

fn main() {

    Application::new(|cx|{
        HStack::new(cx, |cx|{
            custom_view(cx);
            custom_view(cx);
        });
    }).run();
}