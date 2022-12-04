use vizia::prelude::*;
use vizia::state::BindableExt;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    flag: bool,
}

#[derive(Lens, Setter, Model)]
pub struct MoreData {
    flag: bool,
}

fn main() {
    Application::new(|cx| {
        AppData { flag: false }.build(cx);
        MoreData { flag: false }.build(cx);

        Checkbox::new(cx, AppData::flag).on_toggle(|cx| cx.emit(AppDataSetter::Flag(true)));
        Checkbox::new(cx, MoreData::flag).on_toggle(|cx| cx.emit(MoreDataSetter::Flag(true)));

        Binding::new(cx, AppData::flag, |cx, flag1| {
            Binding::new(cx, MoreData::flag, move |cx, flag2| {
                if flag1.get(cx) && flag2.get(cx) {
                    Label::new(cx, "Hello World");
                }
            });
        });

        Binding::new(cx, (AppData::flag, MoreData::flag), move |cx, (flag1, flag2)| {
            if flag1.get(cx) && flag2.get(cx) {
                Label::new(cx, "Hello Multibinding");
            }
        });

        Label::new(cx, "Test").background_color((AppData::flag, MoreData::flag).bind_map(
            |(flag1, flag2)| {
                if flag1 && flag2 {
                    Color::red()
                } else {
                    Color::blue()
                }
            },
        ));
    })
    .run();
}
