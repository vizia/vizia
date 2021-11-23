use vizia::*;

const STYLE: &str = r#"
    slider {
        height: 10px;
        top: 1s;
        bottom: 1s;
        width: 1s;
        background-color: #dfdfdf;
        border-radius: 4.5px;
    }

    slider.vertical {
        height: 1s;
        width: 10px;
        left: 1s;
        right: 1s;
    }

    slider track {
    }

    slider .active {
        background-color: #f74c00;
        border-radius: 4.5px;
    }

    slider .thumb {
        background-color: white;
        top: 1s;
        bottom: 1s;
        border-radius: 14.5px;
        border-color: #757575;
        border-width: 1px;
        width: 40px;
        height: 40px;
    }

    label {
        width: 100px;
        height: 30px;
        top: 1s;
        bottom: 1s;
        border-color: #757575;
        border-width: 1px;
    }
"#;


fn main() {
    Application::new(|cx|{
        
        cx.add_theme(STYLE);
            
        for _ in 0..5 {
            HStack::new(cx, |cx|{

                SliderData {
                    value: 0.5,
                }.build(cx);

                Slider::new(cx, 0.5, Orientation::Horizontal);
                
                Binding::new(cx, SliderData::value, |cx, value|{
                    let value = value.get(cx);
                    Label::new(cx, &format!("{:.*}", 2, value));
                });
            }).height(Pixels(50.0)).child_space(Pixels(50.0)).col_between(Pixels(50.0));            
        }

        Slider::new(cx, 0.75, Orientation::Vertical).class("vertical");
        
    }).run();
}