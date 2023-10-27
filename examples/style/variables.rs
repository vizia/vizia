use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        --custom-color: #FF0000;
        --other: #FF8000;
    }

    .two {
        --custom-color: var(--other);
        --other: #FF00FF;
    }

    .two:over {
        --other: #00FFFF;
    }

    .foo {
        background-color: var(--custom-color);
    }

    .foo:hover {
        background-color: var(--other);
        transition: background-color 1s;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE);

        HStack::new(cx, |cx| {
            Label::new(cx, "This should be green").class("foo");
        })
        .size(Pixels(100.0))
        .class("two");

        Label::new(cx, "This should be red").class("foo");
    })
    .run();
}
