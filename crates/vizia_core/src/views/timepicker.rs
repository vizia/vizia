use crate::{prelude::*, style::Transform2D};

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
pub struct RadialTimepicker {
    hours: u8,
    minutes: u8,
    page: RadialTimepickerPage,
}

impl RadialTimepicker {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self { hours: 1, minutes: 10, page: RadialTimepickerPage::Hours }
            .build(cx, |cx| {
                HStack::new(cx, |cx| {
                    Binding::new(cx, RadialTimepicker::page, |cx, page| match page.get(cx) {
                        RadialTimepickerPage::Hours => {
                            Binding::new(cx, RadialTimepicker::hours, |cx, hours| {
                                let hours = hours.get(cx);

                                let angle = (hours) as f32 * 30.0;

                                let mut transform = Transform2D::identity();
                                transform.rotate(angle);
                                transform
                                    .premultiply(&Transform2D::identity().translate(0.0, -57.0));
                                Element::new(cx)
                                    .width(Pixels(1.0))
                                    .height(Pixels(90.0))
                                    .transform(transform)
                                    .position_type(PositionType::SelfDirected)
                                    .class("clock-hand");
                            });

                            for i in 0..12 {
                                let mut transform = Transform2D::identity();
                                transform.rotate(30.0 * (i + 1) as f32);
                                transform
                                    .premultiply(&Transform2D::identity().translate(0.0, -105.0));
                                transform.premultiply(
                                    &Transform2D::identity().rotate(-30.0 * (i + 1) as f32),
                                );

                                Label::new(cx, i + 1)
                                    .size(Pixels(30.0))
                                    .transform(transform)
                                    .position_type(PositionType::SelfDirected)
                                    .child_space(Stretch(1.0))
                                    .border_radius(Percentage(50.0))
                                    .cursor(CursorIcon::Hand)
                                    .on_press(move |ex| ex.emit(TimepickerEvent::SetHours(i + 1)))
                                    .class("marker")
                                    .checked(
                                        RadialTimepicker::hours.map(move |hours| *hours == i + 1),
                                    );
                            }
                        }

                        RadialTimepickerPage::Minutes => {
                            Binding::new(cx, RadialTimepicker::minutes, |cx, minutes| {
                                let minutes = minutes.get(cx);

                                let angle = (minutes / 5) as f32 * 30.0;

                                let mut transform = Transform2D::identity();
                                transform.rotate(angle);
                                transform
                                    .premultiply(&Transform2D::identity().translate(0.0, -57.0));
                                Element::new(cx)
                                    .width(Pixels(1.0))
                                    .height(Pixels(90.0))
                                    .transform(transform)
                                    .position_type(PositionType::SelfDirected)
                                    .class("clock-hand");
                            });

                            for i in 0..12 {
                                let mut transform = Transform2D::identity();
                                transform.rotate(30.0 * i as f32);
                                transform
                                    .premultiply(&Transform2D::identity().translate(0.0, -105.0));
                                transform
                                    .premultiply(&Transform2D::identity().rotate(-30.0 * i as f32));

                                Label::new(cx, &format!("{:#02}", i * 5))
                                    .size(Pixels(30.0))
                                    .transform(transform)
                                    .position_type(PositionType::SelfDirected)
                                    .child_space(Stretch(1.0))
                                    .border_radius(Percentage(50.0))
                                    .cursor(CursorIcon::Hand)
                                    .on_press(move |ex| ex.emit(TimepickerEvent::SetMinutes(i * 5)))
                                    .class("marker")
                                    .checked(
                                        RadialTimepicker::minutes
                                            .map(move |minutes| *minutes / 5 == i),
                                    );
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
                    .disabled(
                        RadialTimepicker::page.map(|page| page == &RadialTimepickerPage::Hours),
                    )
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
                    .disabled(
                        RadialTimepicker::page.map(|page| page == &RadialTimepickerPage::Minutes),
                    )
                    .class("switch-page-button")
                    .on_press(|cx| {
                        cx.emit(TimepickerEvent::SetPage(RadialTimepickerPage::Minutes))
                    });
            })
            // .child_space(Stretch(1.0))
            .size(Pixels(220.0))
    }
}

impl View for RadialTimepicker {
    fn element(&self) -> Option<&'static str> {
        Some("radial_timepicker")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|timepicker_event, _| match timepicker_event {
            TimepickerEvent::SetHours(hours) => {
                self.hours = *hours;
                self.page = RadialTimepickerPage::Minutes;
            }

            TimepickerEvent::SetMinutes(minutes) => {
                self.minutes = *minutes;
            }

            TimepickerEvent::SetPage(page) => {
                self.page = *page;
            }

            _ => {}
        });
    }

    // fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
    //     let bounds = cx.bounds();

    //     match self.page {
    //         RadialTimepickerPage::Hours => {
    //             let angle = (self.hours - 1) as f32 * 30.0;
    //             canvas.save();
    //             canvas.translate(bounds.x + bounds.w / 2.0, bounds.y + bounds.h / 2.0);
    //             let mut path = vg::Path::new();
    //             path.move_to(0.0, 0.0);
    //             path.line_to(0.0, -100.0);
    //             canvas.stroke_path(&mut path, &vg::Paint::color(Color::black().into()));
    //             canvas.restore();
    //         }

    //         _ => {} //RadialTimepickerPage::Minutes => (self.minutes as f32 / 5.0) * 30.0,
    //     }

    //     // println!("angle: {}", angle);
    // }
}
