mod helpers;
use helpers::*;
use vizia::prelude::*;

struct SwitchApp {
    option1: Signal<bool>,
    option2: Signal<bool>,
}

impl App for SwitchApp {
    fn app_name() -> &'static str {
        "Switch"
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
            Label::new(cx, "Basic Switches");

            HStack::new(cx, |cx| {
                Switch::new(cx, option1).two_way().id("Switch_1");
                Label::new(cx, "Switch 1").describing("Switch_1");
            })
            .size(Auto)
            .horizontal_gap(Pixels(5.0))
            .alignment(Alignment::Center);

            HStack::new(cx, |cx| {
                Switch::new(cx, option2).two_way().id("Switch_2");
                Label::new(cx, "Switch 2").describing("Switch_2");
            })
            .size(Auto)
            .horizontal_gap(Pixels(5.0))
            .alignment(Alignment::Center);
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app)
    }
}

fn main() -> Result<(), ApplicationError> {
    SwitchApp::run()
}
