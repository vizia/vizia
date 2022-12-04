use vizia::prelude::*;
use vizia::state::BindableExt;

#[derive(Lens)]
pub struct AppData {
    flag: bool,
}

#[derive(Lens)]
pub struct MoreData {
    flag: bool,
}

pub enum AppEvent {
    ToggleFlagOne,
    ToggleFlagTwo,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleFlagOne => self.flag ^= true,
            _ => {}
        });
    }
}

impl Model for MoreData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleFlagTwo => self.flag ^= true,
            _ => {}
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData { flag: false }.build(cx);
        MoreData { flag: false }.build(cx);

        Checkbox::new(cx, AppData::flag).on_toggle(|cx| cx.emit(AppEvent::ToggleFlagOne));
        Checkbox::new(cx, MoreData::flag).on_toggle(|cx| cx.emit(AppEvent::ToggleFlagTwo));

        // Binding::new(cx, (AppData::flag, MoreData::flag), move |cx, (flag1, flag2)| {
        //     if flag1.get(cx) && flag2.get(cx) {
        //         Label::new(cx, "Hello Multibinding");
        //     }
        // });

        Label::new(cx, "Test").background_color((AppData::flag, MoreData::flag).map(
            |(flag1, flag2)| {
                if *flag1 && *flag2 {
                    Color::red()
                } else {
                    Color::blue()
                }
            },
        ));

        Label::new(cx, "Test").background_color(AppData::flag.map(|flag| {
            if *flag {
                Color::red()
            } else {
                Color::blue()
            }
        }));
    })
    .run();
}
