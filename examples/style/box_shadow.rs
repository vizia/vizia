use vizia::prelude::*;
use vizia_core::{modifiers::BoxShadowBuilder, style::BoxShadow};

const STYLE: &str = r#"

    :root {
        child-space: 1s;
    }

    hstack {
        child-space: 1s;
        col-between: 40px;
    }

    element {
        size: 100px;
        background-color: rgb(200, 200, 200);
    }

    .shadow-offsetx {
        box-shadow: 5px 0px red;
    }

    .shadow-offsetx:hover {
        box-shadow: 15px 0px red;
        transition: box-shadow 100ms;
    }

    .shadow-offsety {
        box-shadow: 0px 5px red;
    }

    .shadow-offsety:hover {
        box-shadow: 0px 15px red;
        transition: box-shadow 100ms;
    }

    .shadow-offset {
        box-shadow: 5px 5px red;
    }

    .shadow-offset:hover {
        box-shadow: 15px 15px red;
        transition: box-shadow 100ms;
    }

    .shadow-color {
        box-shadow: 5px 5px red;
    }

    .shadow-color:hover {
        box-shadow: 5px 5px blue;
        transition: box-shadow 200ms;
    }

    .shadow-blur {
        box-shadow: 5px 5px 5px red;
    }

    .shadow-blur:hover {
        box-shadow: 5px 5px 15px red;
        transition: box-shadow 100ms;
    }
    
    .shadow {
        box-shadow: 5px 5px blue, 10px 10px red, 15px 15px green;
    }

    .shadow:hover {
        box-shadow: 10px 10px 16px 8px blue, 20px 20px 16px 8px red, 30px 30px 16px 8px green;
        transition: box-shadow 200ms;
    }

    .shadow-inset-offsetx {
        box-shadow: 5px 0px red inset;
    }

    .shadow-inset-offsetx:hover {
        box-shadow: 15px 0px red inset;
        transition: box-shadow 100ms;
    }

    .shadow-inset-offsety {
        box-shadow: 0px 5px red inset;
    }

    .shadow-inset-offsety:hover {
        box-shadow: 0px 15px red inset;
        transition: box-shadow 100ms;
    }

    .shadow-inset-offset {
        box-shadow: 5px 5px red inset;
    }

    .shadow-inset-offset:hover {
        box-shadow: 15px 15px red inset;
        transition: box-shadow 100ms;
    }

    .shadow-inset-color {
        box-shadow: 5px 5px red inset;
    }

    .shadow-inset-color:hover {
        box-shadow: 5px 5px blue inset;
        transition: box-shadow 200ms;
    }

    .shadow-inset-blur {
        box-shadow: 5px 5px 5px red inset;
    }

    .shadow-inset-blur:hover {
        box-shadow: 5px 5px 15px red inset;
        transition: box-shadow 100ms;
    }

    .shadow-inset {
        box-shadow: 5px 5px blue inset, 10px 10px red inset, 15px 15px green inset;
    }

    .shadow-inset:hover {
        box-shadow: 10px 10px 16px blue inset, 20px 20px 16px red inset, 30px 30px 16px green inset;
        transition: box-shadow 200ms;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        HStack::new(cx, |cx| {
            Element::new(cx).class("shadow-offsetx");
            Element::new(cx).class("shadow-offsety");
            Element::new(cx).class("shadow-offset");
            Element::new(cx).class("shadow-color");
            Element::new(cx).class("shadow-blur");
            Element::new(cx).class("shadow");
        });

        HStack::new(cx, |cx| {
            Element::new(cx).class("shadow-inset-offsetx");
            Element::new(cx).class("shadow-inset-offsety");
            Element::new(cx).class("shadow-inset-offset");
            Element::new(cx).class("shadow-inset-color");
            Element::new(cx).class("shadow-inset-blur");
            Element::new(cx).class("shadow-inset");
        });

        Element::new(cx)
            .box_shadow(BoxShadowBuilder::new().x_offset(5.0).y_offset(5.0).color(Color::black()))
            .box_shadow(BoxShadow::new(
                Length::px(10.0),
                Length::px(10.0),
                None,
                None,
                Some(Color::red()),
                false,
            ));
    })
    .title("Box Shadows")
    .inner_size((1000, 600))
    .run();
}
