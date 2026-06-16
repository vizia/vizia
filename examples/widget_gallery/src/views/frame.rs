use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FrameRadioChoice {
    First,
    Second,
    Third,
}

fn frame_radio_group(
    cx: &mut Context,
    selected: Signal<FrameRadioChoice>,
    id_prefix: &'static str,
) {
    VStack::new(cx, |cx| {
        for (index, (choice, label_key)) in [
            (FrameRadioChoice::First, "one"),
            (FrameRadioChoice::Second, "two"),
            (FrameRadioChoice::Third, "three"),
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
                Label::new(cx, Localized::new(label_key)).hoverable(false).describing(id);
            })
            .height(Auto)
            .gap(Pixels(8.0));
        }
    })
    .height(Auto)
    .gap(Pixels(8.0))
    .padding(Pixels(12.0));
}

pub fn frame(cx: &mut Context) {
    let left_choice = Signal::new(FrameRadioChoice::First);
    let center_choice = Signal::new(FrameRadioChoice::First);
    let right_choice = Signal::new(FrameRadioChoice::First);
    let untitled_choice = Signal::new(FrameRadioChoice::First);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("frame")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, Localized::new("frame-demo-title-positions"), move |cx| {
            VStack::new(cx, |cx| {
                // Top-left
                Frame::with_title(
                    cx,
                    |cx| Label::new(cx, Localized::new("frame-title-top-left")).hoverable(false),
                    |cx| {
                        frame_radio_group(cx, left_choice, "wg-frame-left");
                    },
                )
                .title_position(FrameTitlePosition::TopLeft)
                .width(Stretch(1.0));

                // Top-center
                Frame::with_title(
                    cx,
                    |cx| Label::new(cx, Localized::new("frame-title-top-center")).hoverable(false),
                    |cx| {
                        frame_radio_group(cx, center_choice, "wg-frame-center");
                    },
                )
                .title_position(FrameTitlePosition::TopCenter)
                .width(Stretch(1.0));

                // Top-right
                Frame::with_title(
                    cx,
                    |cx| Label::new(cx, Localized::new("frame-title-top-right")).hoverable(false),
                    |cx| {
                        frame_radio_group(cx, right_choice, "wg-frame-right");
                    },
                )
                .title_position(FrameTitlePosition::TopRight)
                .width(Stretch(1.0));
            })
            .height(Auto)
            .gap(Pixels(12.0));
        });

        DemoRegion::new(cx, Localized::new("frame-demo-without-title"), move |cx| {
            Frame::new(cx, |cx| {
                frame_radio_group(cx, untitled_choice, "wg-frame-untitled");
            })
            .width(Stretch(1.0));
        });
    })
    .class("panel");
}
