use vizia::prelude::*;

const STYLE: &str = r#"

    :root {
        child-space: 1s;
    }

    element {
        size: 100px;
        background-color: rgb(200, 200, 200);
    }

    .linear-gradient {
        background-image: linear-gradient(rgb(200, 200, 200), rgb(100, 100, 100)), linear-gradient(to right, transparent, transparent);
    }

    .linear-gradient:hover {
        background-image: linear-gradient(red, yellow), linear-gradient(to right, #0000FF80, #00FF0080);
        transition: background-image 0.5s;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        Element::new(cx).class("linear-gradient");
    })
    .run();
}
