use std::sync::Arc;

use crate::binding::RatioLens;
use crate::prelude::*;

pub(crate) const SCROLL_SENSITIVITY: f32 = 20.0;

pub enum ScrollEvent {
    /// Sets the progress of scroll position between 0 and 1 for the x axis
    SetX(f32),
    /// Sets the progress of scroll position between 0 and 1 for the y axis
    SetY(f32),
    /// Adds given progress to scroll position for the x axis and clamps between 0 and 1
    ScrollX(f32),
    /// Adds given progress to scroll position for the y axis and clamps between 0 and 1
    ScrollY(f32),
    /// Sets the Size for the inner VStack which holds the content
    ChildGeo(f32, f32),
}

#[derive(Lens, Data, Clone)]
pub struct ScrollView {
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

    pub scroll_to_cursor: bool,
}

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
        F: 'static + FnOnce(&mut Context),
    {
        Self {
            scroll_to_cursor: false,
            scroll_x: initial_x,
            scroll_y: initial_y,
            on_scroll: None,
            inner_width: 0.0,
            inner_height: 0.0,
            container_width: 0.0,
            container_height: 0.0,
        }
        .build(cx, move |cx| {
            ScrollContent::new(cx, content).bind(Self::root, |handle, data| {
                let scale_factor = handle.scale_factor();
                let data = data.get(&handle);
                let left = ((data.inner_width - data.container_width) * data.scroll_x).round()
                    / scale_factor;
                let top = ((data.inner_height - data.container_height) * data.scroll_y).round()
                    / scale_factor;
                handle.left(Units::Pixels(-left.abs())).top(Units::Pixels(-top.abs()));
            });

            if scroll_y {
                Scrollbar::new(
                    cx,
                    Self::scroll_y,
                    RatioLens::new(Self::container_height, Self::inner_height),
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
                    Self::scroll_x,
                    RatioLens::new(Self::container_width, Self::inner_width),
                    Orientation::Horizontal,
                    |cx, value| {
                        cx.emit(ScrollEvent::SetX(value));
                    },
                )
                .position_type(PositionType::SelfDirected)
                .scroll_to_cursor(Self::scroll_to_cursor);
            }
        })
        .toggle_class("h-scroll", Self::root.map(|data| data.container_width < data.inner_width))
        .toggle_class("v-scroll", Self::root.map(|data| data.container_height < data.inner_height))
    }

    fn reset(&mut self) {
        if self.inner_width == self.container_width {
            self.scroll_x = 0.0;
        }

        if self.inner_height == self.container_height {
            self.scroll_y = 0.0;
        }
    }
}

impl View for ScrollView {
    fn element(&self) -> Option<&'static str> {
        Some("scrollview")
    }

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
            }

            // Prevent scroll events propagating to any parent scrollviews.
            // TODO: This might be desired behavior when the scrollview is scrolled all the way.
            meta.consume();
        });

        event.map(|window_event, meta| match window_event {
            WindowEvent::GeometryChanged(geo) => {
                if geo.contains(GeoChanged::WIDTH_CHANGED)
                    || geo.contains(GeoChanged::HEIGHT_CHANGED)
                {
                    let bounds = cx.bounds();
                    let scale_factor = cx.scale_factor();
                    let top = ((self.inner_height - self.container_height) * self.scroll_y).round()
                        / scale_factor;
                    let left = ((self.inner_width - self.container_width) * self.scroll_x).round()
                        / scale_factor;
                    self.container_width = bounds.width();
                    self.container_height = bounds.height();
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
            }

            WindowEvent::MouseScroll(x, y) => {
                cx.set_active(true);
                let (x, y) = if cx.modifiers.shift() { (-*y, -*x) } else { (-*x, -*y) };

                // What percentage of the negative space does this cross?
                if x != 0.0 && self.inner_width > self.container_width {
                    let negative_space = self.inner_width - self.container_width;
                    if negative_space != 0.0 {
                        let logical_delta = x * SCROLL_SENSITIVITY / negative_space;
                        cx.emit(ScrollEvent::ScrollX(logical_delta));
                    }
                    // Prevent event propagating to ancestor scrollviews.
                    meta.consume();
                }
                if y != 0.0 && self.inner_height > self.container_height {
                    let negative_space = self.inner_height - self.container_height;
                    if negative_space != 0.0 {
                        let logical_delta = y * SCROLL_SENSITIVITY / negative_space;
                        cx.emit(ScrollEvent::ScrollY(logical_delta));
                    }
                    // Prevent event propagating to ancestor scrollviews.
                    meta.consume();
                }
            }

            WindowEvent::MouseOut => {
                cx.set_active(false);
            }

            _ => {}
        });
    }
}

impl<'a> Handle<'a, ScrollView> {
    /// Sets a callback which will be called when a scrollview is scrolled, either with the mouse wheel, touchpad, or using the scroll bars.
    pub fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.modify(|scrollview: &mut ScrollView| scrollview.on_scroll = Some(Arc::new(callback)))
    }

    pub fn scroll_to_cursor(self, scroll_to_cursor: bool) -> Self {
        self.modify(|scrollview: &mut ScrollView| scrollview.scroll_to_cursor = scroll_to_cursor)
    }

    pub fn scrollx(self, scrollx: impl Res<f32>) -> Self {
        self.bind(scrollx, |handle, scrollx| {
            let sx = scrollx.get(&handle);
            handle.modify(|scrollview| scrollview.scroll_x = sx);
        })
    }

    pub fn scrolly(self, scrollx: impl Res<f32>) -> Self {
        self.bind(scrollx, |handle, scrolly| {
            let sy = scrolly.get(&handle);
            handle.modify(|scrollview| scrollview.scroll_y = sy);
        })
    }
}

struct ScrollContent {}

impl ScrollContent {
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Self {}.build(cx, content)
    }
}

impl View for ScrollContent {
    fn element(&self) -> Option<&'static str> {
        Some("scroll-content")
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
