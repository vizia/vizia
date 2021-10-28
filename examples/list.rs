use vizia::*;

fn main() {
    Application::new(|cx|{
        // List of 10 items 
        List::new(cx, 10, |cx, index|{
            HStack::new(cx, move |cx|{
                Label::new(cx, "Hello");
                Label::new(cx, "World").background_color(
                    if index == 5 {
                        Color::green()
                    } else {
                        Color::blue()
                    }
                );
                Label::new(cx, &index.to_string());
            });
        });
    }).run();
}