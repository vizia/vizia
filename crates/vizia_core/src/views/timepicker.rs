use std::marker::PhantomData;

use crate::prelude::*;
use chrono::{NaiveTime, Timelike};

use super::spinbox::SpinboxIcons;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AMOrPM {
    AM,
    PM,
}

#[derive(PartialEq, Lens, Clone, Copy, Debug)]
pub struct DayTime {
    pub hour: u8,
    pub minutes: u8,
    pub zone: AMOrPM,
}

#[derive(Lens)]
pub struct Timepicker {
    on_change: Option<Box<dyn Fn(&mut EventContext, NaiveTime)>>,
}

pub enum TimepickerEvent {
    Change(NaiveTime),
    ChangePage(AnalogTimepickerPage),
}

impl Timepicker {
    pub fn new<L, T>(cx: &mut Context, lens: L) -> Handle<Self>
    where
        L: Lens<Target = T>,
        T: Timelike + Data + Copy,
    {
        Self { on_change: None }
            .build(cx, move |cx| {
                DigitalTimepicker::new(cx, lens)
                    .on_change(|cx, time| cx.emit(TimepickerEvent::Change(time)));
                AnalogTimepicker::new(cx, lens)
                    .on_change(|cx, time| cx.emit(TimepickerEvent::Change(time)))
                    .show_controls(false)
                    .change_page_on_select(false);
            })
            .layout_type(LayoutType::Column)
    }
}

impl View for Timepicker {
    fn element(&self) -> Option<&'static str> {
        Some("timepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            TimepickerEvent::Change(time) => {
                if let Some(callback) = &self.on_change {
                    (callback)(cx, *time)
                }
            }

            TimepickerEvent::ChangePage(page) => {
                cx.emit_custom(
                    Event::new(AnalogTimepickerEvent::SetPage(*page))
                        .origin(cx.current())
                        .propagate(Propagation::Subtree),
                );
            }
        })
    }
}

impl<'a> Handle<'a, Timepicker> {
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, NaiveTime),
    {
        self.modify(|timepicker: &mut Timepicker| timepicker.on_change = Some(Box::new(callback)))
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum DigitalTimepickerEvent {
    IncrementHour,
    IncrementMinutes,
    DecrementHour,
    DecrementMinutes,
    SetHour(u32),
    SetMinutes(u32),
    ToggleAMOrPM,
}

pub struct DigitalTimepicker<L: Lens, T: Timelike + Data> {
    lens: L,
    p: PhantomData<T>,
    on_change: Option<Box<dyn Fn(&mut EventContext, NaiveTime)>>,
}

impl<L, T> DigitalTimepicker<L, T>
where
    L: Lens<Target = T>,
    T: Timelike + Data,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self { lens, p: PhantomData, on_change: None }
            .build(cx, move |cx| {
                Spinbox::custom(
                    cx,
                    |cx| {
                        Textbox::new(cx, lens.map(|time| format!("{:#02}", time.hour12().1)))
                            .on_submit(|cx, text, _| {
                                if let Ok(parsed) = text.parse::<u32>() {
                                    if parsed < 24 {
                                        cx.emit(DigitalTimepickerEvent::SetHour(parsed))
                                    }
                                }
                            })
                            .width(Pixels(38.))
                            .overflow(Overflow::Hidden)
                    },
                    SpinboxKind::Vertical,
                    SpinboxIcons::PlusMinus,
                )
                .on_increment(|ex| ex.emit(DigitalTimepickerEvent::IncrementHour))
                .on_decrement(|ex| ex.emit(DigitalTimepickerEvent::DecrementHour))
                .on_press_down(|cx| {
                    cx.emit(TimepickerEvent::ChangePage(AnalogTimepickerPage::Hours))
                });
                VStack::new(cx, |cx| {
                    Element::new(cx).class("digitaltimepicker-dot");
                    Element::new(cx).class("digitaltimepicker-dot");
                })
                .class("digitaltimepicker-dots-wrapper");
                Spinbox::custom(
                    cx,
                    |cx| {
                        Textbox::new(cx, lens.map(|time| format!("{:#02}", time.minute())))
                            .on_submit(|cx, text, _| {
                                if let Ok(parsed) = text.parse::<u32>() {
                                    if parsed < 60 {
                                        cx.emit(DigitalTimepickerEvent::SetMinutes(parsed))
                                    }
                                }
                            })
                            .width(Pixels(38.))
                            .overflow(Overflow::Hidden)
                    },
                    SpinboxKind::Vertical,
                    SpinboxIcons::PlusMinus,
                )
                .on_increment(|ex| ex.emit(DigitalTimepickerEvent::IncrementMinutes))
                .on_decrement(|ex| ex.emit(DigitalTimepickerEvent::DecrementMinutes))
                .on_press_down(|cx| {
                    cx.emit(TimepickerEvent::ChangePage(AnalogTimepickerPage::Minutes))
                });
                VStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |cx| cx.emit(DigitalTimepickerEvent::ToggleAMOrPM),
                        |cx| {
                            Label::new(
                                cx,
                                lens.map(|time| match time.hour12().0 {
                                    false => "AM",
                                    true => "PM",
                                }),
                            )
                        },
                    );
                })
                .class("digitaltimepicker-button-wrapper");
            })
            .layout_type(LayoutType::Row)
    }
}

impl<'a, L, T> Handle<'a, DigitalTimepicker<L, T>>
where
    L: Lens<Target = T>,
    T: Timelike + Data,
{
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, NaiveTime),
    {
        self.modify(|timepicker: &mut DigitalTimepicker<L, T>| {
            timepicker.on_change = Some(Box::new(callback))
        })
    }
}

impl<L, T> View for DigitalTimepicker<L, T>
where
    L: Lens<Target = T>,
    T: Timelike + Data,
{
    fn element(&self) -> Option<&'static str> {
        Some("digitaltimepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            DigitalTimepickerEvent::SetHour(mut hour) => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    if hour == 12 {
                        hour = 0;
                    }

                    if hour == 24 {
                        hour = 12;
                    }

                    let new =
                        NaiveTime::from_hms_opt(hour, current.minute(), current.second()).unwrap();

                    (callback)(cx, new);
                }
            }

            DigitalTimepickerEvent::SetMinutes(minutes) => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    let new = NaiveTime::from_hms_opt(current.hour(), *minutes, current.second())
                        .unwrap();

                    (callback)(cx, new);
                }
            }

            DigitalTimepickerEvent::IncrementHour => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    let mut hours = current.hour() + 1;

                    if hours == 12 {
                        hours = 0;
                    }

                    if hours == 24 {
                        hours = 12;
                    }

                    let new =
                        NaiveTime::from_hms_opt(hours, current.minute(), current.second()).unwrap();

                    (callback)(cx, new);
                }
            }

            DigitalTimepickerEvent::IncrementMinutes => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    let mut minutes = current.minute() + 1;

                    if minutes >= 60 {
                        minutes -= 60;
                    }

                    let new =
                        NaiveTime::from_hms_opt(current.hour(), minutes, current.second()).unwrap();

                    (callback)(cx, new);
                }
            }

            DigitalTimepickerEvent::DecrementHour => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    let mut hours = current.hour() as i32 - 1;

                    if hours < 0 {
                        hours += 12;
                    }

                    if current.hour12().0 && hours < 12 {
                        hours += 12;
                    }

                    let new =
                        NaiveTime::from_hms_opt(hours as u32, current.minute(), current.second())
                            .unwrap();

                    (callback)(cx, new);
                }
            }

            DigitalTimepickerEvent::DecrementMinutes => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    let mut minutes = current.minute() as i32 - 1;

                    if minutes < 0 {
                        minutes += 60;
                    }

                    let new =
                        NaiveTime::from_hms_opt(current.hour(), minutes as u32, current.second())
                            .unwrap();

                    (callback)(cx, new);
                }
            }

            DigitalTimepickerEvent::ToggleAMOrPM => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    let new = match current.hour12().0 {
                        false => NaiveTime::from_hms_opt(
                            current.hour() + 12,
                            current.minute(),
                            current.second(),
                        ),

                        true => NaiveTime::from_hms_opt(
                            current.hour() - 12,
                            current.minute(),
                            current.second(),
                        ),
                    }
                    .unwrap();

                    (callback)(cx, new)
                }
            }
        })
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Data)]
pub enum AnalogTimepickerPage {
    Hours,
    Minutes,
    Seconds,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AnalogTimepickerEvent {
    ToggleAMOrPM,
    SetHours(u8),
    SetMinutes(u8),
    SetSeconds(u8),
    SetPage(AnalogTimepickerPage),
    SetZone(bool),
}

#[derive(Lens)]
pub struct AnalogTimepicker<L: Lens, T: Copy + Timelike + Data> {
    #[lens(ignore)]
    lens: L,
    #[lens(ignore)]
    p: PhantomData<T>,
    page: AnalogTimepickerPage,
    show_controls: bool,
    change_page_on_select: bool,
    #[lens(ignore)]
    on_change: Option<Box<dyn Fn(&mut EventContext, NaiveTime)>>,
}

impl<L, T> AnalogTimepicker<L, T>
where
    L: Lens<Target = T>,
    T: Timelike + Data + Copy,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self {
            lens,
            p: PhantomData,
            page: AnalogTimepickerPage::Hours,
            on_change: None,
            show_controls: true,
            change_page_on_select: true,
        }
        .build(cx, move |cx| {
            HStack::new(cx, move |cx| {
                Binding::new(cx, Self::page, move |cx, page| match page.get(cx) {
                    AnalogTimepickerPage::Hours => {
                        Binding::new(cx, lens.map(|time| time.hour()), |cx, hours| {
                            let hours = hours.get(cx);

                            let angle = (hours) as f32 * 30.0;

                            // let mut transform = Transform2D::identity();
                            // transform.rotate(angle);
                            // transform.premultiply(&Transform2D::identity().translate(0.0, -44.0));
                            Element::new(cx)
                                // .transform(vec![
                                //     Transform::Rotate(Angle::Deg(angle)),
                                //     Transform::TranslateY(Pixels(-44.0).into()),
                                // ])
                                .rotate(Angle::Deg(angle))
                                .position_type(PositionType::SelfDirected)
                                .class("clock-hand");
                        });

                        for i in 0..12 {
                            // let mut transform = Transform2D::identity();
                            // transform.rotate(30.0 * (i + 1) as f32);
                            // transform.premultiply(&Transform2D::identity().translate(0.0, -74.0));
                            // transform.premultiply(
                            //     &Transform2D::identity().rotate(-30.0 * (i + 1) as f32),
                            // );

                            Label::new(cx, i + 1)
                                // .transform(vec![
                                //     Transform::Rotate(Angle::Deg(30.0 * (i + 1) as f32)),
                                //     Transform::TranslateY(Pixels(-74.0).into()),
                                // ])
                                .rotate(Angle::Deg(30.0 * (i + 1) as f32))
                                .position_type(PositionType::SelfDirected)
                                .on_press(move |ex| ex.emit(AnalogTimepickerEvent::SetHours(i + 1)))
                                .class("marker")
                                .checked(lens.map(move |time| time.hour12().1 == (i + 1) as u32));
                        }
                    }

                    AnalogTimepickerPage::Minutes => {
                        Binding::new(cx, lens.map(|time| time.minute()), |cx, minutes| {
                            let minutes = minutes.get(cx);

                            let angle = (minutes / 5) as f32 * 30.0;

                            // let mut transform = Transform2D::identity();
                            // transform.rotate(angle);
                            // transform.premultiply(&Transform2D::identity().translate(0.0, -44.0));
                            Element::new(cx)
                                .rotate(Angle::Deg(angle))
                                .position_type(PositionType::SelfDirected)
                                .class("clock-hand");
                        });

                        for i in 0..12 {
                            // let mut transform = Transform2D::identity();
                            // transform.rotate(30.0 * i as f32);
                            // transform.premultiply(&Transform2D::identity().translate(0.0, -74.0));
                            // transform
                            //     .premultiply(&Transform2D::identity().rotate(-30.0 * i as f32));

                            Label::new(cx, &format!("{:#02}", i * 5))
                                .rotate(Angle::Deg(30.0 * i as f32))
                                .position_type(PositionType::SelfDirected)
                                .on_press(move |ex| {
                                    ex.emit(AnalogTimepickerEvent::SetMinutes(i * 5))
                                })
                                .class("marker")
                                .checked(lens.map(move |time| time.minute() / 5 == i as u32));
                        }
                    }

                    AnalogTimepickerPage::Seconds => {
                        Binding::new(cx, lens.map(|time| time.second()), |cx, seconds| {
                            let seconds = seconds.get(cx);

                            let angle = (seconds / 5) as f32 * 30.0;

                            Element::new(cx)
                                .rotate(Angle::Deg(angle))
                                .position_type(PositionType::SelfDirected)
                                .class("clock-hand");
                        });

                        for i in 0..12 {
                            Label::new(cx, &format!("{:#02}", i * 5))
                                .rotate(Angle::Deg(30.0 * i as f32))
                                .position_type(PositionType::SelfDirected)
                                .on_press(move |ex| {
                                    ex.emit(AnalogTimepickerEvent::SetSeconds(i * 5))
                                })
                                .class("marker")
                                .checked(lens.map(move |time| time.second() / 5 == i as u32));
                        }
                    }
                });

                Element::new(cx).position_type(PositionType::SelfDirected).class("center-dot");
            })
            .class("clock-face");

            // Binding::new(cx, Self::show_controls, move |cx, show_controls| {
            //     if show_controls.get(cx) {
            //         VStack::new(cx, |cx| {
            //             Binding::new(cx, lens1.clone(), |cx, lens| {
            //                 let (hour, minute, second) =
            //                     (lens.get(cx).hour(), lens.get(cx).minute(), lens.get(cx).second());

            //                 HStack::new(cx, |cx| {
            //                     Button::new(
            //                         cx,
            //                         |ex| {
            //                             ex.emit(AnalogTimepickerEvent::SetPage(
            //                                 AnalogTimepickerPage::Hours,
            //                             ))
            //                         },
            //                         |cx| {
            //                             Label::new(cx, hour).hoverable(false);
            //                             Element::new(cx).class("indicator")
            //                         },
            //                     )
            //                     .checked(
            //                         Self::page.map(|page| page == &AnalogTimepickerPage::Hours),
            //                     );
            //                     Button::new(
            //                         cx,
            //                         |ex| {
            //                             ex.emit(AnalogTimepickerEvent::SetPage(
            //                                 AnalogTimepickerPage::Minutes,
            //                             ))
            //                         },
            //                         |cx| {
            //                             Label::new(cx, minute).hoverable(false);
            //                             Element::new(cx).class("indicator")
            //                         },
            //                     )
            //                     .checked(
            //                         Self::page.map(|page| page == &AnalogTimepickerPage::Minutes),
            //                     );
            //                     Button::new(
            //                         cx,
            //                         |ex| {
            //                             ex.emit(AnalogTimepickerEvent::SetPage(
            //                                 AnalogTimepickerPage::Seconds,
            //                             ))
            //                         },
            //                         |cx| {
            //                             Label::new(cx, second).hoverable(false);
            //                             Element::new(cx).class("indicator")
            //                         },
            //                     )
            //                     .checked(
            //                         Self::page.map(|page| page == &AnalogTimepickerPage::Seconds),
            //                     );
            //                 })
            //                 .class("time-selector-wrapper");

            //                 Button::new(
            //                     cx,
            //                     |ex| ex.emit(AnalogTimepickerEvent::ToggleAMOrPM),
            //                     |cx| {
            //                         Label::new(cx, "AM").bind(lens, |h, lens| {
            //                             if lens.get(h.cx).hour12().0 {
            //                                 h.text("PM");
            //                             } else {
            //                                 h.text("AM");
            //                             }
            //                         })
            //                     },
            //                 )
            //                 .class("accent");
            //             })
            //         })
            //         .position_type(PositionType::SelfDirected)
            //         .class("controls-wrapper");
            //     }
            // })
        })
    }
}

impl<L, T> View for AnalogTimepicker<L, T>
where
    L: Lens<Target = T>,
    T: Timelike + Data + Copy,
{
    fn element(&self) -> Option<&'static str> {
        Some("analogtimepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|timepicker_event, _| match timepicker_event {
            AnalogTimepickerEvent::SetHours(hours) => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);
                    let mut new_hours =
                        if current.hour12().0 { *hours as u32 + 12 } else { *hours as u32 };
                    if new_hours == 24 {
                        new_hours = 12;
                    }
                    (callback)(
                        cx,
                        NaiveTime::from_hms_opt(new_hours, current.minute(), current.second())
                            .unwrap(),
                    );
                }
                if self.change_page_on_select {
                    self.page = AnalogTimepickerPage::Minutes;
                }
            }

            AnalogTimepickerEvent::SetMinutes(minutes) => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);
                    (callback)(
                        cx,
                        NaiveTime::from_hms_opt(current.hour(), *minutes as u32, current.second())
                            .unwrap(),
                    );
                }
                if self.change_page_on_select {
                    self.page = AnalogTimepickerPage::Seconds;
                }
            }

            AnalogTimepickerEvent::SetSeconds(seconds) => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);
                    (callback)(
                        cx,
                        NaiveTime::from_hms_opt(current.hour(), current.minute(), *seconds as u32)
                            .unwrap(),
                    );
                }
            }

            AnalogTimepickerEvent::SetPage(page) => {
                self.page = *page;
            }

            AnalogTimepickerEvent::ToggleAMOrPM => {
                cx.emit(AnalogTimepickerEvent::SetZone(!self.lens.get(cx).hour12().0))
            }

            AnalogTimepickerEvent::SetZone(zone) => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    match (current.hour12().0, zone) {
                        (false, true) => {
                            (callback)(
                                cx,
                                NaiveTime::from_hms_opt(
                                    current.hour() + 12,
                                    current.minute(),
                                    current.second(),
                                )
                                .unwrap(),
                            );
                        }

                        (true, false) => {
                            (callback)(
                                cx,
                                NaiveTime::from_hms_opt(
                                    current.hour() - 12,
                                    current.minute(),
                                    current.second(),
                                )
                                .unwrap(),
                            );
                        }

                        _ => {}
                    }
                }
            }
        });
    }
}

impl<'v, L, T> Handle<'v, AnalogTimepicker<L, T>>
where
    L: Lens<Target = T>,
    T: Timelike + Data + Copy,
{
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, NaiveTime),
    {
        self.modify(|timepicker: &mut AnalogTimepicker<L, T>| {
            timepicker.on_change = Some(Box::new(callback))
        })
    }

    pub fn show_controls(self, value: bool) -> Self {
        self.modify(|timepicker| timepicker.show_controls = value)
    }

    pub fn change_page_on_select(self, value: bool) -> Self {
        self.modify(|timepicker| timepicker.change_page_on_select = value)
    }
}
