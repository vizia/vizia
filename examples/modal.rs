use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        AppData { show_modal: false }.build(cx);

        VStack::new(cx, |cx| {
            Button::new(cx, |cx| cx.emit(AppEvent::ShowModal), |cx| Label::new(cx, "Show Modal"));

            Popup::new(cx, AppData::show_modal, true, |cx| {
                Label::new(cx, "Modal Title").class("title");
                Label::new(cx, "This is a message");
                Button::new(cx, |cx| cx.emit(AppEvent::HideModal), |cx| Label::new(cx, "Ok"))
                    .class("accent");
            })
            .on_blur(|cx| cx.emit(AppEvent::HideModal))
            .class("modal");
        })
        .class("container");
    })
    .title("Modal")
    .run();
}

#[derive(Debug)]
pub enum AppEvent {
    ShowModal,
    HideModal,
}

#[derive(Lens)]
pub struct AppData {
    show_modal: bool,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ShowModal => {
                self.show_modal = true;
            }
            AppEvent::HideModal => {
                self.show_modal = false;
            }
        });
    }
}
