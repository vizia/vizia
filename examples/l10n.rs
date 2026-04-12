use chrono::Utc;
#[allow(unused_imports)]
use vizia::prelude::*;

pub struct AppData {
    name: Signal<String>,
    emails: Signal<i32>,
}

pub enum AppEvent {
    SetName(String),
    ReceiveEmail,
    ToggleLanguage,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetName(s) => self.name.set(s.clone()),
            AppEvent::ReceiveEmail => self.emails.update(|emails| *emails += 1),
            AppEvent::ToggleLanguage => {
                if cx.environment().locale.get() != "fr" {
                    cx.emit(EnvironmentEvent::SetLocale("fr".parse().unwrap()));
                } else {
                    cx.emit(EnvironmentEvent::SetLocale("en-US".parse().unwrap()));
                }
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        // Add fluent file for the `en-US` locale (American English).
        cx.add_translation(
            "en-US".parse().unwrap(),
            include_str!("resources/translations/en-US/hello.ftl").to_owned(),
        )
        .expect("Failed to add en-US translation");

        // Add fluent file for the `fr` locale (French).
        cx.add_translation(
            "fr".parse().unwrap(),
            include_str!("resources/translations/fr/hello.ftl").to_owned(),
        )
        .expect("Failed to add fr translation");

        let name = Signal::new("Audrey".to_owned());
        let emails = Signal::new(1);
        let item_count = Signal::new(42);
        let price = Signal::new(99.99);
        let event_date = Utc::now();
        let release_date = Utc::now().naive_utc();

        AppData { name, emails }.build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Checkbox::new(cx, cx.environment().locale.map(|locale| *locale == "fr"))
                    .id("toggle-language")
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleLanguage));
                Label::new(cx, "Toggle Language").describing("toggle-language");
            })
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(10.0))
            .height(Auto);

            // Basic key lookup.
            Label::new(cx, Localized::new("hello-world"));

            // String mapping after localization.
            Label::new(cx, Localized::new("refresh").map(|text| text.to_uppercase()));

            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("enter-name"));
                Textbox::new(cx, name).width(Units::Pixels(300.0)).on_edit(|cx, text| {
                    cx.emit(AppEvent::SetName(text));
                });
            })
            .alignment(Alignment::Center)
            .height(Auto)
            .horizontal_gap(Pixels(5.0));

            // Variables.
            Label::new(cx, Localized::new("intro").arg("name", name));
            Label::new(cx, Localized::new("emails").arg("unread_emails", emails));

            Button::new(cx, |cx| Label::new(cx, Localized::new("refresh")))
                .on_press(|cx| cx.emit(AppEvent::ReceiveEmail));

            // Attributes.
            Label::new(cx, Localized::new("dialog").attribute("title"));
            Label::new(cx, Localized::new("dialog").attribute("prompt"));

            // Terms.
            Label::new(cx, Localized::new("brand-welcome"));

            // Message references.
            Label::new(cx, Localized::new("help-menu-save"));

            // Selectors/plurals.
            Label::new(cx, Localized::new("role-label").arg("role", "admin"));
            Label::new(cx, Localized::new("cart-summary").arg("count", 1));
            Label::new(cx, Localized::new("cart-summary").arg("count", 3));

            // Number formatting.
            Label::new(cx, Localized::new("item-count").arg("count", item_count));
            Label::new(cx, Localized::new("price-currency").arg("amount", price));
            Label::new(
                cx,
                Localized::new("price").arg("amount", number_with_fraction(price.get(), 2)),
            );
            Label::new(
                cx,
                Localized::new("percentage-complete").arg("percent", percentage(0.753, 1)),
            );

            // Date formatting with chrono (timezone-aware + naive).
            Label::new(cx, Localized::new("event-date").arg("date", event_date));
            Label::new(cx, Localized::new("last-updated").arg("date", Utc::now()));
            Label::new(cx, Localized::new("release-date").arg("date", release_date));

            // Key exists only in en-US to show per-key fallback when FR is active.
            Label::new(cx, Localized::new("fallback-only"));
        })
        .vertical_gap(Pixels(10.0))
        .space(Pixels(10.0));
    })
    .title("Localization")
    .run()
}
