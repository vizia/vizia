use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        --custom-color:rgb(242, 255, 0);
        --other: var(--custom-color);
    }

    hstack {
        size: 100px;
        --custom-color: rgb(0, 255, 0);
        --test: var(--other);
        
        background-color: var(--test);
        transition: background-color 0.5s ease-in-out;
    }

    hstack:hover {
        --custom-color: rgb(0, 255, 255);
        --test: rgb(255, 0, 0);
        background-color: var(--test);
        transition: background-color 0.5s ease-in-out;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE);

        HStack::new(cx, |cx| {
            // Label::new(cx, "text");
        });
    })
    .run()
}
