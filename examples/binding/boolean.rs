use vizia::prelude::*;

#[derive(Debug, Lens)]
pub struct ModelOne {
    flag1: bool,
    flag2: bool,
    flag3: bool,
}

pub enum AppEvent {
    ToggleOne,
    ToggleTwo,
    ToggleThree,
}

impl Model for ModelOne {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleOne => {
                self.flag1 ^= true;
            }

            AppEvent::ToggleTwo => {
                self.flag2 ^= true;
            }

            AppEvent::ToggleThree => {
                self.flag3 ^= true;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        ModelOne { flag1: true, flag2: false, flag3: false }.build(cx);

        Checkbox::new(cx, ModelOne::flag1).on_toggle(|cx| cx.emit(AppEvent::ToggleOne));
        Checkbox::new(cx, ModelOne::flag2).on_toggle(|cx| cx.emit(AppEvent::ToggleTwo));
        Checkbox::new(cx, ModelOne::flag3).on_toggle(|cx| cx.emit(AppEvent::ToggleThree));

        Element::new(cx).height(Pixels(30.0));

        Checkbox::new(cx, ModelOne::flag1.or(ModelOne::flag2));
        Checkbox::new(cx, ModelOne::flag1 | ModelOne::flag2 | ModelOne::flag3);
        Checkbox::new(cx, ModelOne::flag1.and(ModelOne::flag2));
        Checkbox::new(cx, ModelOne::flag1 & ModelOne::flag2 & ModelOne::flag3);

        // Checkbox::new(
        //     cx,
        //     (ModelOne::flag1, ModelOne::flag2).map(|(f1, f2)| !(*f1 | *f2)) | ModelOne::flag3,
        // );
    })
    .run();
}
