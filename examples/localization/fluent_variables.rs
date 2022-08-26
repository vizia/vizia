// Example which demonstrates a basic text translation using fluent.
#[allow(unused_imports)]
use vizia::prelude::*;

#[cfg(not(feature = "localization"))]
fn main() {
    panic!("This example requires the 'localization' feature!");
}

#[derive(Lens)]
pub struct AppData {
    user: String,
}

impl Model for AppData {}

#[cfg(feature = "localization")]
fn main() {
    Application::new(|cx| {
        AppData { user: String::from("Jane") }.build(cx);

        // Add fluent file for the `fr` locale (french).
        cx.add_translation(langid!("fr"), include_str!("../resources/fr/hello.ftl"));

        // Force application to use the `fr` locale.
        cx.emit(EnvironmentEvent::SetLocale(langid!("fr")));

        // Use the `Localized` type with a `Label` to provide a translation key.
        // The key is used to look up the corresponding translation from the fluent file.
        Label::new(cx, Localized::new("intro").arg("name", AppData::user));
        Label::new(cx, Localized::new("emails").arg_const("unread_emails", 5));
    })
    .run();
}
