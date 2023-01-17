use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

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
                if cx.environment().locale.to_string() == "en-US" {
                    cx.emit(EnvironmentEvent::SetLocale("fr".parse().unwrap()));
                } else {
                    cx.emit(EnvironmentEvent::SetLocale("en-US".parse().unwrap()));
                }
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_translation(
            "en-US".parse().unwrap(),
            include_str!("../resources/en-US/hello.ftl").to_owned(),
        );
        cx.add_translation(
            "fr".parse().unwrap(),
            include_str!("../resources/fr/hello.ftl").to_owned(),
        );

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        AppData { name: "Audrey".to_owned(), emails: 1 }.build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Checkbox::new(cx, Environment::locale.map(|locale| locale.to_string() == "en-US"))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleLanguage));
                Label::new(cx, "Toggle Language");
            })
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(10.0))
            .height(Auto);

            Label::new(cx, Localized::new("hello-world"));
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("enter-name"));
                Textbox::new(cx, AppData::name).width(Units::Pixels(300.0)).on_edit(|cx, text| {
                    cx.emit(AppEvent::SetName(text));
                });
            })
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .height(Auto)
            .col_between(Pixels(5.0));
            Label::new(cx, Localized::new("intro").arg("name", AppData::name));
            Label::new(cx, Localized::new("emails").arg("unread_emails", AppData::emails));
            Button::new(
                cx,
                |cx| cx.emit(AppEvent::ReceiveEmail),
                |cx| Label::new(cx, Localized::new("refresh")),
            );
        })
        .row_between(Pixels(10.0))
        .space(Pixels(10.0));
    })
    .title("Localization")
    .ignore_default_theme()
    .run()
}
