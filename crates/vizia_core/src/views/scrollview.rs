use std::sync::Arc;

use morphorm::PositionType;

use crate::binding::RatioLens;
use crate::prelude::*;
use crate::views::Orientation;

pub(crate) const SCROLL_SENSITIVITY: f32 = 35.0;

#[derive(Lens, Data, Clone)]
pub struct ScrollData {
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub child_x: f32,
    pub child_y: f32,
    pub parent_x: f32,
    pub parent_y: f32,

    #[lens(ignore)]
    pub on_scroll: Option<Arc<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,
}

pub enum ScrollEvent {
    SetX(f32),
    SetY(f32),
    ScrollX(f32),
    ScrollY(f32),
    ScrollXPx(f32),
    ScrollYPx(f32),
    ChildGeo(f32, f32),
    ParentGeo(f32, f32),
    SetOnScroll(Option<Arc<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>),
}

impl ScrollData {
    fn reset(&mut self) {
        if self.child_x == self.parent_x {
            self.scroll_x = 0.0;
        }

        if self.child_y == self.parent_y {
            self.scroll_y = 0.0;
        }
    }
}

impl Model for ScrollData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|scroll_update, meta| {
            match scroll_update {
                ScrollEvent::ScrollX(f) => {
                    self.scroll_x = (self.scroll_x + *f).clamp(0.0, 1.0);

                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x, self.scroll_y);
                    }
                }
                ScrollEvent::ScrollY(f) => {
                    self.scroll_y = (self.scroll_y + *f).clamp(0.0, 1.0);
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x, self.scroll_y);
                    }
                }
                ScrollEvent::SetX(f) => {
                    self.scroll_x = *f;
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x, self.scroll_y);
                    }
                }
                ScrollEvent::SetY(f) => {
                    self.scroll_y = *f;
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x, self.scroll_y);
                    }
                }
                ScrollEvent::ChildGeo(x, y) => {
                    self.child_x = *x;
                    self.child_y = *y;
                    self.reset();
                }
                ScrollEvent::ParentGeo(x, y) => {
                    self.parent_x = *x;
                    self.parent_y = *y;
                    self.reset();
                }
                ScrollEvent::ScrollXPx(delta) => {
                    self.scroll_x = delta / self.parent_x - self.child_x;
                }
                ScrollEvent::ScrollYPx(delta) => {
                    let d = delta / (self.child_y - self.parent_y);
                    self.scroll_y += d;
                    self.scroll_y = self.scroll_y.clamp(0.0, 1.0);
                }
                ScrollEvent::SetOnScroll(on_scroll) => {
                    self.on_scroll = on_scroll.clone();
                }
            }

            meta.consume();
        });
    }
}

pub struct ScrollView<L> {
    data: L,
}

impl ScrollView<Wrapper<scroll_data_derived_lenses::root>> {
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
        Self { data: ScrollData::root }
            .build(cx, move |cx| {
                ScrollData {
                    scroll_x: initial_x,
                    scroll_y: initial_y,
                    child_x: 0.0,
                    child_y: 0.0,
                    parent_x: 0.0,
                    parent_y: 0.0,
                    on_scroll: None,
                }
                .build(cx);

                Self::common_builder(cx, ScrollData::root, content, scroll_x, scroll_y);
            })
            .checked(ScrollData::root.map(|data| data.parent_y != data.child_y))
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
        ScrollContent::new(cx, content).bind(data.clone(), |handle, data| {
            let dpi_factor = handle.scale_factor();
            if dpi_factor > 0.0 {
                let data = data.get(handle.cx);
                let left = ((data.child_x - data.parent_x) * data.scroll_x).round()
                    / handle.cx.style.dpi_factor as f32;
                let top = ((data.child_y - data.parent_y) * data.scroll_y).round()
                    / handle.cx.style.dpi_factor as f32;
                handle.left(Units::Pixels(-left.abs())).top(Units::Pixels(-top.abs()));
            }
        });

        if scroll_y {
            Scrollbar::new(
                cx,
                data.clone().then(ScrollData::scroll_y),
                data.clone().then(RatioLens::new(ScrollData::parent_y, ScrollData::child_y)),
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
                data.clone().then(ScrollData::scroll_x),
                data.then(RatioLens::new(ScrollData::parent_x, ScrollData::child_x)),
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
                if geo.contains(GeoChanged::WIDTH_CHANGED)
                    || geo.contains(GeoChanged::HEIGHT_CHANGED)
                {
                    let bounds = cx.bounds();
                    cx.emit(ScrollEvent::ParentGeo(bounds.w, bounds.h));
                }
            }

            WindowEvent::MouseScroll(x, y) => {
                cx.set_active(true);
                let (x, y) =
                    if cx.modifiers.contains(Modifiers::SHIFT) { (-*y, -*x) } else { (-*x, -*y) };

                // what percentage of the negative space does this cross?
                let data = self.data.get(cx);
                if x != 0.0 && data.child_x > data.parent_x {
                    let negative_space = data.child_x - data.parent_x;
                    let logical_delta = x * SCROLL_SENSITIVITY / negative_space;
                    cx.emit(ScrollEvent::ScrollX(logical_delta));
                }
                let data = cx.data::<ScrollData>().unwrap();
                if y != 0.0 && data.child_y > data.parent_y {
                    let negative_space = data.child_y - data.parent_y;
                    let logical_delta = y * SCROLL_SENSITIVITY / negative_space;
                    cx.emit(ScrollEvent::ScrollY(logical_delta));
                }
            }

            WindowEvent::MouseOut => {
                cx.set_active(false);
            }

            _ => {}
        });
    }
}

impl<'a, L: Lens> Handle<'a, ScrollView<L>> {
    pub fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.cx.emit_to(self.entity(), ScrollEvent::SetOnScroll(Some(Arc::new(callback))));
        self
    }
}

pub struct ScrollContent {}

impl ScrollContent {
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Self {}.build(cx, content).class("scroll_content")
    }
}

impl View for ScrollContent {
    fn element(&self) -> Option<&'static str> {
        Some("scroll_content")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::GeometryChanged(geo) => {
                if geo.contains(GeoChanged::WIDTH_CHANGED)
                    || geo.contains(GeoChanged::HEIGHT_CHANGED)
                {
                    let bounds = cx.bounds();
                    cx.emit(ScrollEvent::ChildGeo(bounds.w, bounds.h));
                }
            }

            _ => {}
        });
    }
}
