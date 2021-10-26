use vizia::*;

fn main() {
    Application::new(|cx|{
        // List of 10 items
        List::new(10).build(cx, |cx, index|{
            HStack::new().build(cx, move |cx|{
                Label::new("Hello").build(cx);
                Label::new("World").background_color(
                    if index == 5 {
                        Color::green()
                    } else {
                        Color::blue()
                    }
                ).build(cx);
                Label::new(&index.to_string()).build(cx);
            })
        });
    }).run();
}