use vizia::prelude::*;

const STYLE: &str = r#"

    :root {
        child-space: 1s;
    }

    element {
        size: 100px;
        background-color: rgb(200, 200, 200);
    }

    .outline {
        border-radius: 5px 10px 15px 20px;
        outline: 5px black;
        outline-offset: 5px;
    }

    .outline:hover {
        outline: 10px blue;
        outline-offset: 10px;
        transition: outline 0.1s, outline-offset 0.1s;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        Element::new(cx).class("outline");
    })
    .run();
}
