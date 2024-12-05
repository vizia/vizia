#[allow(unused_imports)]
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    name: String,
    emails: i32,
}

pub enum AppEvent {
    SetName(String),
    ReceiveEmail,
    ToggleLanguage,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetName(s) => self.name = s.clone(),
            AppEvent::ReceiveEmail => self.emails += 1,
            AppEvent::ToggleLanguage => {
                if cx.environment().locale != "fr" {
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
        );

        // Add fluent file for the `fr` locale (French).
        cx.add_translation(
            "fr".parse().unwrap(),
            include_str!("resources/translations/fr/hello.ftl").to_owned(),
        );

        AppData { name: "Audrey".to_owned(), emails: 1 }.build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Checkbox::new(cx, Environment::locale.map(|locale| *locale == "fr"))
                    .id("toggle-language")
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleLanguage));
                Label::new(cx, "Toggle Language").describing("toggle-language");
            })
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(10.0))
            .height(Auto);

            // Use the `Localized` type with a `Label` to provide a translation key.
            // The key is used to look up the corresponding translation from the fluent file.
            Label::new(cx, Localized::new("hello-world"));

            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("enter-name"));
                Textbox::new(cx, AppData::name).width(Units::Pixels(300.0)).on_edit(|cx, text| {
                    cx.emit(AppEvent::SetName(text));
                });
            })
            .alignment(Alignment::Center)
            .height(Auto)
            .horizontal_gap(Pixels(5.0));

            Label::new(cx, Localized::new("intro").arg("name", AppData::name));

            // Use the `arg` method on the `Localized` type to supply a variable argument or appropriate lens.
            // When localization is resolved the argument will be used with the fluent file to select an appropriate translation.
            Label::new(cx, Localized::new("emails").arg("unread_emails", AppData::emails));

            Button::new(cx, |cx| Label::new(cx, Localized::new("refresh")))
                .on_press(|cx| cx.emit(AppEvent::ReceiveEmail));
        })
        .vertical_gap(Pixels(10.0))
        .space(Pixels(10.0));
    })
    .title("Localization")
    .run()
}
