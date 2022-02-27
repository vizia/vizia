use vizia::*;

const STYLE: &str = r#"

    .modal {
        space: 1s;
        background-color: white;
        border-radius: 3px;
        border-width: 1px;
        border-color: #999999;
        outer-shadow: 0 3 10 #00000055;
        overflow: visible;
        child-space: 10px;
    }

    modal>popup>label {
        width: auto;
        height: auto;
        space: 5px;
        child-space: 1s;
    }

    button {
        border-radius: 3px;
        child-space: 1s;
    }

    hstack {
        child-space: 1s;
        col-between: 20px;
    }
"#;

fn main() {
    Application::new(WindowDescription::new().with_title("Modal"), |cx| {
        cx.add_theme(STYLE);

        AppData { show_modal: false }.build(cx);

        Button::new(cx, |cx| cx.emit(AppEvent::ShowModal), |cx| Label::new(cx, "Show Modal"))
            .width(Pixels(150.0))
            .space(Pixels(50.0));

        Popup::new(cx, AppData::show_modal, |cx| {
            Label::new(cx, "This is a message").width(Stretch(1.0));
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| cx.emit(AppEvent::HideModal), |cx| Label::new(cx, "Ok"))
                    .width(Pixels(100.0))
                    .class("accent");

                Button::new(cx, |cx| cx.emit(AppEvent::HideModal), |cx| Label::new(cx, "Cancel"))
                    .width(Pixels(100.0));
            });
        })
        .something(|cx| cx.emit(AppEvent::HideModal))
        .width(Pixels(300.0))
        .height(Auto)
        .row_between(Pixels(10.0))
        .class("modal");
    })
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
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::ShowModal => {
                    self.show_modal = true;
                }

                AppEvent::HideModal => {
                    self.show_modal = false;
                }
            }
        }
    }
}
