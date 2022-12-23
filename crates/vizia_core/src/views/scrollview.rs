use morphorm::{GeometryChanged, PositionType};

use crate::prelude::*;
use crate::state::RatioLens;
use crate::views::Orientation;

pub(crate) const SCROLL_SENSITIVITY: f32 = 35.0;

#[derive(Lens, Clone, Debug, PartialEq, Eq)]
pub struct ScrollData {
    scroll_x_bits: u32,
    scroll_y_bits: u32,
    child_x_bits: u32,
    child_y_bits: u32,
    parent_x_bits: u32,
    parent_y_bits: u32,
}

pub enum ScrollEvent {
    SetX(f32),
    SetY(f32),
    ScrollX(f32),
    ScrollY(f32),
    ChildGeo(f32, f32),
    ParentGeo(f32, f32),
}

impl ScrollData {
    pub fn new(
        scroll_x: f32,
        scroll_y: f32,
        child_x: f32,
        child_y: f32,
        parent_x: f32,
        parent_y: f32,
    ) -> Self {
        Self {
            scroll_x_bits: scroll_x.to_bits(),
            scroll_y_bits: scroll_y.to_bits(),
            child_x_bits: child_x.to_bits(),
            child_y_bits: child_y.to_bits(),
            parent_x_bits: parent_x.to_bits(),
            parent_y_bits: parent_y.to_bits(),
        }
    }

    pub fn scroll_x(&self) -> f32 {
        f32::from_bits(self.scroll_x_bits)
    }

    pub fn scroll_y(&self) -> f32 {
        f32::from_bits(self.scroll_y_bits)
    }

    pub fn parent_x(&self) -> f32 {
        f32::from_bits(self.parent_x_bits)
    }

    pub fn parent_y(&self) -> f32 {
        f32::from_bits(self.parent_y_bits)
    }

    pub fn child_x(&self) -> f32 {
        f32::from_bits(self.child_x_bits)
    }

    pub fn child_y(&self) -> f32 {
        f32::from_bits(self.child_y_bits)
    }

    fn reset(&mut self) {
        if self.child_x() == self.parent_x() {
            self.scroll_x_bits = 0.0f32.to_bits();
        }

        if self.child_y_bits == self.parent_y_bits {
            self.scroll_y_bits = 0.0f32.to_bits();
        }
    }
}

impl Model for ScrollData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|scroll_update, meta| {
            match scroll_update {
                ScrollEvent::ScrollX(f) => {
                    self.scroll_x_bits =
                        (f32::from_bits(self.scroll_x_bits) + *f).clamp(0.0, 1.0).to_bits()
                }
                ScrollEvent::ScrollY(f) => {
                    self.scroll_y_bits =
                        (f32::from_bits(self.scroll_y_bits) + *f).clamp(0.0, 1.0).to_bits()
                }
                ScrollEvent::SetX(f) => self.scroll_x_bits = (*f).to_bits(),
                ScrollEvent::SetY(f) => self.scroll_y_bits = (*f).to_bits(),
                ScrollEvent::ChildGeo(x, y) => {
                    self.child_x_bits = (*x).to_bits();
                    self.child_y_bits = (*y).to_bits();
                    self.reset();
                }
                ScrollEvent::ParentGeo(x, y) => {
                    self.parent_x_bits = (*x).to_bits();
                    self.parent_y_bits = (*y).to_bits();
                    self.reset();
                }
            }

            meta.consume();
        });
    }
}

pub struct ScrollView<L> {
    data: L,
}

impl ScrollView<scroll_data_derived_lenses::root> {
    pub fn new<F>(
        cx: &mut Context,
        initial_x: f32,
        initial_y: f32,
        scroll_x: bool,
        scroll_y: bool,
        content: F,
    ) -> Handle<Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        Self { data: ScrollData::root }.build(cx, move |cx| {
            ScrollData::new(initial_x, initial_y, 0.0, 0.0, 0.0, 0.0).build(cx);

            Self::common_builder(cx, ScrollData::root, content, scroll_x, scroll_y);
        })
    }
}

impl<L: Lens<Target = ScrollData>> ScrollView<L> {
    pub fn custom<F>(
        cx: &mut Context,
        scroll_x: bool,
        scroll_y: bool,
        data: L,
        content: F,
    ) -> Handle<Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        if cx.data::<ScrollData>().is_none() {
            panic!("ScrollView::custom requires a ScrollData to be built into a parent");
        }

        Self { data: data.clone() }.build(cx, |cx| {
            Self::common_builder(cx, data, content, scroll_x, scroll_y);
        })
    }

    fn common_builder<F>(cx: &mut Context, data: L, content: F, scroll_x: bool, scroll_y: bool)
    where
        F: 'static + FnOnce(&mut Context),
    {
        VStack::new(cx, content)
            .class("scroll_content")
            .bind(data.clone(), |handle, data| {
                let dpi_factor = handle.cx.style.dpi_factor;
                if dpi_factor > 0.0 {
                    let data = data.get(handle.cx);
                    let left = ((data.child_x() - data.parent_x()) * data.scroll_x()).round()
                        / handle.cx.style.dpi_factor as f32;
                    let top = ((data.child_y() - data.parent_y()) * data.scroll_y()).round()
                        / handle.cx.style.dpi_factor as f32;
                    handle.left(Pixels(-left.abs())).top(Pixels(-top.abs()));
                }
            })
            .on_geo_changed(|cx, geo| {
                if geo.contains(GeometryChanged::HEIGHT_CHANGED)
                    || geo.contains(GeometryChanged::WIDTH_CHANGED)
                {
                    let current = cx.current();
                    let width = cx.cache().get_width(current);
                    let height = cx.cache().get_height(current);
                    cx.emit(ScrollEvent::ChildGeo(width, height));
                }
            });
        if scroll_y {
            Scrollbar::new(
                cx,
                data.clone().then(ScrollData::scroll_y_bits).to_f32(),
                data.clone().then(RatioLens::new(
                    ScrollData::parent_y_bits.to_f32(),
                    ScrollData::child_y_bits.to_f32(),
                )),
                Orientation::Vertical,
                |cx, value| {
                    cx.emit(ScrollEvent::SetY(value));
                },
            )
            .position_type(PositionType::SelfDirected);
        }
        if scroll_x {
            Scrollbar::new(
                cx,
                data.clone().then(ScrollData::scroll_x_bits).to_f32(),
                data.clone().then(RatioLens::new(
                    ScrollData::parent_x_bits.to_f32(),
                    ScrollData::child_x_bits.to_f32(),
                )),
                Orientation::Horizontal,
                |cx, value| {
                    cx.emit(ScrollEvent::SetX(value));
                },
            )
            .position_type(PositionType::SelfDirected);
        }
    }
}

impl<L: Lens<Target = ScrollData>> View for ScrollView<L> {
    fn element(&self) -> Option<&'static str> {
        Some("scrollview")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::GeometryChanged(geo) => {
                if geo.contains(GeometryChanged::HEIGHT_CHANGED)
                    || geo.contains(GeometryChanged::WIDTH_CHANGED)
                {
                    let current = cx.current();
                    let width = cx.cache.get_width(current);
                    let height = cx.cache.get_height(current);
                    cx.emit(ScrollEvent::ParentGeo(width, height));
                }
            }

            WindowEvent::MouseScroll(x, y) => {
                let (x, y) =
                    if cx.modifiers.contains(Modifiers::SHIFT) { (-*y, -*x) } else { (-*x, -*y) };

                // what percentage of the negative space does this cross?
                let data = self.data.get(cx);
                if x != 0.0 {
                    let negative_space = data.child_x() - data.parent_x();
                    let logical_delta = x * SCROLL_SENSITIVITY / negative_space;
                    cx.emit(ScrollEvent::ScrollX(logical_delta));
                }
                let data = cx.data::<ScrollData>().unwrap();
                if y != 0.0 {
                    let negative_space = data.child_y() - data.parent_y();
                    let logical_delta = y * SCROLL_SENSITIVITY / negative_space;
                    cx.emit(ScrollEvent::ScrollY(logical_delta));
                }
            }

            _ => {}
        });
    }
}
