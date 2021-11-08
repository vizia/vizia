use vizia::*;

fn main() {
    Application::new(|cx|{
        cx.add_stylesheet("examples/resources/test.css").expect("Failed to find file");

        Button::new(cx, |_|{}, |_|{});

    }).run();
}