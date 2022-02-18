use vizia::*;

const STYLE: &str = r#"

    zstack {
        space: 1s;
        background-color: #CCCCCC;
        border-radius: 3px;
    }

    label {
        width: auto;
        height: 1s;
        child-space: 1s;
    }

    button {
        border-radius: 3px;
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
            .space(Pixels(50.0));

        //Binding::new(cx, AppData::show_modal, |cx, show| {
        ZStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "This is a message").width(Stretch(1.0));
                HStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |cx| cx.emit(AppEvent::HideModal),
                        |cx| Label::new(cx, "Cancel"),
                    );
                    Button::new(cx, |cx| cx.emit(AppEvent::HideModal), |cx| Label::new(cx, "Ok"));
                });
            });
        })
        .width(Pixels(300.0))
        .height(Pixels(100.0))
        .visibility(AppData::show_modal);
        //});
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
                    println!("Show Modal");
                    self.show_modal = true;
                }

                AppEvent::HideModal => {
                    println!("Hide Modal");
                    self.show_modal = false;
                }
            }
        }
    }
}
