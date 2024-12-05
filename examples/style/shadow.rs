use vizia::prelude::*;

const STYLE: &str = r#"

    :root {
        padding: 1s;
    }

    hstack {
        padding: 1s;
        horizontal-gap: 40px;
    }

    element {
        size: 100px;
        background-color: rgb(200, 200, 200);
    }

    .shadow-offsetx {
        shadow: 5px 0px red;
    }

    .shadow-offsetx:hover {
        shadow: 15px 0px red;
        transition: shadow 100ms;
    }

    .shadow-offsety {
        shadow: 0px 5px red;
    }

    .shadow-offsety:hover {
        shadow: 0px 15px red;
        transition: shadow 100ms;
    }

    .shadow-offset {
        shadow: 5px 5px red;
    }

    .shadow-offset:hover {
        shadow: 15px 15px red;
        transition: shadow 100ms;
    }

    .shadow-color {
        shadow: 5px 5px red;
    }

    .shadow-color:hover {
        shadow: 5px 5px blue;
        transition: shadow 200ms;
    }

    .shadow-blur {
        shadow: 5px 5px 5px red;
    }

    .shadow-blur:hover {
        shadow: 5px 5px 15px red;
        transition: shadow 100ms;
    }

    .shadow-spread {
        shadow: 0px 0px 0px 5px red;
    }

    .shadow-spread:hover {
        shadow: 0px 0px 0px 10px red;
        transition: shadow 100ms;
    }
    
    .shadow {
        shadow: 5px 5px blue, 10px 10px red, 15px 15px green;
    }

    .shadow:hover {
        shadow: 10px 10px 16px 8px blue, 20px 20px 16px 8px red, 30px 30px 16px 8px green;
        transition: shadow 200ms;
    }

    .shadow-inset-offsetx {
        shadow: 5px 0px 0px red inset;
    }

    .shadow-inset-offsetx:hover {
        shadow: 15px 0px 0px red inset;
        transition: shadow 100ms;
    }

    .shadow-inset-offsety {
        shadow: 0px 5px red inset;
    }

    .shadow-inset-offsety:hover {
        shadow: 0px 15px red inset;
        transition: shadow 100ms;
    }

    .shadow-inset-offset {
        shadow: 5px 5px red inset;
    }

    .shadow-inset-offset:hover {
        shadow: 15px 15px red inset;
        transition: shadow 100ms;
    }

    .shadow-inset-color {
        shadow: 5px 5px red inset;
    }

    .shadow-inset-color:hover {
        shadow: 5px 5px blue inset;
        transition: shadow 200ms;
    }

    .shadow-inset-blur {
        shadow: 5px 5px 5px red inset;
    }

    .shadow-inset-blur:hover {
        shadow: 5px 5px 15px red inset;
        transition: shadow 100ms;
    }

    .shadow-inset {
        shadow: 5px 5px blue inset, 10px 10px red inset, 15px 15px green inset;
    }

    .shadow-inset:hover {
        shadow: 10px 10px 16px blue inset, 20px 20px 16px red inset, 30px 30px 16px green inset;
        transition: shadow 200ms;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        HStack::new(cx, |cx| {
            Element::new(cx).class("shadow-offsetx");
            Element::new(cx).class("shadow-offsety");
            Element::new(cx).class("shadow-offset");
            Element::new(cx).class("shadow-color");
            Element::new(cx).class("shadow-blur");
            Element::new(cx).class("shadow-spread");
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
            .shadow(ShadowBuilder::new().x_offset(5.0).y_offset(5.0).color(Color::black()))
            .shadow(Shadow::new(
                Length::px(10.0),
                Length::px(10.0),
                None,
                None,
                Some(Color::red()),
                true,
            ));
    })
    .title("Box Shadow")
    .inner_size((1000, 600))
    .run()
}
