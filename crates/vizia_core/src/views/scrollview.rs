use std::sync::Arc;

use crate::prelude::*;

pub(crate) const SCROLL_SENSITIVITY: f32 = 20.0;

/// Events for setting the properties of a scroll view.
pub enum ScrollEvent {
    /// Sets the progress of scroll position between 0 and 1 for the x axis
    SetX(f32),
    /// Sets the progress of scroll position between 0 and 1 for the y axis
    SetY(f32),
    /// Adds given progress to scroll position for the x axis and clamps between 0 and 1
    ScrollX(f32),
    /// Adds given progress to scroll position for the y axis and clamps between 0 and 1
    ScrollY(f32),
    /// Sets the size for the inner scroll-content view which holds the content
    ChildGeo(f32, f32),

    ScrollToView(Entity),
}

/// A container a view which allows the user to scroll any overflowed content.
pub struct ScrollView {
    /// Progress of scroll position between 0 and 1 for the x axis
    scroll_x: Signal<f32>,
    /// Progress of scroll position between 0 and 1 for the y axis
    scroll_y: Signal<f32>,
    /// Callback called when the scrollview is scrolled.
    on_scroll: Option<Arc<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,
    /// Width of the inner VStack which holds the content (typically bigger than container_width)
    inner_width: Signal<f32>,
    /// Height of the inner VStack which holds the content (typically bigger than container_height)
    inner_height: Signal<f32>,
    /// Width of the outer `ScrollView` which wraps the inner (typically smaller than inner_width)
    container_width: Signal<f32>,
    /// Height of the outer `ScrollView` which wraps the inner (typically smaller than inner_height)
    container_height: Signal<f32>,
    /// Whether the scrollbar should move to the cursor when pressed.
    scroll_to_cursor: Signal<bool>,
    /// Whether the horizontal scrollbar should be visible.
    show_horizontal_scrollbar: Signal<bool>,
    /// Whether the vertical scrollbar should be visible.
    show_vertical_scrollbar: Signal<bool>,
}

impl ScrollView {
    /// Creates a new [ScrollView].
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        let scroll_x = cx.state(0.0);
        let scroll_y = cx.state(0.0);
        let inner_width = cx.state(0.0);
        let inner_height = cx.state(0.0);
        let container_width = cx.state(0.0);
        let container_height = cx.state(0.0);
        let scroll_to_cursor = cx.state(false);
        let show_horizontal_scrollbar = cx.state(true);
        let show_vertical_scrollbar = cx.state(true);
        let position_absolute = cx.state(PositionType::Absolute);
        let horizontal_offset = cx.state(0.0);
        let vertical_offset = cx.state(0.0);

        let ratio_x = cx.derived({
            let container_width = container_width;
            let inner_width = inner_width;
            move |store| *container_width.get(store) / *inner_width.get(store)
        });

        let ratio_y = cx.derived({
            let container_height = container_height;
            let inner_height = inner_height;
            move |store| *container_height.get(store) / *inner_height.get(store)
        });

        let scroll_metrics = cx.derived({
            let scroll_x = scroll_x;
            let scroll_y = scroll_y;
            let inner_width = inner_width;
            let inner_height = inner_height;
            let container_width = container_width;
            let container_height = container_height;
            move |store| {
                let left =
                    (*inner_width.get(store) - *container_width.get(store)) * *scroll_x.get(store);
                let top = (*inner_height.get(store) - *container_height.get(store))
                    * *scroll_y.get(store);
                (left, top)
            }
        });

        let h_scroll = cx.derived({
            let container_width = container_width;
            let inner_width = inner_width;
            move |store| *container_width.get(store) < *inner_width.get(store)
        });

        let v_scroll = cx.derived({
            let container_height = container_height;
            let inner_height = inner_height;
            move |store| *container_height.get(store) < *inner_height.get(store)
        });

        Self {
            scroll_x,
            scroll_y,
            on_scroll: None,
            inner_width,
            inner_height,
            container_width,
            container_height,
            scroll_to_cursor,
            show_horizontal_scrollbar,
            show_vertical_scrollbar,
        }
        .build(cx, move |cx| {
            ScrollContent::new(cx, content);

            Binding::new(cx, show_vertical_scrollbar, move |cx| {
                if *show_vertical_scrollbar.get(cx) {
                    Scrollbar::new(cx, scroll_y, ratio_y, Orientation::Vertical, |cx, value| {
                        cx.emit(ScrollEvent::SetY(value));
                    })
                    .position_type(position_absolute)
                    .scroll_to_cursor(scroll_to_cursor);
                }
            });

            Binding::new(cx, show_horizontal_scrollbar, move |cx| {
                if *show_horizontal_scrollbar.get(cx) {
                    Scrollbar::new(cx, scroll_x, ratio_x, Orientation::Horizontal, |cx, value| {
                        cx.emit(ScrollEvent::SetX(value));
                    })
                    .position_type(position_absolute)
                    .scroll_to_cursor(scroll_to_cursor);
                }
            });
        })
        .horizontal_scroll(horizontal_offset)
        .vertical_scroll(vertical_offset)
        .bind(scroll_metrics, move |mut handle, metrics| {
            let (left, top) = *metrics.get(&handle);
            let scale_factor = handle.context().scale_factor();
            let top = top.round() / scale_factor;
            let left = left.round() / scale_factor;
            let mut event_cx = EventContext::new(handle.cx);
            horizontal_offset.set(&mut event_cx, -left.abs());
            vertical_offset.set(&mut event_cx, -top.abs());
        })
        .toggle_class("h-scroll", h_scroll)
        .toggle_class("v-scroll", v_scroll)
    }

    fn reset(&mut self, cx: &mut EventContext) {
        if *self.inner_width.get(cx) == *self.container_width.get(cx) {
            self.scroll_x.set(cx, 0.0);
        }

        if *self.inner_height.get(cx) == *self.container_height.get(cx) {
            self.scroll_y.set(cx, 0.0);
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
                    self.scroll_x.upd(cx, |scroll_x| {
                        *scroll_x = (*scroll_x + *f).clamp(0.0, 1.0);
                    });

                    if let Some(callback) = &self.on_scroll {
                        let scroll_x = *self.scroll_x.get(cx);
                        let scroll_y = *self.scroll_y.get(cx);
                        (callback)(cx, scroll_x, scroll_y);
                    }
                }

                ScrollEvent::ScrollY(f) => {
                    self.scroll_y.upd(cx, |scroll_y| {
                        *scroll_y = (*scroll_y + *f).clamp(0.0, 1.0);
                    });
                    if let Some(callback) = &self.on_scroll {
                        let scroll_x = *self.scroll_x.get(cx);
                        let scroll_y = *self.scroll_y.get(cx);
                        (callback)(cx, scroll_x, scroll_y);
                    }
                }

                ScrollEvent::SetX(f) => {
                    self.scroll_x.set(cx, *f);
                    if let Some(callback) = &self.on_scroll {
                        let scroll_y = *self.scroll_y.get(cx);
                        (callback)(cx, *f, scroll_y);
                    }
                }

                ScrollEvent::SetY(f) => {
                    self.scroll_y.set(cx, *f);
                    if let Some(callback) = &self.on_scroll {
                        let scroll_x = *self.scroll_x.get(cx);
                        (callback)(cx, scroll_x, *f);
                    }
                }

                ScrollEvent::ChildGeo(w, h) => {
                    let bounds = cx.bounds();
                    let scale_factor = cx.scale_factor();

                    let inner_width = *self.inner_width.get(cx);
                    let inner_height = *self.inner_height.get(cx);
                    let container_width = *self.container_width.get(cx);
                    let container_height = *self.container_height.get(cx);

                    if inner_width != 0.0
                        && inner_height != 0.0
                        && container_width != 0.0
                        && container_height != 0.0
                    {
                        let scroll_x = *self.scroll_x.get(cx);
                        let scroll_y = *self.scroll_y.get(cx);

                        let top =
                            ((inner_height - container_height) * scroll_y).round() / scale_factor;
                        let left =
                            ((inner_width - container_width) * scroll_x).round() / scale_factor;

                        let new_container_width = bounds.width();
                        let new_container_height = bounds.height();
                        let new_inner_width = *w;
                        let new_inner_height = *h;

                        self.container_width.set(cx, new_container_width);
                        self.container_height.set(cx, new_container_height);
                        self.inner_width.set(cx, new_inner_width);
                        self.inner_height.set(cx, new_inner_height);

                        let new_scroll_x = if new_inner_width != new_container_width {
                            ((left * scale_factor) / (new_inner_width - new_container_width))
                                .clamp(0.0, 1.0)
                        } else {
                            0.0
                        };

                        let new_scroll_y = if new_inner_height != new_container_height {
                            ((top * scale_factor) / (new_inner_height - new_container_height))
                                .clamp(0.0, 1.0)
                        } else {
                            0.0
                        };

                        self.scroll_x.set(cx, new_scroll_x);
                        self.scroll_y.set(cx, new_scroll_y);

                        if let Some(callback) = &self.on_scroll {
                            (callback)(cx, new_scroll_x, new_scroll_y);
                        }

                        self.reset(cx);
                    }

                    self.inner_width.set(cx, *w);
                    self.inner_height.set(cx, *h);
                    self.reset(cx);
                }

                ScrollEvent::ScrollToView(entity) => {
                    let view_bounds = cx.cache.get_bounds(*entity);

                    let content_bounds = cx.bounds();

                    let inner_width = *self.inner_width.get(cx);
                    let inner_height = *self.inner_height.get(cx);
                    let container_width = *self.container_width.get(cx);
                    let container_height = *self.container_height.get(cx);

                    let mut scroll_x = *self.scroll_x.get(cx);
                    let mut scroll_y = *self.scroll_y.get(cx);

                    let dx = content_bounds.right() - view_bounds.right();
                    let dy = content_bounds.bottom() - view_bounds.bottom();

                    // Calculate the scroll position to bring the child into view.
                    if dx < 0.0 {
                        let sx = (-dx / (inner_width - container_width)).clamp(0.0, 1.0);
                        scroll_x = (scroll_x + sx).clamp(0.0, 1.0);
                    }

                    if dy < 0.0 {
                        let sy = (-dy / (inner_height - container_height)).clamp(0.0, 1.0);
                        scroll_y = (scroll_y + sy).clamp(0.0, 1.0);
                    }

                    let dx = view_bounds.left() - content_bounds.left();
                    let dy = view_bounds.top() - content_bounds.top();

                    if dx < 0.0 {
                        let sx = (-dx / (inner_width - container_width)).clamp(0.0, 1.0);
                        scroll_x = (scroll_x - sx).clamp(0.0, 1.0);
                    }

                    if dy < 0.0 {
                        let sy = (-dy / (inner_height - container_height)).clamp(0.0, 1.0);
                        scroll_y = (scroll_y - sy).clamp(0.0, 1.0);
                    }

                    self.scroll_x.set(cx, scroll_x);
                    self.scroll_y.set(cx, scroll_y);

                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, scroll_x, scroll_y);
                    }
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

                    let inner_width = *self.inner_width.get(cx);
                    let inner_height = *self.inner_height.get(cx);
                    let container_width = *self.container_width.get(cx);
                    let container_height = *self.container_height.get(cx);

                    if inner_width != 0.0
                        && inner_height != 0.0
                        && container_width != 0.0
                        && container_height != 0.0
                    {
                        let scroll_x = *self.scroll_x.get(cx);
                        let scroll_y = *self.scroll_y.get(cx);

                        let top =
                            ((inner_height - container_height) * scroll_y).round() / scale_factor;
                        let left =
                            ((inner_width - container_width) * scroll_x).round() / scale_factor;

                        let new_container_width = bounds.width();
                        let new_container_height = bounds.height();

                        self.container_width.set(cx, new_container_width);
                        self.container_height.set(cx, new_container_height);

                        let new_scroll_x = if inner_width != new_container_width {
                            ((left * scale_factor) / (inner_width - new_container_width))
                                .clamp(0.0, 1.0)
                        } else {
                            0.0
                        };

                        let new_scroll_y = if inner_height != new_container_height {
                            ((top * scale_factor) / (inner_height - new_container_height))
                                .clamp(0.0, 1.0)
                        } else {
                            0.0
                        };

                        self.scroll_x.set(cx, new_scroll_x);
                        self.scroll_y.set(cx, new_scroll_y);

                        if let Some(callback) = &self.on_scroll {
                            (callback)(cx, new_scroll_x, new_scroll_y);
                        }

                        self.reset(cx);
                    }

                    self.container_width.set(cx, bounds.width());
                    self.container_height.set(cx, bounds.height());
                }
            }

            WindowEvent::MouseScroll(x, y) => {
                cx.set_active(true);
                let (x, y) = if cx.modifiers.shift() { (-*y, -*x) } else { (-*x, -*y) };

                let inner_width = *self.inner_width.get(cx);
                let inner_height = *self.inner_height.get(cx);
                let container_width = *self.container_width.get(cx);
                let container_height = *self.container_height.get(cx);

                // What percentage of the negative space does this cross?
                if x != 0.0 && inner_width > container_width {
                    let negative_space = inner_width - container_width;
                    if negative_space != 0.0 {
                        let logical_delta = x * SCROLL_SENSITIVITY / negative_space;
                        cx.emit(ScrollEvent::ScrollX(logical_delta));
                    }
                    // Prevent event propagating to ancestor scrollviews.
                    meta.consume();
                }
                if y != 0.0 && inner_height > container_height {
                    let negative_space = inner_height - container_height;
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

impl Handle<'_, ScrollView> {
    /// Sets a callback which will be called when a scrollview is scrolled, either with the mouse wheel, touchpad, or using the scroll bars.
    pub fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.modify(|scrollview| scrollview.on_scroll = Some(Arc::new(callback)))
    }

    /// Sets whether the scrollbar should move to the cursor when pressed.
    pub fn scroll_to_cursor(self, scroll_to_cursor: Signal<bool>) -> Self {
        self.bind(scroll_to_cursor, |handle, scroll_to_cursor| {
            let scroll_to_cursor = *scroll_to_cursor.get(&handle);
            handle.modify2(|scrollview, cx| scrollview.scroll_to_cursor.set(cx, scroll_to_cursor));
        })
    }

    /// Set the horizontal scroll position of the [ScrollView]. Accepts a signal to an 'f32' between 0 and 1.
    pub fn scroll_x(self, scrollx: Signal<f32>) -> Self {
        self.bind(scrollx, |handle, scrollx| {
            let sx = *scrollx.get(&handle);
            handle.modify2(|scrollview, cx| scrollview.scroll_x.set(cx, sx));
        })
    }

    /// Set the vertical scroll position of the [ScrollView]. Accepts a signal to an 'f32' between 0 and 1.
    pub fn scroll_y(self, scrolly: Signal<f32>) -> Self {
        self.bind(scrolly, |handle, scrolly| {
            let sy = *scrolly.get(&handle);
            handle.modify2(|scrollview, cx| scrollview.scroll_y.set(cx, sy));
        })
    }

    /// Sets whether the horizontal scrollbar should be visible.
    pub fn show_horizontal_scrollbar(self, flag: Signal<bool>) -> Self {
        self.bind(flag, |handle, show_scrollbar| {
            let show_scrollbar = *show_scrollbar.get(&handle);
            handle.modify2(|scrollview, cx| {
                scrollview.show_horizontal_scrollbar.set(cx, show_scrollbar);
            });
        })
    }

    /// Sets whether the vertical scrollbar should be visible.
    pub fn show_vertical_scrollbar(self, flag: Signal<bool>) -> Self {
        self.bind(flag, |handle, show_scrollbar| {
            let show_scrollbar = *show_scrollbar.get(&handle);
            handle.modify2(|scrollview, cx| {
                scrollview.show_vertical_scrollbar.set(cx, show_scrollbar);
            });
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
