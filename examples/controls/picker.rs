use vizia::*;

#[derive(Debug, Data, Clone, Copy, PartialEq)]
pub enum Options {
    First,
    Second,
    Third,
}

#[derive(Lens)]
pub struct AppData {
    option: Options,
}

#[derive(Debug)]
pub enum AppEvent {
    SetOption(Options),
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetOption(option) => {
                    self.option = *option;
                }
            }
        }

        if let Some(picker_event) = event.message.downcast() {
            match picker_event {
                PickerEvent::SetOption(option) => {
                    self.option = *option;
                }
            }
        }
    }
}

fn main() {
    let window_description = WindowDescription::new();
    Application::new(window_description, |cx| {
        AppData { option: Options::First }.build(cx);

        // Picker::new(cx, AppData::option, |cx, option|{

        //     let opt = *option.get(cx);

        //     Button::new(cx, |cx| cx.emit(AppEvent::SetOption(Options::First)), |cx|{
        //         Label::new(cx, "First")
        //     }).background_color(if opt == Options::First {Color::red()} else {Color::blue()});

        //     Button::new(cx, |cx| cx.emit(AppEvent::SetOption(Options::Second)), |cx|{
        //         Label::new(cx, "Second")
        //     }).background_color(if opt == Options::Second {Color::red()} else {Color::blue()});

        //     Button::new(cx, |cx| cx.emit(AppEvent::SetOption(Options::Third)), |cx|{
        //         Label::new(cx, "Third")
        //     }).background_color(if opt == Options::Third {Color::red()} else {Color::blue()});
        // });

        // Picker::new(cx, AppData::option, |cx, option|{
        //     let opt = *option.get(cx);
        //     PickerItem::new(cx, "First", Options::First, opt);
        //     PickerItem::new(cx, "Second",Options::Second, opt);
        //     PickerItem::new(cx, "Third",Options::Third, opt);
        // });

        // Picker::new(cx, AppData::option, |cx, option|{
        //     let opt = *option.get(cx);
        //     picker_item(cx, "First", Options::First, opt);
        //     picker_item(cx, "Second",Options::Second, opt);
        //     picker_item(cx, "Third",Options::Third, opt);
        // });

        Dropdown::new(cx, |cx| Label::new(cx, "Options"), |cx| {
            Picker::new(cx, AppData::option, |cx, option| {
                let opt = *option.get(cx);
                picker_item(cx, "First", Options::First, opt);
                picker_item(cx, "Second", Options::Second, opt);
                picker_item(cx, "Third", Options::Third, opt);
            });
        });
    })
    .run();
}

pub fn picker_item(cx: &mut Context, text: &'static str, option: Options, current: Options) {
    Button::new(
        cx,
        move |cx| cx.emit(AppEvent::SetOption(option.clone())),
        move |cx| Label::new(cx, text),
    )
    .background_color(if current == option { Color::red() } else { Color::blue() });
}
