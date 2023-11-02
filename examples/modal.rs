use vizia::prelude::*;

const STYLE: &str = r#"

    .modal {
        space: 1s;
        child-space: 8px;
        child-left: 1s;
        child-right: 1s;
        background-color: white;
        border-radius: 3px;
        border-width: 1px;
        border-color: #999999;
        outer-shadow: 0 3 10 #00000055;
        overflow: visible;
    }

    .modal>vstack>label {
        width: auto;
        height: auto;
        space: 5px;
        child-space: 1s;
    }

    .modal button {
        border-radius: 3px;
        child-space: 1s;
    }

    .modal hstack {
        col-between: 20px;
        size: auto;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        AppData { show_modal: false }.build(cx);

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
