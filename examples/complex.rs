use vizia::*;


// Example of a more complex app with multiple composed levels
fn main() {

    Application::new(|cx|{
        VStack::new(cx, |cx| {
            Label::new(cx, "");
            Label::new(cx, "");
        });

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "");
                Label::new(cx, "");
            });
            Label::new(cx, "");
            Label::new(cx, "");
        });
    }).run();
}