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
    }

    .foo {
        size: 100px;
        background-color: red;
        transform: translate(0px, 0px);
    }

    
    .translate:hover {
        transform: translate(10px, 10px);
        transition: transform 500ms;
    }

    .translateX:hover {
        transform: translateX(10px);
        transition: transform 500ms;
    }

    .translateY:hover {
        transform: translateY(10px);
        transition: transform 500ms;
    }
    

    .rotate:hover {
        transform: rotate(40deg);
        transition: transform 500ms;
    }

    .scale:hover {
        transform: scale(1.5, 0.5);
        transition: transform 500ms;
    }

    .scaleX:hover {
        transform: scaleX(1.5);
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
        transform: translate(10px, 15px) rotate(40deg) scale(1.5, 0.5) skew(30deg, 20deg);
        transition: transform 500ms;
    }

    .matrix:hover {
        transform: matrix(1, 2, 3, 4, 5, 6);
        transition: transform 500ms;
    }

    .translate2 {
        translate: 0px 0px;
    }

    .translate2:hover {
        translate: 10px 10px;
        transition: translate 500ms;
    }
    
    .rotate2 {
        rotate: 0deg;
    }
    
    .rotate2:hover {
        rotate: 45deg;
        transition: rotate 500ms;
    }
    
    .scale2 {
        scale: 1 1;
    }
    
    .scale2:hover {
        scale: 2 2;
        transition: scale 500ms;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx).class("translate").text("translate");
                Element::new(cx).class("translateX").text("translateX");
                Element::new(cx).class("translateY").text("translateY");
            })
            .size(Auto)
            .col_between(Pixels(10.0));
            Element::new(cx).text("rotate").class("rotate");
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
            Element::new(cx).class("matrix").text("matrix");
        })
        .child_space(Stretch(1.0))
        .row_between(Pixels(10.0));

        // VStack::new(cx, |cx| {
        //     HStack::new(cx, |cx| {
        //         Element::new(cx)
        //             .text("translate")
        //             .transform(Transform::Translate((Pixels(10.0).into(), Pixels(10.0).into())));
        //         Element::new(cx)
        //             .text("translateX")
        //             .transform(Transform::TranslateX(Pixels(10.0).into()));
        //         Element::new(cx)
        //             .text("translateY")
        //             .transform(Transform::TranslateY(Pixels(10.0).into()));
        //     })
        //     .size(Auto)
        //     .col_between(Pixels(10.0));
        //     Element::new(cx).text("rotate").transform(Transform::Rotate(Angle::Deg(40.0)));
        //     HStack::new(cx, |cx| {
        //         Element::new(cx)
        //             .text("scale")
        //             .transform(Transform::Scale((1.5.into(), 0.5.into())));
        //         Element::new(cx).text("scaleX").transform(Transform::ScaleX(1.5.into()));
        //         Element::new(cx).text("scaleY").transform(Transform::ScaleY(0.5.into()));
        //     })
        //     .size(Auto)
        //     .col_between(Pixels(10.0));

        //     HStack::new(cx, |cx| {
        //         Element::new(cx)
        //             .text("skew")
        //             .transform(Transform::Skew(Angle::Deg(30.0), Angle::Deg(20.0)));
        //         Element::new(cx).text("skewX").transform(Transform::SkewX(Angle::Deg(30.0)));
        //         Element::new(cx).text("skewY").transform(Transform::SkewY(Angle::Deg(20.0)));
        //     })
        //     .size(Auto)
        //     .col_between(Pixels(10.0));
        //     Element::new(cx).text("multi").transform([
        //         Transform::Translate((Pixels(10.0).into(), Pixels(10.0).into())),
        //         Transform::Rotate(Angle::Deg(40.0)),
        //         Transform::Scale((1.5.into(), 0.5.into())),
        //         Transform::Skew(Angle::Deg(30.0), Angle::Deg(20.0)),
        //     ]);
        //     Element::new(cx)
        //         .text("matrix")
        //         .transform(Transform::Matrix((1.0, 2.0, 3.0, 4.0, 5.0, 6.0).into()));
        // })
        // .child_space(Stretch(1.0))
        // .row_between(Pixels(10.0));

        // Element::new(cx).class("translate2");
        // Element::new(cx).class("rotate2");
        // Element::new(cx).class("scale2");

        // Element::new(cx).translate((Percentage(10.0), Pixels(10.0)));
        // Element::new(cx).rotate(Angle::Deg(40.0));
        // Element::new(cx).scale((0.5, 0.5));
    })
    .run();
}
