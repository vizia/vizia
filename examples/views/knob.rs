use vizia::prelude::*;

const STYLE: &str = r#"

    knob {
        width: 76px;
        height: 76px;
        background-color: #262a2d;
        border-radius: 38px;
        border-width: 2px;
        border-color: #363636;
    }
    
    knob .track {
        background-color: #ffb74d;
    }

"#;

#[derive(Lens, Model, Setter)]
pub struct AppData {
    value: f32,
}

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);

        AppData { value: 0.2 }.build(cx);

        Knob::new(cx, 0.5, AppData::value, false).on_changing(|cx, val| {
            cx.emit(AppDataSetter::Value(val));
        });
        Knob::new(cx, 0.5, AppData::value, true).on_changing(|cx, val| {
            cx.emit(AppDataSetter::Value(val));
        });

        //ArcTrack::new(cx).width(Pixels(50.0)).height(Pixels(50.0)).space(Pixels(20.0));
    })
    .title("Knob")
    .run();
}
