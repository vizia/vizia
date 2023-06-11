use vizia::prelude::*;

const STYLE: &str = r#"

    :root {
        layout-type: row;
    }

    element {
        size: 50px;
        background-color: rgb(200, 200, 200);
        transform: translate(0px, 0px);
    }
    
    element:hover {
        background-color: red;
        transform: rotate(45deg);
        transition: transform 500ms;
    }

    .top-left {
        transform-origin: top left;
    }

    .bottom-center {
        transform-origin: top right;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx).class("top-left");
                Element::new(cx).class("bottom-center");
            })
            .size(Auto)
            .col_between(Pixels(10.0));

            // Element::new(cx).text("rotate").class("rotate");
            // HStack::new(cx, |cx| {
            //     Element::new(cx).class("scale").text("scale");
            //     Element::new(cx).class("scaleX").text("scaleX");
            //     Element::new(cx).class("scaleY").text("scaleY");
            // })
            // .size(Auto)
            // .col_between(Pixels(10.0));

            // HStack::new(cx, |cx| {
            //     Element::new(cx).class("skew").text("skew");
            //     Element::new(cx).class("skewX").text("skewX");
            //     Element::new(cx).class("skewY").text("skewY");
            // })
            // .size(Auto)
            // .col_between(Pixels(10.0));
            // Element::new(cx).class("multi").text("multi");
            // Element::new(cx).class("matrix").text("matrix");
        })
        .child_space(Stretch(1.0))
        .row_between(Pixels(10.0));
    })
    .run();
}
