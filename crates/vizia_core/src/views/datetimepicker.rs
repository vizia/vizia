use crate::prelude::*;

const ICON_CALENDAR: &str = "\u{1f4c5}";
const ICON_CLOCK: &str = "\u{1f554}";

#[derive(Lens, Setter)]
pub struct DatetimePicker {
    tabs: Vec<&'static str>,
    current_time: DayTime,
}

impl DatetimePicker {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            tabs: vec!["calendar", "clock"],
            current_time: DayTime { hour: 10, minutes: 9, zone: AMOrPM::PM },
        }
        .build(cx, |cx| {
            TabView::new(cx, DatetimePicker::tabs, |cx, item| match item.get(cx) {
                "calendar" => TabPair::new(
                    move |cx| {
                        Label::new(cx, ICON_CALENDAR);
                        Element::new(cx).class("indicator");
                    },
                    |cx| {
                        Datepicker::new(cx);
                    },
                ),

                "clock" => TabPair::new(
                    move |cx| {
                        Label::new(cx, ICON_CLOCK).font("icons");
                        Element::new(cx).class("indicator");
                    },
                    |cx| {
                        RadialTimepicker::new(cx)
                            // .on_changing(|cx, day_time| {
                            //     cx.emit(DatetimePickerSetter::CurrentTime(day_time.clone()));
                            // })
                            .size(Stretch(1.0));
                    },
                ),

                _ => TabPair::new(|_| {}, |_| {}),
            });
        })
    }
}

impl View for DatetimePicker {
    fn element(&self) -> Option<&'static str> {
        Some("datetimepicker")
    }
}
