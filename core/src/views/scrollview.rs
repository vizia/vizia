use morphorm::{GeometryChanged, PositionType};

use crate::prelude::*;
use crate::state::RatioLens;
use crate::views::Orientation;

const SCROLL_SENSITIVITY: f32 = 35.0;

#[derive(Lens, Data, Clone)]
pub struct ScrollData {
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub child_x: f32,
    pub child_y: f32,
    pub parent_x: f32,
    pub parent_y: f32,
}

pub enum ScrollEvent {
    SetX(f32),
    SetY(f32),
    ScrollX(f32),
    ScrollY(f32),
    ChildGeo(f32, f32),
    ParentGeo(f32, f32),
}

impl Model for ScrollData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        event.map(|scroll_update, meta| {
            match scroll_update {
                ScrollEvent::ScrollX(f) => self.scroll_x = (self.scroll_x + *f).clamp(0.0, 1.0),
                ScrollEvent::ScrollY(f) => self.scroll_y = (self.scroll_y + *f).clamp(0.0, 1.0),
                ScrollEvent::SetX(f) => self.scroll_x = *f,
                ScrollEvent::SetY(f) => self.scroll_y = *f,
                ScrollEvent::ChildGeo(x, y) => {
                    self.child_x = *x;
                    self.child_y = *y;
                }
                ScrollEvent::ParentGeo(x, y) => {
                    self.parent_x = *x;
                    self.parent_y = *y;
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
    pub fn new<F>(cx: &mut Context, settings: ScrollViewSettings, content: F) -> Handle<Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        Self { data: ScrollData::root }.build(cx, move |cx| {
            ScrollData {
                scroll_x: settings.initial_x,
                scroll_y: settings.initial_y,
                child_x: 0.0,
                child_y: 0.0,
                parent_x: 0.0,
                parent_y: 0.0,
            }
            .build(cx);

            Self::common_builder(cx, ScrollData::root, content, settings);
        })
    }
}

impl<L: Lens<Target = ScrollData>> ScrollView<L> {
    pub fn custom<F>(
        cx: &mut Context,
        settings: ScrollViewSettings,
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
            Self::common_builder(cx, data, content, settings);
        })
    }

    fn common_builder<F>(cx: &mut Context, data: L, content: F, settings: ScrollViewSettings)
    where
        F: 'static + FnOnce(&mut Context),
    {
        VStack::new(cx, content)
            .class("scroll_content")
            .bind(data.clone(), move |handle, data| {
                let dpi_factor = handle.cx.style().dpi_factor;
                if dpi_factor > 0.0 {
                    let data = data.get(handle.cx);
                    let left = if settings.scroll_x {
                        ((data.child_x - data.parent_x) * data.scroll_x).round()
                            / handle.cx.style().dpi_factor as f32
                    } else {
                        0.0
                    };
                    let top = if settings.scroll_y {
                        ((data.child_y - data.parent_y) * data.scroll_y).round()
                            / handle.cx.style().dpi_factor as f32
                    } else {
                        0.0
                    };
                    handle.left(Units::Pixels(-left.abs())).top(Units::Pixels(-top.abs()));
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

        if settings.scrollbar_y {
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

        if settings.scrollbar_x {
            Scrollbar::new(
                cx,
                data.clone().then(ScrollData::scroll_x),
                data.clone().then(RatioLens::new(ScrollData::parent_x, ScrollData::child_x)),
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

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::GeometryChanged(geo) => {
                if geo.contains(GeometryChanged::HEIGHT_CHANGED)
                    || geo.contains(GeometryChanged::WIDTH_CHANGED)
                {
                    let current = cx.current();
                    let width = cx.cache().get_width(current);
                    let height = cx.cache().get_height(current);
                    cx.emit(ScrollEvent::ParentGeo(width, height));
                }
            }

            WindowEvent::MouseScroll(x, y) => {
                let (x, y) =
                    if cx.modifiers().contains(Modifiers::SHIFT) { (-*y, *x) } else { (*x, -*y) };

                // what percentage of the negative space does this cross?
                let data = self.data.get(cx);
                if x != 0.0 {
                    let negative_space = data.child_x - data.parent_x;
                    let logical_delta = x * SCROLL_SENSITIVITY / negative_space;
                    cx.emit(ScrollEvent::ScrollX(logical_delta));
                }
                let data = cx.data::<ScrollData>().unwrap();
                if y != 0.0 {
                    let negative_space = data.child_y - data.parent_y;
                    let logical_delta = y * SCROLL_SENSITIVITY / negative_space;
                    cx.emit(ScrollEvent::ScrollY(logical_delta));
                }
            }

            _ => {}
        });
    }
}

/// The settings of a scroll view.
///
/// These settings are used at the construction time of a [`ScrollView`].
#[derive(Debug, Clone, PartialEq)]
pub struct ScrollViewSettings {
    /// Specifies the initial horizontal position of the scroll view ranging from 0.0 (left) to 1.0 (right).
    pub initial_x: f32,
    /// Specifies the initial vertical position of the scroll view ranging from 0.0 (top) to 1.0 (bottom).
    pub initial_y: f32,
    /// Specifies if a horizontal scrollbar should automatically be added to the scroll view.
    pub scrollbar_x: bool,
    /// Specifies if a vertical scrollbar should automatically be added to the scroll view.
    pub scrollbar_y: bool,
    /// Specifies if the scroll view can be scrolled horizontally.
    ///
    /// If this is set to `false` the scroll view won't bind to the [`ScrollData::scroll_x`] lens.
    pub scroll_x: bool,
    /// Specifies if the scroll view can be scrolled vertically.
    ///
    /// If this is set to `false` the scroll view won't bind to the [`ScrollData::scroll_y`] lens.
    pub scroll_y: bool,
}

impl ScrollViewSettings {
    /// Creates new scroll view settings.
    pub fn new(
        initial_x: f32,
        initial_y: f32,
        scrollbar_x: bool,
        scrollbar_y: bool,
        scroll_x: bool,
        scroll_y: bool,
    ) -> Self {
        Self { initial_x, initial_y, scrollbar_x, scrollbar_y, scroll_x, scroll_y }
    }
}

impl Default for ScrollViewSettings {
    fn default() -> Self {
        Self {
            initial_x: 0.0,
            initial_y: 0.0,
            scrollbar_x: true,
            scrollbar_y: true,
            scroll_x: true,
            scroll_y: true,
        }
    }
}
