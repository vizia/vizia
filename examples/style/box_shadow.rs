use vizia::prelude::*;

const STYLE: &str = r#"

    :root {
        child-space: 1s;
    }

    element {
        size: 100px;
        background-color: rgb(200, 200, 200);
    }
    
    .shadow {
        box-shadow: 5px 5px blue, 10px 10px red, 15px 15px green;
    }

    .shadow:hover {
        box-shadow: 10px 10px 16px blue, 20px 20px 16px red, 30px 30px 16px green;
        transition: box-shadow 0.5s;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        Element::new(cx).class("shadow");
    })
    .run();
}
