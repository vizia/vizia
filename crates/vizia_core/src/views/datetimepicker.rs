use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::prelude::*;

const ICON_CALENDAR: &str = "\u{1f4c5}";
const ICON_CLOCK: &str = "\u{1f554}";

pub enum DatetimePickerEvent {
    SetDate(NaiveDate),
    SetTime(NaiveTime),
}

#[derive(Lens)]
pub struct DatetimePicker<L: Lens> {
    lens: L,
    tabs: Vec<&'static str>,
    on_change: Option<Box<dyn Fn(&mut EventContext, NaiveDateTime)>>,
}

impl<L> DatetimePicker<L>
where
    L: Lens<Target = NaiveDateTime>,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self { lens, tabs: vec!["calendar", "clock"], on_change: None }.build(cx, |cx| {
            TabView::new(cx, Self::tabs, move |cx, item| match item.get(cx) {
                "calendar" => TabPair::new(
                    move |cx| {
                        Label::new(cx, ICON_CALENDAR).hoverable(false);
                        Element::new(cx).class("indicator");
                    },
                    move |cx| {
                        Datepicker::new(cx, lens.map(|datetime| datetime.date()))
                            .on_select(|cx, date| cx.emit(DatetimePickerEvent::SetDate(date)));
                    },
                ),

                "clock" => TabPair::new(
                    move |cx| {
                        Label::new(cx, ICON_CLOCK).font("icons").hoverable(false);
                        Element::new(cx).class("indicator");
                    },
                    move |cx| {
                        AnalogTimepicker::new(cx, lens.map(|datetime| datetime.time()))
                            .on_change(|cx, time| {
                                cx.emit(DatetimePickerEvent::SetTime(time));
                            })
                            .size(Stretch(1.0));
                    },
                ),

                _ => TabPair::new(|_| {}, |_| {}),
            });
        })
    }
}

impl<L> View for DatetimePicker<L>
where
    L: Lens<Target = NaiveDateTime>,
{
    fn element(&self) -> Option<&'static str> {
        Some("datetimepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|datetimepicker_event, _| match datetimepicker_event {
            DatetimePickerEvent::SetDate(date) => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);
                    let new = NaiveDateTime::new(*date, current.time());
                    (callback)(cx, new);
                }
            }

            DatetimePickerEvent::SetTime(time) => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);
                    let new = NaiveDateTime::new(current.date(), *time);
                    (callback)(cx, new);
                }
            }
        });
    }
}

impl<'v, L> Handle<'v, DatetimePicker<L>>
where
    L: Lens<Target = NaiveDateTime>,
{
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, NaiveDateTime),
    {
        self.modify(|datetimepicker: &mut DatetimePicker<L>| {
            datetimepicker.on_change = Some(Box::new(callback))
        })
    }
}
