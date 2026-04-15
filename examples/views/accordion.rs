use vizia::prelude::*;
mod helpers;
use helpers::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let items = Signal::new(vec![
            (
                "Project overview".to_string(),
                "Vizia is a declarative GUI framework for desktop applications.".to_string(),
            ),
            (
                "Installation".to_string(),
                "Add `vizia` to your dependencies and run with either winit or baseview backend."
                    .to_string(),
            ),
            (
                "Styling".to_string(),
                "Use CSS-like stylesheets and class selectors to customize your UI.".to_string(),
            ),
        ]);
        let open_index = Signal::new(Some(0));

        ExamplePage::vertical(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Toggle second section")).on_press(move |_cx| {
                open_index.update(|index| *index = if *index == Some(1) { None } else { Some(1) });
            });

            Accordion::new(cx, items, |_cx, _index, item| {
                let header_text = item.0;
                let content_text = item.1;

                AccordionPair::new(
                    move |cx| {
                        Label::new(cx, header_text.clone()).hoverable(false);
                    },
                    move |cx| {
                        Label::new(cx, content_text.clone()).hoverable(false);
                    },
                )
            })
            .with_open(open_index)
            .width(Pixels(420.0));
        });
    })
    .title("Accordion")
    .run()
}
