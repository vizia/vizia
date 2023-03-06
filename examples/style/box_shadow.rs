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
        box-shadow: 10px 10px 16px 8px blue, 20px 20px 16px 8px red, 30px 30px 128px 8px green;
        transition: box-shadow 0.5s;
    }

    .inner-shadow {
        border-radius: 20px;
        box-shadow: 25px 0px red inset, 50px 0px 0px gold inset;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        Element::new(cx).class("shadow");
        // Element::new(cx).box_shadow(&[])
        // Element::new(cx).class("inner-shadow");
    })
    .run();
}
