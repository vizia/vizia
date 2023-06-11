use vizia::prelude::*;

const STYLE: &str = r#"

    :root {
        layout-type: row;
        col-between: 40px;
        child-space: 1s;
    }

    * {
        border-radius: 20px;
    }

    element {
        size: 100px;
        background-color: rgb(200, 200, 200);
        child-space: 1s;
        color: #181818;
    }

    #id {
        background-color: rgb(230, 200, 200);
    }

    .class {
        background-color: rgb(200, 230, 200);
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        Element::new(cx).text("element");
        Element::new(cx).id("id").text("id");
        Element::new(cx).class("class").text("class");
    })
    .title("Combinators")
    .inner_size((800, 400))
    .run();
}
