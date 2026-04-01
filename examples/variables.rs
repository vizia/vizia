use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        --custom-color:rgb(242, 255, 0);
        --other: var(--custom-color);
        --text-color: rgb(255, 0, 255);
    }

    hstack {
        size: 100px;

        --test: var(--other);
        color: var(--text-color);
        background-color: var(--custom-color);
        transition: --custom-color 1s;
    }

    hstack:hover {
        --custom-color: rgb(0, 255, 255);
        --test: rgb(255, 0, 0);
        background-color: var(--custom-color);
        transition: --custom-color 1s;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed");

        HStack::new(cx, |cx| {
            Label::new(cx, "text");
        });
    })
    .run()
}
