use vizia::prelude::*;
mod helpers;
use helpers::*;

// Define the data model for the application.
pub struct AppData {
    collapsed: Signal<bool>,
}

// Define the events for the application.
pub enum AppEvent {
    ToggleCollapse,
}

impl Model for AppData {
    /// Handles events for the application
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleCollapse => {
                self.collapsed.update(|collapsed| *collapsed = !*collapsed);
            }
        })
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let collapsed = Signal::new(false);
        AppData { collapsed }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            // Create a new button that toggles the collapsed state.
            Button::new(cx, |cx| Label::new(cx, "Toggle collapsed"))
                .on_press(|cx| cx.emit(AppEvent::ToggleCollapse));

            VStack::new(cx, |cx| {

                // Create a new collapsible view with a header and content.
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, "Click me to collapse the content").hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10").hoverable(false);
                    },
                )
                .open(collapsed);

                Divider::new(cx);

                // Create a new collapsible view with a header and content.
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, "Click me to collapse the content").hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5").text_wrap(true).hoverable(false);
                    },
                )
                .open(collapsed);

                Divider::new(cx);

                // Create a new collapsible view with a header and content.
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, "Click me to collapse the content").hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5").hoverable(false);
                        Divider::new(cx);
                        HStack::new(cx, |cx|{
                            Button::new(cx, |cx| Label::new(cx, "CANCEL")).variant(ButtonVariant::Text);
                            Button::new(cx, |cx| Label::new(cx, "SAVE")).variant(ButtonVariant::Text);
                        })
                        .height(Auto)
                        .gap(Pixels(8.0))
                        .padding_right(Pixels(8.0))
                        .alignment(Alignment::Right);
                    },
                )
                .open(collapsed);
            })
            .alignment(Alignment::TopCenter);
        });


    })
    .title("Collapsible")
    .run()
}
