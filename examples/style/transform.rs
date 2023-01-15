use vizia::prelude::*;

const STYLE: &str = r#"
    element {
        size: 100px;
        background-color: red;
        transform: translate(0px, 0px);
    }

    .foo {
        size: 100px;
        background-color: red;
        transform: translate(0px, 0px);
    }

    .translate:hover {
        transform: translate(40px, 20px);
        transition: transform 500ms;
    }

    .translateX:hover {
        transform: translateX(40px);
        transition: transform 500ms;
    }

    .translateY:hover {
        transform: translateY(40px);
        transition: transform 500ms;
    }

    .rotate:hover {
        transform: rotate(90deg);
        transition: transform 500ms;
    }

    .scale:hover {
        transform: scale(2, 0.5);
        transition: transform 500ms;
    }

    .scaleX:hover {
        transform: scaleX(2);
        transition: transform 500ms;
    }

    .scaleY:hover {
        transform: scaleY(0.5);
        transition: transform 500ms;
    }

    .skew:hover {
        transform: skew(30deg, 20deg);
        transition: transform 500ms;
    }

    .skewX:hover {
        transform: skewX(30deg);
        transition: transform 500ms;
    }

    .skewY:hover {
        transform: skewY(20deg);
        transition: transform 500ms;
    }

    .multi:hover {
        transform: translate(40px, 20px) rotate(40deg) scale(2, 0.5) skew(30deg, 20deg);
        transition: transform 500ms;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx).class("translate").text("translate");
                Element::new(cx).class("translateX").text("translateX");
                Element::new(cx).class("translateY").text("translateY");
            })
            .size(Auto)
            .col_between(Pixels(10.0));
            HStack::new(cx, |cx| {
                Label::new(cx, "rotate").background_color(RGBA::GREEN);
            })
            .class("foo")
            .class("rotate");
            HStack::new(cx, |cx| {
                Element::new(cx).class("scale").text("scale");
                Element::new(cx).class("scaleX").text("scaleX");
                Element::new(cx).class("scaleY").text("scaleY");
            })
            .size(Auto)
            .col_between(Pixels(10.0));

            HStack::new(cx, |cx| {
                Element::new(cx).class("skew").text("skew");
                Element::new(cx).class("skewX").text("skewX");
                Element::new(cx).class("skewY").text("skewY");
            })
            .size(Auto)
            .col_between(Pixels(10.0));
            Element::new(cx).class("multi").text("multi");
        })
        .child_space(Stretch(1.0))
        .row_between(Pixels(10.0));
    })
    .run();
}
