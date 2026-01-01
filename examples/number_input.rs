use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        alignment: center;
    }
    number-input:invalid textbox {
        border-color: #ff0000;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        let number = cx.state(5i32);
        let width_200 = cx.state(Pixels(200.0));
        let padding_5 = cx.state(Pixels(5.0));
        let height_32 = cx.state(Pixels(32.0));
        let align_center = cx.state(Alignment::Center);
        let auto = cx.state(Auto);
        let stretch_one = cx.state(Stretch(1.0));
        let gap_10 = cx.state(Pixels(10.0));

        HStack::new(cx, |cx| {
            // NumberInput with max validation and two-way binding
            NumberInput::new(cx, number)
                .max(49)
                .two_way()
                .width(width_200)
                .padding_left(padding_5);

            Label::new(cx, number)
                .width(width_200)
                .height(height_32)
                .alignment(align_center)
                .padding_left(padding_5);
        })
        .alignment(align_center)
        .height(auto)
        .space(stretch_one)
        .alignment(align_center)
        .horizontal_gap(gap_10);
        (cx.state("Number Input"), cx.state((600, 300)))
    });

    app.title(title).inner_size(size).run()
}
