pub use vizia::prelude::*;

const STYLE: &str = r#"
    element.one {
        background-color: red;
    }

    element.one:hover {
        background-color: green;
    }

    element.two {
        background-color: blue;
    }

    element.two:hover {
        background-color: yellow;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE);
        Element::new(cx).size(Pixels(100.0)).class("one");

        Window::new(cx, |cx| {
            Element::new(cx).size(Pixels(50.0)).class("two");
            Label::new(cx, "Subwindow");
        })
        .always_on_top(true)
        .title("Secondary")
        .inner_size((400, 400));

        Label::new(cx, "Main window");
    })
    .title("Main")
    .run();
}
