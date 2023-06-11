use vizia::prelude::*;

const STYLE: &str = r#"

    .row {
        child-space: 1s;
        col-between: 20px;
    }

    element {
        size: 100px;
        background-color: rgb(200, 200, 200);
    }

    .border {
        border: black 5px;
    }

    .border:hover {
        border: 10px blue;
        transition: border 0.1s;
    }

    .border_radius {
        border-radius: 5px 10px 15px 20px;
    }

    .border_radius:hover {
        border-radius: 10px 20px 30px 40px;
        transition: border-radius 0.1s;
    }

    .border_shape {
        border-radius: 30px;
        border-corner-shape: round bevel round bevel;
    }

    .border_shape:hover {
        border-radius: 30px;
        border-corner-shape: bevel round bevel round;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        HStack::new(cx, |cx| {
            Element::new(cx).class("border");
            Element::new(cx).class("border_radius");
            Element::new(cx).class("border_shape");
        })
        .class("row");

        HStack::new(cx, |cx| {
            Element::new(cx).border_color(Color::black()).border_width(Pixels(10.0));

            Element::new(cx).border_radius((
                Length::Value(LengthValue::Px(5.0)),
                Pixels(20.0),
                "30px",
                LengthValue::Px(40.0),
            ));

            Element::new(cx).border_radius(Pixels(30.0)).border_corner_shape((
                BorderCornerShape::Bevel,
                BorderCornerShape::Round,
                BorderCornerShape::Bevel,
                BorderCornerShape::Round,
            ));
        })
        .class("row");
    })
    .run();
}
