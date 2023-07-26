use std::sync::Arc;

use morphorm::PositionType;

use crate::binding::RatioLens;
use crate::prelude::*;
use crate::views::Orientation;

pub(crate) const SCROLL_SENSITIVITY: f32 = 35.0;

#[derive(Lens, Data, Clone)]
pub struct ScrollData {
    /// Progress of scroll position between 0 and 1 for the x axis
    pub scroll_x: f32,
    /// Progress of scroll position between 0 and 1 for the y axis
    pub scroll_y: f32,

    /// Callback called when the scrollview is scrolled.
    #[lens(ignore)]
    pub on_scroll: Option<Arc<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,

    /// Width of the inner VStack which holds the content (typically bigger than container_width)
    pub inner_width: f32,
    /// Height of the inner VStack which holds the content (typically bigger than container_height)
    pub inner_height: f32,
    /// Width of the outer `ScrollView` which wraps the inner (typically smaller than inner_width)
    pub container_width: f32,
    /// Height of the outer `ScrollView` which wraps the inner (typically smaller than inner_height)
    pub container_height: f32,
}

pub enum ScrollEvent {
    /// Sets the progress of scroll position between 0 and 1 for the x axis
    SetX(f32),
    /// Sets the progress of scroll position between 0 and 1 for the y axis
    SetY(f32),
    /// Adds given progress to scroll position for the x axis and clamps between 0 and 1
    ScrollX(f32),
    /// Adds given progress to scroll position for the y axis and clamps between 0 and 1
    ScrollY(f32),
    /// Sets the Size for the inner VStack which holds the content (typically bigger than `ParentGeo(f32, f32)`)
    ChildGeo(f32, f32),
    /// Sets the Size for the outer `ScrollView` which wraps the inner (typically smaller than `ChildGeo(f32, f32)`)
    ParentGeo(f32, f32),
    /// Sets the `on_scroll` callback.
    SetOnScroll(Option<Arc<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>),
}

impl ScrollData {
    fn reset(&mut self) {
        if self.inner_width == self.container_width {
            self.scroll_x = 0.0;
        }

        if self.inner_height == self.container_height {
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

                ScrollEvent::ChildGeo(w, h) => {
                    self.inner_width = *w;
                    self.inner_height = *h;
                    self.reset();
                }
                ScrollEvent::ParentGeo(w, h) => {
                    let scale_factor = cx.scale_factor();
                    let top = ((self.inner_height - self.container_height) * self.scroll_y).round()
                        / scale_factor;
                    let left = ((self.inner_width - self.container_width) * self.scroll_x).round()
                        / scale_factor;
                    self.container_width = *w;
                    self.container_height = *h;
                    self.scroll_y = ((top * scale_factor)
                        / (self.inner_height - self.container_height))
                        .clamp(0.0, 1.0);
                    self.scroll_x = ((left * scale_factor)
                        / (self.inner_width - self.container_width))
                        .clamp(0.0, 1.0);
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x, self.scroll_y);
                    }

                    self.reset();
                }

                ScrollEvent::SetOnScroll(on_scroll) => {
                    self.on_scroll = on_scroll.clone();
                }
            }

            // Prevent scroll events propagating to any parent scrollviews.
            // TODO: This might be desired behavior when the scrollview is scrolled all the way.
            meta.consume();
        });
    }
}

#[derive(Lens)]
pub struct ScrollView<L: Lens> {
    data: L,
    scroll_to_cursor: bool,
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
        Self { data: ScrollData::root, scroll_to_cursor: false }
            .build(cx, move |cx| {
                ScrollData {
                    scroll_x: initial_x,
                    scroll_y: initial_y,
                    inner_width: 0.0,
                    inner_height: 0.0,
                    container_width: 0.0,
                    container_height: 0.0,
                    on_scroll: None,
                }
                .build(cx);

                Self::common_builder(cx, ScrollData::root, content, scroll_x, scroll_y);
            })
            .checked(ScrollData::root.map(|data| {
                (data.container_height < data.inner_height)
                    || (data.container_width < data.inner_width)
            }))
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

        Self { data, scroll_to_cursor: false }.build(cx, |cx| {
            Self::common_builder(cx, data, content, scroll_x, scroll_y);
        })
    }

    fn common_builder<F>(cx: &mut Context, data: L, content: F, scroll_x: bool, scroll_y: bool)
    where
        F: 'static + FnOnce(&mut Context),
    {
        ScrollContent::new(cx, content).bind(data, |handle, data| {
            let scale_factor = handle.scale_factor();
            let data = data.get(handle.cx);
            let left =
                ((data.inner_width - data.container_width) * data.scroll_x).round() / scale_factor;
            let top = ((data.inner_height - data.container_height) * data.scroll_y).round()
                / scale_factor;
            handle.left(Units::Pixels(-left.abs())).top(Units::Pixels(-top.abs()));
        });

        if scroll_y {
            Scrollbar::new(
                cx,
                data.then(ScrollData::scroll_y),
                data.then(RatioLens::new(ScrollData::container_height, ScrollData::inner_height)),
                Orientation::Vertical,
                |cx, value| {
                    cx.emit(ScrollEvent::SetY(value));
                },
            )
            .position_type(PositionType::SelfDirected)
            .scroll_to_cursor(Self::scroll_to_cursor);
        }

        if scroll_x {
            Scrollbar::new(
                cx,
                data.then(ScrollData::scroll_x),
                data.then(RatioLens::new(ScrollData::container_width, ScrollData::inner_width)),
                Orientation::Horizontal,
                |cx, value| {
                    cx.emit(ScrollEvent::SetX(value));
                },
            )
            .position_type(PositionType::SelfDirected)
            .scroll_to_cursor(Self::scroll_to_cursor);
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

                // What percentage of the negative space does this cross?
                let data = self.data.get(cx);
                if x != 0.0 && data.inner_width > data.container_width {
                    let negative_space = data.inner_width - data.container_width;
                    let logical_delta = x * SCROLL_SENSITIVITY / negative_space;
                    cx.emit(ScrollEvent::ScrollX(logical_delta));
                }
                if y != 0.0 && data.inner_height > data.container_height {
                    let negative_space = data.inner_height - data.container_height;
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
    /// Sets a callback which will be called when a scrollview is scrolled, either with the mouse wheel, touchpad, or using the scroll bars.
    pub fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.cx.emit_to(self.entity(), ScrollEvent::SetOnScroll(Some(Arc::new(callback))));
        self
    }

    pub fn scroll_to_cursor(self, scroll_to_cursor: bool) -> Self {
        self.modify(|scrollview: &mut ScrollView<L>| scrollview.scroll_to_cursor = scroll_to_cursor)
    }
}

struct ScrollContent {}

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
                    // If the width or height have changed then send this back up to the ScrollData.
                    cx.emit(ScrollEvent::ChildGeo(bounds.w, bounds.h));
                }
            }

            _ => {}
        });
    }
}
