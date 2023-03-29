use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    text: Vec<String>,
}

pub enum AppEvent {
    SetText(usize, String),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetText(index, txt) => self.text[*index] = txt.clone(),
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData {
            text: vec![
                String::from("This is some text"),
                String::from("This is some text"),
                String::from("This is some text"),
                String::from("This is some text"),
            ],
        }
        .build(cx);
        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "This is some non-wrapping text")
                        .child_space(Pixels(10.0))
                        .background_color(Color::rgb(200, 100, 200));
                })
                .size(Auto)
                .child_space(Pixels(10.0))
                .background_color(Color::rgb(100, 200, 100));

                HStack::new(cx, |cx| {
                    Label::new(cx, "This is some text which should wrap on every word.")
                        .size(Auto)
                        .text_wrap(true)
                        .child_space(Pixels(10.0))
                        .background_color(Color::rgb(200, 100, 100));
                })
                .size(Auto)
                .child_space(Pixels(10.0))
                .background_color(Color::rgb(100, 200, 100));

                HStack::new(cx, |cx| {
                    Label::new(
                        cx,
                        "This is some text which should wrap because its container is too narrow.",
                    )
                    .text_wrap(true)
                    .width(Pixels(100.0))
                    .child_space(Pixels(10.0))
                    .background_color(Color::rgb(200, 100, 100));
                })
                .size(Auto)
                .child_space(Pixels(10.0))
                .background_color(Color::rgb(100, 200, 100));

                HStack::new(cx, |cx| {
                    Label::new(cx, "This is some text which should\nwrap because of a hard break.")
                        .text_wrap(false)
                        .width(Auto)
                        .child_space(Pixels(10.0))
                        .background_color(Color::rgb(200, 100, 100));
                })
                .size(Auto)
                .child_space(Pixels(10.0))
                .background_color(Color::rgb(100, 200, 100));

                HStack::new(cx, |cx| {
                    Label::new(
                        cx,
                        "This is some text which should\nwrap because of soft and hard breaks.",
                    )
                    .text_wrap(true)
                    .width(Pixels(100.0))
                    .child_space(Pixels(10.0))
                    .background_color(Color::rgb(200, 100, 100));
                })
                .size(Auto)
                .child_space(Pixels(10.0))
                .background_color(Color::rgb(100, 200, 100));
            })
            .col_between(Pixels(50.0))
            .child_space(Pixels(50.0));

            // HStack::new(cx, |cx| {
            //     HStack::new(cx, |cx| {
            //         Label::new(cx, "This is some\ntext aligned to\nthe top left")
            //             .text_wrap(false)
            //             .size(Pixels(150.0))
            //             .background_color(Color::rgb(200, 100, 100));
            //     })
            //     .size(Auto)
            //     .child_space(Pixels(10.0))
            //     .background_color(Color::rgb(100, 200, 100));

            //     HStack::new(cx, |cx| {
            //         Label::new(cx, "This is some\ntext aligned to\nthe top center")
            //             .text_wrap(false)
            //             .size(Pixels(150.0))
            //             .child_left(Stretch(1.0))
            //             .child_right(Stretch(1.0))
            //             .background_color(Color::rgb(200, 100, 100));
            //     })
            //     .size(Auto)
            //     .child_space(Pixels(10.0))
            //     .background_color(Color::rgb(100, 200, 100));

            //     HStack::new(cx, |cx| {
            //         Label::new(cx, "This is some\ntext aligned to\nthe top right")
            //             .size(Pixels(150.0))
            //             .child_left(Stretch(1.0))
            //             .child_right(Pixels(0.0))
            //             .background_color(Color::rgb(200, 100, 100));
            //     })
            //     .size(Auto)
            //     .child_space(Pixels(10.0))
            //     .background_color(Color::rgb(100, 200, 100));
            // })
            // .size(Auto)
            // .col_between(Pixels(20.0))
            // .child_space(Pixels(20.0));

            // HStack::new(cx, |cx| {
            //     HStack::new(cx, |cx| {
            //         Label::new(cx, "This is some\ntext aligned to\nthe middle left")
            //             .text_wrap(false)
            //             .size(Pixels(150.0))
            //             .child_top(Stretch(1.0))
            //             .child_bottom(Stretch(1.0))
            //             .background_color(Color::rgb(200, 100, 100));
            //     })
            //     .size(Auto)
            //     .child_space(Pixels(10.0))
            //     .background_color(Color::rgb(100, 200, 100));

            //     HStack::new(cx, |cx| {
            //         Label::new(cx, "This is some\ntext aligned to\nthe middle center")
            //             .text_wrap(false)
            //             .size(Pixels(150.0))
            //             .child_space(Stretch(1.0))
            //             .background_color(Color::rgb(200, 100, 100));
            //     })
            //     .size(Auto)
            //     .child_space(Pixels(10.0))
            //     .background_color(Color::rgb(100, 200, 100));

            //     HStack::new(cx, |cx| {
            //         Label::new(cx, "This is some\ntext aligned to\nthe middle right")
            //             .size(Pixels(150.0))
            //             .child_top(Stretch(1.0))
            //             .child_bottom(Stretch(1.0))
            //             .child_left(Stretch(1.0))
            //             .child_right(Pixels(0.0))
            //             .background_color(Color::rgb(200, 100, 100));
            //     })
            //     .size(Auto)
            //     .child_space(Pixels(10.0))
            //     .background_color(Color::rgb(100, 200, 100));
            // })
            // .size(Auto)
            // .col_between(Pixels(20.0))
            // .child_space(Pixels(20.0));

            // HStack::new(cx, |cx| {
            //     HStack::new(cx, |cx| {
            //         Label::new(cx, "This is some\ntext aligned to\nthe bottom left")
            //             .text_wrap(false)
            //             .size(Pixels(150.0))
            //             .child_top(Stretch(1.0))
            //             .child_bottom(Pixels(0.0))
            //             .background_color(Color::rgb(200, 100, 100));
            //     })
            //     .size(Auto)
            //     .child_space(Pixels(10.0))
            //     .background_color(Color::rgb(100, 200, 100));

            //     HStack::new(cx, |cx| {
            //         Label::new(cx, "This is some\ntext aligned to\nthe bottom center")
            //             .text_wrap(false)
            //             .size(Pixels(150.0))
            //             .child_space(Stretch(1.0))
            //             .child_bottom(Pixels(0.0))
            //             .background_color(Color::rgb(200, 100, 100));
            //     })
            //     .size(Auto)
            //     .child_space(Pixels(10.0))
            //     .background_color(Color::rgb(100, 200, 100));

            //     HStack::new(cx, |cx| {
            //         Label::new(cx, "This is some\ntext aligned to\nthe bottom right")
            //             .size(Pixels(150.0))
            //             .child_top(Stretch(1.0))
            //             .child_bottom(Pixels(0.0))
            //             .child_left(Stretch(1.0))
            //             .child_right(Pixels(0.0))
            //             .background_color(Color::rgb(200, 100, 100));
            //     })
            //     .size(Auto)
            //     .child_space(Pixels(10.0))
            //     .background_color(Color::rgb(100, 200, 100));
            // })
            // .size(Auto)
            // .col_between(Pixels(20.0))
            // .child_space(Pixels(20.0));

            // Textbox::new(cx, AppData::text.index(0))
            //     .on_submit(|ex, txt, _| ex.emit(AppEvent::SetText(0, txt.clone())));

            // Textbox::new(cx, AppData::text.index(0))
            //     .width(Pixels(200.0))
            //     .on_submit(|ex, txt, _| ex.emit(AppEvent::SetText(0, txt.clone())));

            // Textbox::new_multiline(cx, AppData::text.index(1), false)
            //     .on_submit(|ex, txt, _| ex.emit(AppEvent::SetText(1, txt.clone())));

            // Textbox::new_multiline(cx, AppData::text.index(2), true)
            //     .on_submit(|ex, txt, _| ex.emit(AppEvent::SetText(2, txt.clone())));

            // Textbox::new_multiline(cx, AppData::text.index(3), true)
            // .width(Pixels(200.0))
            // .child_left(Stretch(1.0))
            // .on_submit(|ex, txt, _| ex.emit(AppEvent::SetText(3, txt.clone())));
        })
        .child_space(Pixels(0.0))
        .row_between(Pixels(20.0));
    })
    .title("Text")
    .inner_size((1200, 600))
    .run();
}
