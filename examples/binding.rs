use vizia::prelude::*;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    flag1: bool,
    flag2: bool,
}

fn main() {
    Application::new(|cx| {
        AppData { flag1: false, flag2: false }.build(cx);

        Checkbox::new(cx, AppData::flag1).on_toggle(|cx| cx.emit(AppDataSetter::Flag1(true)));
        Checkbox::new(cx, AppData::flag2).on_toggle(|cx| cx.emit(AppDataSetter::Flag2(true)));
        Binding::new(cx, AppData::flag1, |cx, flag1| {
            Binding::new(cx, AppData::flag2, move |cx, flag2| {
                if flag1.get(cx) && flag2.get(cx) {
                    Label::new(cx, "Hello World");
                }
            });
        });

        Binding::new(cx, (AppData::flag1, AppData::flag2), move |cx, (flag1, flag2)| {
            if flag1.get(cx) && flag2.get(cx) {
                Label::new(cx, "Hello Multibinding");
            }
        });
    })
    .run();
}
