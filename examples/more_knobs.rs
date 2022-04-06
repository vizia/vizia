use vizia::*;
const STYLE: &str = r#"
    label {
        width: 200px;
        height: 30px;
        child-space: 1s;
        font-size: 20;
        color: #C2C2C2;
    }
    
    knob {
        width: 100px;
        height: 100px;
    }
    
    knob .track {
        background-color: #ffb74d;
    }
    .label_knob {
        border-width: 2px;
        border-color: #28282b;
        background-color: #000000;
        col-between: 10px;
        child-space: 1s;
    }
"#;
fn main() {
    Application::new(
        WindowDescription::new().with_title("More Knobs").with_inner_size(1200, 200),
        |cx| {
            cx.add_theme(STYLE);
            KnobData { knobs: vec![0.5; 5] }.build(cx);

            HStack::new(cx, |cx| {
                // default knob
                VStack::new(cx, move |cx| {
                    Label::new(cx, "Default knob").height(Pixels(30.));

                    Knob::new(cx, 0.5, KnobData::knobs.map(|knobs| knobs[0]), false)
                        .on_changing(move |cx, val| cx.emit(KnobChangeEvent::SetKnob(0, val)))
                        .color(Color::red());

                    Label::new(cx, KnobData::knobs.map(|knobs| format!("{:.3}", knobs[0])));
                })
                .row_between(Pixels(10.0))
                .child_space(Stretch(1.0));

                // simple tick knob
                VStack::new(cx, move |cx| {
                    Label::new(cx, "Tick knob");
                    Knob::custom(
                        cx,
                        0.5,
                        KnobData::knobs.map(|knobs| knobs[1]),
                        move |cx, lens| {
                            TickKnob::new(
                                cx,
                                Percentage(100.0),
                                Percentage(20.0),
                                Percentage(50.0),
                                300.0,
                                KnobMode::Continuous,
                            )
                            .value(lens)
                            .class("track")
                        },
                    )
                    .on_changing(move |cx, val| cx.emit(KnobChangeEvent::SetKnob(1, val)));
                    Label::new(cx, KnobData::knobs.map(|knobs| format!("{:.3}", knobs[1])));
                })
                .row_between(Pixels(10.0))
                .child_space(Stretch(1.0));

                // steppy knob
                VStack::new(cx, move |cx| {
                    Label::new(cx, "Steppy knob");
                    Knob::custom(
                        cx,
                        0.5,
                        KnobData::knobs.map(|knobs| knobs[2]),
                        move |cx, lens| {
                            let mode = KnobMode::Discrete(5);
                            TickKnob::new(
                                cx,
                                Percentage(60.0),
                                Percentage(20.0),
                                Percentage(50.0),
                                300.0,
                                mode,
                            )
                            .value(lens)
                            .class("track");
                            Ticks::new(
                                cx,
                                Percentage(100.0),
                                Percentage(25.0),
                                Pixels(5.0),
                                300.0,
                                mode,
                            )
                            .class("track")
                        },
                    )
                    .on_changing(move |cx, val| cx.emit(KnobChangeEvent::SetKnob(2, val)));
                    Label::new(
                        cx,
                        KnobData::knobs
                            .map(|knobs| format!("{:.3}", (knobs[2] * 4.0).floor() / 4.0)),
                    );
                })
                .row_between(Pixels(10.0))
                .child_space(Stretch(1.0));

                // Arc+tick knob knob
                VStack::new(cx, move |cx| {
                    Label::new(cx, "Arc knob");
                    Knob::custom(
                        cx,
                        0.5,
                        KnobData::knobs.map(|knobs| knobs[3]),
                        move |cx, lens| {
                            TickKnob::new(
                                cx,
                                Percentage(90.0),
                                // setting tick_width to 0 to make the tick invisible
                                Percentage(0.0),
                                Percentage(0.0),
                                300.0,
                                KnobMode::Continuous,
                            )
                            .value(lens.clone())
                            .class("track");
                            ArcTrack::new(
                                cx,
                                false,
                                Percentage(100.0),
                                Percentage(10.),
                                300.,
                                KnobMode::Continuous,
                            )
                            .value(lens)
                            .class("track")
                        },
                    )
                    .on_changing(move |cx, val| cx.emit(KnobChangeEvent::SetKnob(3, val)));
                    Label::new(cx, KnobData::knobs.map(|knobs| format!("{:.3}", knobs[3])));
                })
                .row_between(Pixels(10.0))
                .child_space(Stretch(1.0));

                // drag-able label
                VStack::new(cx, move |cx| {
                    Label::new(cx, "Label \"knob\"");
                    Knob::custom(cx, 0.5, KnobData::knobs.map(|knobs| knobs[4]), move |cx, val| {
                        HStack::new(cx, move |cx| {
                            Label::new(cx, "val:").width(Pixels(40.0));
                            Label::new(cx, val.map(|val| format!("{:.2}", val)))
                                .width(Pixels(40.0));
                        })
                        .class("label_knob")
                    })
                    .on_changing(move |cx, val| cx.emit(KnobChangeEvent::SetKnob(4, val)));
                    Label::new(cx, KnobData::knobs.map(|knobs| format!("{:.3}", knobs[4])));
                })
                .row_between(Pixels(10.0))
                .child_space(Stretch(1.0));
            })
            .col_between(Pixels(10.0))
            .background_color(Color::from("#191919"));
        },
    )
    .run();
}

#[derive(Lens)]
pub struct KnobData {
    knobs: Vec<f32>,
}

#[derive(Debug)]
pub enum KnobChangeEvent {
    SetKnob(usize, f32),
}
impl Model for KnobData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        if let Some(param_change_event) = event.message.downcast() {
            match param_change_event {
                KnobChangeEvent::SetKnob(idx, new_val) => {
                    self.knobs[*idx] = *new_val;
                }
            }
        }
    }
}
