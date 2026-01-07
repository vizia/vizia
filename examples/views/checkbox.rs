mod helpers;
use helpers::*;
use vizia::icons::{ICON_EYE, ICON_EYE_OFF};
use vizia::prelude::*;

struct CheckboxApp {
    option1: Signal<bool>,
    option2: Signal<bool>,
}

impl App for CheckboxApp {
    fn app_name() -> &'static str {
        "Checkbox"
    }

    fn new(cx: &mut Context) -> Self {
        Self {
            option1: cx.state(true),
            option2: cx.state(false),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let option1 = self.option1;
        let option2 = self.option2;

        ExamplePage::vertical(cx, |cx| {
            Label::new(cx, "Checkbox with label").class("h2");

            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, option1)
                        .on_toggle(move |cx| {
                            option1.set(cx, !option1.get(cx));
                            option2.set(cx, !option2.get(cx));
                        })
                        .id("checkbox_1");
                    Label::new(cx, "Checkbox 1").describing("checkbox_1");
                })
                .size(Auto)
                .horizontal_gap(Pixels(5.0))
                .alignment(Alignment::Center);

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, option2)
                        .on_toggle(move |cx| {
                            option1.set(cx, !option1.get(cx));
                            option2.set(cx, !option2.get(cx));
                        })
                        .id("checkbox_2");
                    Label::new(cx, "Checkbox 2").describing("checkbox_2");
                })
                .size(Auto)
                .horizontal_gap(Pixels(5.0))
                .alignment(Alignment::Center);
            })
            .vertical_gap(Pixels(10.0))
            .size(Auto);

            Label::new(cx, "Checkbox with custom icon and label").class("h2");

            let icon_off = cx.state(ICON_EYE_OFF);
            let icon_on = cx.state(ICON_EYE);

            HStack::new(cx, |cx| {
                Checkbox::with_icons(cx, option1, Some(icon_off), Some(icon_on))
                    .on_toggle(move |cx| {
                        option1.set(cx, !option1.get(cx));
                        option2.set(cx, !option2.get(cx));
                    })
                    .id("checkbox_3");
                Label::new(cx, "Checkbox 3").describing("checkbox_3");
            })
            .size(Auto)
            .horizontal_gap(Pixels(5.0))
            .alignment(Alignment::Center);
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((300, 320)))
    }
}

fn main() -> Result<(), ApplicationError> {
    CheckboxApp::run()
}
