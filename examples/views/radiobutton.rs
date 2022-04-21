use vizia::*;

#[derive(Debug, Default, Lens)]
pub struct Options {
    pub option1: bool,
    pub option2: bool,
    pub option3: bool,
}

impl Model for Options {}

#[derive(Debug, Default, Lens)]
pub struct AppData {
    pub options: Options,
}

#[derive(Debug)]
pub enum AppEvent {
    ToggleOption(u32),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::ToggleOption(index) => match *index {
                    0 => {
                        self.options.option1 = true;
                        self.options.option2 = false;
                        self.options.option3 = false;
                    }

                    1 => {
                        self.options.option1 = false;
                        self.options.option2 = true;
                        self.options.option3 = false;
                    }

                    2 => {
                        self.options.option1 = false;
                        self.options.option2 = false;
                        self.options.option3 = true;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn main() {
    Application::new(WindowDescription::new().with_title("Checkbox"), |cx| {
        if cx.data::<AppData>().is_none() {
            AppData { options: Options { option1: true, option2: false, option3: false } }
                .build(cx);
        }

        // Exclusive checkboxes (radio buttons) with labels
        // Only one checkbox can be checked at a time and cannot be unchecked
        VStack::new(cx, |cx| {
            Label::new(cx, "Radio Buttons").class("h1");

            HStack::new(cx, |cx| {
                //Binding::new(cx, AppData::options.then(Options::option1), |cx, option1| {
                RadioButton::new(cx, AppData::options.then(Options::option1))
                    .on_select(|cx| cx.emit(AppEvent::ToggleOption(0)));
                //});
                Label::new(cx, "Option 1");
            })
            .size(Auto)
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(5.0));

            HStack::new(cx, |cx| {
                //Binding::new(cx, AppData::options.then(Options::option2), |cx, option2| {
                RadioButton::new(cx, AppData::options.then(Options::option2))
                    .on_select(|cx| cx.emit(AppEvent::ToggleOption(1)));
                //});
                Label::new(cx, "Option 2");
            })
            .size(Auto)
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(5.0));

            HStack::new(cx, |cx| {
                //Binding::new(cx, AppData::options.then(Options::option3), |cx, option3| {
                RadioButton::new(cx, AppData::options.then(Options::option3))
                    .on_select(|cx| cx.emit(AppEvent::ToggleOption(2)));
                //});
                Label::new(cx, "Option 3");
            })
            .size(Auto)
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(5.0));
        })
        .row_between(Pixels(5.0))
        .child_space(Stretch(1.0));
    })
    .run();
}
