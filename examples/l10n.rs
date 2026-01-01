#[allow(unused_imports)]
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
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

        let name = cx.state("Audrey".to_owned());
        let emails = cx.state(1i32);
        let title = cx.state("Localization".to_string());
        let align_center = cx.state(Alignment::Center);
        let gap_10 = cx.state(Pixels(10.0));
        let gap_5 = cx.state(Pixels(5.0));
        let height_auto = cx.state(Auto);
        let width_300 = cx.state(Units::Pixels(300.0));
        let space_10 = cx.state(Pixels(10.0));

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                let locale_signal = cx.environment().locale;
                let is_french = cx.derived({
                    move |cx| *locale_signal.get(cx) == "fr"
                });
                Checkbox::new(cx, is_french).id("toggle-language").on_toggle(|cx| {
                    let current_locale = cx.environment().locale.get(cx);
                    if *current_locale != "fr" {
                        cx.emit(EnvironmentEvent::SetLocale("fr".parse().unwrap()));
                    } else {
                        cx.emit(EnvironmentEvent::SetLocale("en-US".parse().unwrap()));
                    }
                });
                Label::static_text(cx, "Toggle Language").describing("toggle-language");
            })
            .alignment(align_center)
            .horizontal_gap(gap_10)
            .height(height_auto);

            // Use the `Localized` type with a `Label` to provide a translation key.
            // The key is used to look up the corresponding translation from the fluent file.
            let hello_world = Localized::new("hello-world").signal(cx);
            Label::new(cx, hello_world);

            HStack::new(cx, |cx| {
                let enter_name = Localized::new("enter-name").signal(cx);
                Label::new(cx, enter_name);
                Textbox::new(cx, name)
                    .width(width_300)
                    .on_edit(move |cx, text| name.set(cx, text));
            })
            .alignment(align_center)
            .height(height_auto)
            .horizontal_gap(gap_5);

            let intro_name = name;
            Binding::new(cx, intro_name, move |cx| {
                let current = intro_name.get(cx).clone();
                let intro = Localized::new("intro").arg_const("name", current).signal(cx);
                Label::new(cx, intro);
            });

            // Use the `arg` method on the `Localized` type to supply a variable argument or appropriate signal.
            // When localization is resolved the argument will be used with the fluent file to select an appropriate translation.
            let unread_emails = emails;
            Binding::new(cx, unread_emails, move |cx| {
                let unread = *unread_emails.get(cx);
                let emails_label =
                    Localized::new("emails").arg_const("unread_emails", unread).signal(cx);
                Label::new(cx, emails_label);
            });

            let refresh = Localized::new("refresh").signal(cx);
            Button::new(cx, |cx| Label::new(cx, refresh))
                .on_press(move |cx| emails.update(cx, |count| *count += 1));
        })
        .vertical_gap(gap_10)
        .space(space_10);

        title
    });

    app.title(title).run()
}
