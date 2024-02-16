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
        child-space: 10px;
        height: auto;
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

#[derive(Lens)]
pub struct AppData {
    is_saved: bool,
    show_dialog: bool,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            // Intercept WindowClose event to show a dialog if not 'saved'.
            WindowEvent::WindowClose => {
                if !self.is_saved {
                    self.show_dialog = true;
                    meta.consume();
                }
            }
            _ => {}
        });

        event.map(|app_event, _| match app_event {
            AppEvent::HideModal => {
                self.show_dialog = false;
            }

            AppEvent::Save => {
                self.is_saved = true;
            }

            AppEvent::SaveAndClose => {
                self.is_saved = true;
                cx.emit(WindowEvent::WindowClose);
            }

            AppEvent::Cancel => {
                self.is_saved = false;
            }
        });
    }
}

pub enum AppEvent {
    HideModal,
    Save,
    SaveAndClose,
    Cancel,
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        AppData { is_saved: false, show_dialog: false }.build(cx);

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Close"))
                .on_press(|cx| cx.emit(WindowEvent::WindowClose));
            Button::new(cx, |cx| Label::new(cx, "Save")).on_press(|cx| cx.emit(AppEvent::Save));
        })
        .col_between(Pixels(10.0))
        .space(Pixels(20.0));

        Dialog::new(cx, AppData::show_dialog, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Save before close?").width(Stretch(1.0)).child_space(Stretch(1.0));
                HStack::new(cx, |cx| {
                    Button::new(cx, |cx| Label::new(cx, "Save & Close"))
                        .on_press(|cx| cx.emit(AppEvent::SaveAndClose))
                        .width(Pixels(120.0))
                        .class("accent");

                    Button::new(cx, |cx| Label::new(cx, "Cancel"))
                        .on_press(|cx| cx.emit(AppEvent::HideModal))
                        .width(Pixels(120.0));
                })
                .height(Auto);
            })
            .size(Auto)
            .row_between(Pixels(20.0))
            .height(Auto);
        })
        // .on_blur(|cx| cx.emit(AppEvent::HideModal))
        .width(Pixels(300.0))
        .height(Auto)
        .row_between(Pixels(20.0))
        .class("modal");
    })
    .run();
}
