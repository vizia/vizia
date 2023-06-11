use vizia::prelude::*;
use vizia_core::modifiers::LinearGradientBuilder;

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

    .grad {
        background-image: linear-gradient(0.25turn, #3f87a6, #ebf8e1, #f69d3c);
    }

    .grad:hover {
        background-image: linear-gradient(0.5turn, #3f87a6, #ebf8e1, #f69d3c);
        transition: background-image 500ms;
    }

    .grad2 {
        background-image: radial-gradient(cyan 0%, transparent 20%, salmon 40%);
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        // Element::new(cx).class("linear-gradient");
        // Element::new(cx).class("grad2").width(Pixels(200.0));

        Element::new(cx).background_gradient(
            LinearGradientBuilder::with_direction("to right")
                .add_stop(Color::red())
                .add_stop(Color::blue()),
        );
    })
    .run();
}
