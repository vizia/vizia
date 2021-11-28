use vizia::*;

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


fn main() {
    Application::new(WindowDescription::new().with_title("Knob"), |cx|{
        
        cx.add_theme(STYLE);

        let map = DecibelMap::new(-12.0, 12.0, ValueScaling::Linear, DisplayDecimals::One, true);
        let normalized_default = map.db_to_normalized(0.0);

        Knob::new(cx, map, normalized_default);

        //ArcTrack::new(cx).width(Pixels(50.0)).height(Pixels(50.0)).space(Pixels(20.0));

    }).run();
}