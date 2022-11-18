use std::marker::PhantomData;

use crate::{prelude::*, style::Transform2D};
use chrono::{NaiveTime, Timelike};

use super::spinbox::SpinboxIcons;

const ICON_LEFT_OPEN: &str = "\u{e75d}";
const ICON_RIGHT_OPEN: &str = "\u{e75e}";

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
    SetHours(u8),
    SetMinutes(u8),
    SetPage(RadialTimepickerPage),
    SetZone(bool),
}

pub struct Timepicker<L: Lens, T: Timelike + Data> {
    lens: L,
    p: PhantomData<T>,
    on_change: Option<Box<dyn Fn(&mut EventContext, NaiveTime)>>,
}

impl<L, T> Timepicker<L, T>
where
    L: Lens<Target = T>,
    T: Timelike + Data,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self { lens, p: PhantomData::default(), on_change: None }.build(cx, move |cx| {
            HStack::new(cx, |cx| {
                Spinbox::new(
                    cx,
                    lens.map(|time| format!("{:#02}", time.hour12().1)),
                    SpinboxKind::Vertical,
                    SpinboxIcons::Math,
                )
                .on_increment(|ex| ex.emit(TimepickerEvent::IncrementHour))
                .on_decrement(|ex| ex.emit(TimepickerEvent::DecrementHour));
                VStack::new(cx, |cx| {
                    Element::new(cx).class("timepicker-dot");
                    Element::new(cx).class("timepicker-dot");
                })
                .class("timepicker-dots-wrapper");
                Spinbox::new(
                    cx,
                    lens.map(|time| format!("{:#02}", time.minute())),
                    SpinboxKind::Vertical,
                    SpinboxIcons::Math,
                )
                .on_increment(|ex| ex.emit(TimepickerEvent::IncrementMinutes))
                .on_decrement(|ex| ex.emit(TimepickerEvent::DecrementMinutes));
                VStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |cx| cx.emit(TimepickerEvent::ToggleAMOrPM),
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
                .class("timepicker-button-wrapper");
            })
            .class("timepicker-wrapper");
        })
    }
}

impl<'a, L, T> Handle<'a, Timepicker<L, T>>
where
    L: Lens<Target = T>,
    T: Timelike + Data,
{
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, NaiveTime),
    {
        self.modify(|timepicker: &mut Timepicker<L, T>| {
            timepicker.on_change = Some(Box::new(callback))
        })
    }
}

impl<L, T> View for Timepicker<L, T>
where
    L: Lens<Target = T>,
    T: Timelike + Data,
{
    fn element(&self) -> Option<&'static str> {
        Some("timepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            TimepickerEvent::IncrementHour => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    let mut hours = current.hour() + 1;

                    if hours == 12 {
                        hours = 0;
                    }

                    if hours == 24 {
                        hours = 12;
                    }

                    let new = NaiveTime::from_hms(hours, current.minute(), current.second());

                    (callback)(cx, new);
                }
            }

            TimepickerEvent::IncrementMinutes => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    let mut minutes = current.minute() + 1;

                    if minutes >= 60 {
                        minutes -= 60;
                    }

                    let new = NaiveTime::from_hms(current.hour(), minutes, current.second());

                    (callback)(cx, new);
                }
            }

            TimepickerEvent::DecrementHour => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    let mut hours = current.hour() as i32 - 1;

                    if hours < 0 {
                        hours += 12;
                    }

                    if current.hour12().0 && hours < 12 {
                        hours += 12;
                    }

                    let new = NaiveTime::from_hms(hours as u32, current.minute(), current.second());

                    (callback)(cx, new);
                }
            }

            TimepickerEvent::DecrementMinutes => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    let mut minutes = current.minute() as i32 - 1;

                    if minutes < 0 {
                        minutes += 60;
                    }

                    let new = NaiveTime::from_hms(current.hour(), minutes as u32, current.second());

                    (callback)(cx, new);
                }
            }

            TimepickerEvent::ToggleAMOrPM => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    let new = match current.hour12().0 {
                        false => NaiveTime::from_hms(
                            current.hour() + 12,
                            current.minute(),
                            current.second(),
                        ),

                        true => NaiveTime::from_hms(
                            current.hour() - 12,
                            current.minute(),
                            current.second(),
                        ),
                    };

                    (callback)(cx, new)
                }
            }

            _ => {}
        })
    }
}

#[derive(PartialEq, Data, Clone, Copy, Debug)]
pub enum RadialTimepickerPage {
    Hours,
    Minutes,
}

#[derive(Lens)]
pub struct RadialTimepicker<L: Lens, T: Copy + Timelike + Data> {
    #[lens(ignore)]
    lens: L,
    #[lens(ignore)]
    p: PhantomData<T>,
    page: RadialTimepickerPage,
    #[lens(ignore)]
    on_change: Option<Box<dyn Fn(&mut EventContext, NaiveTime)>>,
}

impl<L, T> RadialTimepicker<L, T>
where
    L: Lens<Target = T>,
    T: Timelike + Data + Copy,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self { lens, p: PhantomData::default(), page: RadialTimepickerPage::Hours, on_change: None }
            .build(cx, |cx| {
                HStack::new(cx, move |cx| {
                    Binding::new(cx, Self::page, move |cx, page| match page.get(cx) {
                        RadialTimepickerPage::Hours => {
                            Binding::new(cx, lens.map(|time| time.hour()), |cx, hours| {
                                let hours = hours.get(cx);

                                let angle = (hours) as f32 * 30.0;

                                let mut transform = Transform2D::identity();
                                transform.rotate(angle);
                                transform
                                    .premultiply(&Transform2D::identity().translate(0.0, -50.0));
                                Element::new(cx)
                                    .width(Pixels(1.0))
                                    .height(Pixels(100.0))
                                    .transform(transform)
                                    .position_type(PositionType::SelfDirected)
                                    .class("clock-hand");
                            });

                            for i in 0..12 {
                                let mut transform = Transform2D::identity();
                                transform.rotate(30.0 * (i + 1) as f32);
                                transform
                                    .premultiply(&Transform2D::identity().translate(0.0, -100.0));
                                transform.premultiply(
                                    &Transform2D::identity().rotate(-30.0 * (i + 1) as f32),
                                );

                                Label::new(cx, i + 1)
                                    .size(Pixels(32.0))
                                    .transform(transform)
                                    .position_type(PositionType::SelfDirected)
                                    .child_space(Stretch(1.0))
                                    .border_radius(Percentage(50.0))
                                    .cursor(CursorIcon::Hand)
                                    .on_press(move |ex| ex.emit(TimepickerEvent::SetHours(i + 1)))
                                    .class("marker")
                                    .checked(
                                        lens.map(move |time| time.hour12().1 == (i + 1) as u32),
                                    );
                            }
                        }

                        RadialTimepickerPage::Minutes => {
                            Binding::new(cx, lens.map(|time| time.minute()), |cx, minutes| {
                                let minutes = minutes.get(cx);

                                let angle = (minutes / 5) as f32 * 30.0;

                                let mut transform = Transform2D::identity();
                                transform.rotate(angle);
                                transform
                                    .premultiply(&Transform2D::identity().translate(0.0, -50.0));
                                Element::new(cx)
                                    .width(Pixels(1.0))
                                    .height(Pixels(100.0))
                                    .transform(transform)
                                    .position_type(PositionType::SelfDirected)
                                    .class("clock-hand");
                            });

                            for i in 0..12 {
                                let mut transform = Transform2D::identity();
                                transform.rotate(30.0 * i as f32);
                                transform
                                    .premultiply(&Transform2D::identity().translate(0.0, -100.0));
                                transform
                                    .premultiply(&Transform2D::identity().rotate(-30.0 * i as f32));

                                Label::new(cx, &format!("{:#02}", i * 5))
                                    .size(Pixels(32.0))
                                    .transform(transform)
                                    .position_type(PositionType::SelfDirected)
                                    .child_space(Stretch(1.0))
                                    .border_radius(Percentage(50.0))
                                    .cursor(CursorIcon::Hand)
                                    .on_press(move |ex| ex.emit(TimepickerEvent::SetMinutes(i * 5)))
                                    .class("marker")
                                    .checked(lens.map(move |time| time.minute() / 5 == i as u32));
                            }
                        }
                    });

                    Element::new(cx)
                        .size(Pixels(4.0))
                        .border_radius(Percentage(50.0))
                        .position_type(PositionType::SelfDirected)
                        .class("center-dot");
                })
                .child_space(Stretch(1.0))
                .border_radius(Percentage(50.0))
                .class("clock-face");

                Label::new(cx, ICON_LEFT_OPEN)
                    .position_type(PositionType::SelfDirected)
                    .size(Pixels(30.0))
                    .space(Stretch(1.0))
                    .left(Pixels(8.0))
                    .top(Pixels(8.0))
                    .child_space(Stretch(1.0))
                    //.background_color(Color::rgb(200, 200, 200))
                    .border_radius(Percentage(50.0))
                    .font("icons")
                    .disabled(Self::page.map(|page| page == &RadialTimepickerPage::Hours))
                    .class("switch-page-button")
                    .on_press(|cx| cx.emit(TimepickerEvent::SetPage(RadialTimepickerPage::Hours)));

                Label::new(cx, ICON_RIGHT_OPEN)
                    .position_type(PositionType::SelfDirected)
                    .size(Pixels(30.0))
                    .space(Stretch(1.0))
                    .right(Pixels(8.0))
                    .top(Pixels(8.0))
                    .child_space(Stretch(1.0))
                    //.background_color(Color::rgb(200, 200, 200))
                    .border_radius(Percentage(50.0))
                    .font("icons")
                    .disabled(Self::page.map(|page| page == &RadialTimepickerPage::Minutes))
                    .class("switch-page-button")
                    .on_press(|cx| {
                        cx.emit(TimepickerEvent::SetPage(RadialTimepickerPage::Minutes))
                    });

                Label::new(cx, "AM")
                    .position_type(PositionType::SelfDirected)
                    .size(Pixels(30.0))
                    .space(Stretch(1.0))
                    .left(Pixels(8.0))
                    .bottom(Pixels(8.0))
                    .child_space(Stretch(1.0))
                    //.background_color(Color::rgb(200, 200, 200))
                    .border_radius(Percentage(50.0))
                    .checked(lens.map(|time| !time.hour12().0))
                    .class("switch-zone-button")
                    .on_press(|cx| cx.emit(TimepickerEvent::SetZone(false)));

                Label::new(cx, "PM")
                    .position_type(PositionType::SelfDirected)
                    .size(Pixels(30.0))
                    .space(Stretch(1.0))
                    .right(Pixels(8.0))
                    .bottom(Pixels(8.0))
                    .child_space(Stretch(1.0))
                    //.background_color(Color::rgb(200, 200, 200))
                    .border_radius(Percentage(50.0))
                    .checked(lens.map(|time| time.hour12().0))
                    .class("switch-zone-button")
                    .on_press(|cx| cx.emit(TimepickerEvent::SetZone(true)));
            })
            // .child_space(Stretch(1.0))
            .size(Pixels(220.0))
    }
}

impl<L, T> View for RadialTimepicker<L, T>
where
    L: Lens<Target = T>,
    T: Timelike + Data + Copy,
{
    fn element(&self) -> Option<&'static str> {
        Some("radial_timepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|timepicker_event, _| match timepicker_event {
            TimepickerEvent::SetHours(hours) => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);
                    let mut new_hours =
                        if current.hour12().0 { *hours as u32 + 12 } else { *hours as u32 };
                    if new_hours == 24 {
                        new_hours = 12;
                    }
                    (callback)(
                        cx,
                        NaiveTime::from_hms(new_hours, current.minute(), current.second()),
                    );
                }
                self.page = RadialTimepickerPage::Minutes;
            }

            TimepickerEvent::SetMinutes(minutes) => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);
                    (callback)(
                        cx,
                        NaiveTime::from_hms(current.hour(), *minutes as u32, current.second()),
                    );
                }
            }

            TimepickerEvent::SetPage(page) => {
                self.page = *page;
            }

            TimepickerEvent::SetZone(zone) => {
                if let Some(callback) = &self.on_change {
                    let current = self.lens.get(cx);

                    match (current.hour12().0, zone) {
                        (false, true) => {
                            (callback)(
                                cx,
                                NaiveTime::from_hms(
                                    current.hour() + 12,
                                    current.minute(),
                                    current.second(),
                                ),
                            );
                        }

                        (true, false) => {
                            (callback)(
                                cx,
                                NaiveTime::from_hms(
                                    current.hour() - 12,
                                    current.minute(),
                                    current.second(),
                                ),
                            );
                        }

                        _ => {}
                    }
                }
            }
            _ => {}
        });
    }
}

impl<'v, L, T> Handle<'v, RadialTimepicker<L, T>>
where
    L: Lens<Target = T>,
    T: Timelike + Data + Copy,
{
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, NaiveTime),
    {
        self.modify(|timepicker: &mut RadialTimepicker<L, T>| {
            timepicker.on_change = Some(Box::new(callback))
        })
    }
}
