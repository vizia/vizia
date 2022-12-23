use vizia::prelude::*;

#[derive(Lens, Clone, Data)]
pub struct AppData {
    // more_data: MoreData,
    selected: bool,
}

#[derive(Lens, Clone, Data)]
pub struct MoreData {
    // selected: bool,
    decks: bool,
}

pub enum AppEvent {
    ToggleFlagOne,
    ToggleFlagTwo,
    ToggleAll,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleFlagOne => self.selected ^= true,
            // AppEvent::ToggleFlagTwo => self.more_data.decks ^= true,
            // AppEvent::ToggleAll => {
            //     self.more_data.selected ^= true;
            //     self.more_data.decks ^= true;
            // }
            _ => {}
        });
    }
}

impl Model for MoreData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleFlagTwo => self.decks ^= true,
            _ => {}
        });
    }
}

fn main() {
    Application::new(|cx| {
        // AppData { selected: true, text: String::from("Some text") }.build(cx);
        // MoreData { flag: false }.build(cx);

        AppData { selected: true }.build(cx);
        MoreData { decks: true }.build(cx);

        VStack::new(cx, |cx| {
            Checkbox::new(cx, AppData::selected).on_toggle(|cx| cx.emit(AppEvent::ToggleFlagOne));
            Checkbox::new(cx, MoreData::decks).on_toggle(|cx| cx.emit(AppEvent::ToggleFlagTwo));

            Checkbox::new(
                cx,
                (AppData::selected, MoreData::decks).map(|(selected, decks)| *selected && *decks),
            )
            .on_toggle(|cx| {
                cx.emit(AppEvent::ToggleFlagOne);
                cx.emit(AppEvent::ToggleFlagTwo);
            });

            Textbox::new(
                cx,
                (AppData::selected, MoreData::decks).map(|(selected, decks)| {
                    if *selected && *decks {
                        "TEST".to_string()
                    } else {
                        "DISABLED".to_string()
                    }
                }),
            )
            .width(Pixels(200.0));
        })
        .child_space(Pixels(10.0))
        .row_between(Pixels(10.0));

        // Binding::new(cx, (AppData::flag, MoreData::flag), move |cx, (flag1, flag2)| {
        //     if flag1.get(cx) && flag2.get(cx) {
        //         Label::new(cx, "Hello Multibinding");
        //     }
        // });

        // Label::new(cx, "Test").background_color((AppData::flag, MoreData::flag).map(
        //     |(flag1, flag2)| {
        //         if *flag1 && *flag2 {
        //             Color::red()
        //         } else {
        //             Color::blue()
        //         }
        //     },
        // ));

        // Label::new(cx, "Test").background_color(AppData::flag.map(|flag| {
        //     if *flag {
        //         Color::red()
        //     } else {
        //         Color::blue()
        //     }
        // }));
    })
    .run();
}
