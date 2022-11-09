use crate::prelude::*;

#[derive(PartialEq, Data, Clone, Copy, Debug)]
pub enum AMOrPM {
    AM,
    PM,
}

#[derive(PartialEq, Data, Lens, Clone, Copy, Debug)]
pub struct DayTime {
    pub hour: u8,
    pub minutes: u8,
    pub zone: AMOrPM,
}

pub enum TimepickerEvent {
    Ok,
    IncrementHour,
    IncrementMinutes,
    DecrementHour,
    DecrementMinutes,
    ToggleAMOrPM,
}

#[derive(Lens)]
pub struct Timepicker {
    current: DayTime,
    on_changing: Option<Box<dyn Fn(&mut EventContext, DayTime) + Send + Sync>>,
    on_ok: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
}

impl Timepicker {
    pub fn new<L>(cx: &mut Context, lens: L) -> Handle<Self>
    where
        L: Lens<Target = DayTime>,
    {
        Self { on_changing: None, on_ok: None, current: lens.clone().get(cx).clone() }.build(
            cx,
            move |cx| {
                HStack::new(cx, |cx| {
                    Spinbox::new(cx, lens.clone().then(DayTime::hour), SpinboxKind::Vertical)
                        .on_increment(|ex| ex.emit(TimepickerEvent::IncrementHour))
                        .on_decrement(|ex| ex.emit(TimepickerEvent::DecrementHour));
                    VStack::new(cx, |cx| {
                        Element::new(cx).class("timepicker-dot");
                        Element::new(cx).class("timepicker-dot");
                    })
                    .class("timepicker-dots-wrapper");
                    Spinbox::new(cx, lens.clone().then(DayTime::minutes), SpinboxKind::Vertical)
                        .on_increment(|ex| ex.emit(TimepickerEvent::IncrementMinutes))
                        .on_decrement(|ex| ex.emit(TimepickerEvent::DecrementMinutes));
                    VStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |cx| cx.emit(TimepickerEvent::ToggleAMOrPM),
                            |cx| {
                                Label::new(
                                    cx,
                                    lens.then(DayTime::zone).map(|zone| match zone {
                                        AMOrPM::AM => "AM",
                                        AMOrPM::PM => "PM",
                                    }),
                                )
                            },
                        );

                        Button::new(
                            cx,
                            |cx| cx.emit(TimepickerEvent::Ok),
                            |cx| Label::new(cx, "Ok").width(Stretch(1.0)),
                        )
                        .width(Stretch(1.0))
                        .class("accent");
                    })
                    .class("timepicker-button-wrapper");
                })
                .class("timepicker-wrapper");
            },
        )
    }
}

impl<'a> Handle<'a, Timepicker> {
    pub fn on_changing<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, DayTime) + Send + Sync,
    {
        self.modify(|timepicker: &mut Timepicker| timepicker.on_changing = Some(Box::new(callback)))
    }

    pub fn on_ok<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.modify(|timepicker: &mut Timepicker| timepicker.on_ok = Some(Box::new(callback)))
    }
}

impl View for Timepicker {
    fn element(&self) -> Option<&'static str> {
        Some("timepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            TimepickerEvent::IncrementHour => {
                self.current.hour += 1;
                if self.current.hour > 12 {
                    self.current.hour -= 12;
                }

                if let Some(callback) = &self.on_changing {
                    (callback)(cx, self.current)
                }
            }

            TimepickerEvent::IncrementMinutes => {
                self.current.minutes += 1;
                if self.current.minutes >= 60 {
                    self.current.minutes -= 60;
                }

                if let Some(callback) = &self.on_changing {
                    (callback)(cx, self.current)
                }
            }

            TimepickerEvent::DecrementHour => {
                self.current.hour -= 1;
                if self.current.hour == 0 {
                    self.current.hour += 12;
                }

                if let Some(callback) = &self.on_changing {
                    (callback)(cx, self.current)
                }
            }

            TimepickerEvent::DecrementMinutes => {
                if self.current.minutes < 5 {
                    self.current.minutes += 60;
                }
                self.current.minutes -= 5;

                if let Some(callback) = &self.on_changing {
                    (callback)(cx, self.current)
                }
            }

            TimepickerEvent::Ok => {
                if let Some(callback) = &self.on_ok {
                    (callback)(cx)
                }
            }

            TimepickerEvent::ToggleAMOrPM => {
                match self.current.zone {
                    AMOrPM::AM => self.current.zone = AMOrPM::PM,
                    AMOrPM::PM => self.current.zone = AMOrPM::AM,
                }

                if let Some(callback) = &self.on_changing {
                    (callback)(cx, self.current.clone())
                }
            }
        })
    }
}
