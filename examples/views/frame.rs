mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum RadioChoice {
    First,
    Second,
    Third,
}

fn radio_group(cx: &mut Context, selected: Signal<RadioChoice>, id_prefix: &'static str) {
    VStack::new(cx, |cx| {
        for (index, (choice, label)) in [
            (RadioChoice::First, "Option 1"),
            (RadioChoice::Second, "Option 2"),
            (RadioChoice::Third, "Option 3"),
        ]
        .into_iter()
        .enumerate()
        {
            let id = format!("{id_prefix}-radio-{index}");
            HStack::new(cx, move |cx| {
                let selected_choice = Memo::new(move |_| selected.get() == choice);
                RadioButton::new(cx, selected_choice)
                    .on_select(move |_cx| selected.set(choice))
                    .id(id.clone());
                Label::new(cx, label).hoverable(false).describing(id);
            })
            .height(Auto)
            .gap(Pixels(8.0));
        }
    })
    .height(Auto)
    .gap(Pixels(8.0))
    .padding(Pixels(12.0));
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let left_choice = Signal::new(RadioChoice::First);
        let center_choice = Signal::new(RadioChoice::First);
        let right_choice = Signal::new(RadioChoice::First);
        let untitled_choice = Signal::new(RadioChoice::First);

        ExamplePage::vertical(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, Localized::new("frame-example-heading"))
                    .font_size(20.0)
                    .font_weight(700);
                Divider::horizontal(cx);

                HStack::new(cx, |cx| {
                    // Frame with title at top-left
                    Frame::with_title(
                        cx,
                        |cx| {
                            Label::new(cx, Localized::new("frame-title-settings")).hoverable(false);
                        },
                        |cx| {
                            radio_group(cx, left_choice, "frame-left");
                        },
                    )
                    .width(Stretch(1.0));

                    // Frame with title at top-center
                    Frame::with_title(
                        cx,
                        |cx| {
                            Label::new(cx, Localized::new("frame-title-user-profile"))
                                .hoverable(false);
                        },
                        |cx| {
                            radio_group(cx, center_choice, "frame-center");
                        },
                    )
                    .title_position(FrameTitlePosition::TopCenter)
                    .width(Stretch(1.0));
                })
                .width(Stretch(1.0))
                .height(Auto)
                .gap(Pixels(16.0));

                HStack::new(cx, |cx| {
                    // Frame with title at top-right
                    Frame::with_title(
                        cx,
                        |cx| {
                            Label::new(cx, Localized::new("frame-title-details")).hoverable(false);
                        },
                        |cx| {
                            radio_group(cx, right_choice, "frame-right");
                        },
                    )
                    .title_position(FrameTitlePosition::TopRight)
                    .width(Stretch(1.0));

                    // Frame without title
                    Frame::new(cx, |cx| {
                        radio_group(cx, untitled_choice, "frame-untitled");
                    })
                    .width(Stretch(1.0));
                })
                .width(Stretch(1.0))
                .height(Auto)
                .gap(Pixels(16.0));
            })
            .height(Auto)
            .width(Stretch(1.0))
            .gap(Pixels(16.0));
        });
    })
    .title(Localized::new("view-title-frame"))
    .inner_size((400, 500))
    .run()
}
