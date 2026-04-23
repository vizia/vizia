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
            Button::new(cx, |cx| Label::new(cx, Localized::new("collapsible-toggle")))
                .on_press(|cx| cx.emit(AppEvent::ToggleCollapse));

            VStack::new(cx, |cx| {
                // Create a new collapsible view with a header and content.
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, Localized::new("collapsible-header")).hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, Localized::new("collapsible-content-long"))
                            .width(Stretch(1.0))
                            .hoverable(false);
                    },
                )
                .open(collapsed);

                Divider::new(cx);

                // Create a new collapsible view with a header and content.
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, Localized::new("collapsible-header")).hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, Localized::new("collapsible-content-short"))
                            .width(Stretch(1.0))
                            .hoverable(false);
                    },
                )
                .open(collapsed);

                Divider::new(cx);

                // Create a new collapsible view with a header and content.
                Collapsible::new(
                    cx,
                    |cx| {
                        Label::new(cx, Localized::new("collapsible-header")).hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, Localized::new("collapsible-content-short"))
                            .width(Stretch(1.0))
                            .hoverable(false);
                        Divider::new(cx);
                        HStack::new(cx, |cx| {
                            Button::new(cx, |cx| Label::new(cx, Localized::new("action-cancel")))
                                .variant(ButtonVariant::Secondary);
                            Button::new(cx, |cx| Label::new(cx, Localized::new("action-save")))
                                .variant(ButtonVariant::Secondary);
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
    .title(Localized::new("view-title-collapsible"))
    .run()
}
