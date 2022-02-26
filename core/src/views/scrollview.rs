use morphorm::{GeometryChanged, PositionType};

use crate::{
    Actions, Canvas, Context, Data, Event, HStack, Handle, Lens, LensExt, Model, Modifiers,
    Orientation, Scrollbar, Units, VStack, View, WindowEvent,
};

const SCROLL_SENSITIVITY: f32 = 35.0;

#[derive(Copy, Clone, Debug)]
pub struct RatioLens<L1, L2> {
    numerator: L1,
    denominator: L2,
}

impl<L1, L2> RatioLens<L1, L2> {
    pub fn new(numerator: L1, denominator: L2) -> Self {
        Self { numerator, denominator }
    }
}

impl<L1, L2> Lens for RatioLens<L1, L2>
where
    L1: 'static + Clone + Lens<Target = f32>,
    L2: 'static + Clone + Lens<Target = f32, Source = <L1 as Lens>::Source>,
{
    type Source = L1::Source;
    type Target = f32;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        let num = self.numerator.view(source, |num| num.copied());
        if let Some(num) = num {
            let den = self.denominator.view(source, |den| den.copied());
            if let Some(den) = den {
                map(Some(&(num / den)))
            } else {
                map(None)
            }
        } else {
            map(None)
        }
    }
}

#[derive(Lens, Data, Clone)]
pub struct ScrollData {
    scroll_x: f32,
    scroll_y: f32,
    child_x: f32,
    child_y: f32,
    parent_x: f32,
    parent_y: f32,
}

pub enum ScrollUpdate {
    SetX(f32),
    SetY(f32),
    ScrollX(f32),
    ScrollY(f32),
    ChildGeo(f32, f32),
    ParentGeo(f32, f32),
}

impl Model for ScrollData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(msg) = event.message.downcast() {
            match msg {
                ScrollUpdate::ScrollX(f) => self.scroll_x = (self.scroll_x + *f).clamp(0.0, 1.0),
                ScrollUpdate::ScrollY(f) => self.scroll_y = (self.scroll_y + *f).clamp(0.0, 1.0),
                ScrollUpdate::SetX(f) => self.scroll_x = *f,
                ScrollUpdate::SetY(f) => self.scroll_y = *f,
                ScrollUpdate::ChildGeo(x, y) => {
                    self.child_x = *x;
                    self.child_y = *y;
                }
                ScrollUpdate::ParentGeo(x, y) => {
                    self.parent_x = *x;
                    self.parent_y = *y;
                }
            }
            event.consume();
        }
    }
}

pub struct ScrollView {}

impl ScrollView {
    pub fn new<F>(
        cx: &mut Context,
        initial_x: f32,
        initial_y: f32,
        scroll_x: bool,
        scroll_y: bool,
        content: F,
    ) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        Self {}.build2(cx, move |cx| {
            ScrollData {
                scroll_x: initial_x,
                scroll_y: initial_y,
                child_x: 0.0,
                child_y: 0.0,
                parent_x: 0.0,
                parent_y: 0.0,
            }
            .build(cx);

            VStack::new(cx, content)
                .size(Units::Auto)
                .position_type(PositionType::SelfDirected)
                .bind(ScrollData::root, |handle, data| {
                    let data = data.get(handle.cx);
                    let left = (data.child_x - data.parent_x) * data.scroll_x;
                    let top = (data.child_y - data.parent_y) * data.scroll_y;
                    handle.left(Units::Pixels(-left)).top(Units::Pixels(-top));
                })
                .on_geo_changed(|cx, geo| {
                    if geo.contains(GeometryChanged::HEIGHT_CHANGED)
                        || geo.contains(GeometryChanged::WIDTH_CHANGED)
                    {
                        cx.emit(ScrollUpdate::ChildGeo(
                            cx.cache.get_width(cx.current),
                            cx.cache.get_height(cx.current),
                        ));
                    }
                });
            if scroll_y {
                Scrollbar::new(
                    cx,
                    ScrollData::scroll_y,
                    RatioLens::new(ScrollData::parent_y, ScrollData::child_y),
                    Orientation::Vertical,
                    |cx, value| {
                        cx.emit(ScrollUpdate::SetY(value));
                    },
                )
                .position_type(PositionType::SelfDirected);
            }
            if scroll_x {
                Scrollbar::new(
                    cx,
                    ScrollData::scroll_x,
                    RatioLens::new(ScrollData::parent_x, ScrollData::child_x),
                    Orientation::Horizontal,
                    |cx, value| {
                        cx.emit(ScrollUpdate::SetX(value));
                    },
                )
                .position_type(PositionType::SelfDirected);
            }
        })
    }
}

impl View for ScrollView {
    fn element(&self) -> Option<String> {
        Some("scrollview".to_owned())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::GeometryChanged(geo) => {
                    if geo.contains(GeometryChanged::HEIGHT_CHANGED)
                        || geo.contains(GeometryChanged::WIDTH_CHANGED)
                    {
                        cx.emit(ScrollUpdate::ParentGeo(
                            cx.cache.get_width(cx.current),
                            cx.cache.get_height(cx.current),
                        ));
                    }
                }

                WindowEvent::MouseScroll(x, y) => {
                    let (x, y) =
                        if cx.modifiers.contains(Modifiers::SHIFT) { (-*y, *x) } else { (*x, -*y) };

                    // what percentage of the negative space does this cross?
                    let data = cx.data::<ScrollData>().unwrap();
                    if x != 0.0 {
                        let negative_space = data.child_x - data.parent_x;
                        let logical_delta = x * SCROLL_SENSITIVITY / negative_space;
                        cx.emit(ScrollUpdate::ScrollX(logical_delta));
                    }
                    let data = cx.data::<ScrollData>().unwrap();
                    if y != 0.0 {
                        let negative_space = data.child_y - data.parent_y;
                        let logical_delta = y * SCROLL_SENSITIVITY / negative_space;
                        cx.emit(ScrollUpdate::ScrollY(logical_delta));
                    }
                }

                _ => {}
            }
        }
    }
}
