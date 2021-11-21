use vizia::*;


// Example of a more complex app with multiple composed levels
fn main() {

    Application::new(|cx|{
        VStack::new(cx, |cx| {
            Label::new(cx, "One");
            Label::new(cx, "Two");
        });

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Three");
                Label::new(cx, "Four");
            });
            Label::new(cx, "Five");
            Label::new(cx, "Six");
        });
    }).run();
}