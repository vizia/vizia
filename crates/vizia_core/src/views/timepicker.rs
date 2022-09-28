use std::sync::Arc;

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
    SetOnChanging(Option<Arc<dyn Fn(&mut EventContext, DayTime) + Send + Sync>>),
    SetOnOk(Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>),
}

#[derive(Lens, Clone)]
pub struct Timepicker<L: Clone + Lens<Target = DayTime>> {
    lens: L,
    temp: DayTime,
    on_changing: Option<Arc<dyn Fn(&mut EventContext, DayTime) + Send + Sync>>,
    on_ok: Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>,
}

impl<L> Timepicker<L>
where
    L: Lens<Target = DayTime> + Clone,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self {
            lens: lens.clone(),
            on_changing: None,
            on_ok: None,
            temp: lens.clone().get(cx).clone(),
        }
        .build(cx, move |cx| {
            Binding::new(cx, lens.clone(), |cx, lens| {
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
                                Label::new(cx, "x").bind(
                                    lens.then(DayTime::zone),
                                    |handle, lens| {
                                        let lens = lens.get(handle.cx);
                                        match lens {
                                            AMOrPM::AM => handle.text("AM"),
                                            AMOrPM::PM => handle.text("PM"),
                                        };
                                    },
                                )
                            },
                        );

                        Button::new(
                            cx,
                            |cx| cx.emit(TimepickerEvent::Ok),
                            |cx| Label::new(cx, "Ok"),
                        );
                    })
                    .class("timepicker-button-wrapper");
                })
                .class("timepicker-wrapper");
            });
        })
    }
}

impl<'a, L> Handle<'a, Timepicker<L>>
where
    L: Lens<Target = DayTime>,
{
    pub fn on_changing<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, DayTime) + Send + Sync,
    {
        self.cx.emit_to(self.entity(), TimepickerEvent::SetOnChanging(Some(Arc::new(callback))));

        self
    }

    pub fn on_ok<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.cx.emit_to(self.entity(), TimepickerEvent::SetOnOk(Some(Arc::new(callback))));

        self
    }
}

impl<L> View for Timepicker<L>
where
    L: Lens<Target = DayTime>,
{
    fn element(&self) -> Option<&'static str> {
        Some("timepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            TimepickerEvent::IncrementHour => {
                self.temp.hour += 1;
                if self.temp.hour > 12 {
                    self.temp.hour -= 12;
                }

                if let Some(callback) = &self.on_changing {
                    (callback)(cx, self.temp.clone())
                }
            }

            TimepickerEvent::IncrementMinutes => {
                self.temp.minutes += 5;
                if self.temp.minutes >= 60 {
                    self.temp.minutes -= 60;
                }

                if let Some(callback) = &self.on_changing {
                    (callback)(cx, self.temp.clone())
                }
            }

            TimepickerEvent::DecrementHour => {
                self.temp.hour -= 1;
                if self.temp.hour == 0 {
                    self.temp.hour += 12;
                }

                if let Some(callback) = &self.on_changing {
                    (callback)(cx, self.temp.clone())
                }
            }

            TimepickerEvent::DecrementMinutes => {
                if self.temp.minutes < 5 {
                    self.temp.minutes += 60;
                }
                self.temp.minutes -= 5;

                if let Some(callback) = &self.on_changing {
                    (callback)(cx, self.temp.clone())
                }
            }

            TimepickerEvent::Ok => {
                if let Some(callback) = &self.on_ok {
                    (callback)(cx)
                }
            }

            TimepickerEvent::ToggleAMOrPM => {
                match self.temp.zone {
                    AMOrPM::AM => self.temp.zone = AMOrPM::PM,
                    AMOrPM::PM => self.temp.zone = AMOrPM::AM,
                }

                if let Some(callback) = &self.on_changing {
                    (callback)(cx, self.temp.clone())
                }
            }

            TimepickerEvent::SetOnChanging(callback) => {
                self.on_changing = callback.clone();
            }

            TimepickerEvent::SetOnOk(callback) => {
                self.on_ok = callback.clone();
            }
        })
    }
}
