use vizia::fonts::unicode_names::CHECK;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    flag1: bool,
    flag2: bool,
    flag3: bool,

    value: f32,
}

pub enum AppEvent {
    ToggleFlag(u32),
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleFlag(index) => match index {
                0 => self.flag1 ^= true,
                1 => self.flag2 ^= true,
                2 => self.flag3 ^= true,
                _ => {}
            },

            AppEvent::SetValue(val) => {
                self.value = *val;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData { flag1: false, flag2: false, flag3: false, value: 25.0 }.build(cx);

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Labels").font_size(24.0);
                Label::new(cx, "This vizia application is accessible thanks to AccessKit");
            })
            .row_between(Pixels(10.0))
            .height(Auto);

            VStack::new(cx, |cx| {
                Label::new(cx, "Checkoboxes").font_size(24.0);
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, AppData::flag1)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleFlag(0)))
                        .id("first");
                    Label::new(cx, "First").describing("first");
                })
                .height(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, AppData::flag2)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleFlag(1)))
                        .id("second");
                    Label::new(cx, "Second").describing("second");
                })
                .height(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, AppData::flag3)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleFlag(2)))
                        .id("third");
                    Label::new(cx, "Third").describing("third");
                })
                .height(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));
            })
            .height(Auto)
            .row_between(Pixels(10.0));

            VStack::new(cx, |cx| {
                Label::new(cx, "Button").font_size(24.0);
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Push"));
                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, CHECK).class("icon");
                            Label::new(cx, "Button with Icon");
                        })
                        .size(Auto)
                        .child_space(Stretch(1.0))
                        .col_between(Pixels(2.0))
                    },
                );
            })
            .height(Auto)
            .row_between(Pixels(10.0));

            VStack::new(cx, |cx| {
                Label::new(cx, "Slider").font_size(24.0);
                HStack::new(cx, |cx| {
                    Slider::new(cx, AppData::value)
                        .name("Volume Control")
                        .range(0.0..100.0)
                        .step(1.0)
                        .on_changing(|cx, val| cx.emit(AppEvent::SetValue(val)))
                        .top(Stretch(1.0))
                        .bottom(Stretch(1.0));
                    Label::new(cx, AppData::value).width(Pixels(50.0)).child_left(Pixels(5.0));
                })
                .height(Auto)
                .child_space(Pixels(10.0))
                .col_between(Pixels(10.0));
            })
            .height(Auto)
            .row_between(Pixels(10.0));
        })
        .row_between(Pixels(20.0))
        .child_space(Pixels(10.0));
    })
    .title("AccessKit")
    .run();
}
