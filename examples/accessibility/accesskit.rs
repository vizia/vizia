use vizia::icons::ICON_CHECK;
use vizia::prelude::*;

static NAMES: [&str; 3] = ["First", "Second", "Third"];

#[derive(Lens)]
pub struct AppData {
    flags: [bool; 3],
    radio_flags: [bool; 3],

    value: f32,
    text: String,
}

pub enum AppEvent {
    ToggleFlag(usize),
    ToggleRadio(usize),
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleFlag(index) => {
                self.flags[*index] ^= true;
            }

            AppEvent::ToggleRadio(index) => {
                for flag in self.radio_flags.iter_mut() {
                    *flag = false;
                }

                self.radio_flags[*index] = true;
            }

            AppEvent::SetValue(val) => {
                self.value = *val;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData {
            flags: [false, false, false],
            radio_flags: [true, false, false],
            value: 25.0,
            text: String::from("something"),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Labels").font_size(24.0);
                Label::new(cx, "This vizia application is accessible thanks to AccessKit");
            })
            .row_between(Pixels(10.0))
            .height(Auto);

            VStack::new(cx, |cx| {
                Label::new(cx, "Checkboxes").font_size(24.0);
                for i in 0..3 {
                    HStack::new(cx, move |cx| {
                        Checkbox::new(cx, AppData::flags.map(move |flags| flags[i]))
                            .on_toggle(move |cx| cx.emit(AppEvent::ToggleFlag(i)))
                            .id(format!("check_{}", i));
                        Label::new(cx, NAMES[i]).describing(format!("check_{}", i)).hidden(true);
                    })
                    .height(Auto)
                    .child_top(Stretch(1.0))
                    .child_bottom(Stretch(1.0))
                    .col_between(Pixels(5.0));
                }
            })
            .height(Auto)
            .row_between(Pixels(10.0));

            VStack::new(cx, |cx| {
                for i in 0..3 {
                    HStack::new(cx, move |cx| {
                        RadioButton::new(cx, AppData::radio_flags.map(move |flags| flags[i]))
                            .on_select(move |cx| cx.emit(AppEvent::ToggleRadio(i)))
                            .id(format!("check_{}", i));
                        Label::new(cx, NAMES[i]).describing(format!("check_{}", i)).hidden(true);
                    })
                    .size(Auto)
                    .child_top(Stretch(1.0))
                    .child_bottom(Stretch(1.0))
                    .col_between(Pixels(5.0));
                }
            })
            .height(Auto)
            .row_between(Pixels(10.0));

            Textbox::new(cx, AppData::text).width(Pixels(200.0)).height(Pixels(30.0));

            VStack::new(cx, |cx| {
                Label::new(cx, "Buttons").font_size(24.0);
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Push"));
                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, ICON_CHECK).class("icon");
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
                        .name("Value Control")
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
