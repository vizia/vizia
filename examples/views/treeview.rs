use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        Label::new(cx, "Coming soon...");
    })
    .run()
}
