use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    flag1: bool,
    flag2: bool,
    flag3: bool,
}

pub enum AppEvent {
    ToggleFlag(u32),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleFlag(index) => match index {
                0 => self.flag1 ^= true,
                1 => self.flag2 ^= true,
                2 => self.flag3 ^= true,
                _ => {}
            },
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData { flag1: false, flag2: false, flag3: false }.build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Hello");
            Label::new(cx, "World");
            Label::new(cx, "This vizia application is accessible thanks to Accesskit");
        })
        .child_space(Pixels(10.0))
        .row_between(Pixels(10.0))
        .height(Auto);

        HStack::new(cx, |cx| {
            Checkbox::new(cx, AppData::flag1)
                .on_toggle(|cx| cx.emit(AppEvent::ToggleFlag(0)))
                .name("First");
            Checkbox::new(cx, AppData::flag2)
                .on_toggle(|cx| cx.emit(AppEvent::ToggleFlag(1)))
                .name("Second");
            Checkbox::new(cx, AppData::flag3)
                .on_toggle(|cx| cx.emit(AppEvent::ToggleFlag(2)))
                .name("Third");
        })
        .child_space(Pixels(10.0))
        .col_between(Pixels(10.0));
    })
    .title("AccessKit")
    .run();
}
