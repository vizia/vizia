#[allow(unused_imports)]
use vizia::prelude::*;

struct LocalizationApp {
    name: Signal<String>,
    emails: Signal<i32>,
    align_center: Signal<Alignment>,
    gap_10: Signal<Units>,
    gap_5: Signal<Units>,
    height_auto: Signal<Units>,
    width_300: Signal<Units>,
    space_10: Signal<Units>,
}

impl App for LocalizationApp {
    fn new(cx: &mut Context) -> Self {
        cx.add_translation(
            "en-US".parse().unwrap(),
            include_str!("resources/translations/en-US/hello.ftl").to_owned(),
        );

        cx.add_translation(
            "fr".parse().unwrap(),
            include_str!("resources/translations/fr/hello.ftl").to_owned(),
        );

        Self {
            name: cx.state("Audrey".to_owned()),
            emails: cx.state(1i32),
            align_center: cx.state(Alignment::Center),
            gap_10: cx.state(Pixels(10.0)),
            gap_5: cx.state(Pixels(5.0)),
            height_auto: cx.state(Auto),
            width_300: cx.state(Units::Pixels(300.0)),
            space_10: cx.state(Pixels(10.0)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let name = self.name;
        let emails = self.emails;
        let align_center = self.align_center;
        let gap_10 = self.gap_10;
        let gap_5 = self.gap_5;
        let height_auto = self.height_auto;
        let width_300 = self.width_300;
        let space_10 = self.space_10;

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                let locale_signal = cx.environment().locale;
                let is_french = locale_signal.drv(cx, |v, _| *v == "fr");
                Checkbox::new(cx, is_french).id("toggle-language").on_toggle(|cx| {
                    let current_locale = cx.environment().locale.get(cx);
                    if *current_locale != "fr" {
                        cx.emit(EnvironmentEvent::SetLocale("fr".parse().unwrap()));
                    } else {
                        cx.emit(EnvironmentEvent::SetLocale("en-US".parse().unwrap()));
                    }
                });
                Label::new(cx, "Toggle Language").describing("toggle-language");
            })
            .alignment(align_center)
            .horizontal_gap(gap_10)
            .height(height_auto);

            let hello_world = Localized::new("hello-world").signal(cx);
            Label::new(cx, hello_world);

            HStack::new(cx, |cx| {
                let enter_name = Localized::new("enter-name").signal(cx);
                Label::new(cx, enter_name);
                Textbox::new(cx, name).width(width_300).on_edit(move |cx, text| name.set(cx, text));
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

            let unread_emails = emails;
            Binding::new(cx, unread_emails, move |cx| {
                let unread = *unread_emails.get(cx);
                let emails_label =
                    Localized::new("emails").arg_const("unread_emails", unread).signal(cx);
                Label::new(cx, emails_label);
            });

            let refresh = Localized::new("refresh").signal(cx);
            Button::new(cx, |cx| Label::new(cx, refresh))
                .on_press(move |cx| emails.upd(cx, |count| *count += 1));
        })
        .vertical_gap(gap_10)
        .space(space_10);

        self
    }
}

fn main() -> Result<(), ApplicationError> {
    LocalizationApp::run()
}
