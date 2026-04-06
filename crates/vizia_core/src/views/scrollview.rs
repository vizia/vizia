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
#[derive(Clone)]
pub struct ScrollView {
    /// Progress of scroll position between 0 and 1 for the x axis
    pub scroll_x: Signal<f32>,
    /// Progress of scroll position between 0 and 1 for the y axis
    pub scroll_y: Signal<f32>,
    /// Callback called when the scrollview is scrolled.
    pub on_scroll: Option<Arc<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,
    /// Width of the inner VStack which holds the content (typically bigger than container_width)
    pub inner_width: f32,
    /// Height of the inner VStack which holds the content (typically bigger than container_height)
    pub inner_height: f32,
    /// Width of the outer `ScrollView` which wraps the inner (typically smaller than inner_width)
    pub container_width: f32,
    /// Height of the outer `ScrollView` which wraps the inner (typically smaller than inner_height)
    pub container_height: f32,
    /// Whether the scrollbar should move to the cursor when pressed.
    pub scroll_to_cursor: Signal<bool>,
    /// Whether the horizontal scrollbar should be visible.
    pub show_horizontal_scrollbar: Signal<bool>,
    /// Whether the vertical scrollbar should be visible.
    pub show_vertical_scrollbar: Signal<bool>,
    h_scroll: Signal<bool>,
    v_scroll: Signal<bool>,
    scroll_offset: Signal<(f32, f32)>,
    ratio_y: Signal<f32>,
    ratio_x: Signal<f32>,
}

impl ScrollView {
    /// Creates a new [ScrollView].
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        let scroll_x = Signal::new(0.0_f32);
        let scroll_y = Signal::new(0.0_f32);
        let scroll_to_cursor = Signal::new(false);
        let show_horizontal_scrollbar = Signal::new(true);
        let show_vertical_scrollbar = Signal::new(true);
        let h_scroll = Signal::new(false);
        let v_scroll = Signal::new(false);
        let scroll_offset = Signal::new((0.0_f32, 0.0_f32));
        let ratio_y = Signal::new(1.0_f32);
        let ratio_x = Signal::new(1.0_f32);
        Self {
            scroll_to_cursor,
            scroll_x,
            scroll_y,
            on_scroll: None,
            inner_width: 0.0,
            inner_height: 0.0,
            container_width: 0.0,
            container_height: 0.0,
            show_horizontal_scrollbar,
            show_vertical_scrollbar,
            h_scroll,
            v_scroll,
            scroll_offset,
            ratio_y,
            ratio_x,
        }
        .build(cx, move |cx| {
            ScrollContent::new(cx, content);

            Scrollbar::new(cx, scroll_y, ratio_y, Orientation::Vertical, |cx, value| {
                cx.emit(ScrollEvent::SetY(value));
            })
            .position_type(PositionType::Absolute)
            .scroll_to_cursor(scroll_to_cursor);

            Scrollbar::new(cx, scroll_x, ratio_x, Orientation::Horizontal, |cx, value| {
                cx.emit(ScrollEvent::SetX(value));
            })
            .position_type(PositionType::Absolute)
            .scroll_to_cursor(scroll_to_cursor);
        })
        .bind(scroll_offset, move |handle| {
            let (left, top) = scroll_offset.get();
            handle.horizontal_scroll(-left.abs()).vertical_scroll(-top.abs());
        })
        .toggle_class("h-scroll", h_scroll)
        .toggle_class("v-scroll", v_scroll)
    }

    fn reset(&mut self) {
        if self.inner_width == self.container_width {
            self.scroll_x.set(0.0);
        }

        if self.inner_height == self.container_height {
            self.scroll_y.set(0.0);
        }
    }

    fn sync_signals(&mut self, scale_factor: f32) {
        let scroll_x = self.scroll_x.get();
        let scroll_y = self.scroll_y.get();
        self.h_scroll.set_if_changed(
            self.show_horizontal_scrollbar.get() && self.container_width < self.inner_width,
        );
        self.v_scroll.set_if_changed(
            self.show_vertical_scrollbar.get() && self.container_height < self.inner_height,
        );
        let top = ((self.inner_height - self.container_height) * scroll_y).round() / scale_factor;
        let left = ((self.inner_width - self.container_width) * scroll_x).round() / scale_factor;
        self.scroll_offset.set((left, top));
        self.ratio_y.set(if self.inner_height == 0.0 {
            1.0
        } else {
            (self.container_height / self.inner_height).min(1.0)
        });
        self.ratio_x.set(if self.inner_width == 0.0 {
            1.0
        } else {
            (self.container_width / self.inner_width).min(1.0)
        });
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
                    self.scroll_x.set((self.scroll_x.get() + *f).clamp(0.0, 1.0));
                    self.sync_signals(cx.scale_factor());
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                    }
                }

                ScrollEvent::ScrollY(f) => {
                    self.scroll_y.set((self.scroll_y.get() + *f).clamp(0.0, 1.0));
                    self.sync_signals(cx.scale_factor());
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                    }
                }

                ScrollEvent::SetX(f) => {
                    self.scroll_x.set(*f);
                    self.sync_signals(cx.scale_factor());
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                    }
                }

                ScrollEvent::SetY(f) => {
                    self.scroll_y.set(*f);
                    self.sync_signals(cx.scale_factor());
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                    }
                }

                ScrollEvent::ChildGeo(w, h) => {
                    let bounds = cx.bounds();
                    let scale_factor = cx.scale_factor();

                    if self.inner_width != 0.0
                        && self.inner_height != 0.0
                        && self.container_width != 0.0
                        && self.container_height != 0.0
                    {
                        let top = ((self.inner_height - self.container_height)
                            * self.scroll_y.get())
                        .round()
                            / scale_factor;
                        let left = ((self.inner_width - self.container_width)
                            * self.scroll_x.get())
                        .round()
                            / scale_factor;

                        self.container_width = bounds.width();
                        self.container_height = bounds.height();
                        self.inner_width = *w;
                        self.inner_height = *h;

                        if self.inner_width != self.container_width {
                            self.scroll_x.set(
                                ((left * scale_factor) / (self.inner_width - self.container_width))
                                    .clamp(0.0, 1.0),
                            );
                        } else {
                            self.scroll_x.set(0.0);
                        }

                        if self.inner_height != self.container_height {
                            self.scroll_y.set(
                                ((top * scale_factor)
                                    / (self.inner_height - self.container_height))
                                    .clamp(0.0, 1.0),
                            );
                        } else {
                            self.scroll_y.set(0.0);
                        }

                        if let Some(callback) = &self.on_scroll {
                            (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                        }

                        self.reset();
                    }

                    self.inner_width = *w;
                    self.inner_height = *h;
                    self.reset();
                    self.sync_signals(cx.scale_factor());
                }

                ScrollEvent::ScrollToView(entity) => {
                    let view_bounds = cx.cache.get_bounds(*entity);

                    let content_bounds = cx.bounds();

                    let dx = content_bounds.right() - view_bounds.right();
                    let dy = content_bounds.bottom() - view_bounds.bottom();

                    // Calculate the scroll position to bring the child into view.
                    if dx < 0.0 {
                        let sx = (-dx / (self.inner_width - self.container_width)).clamp(0.0, 1.0);
                        self.scroll_x.set((self.scroll_x.get() + sx).clamp(0.0, 1.0));
                    }

                    if dy < 0.0 {
                        let sy =
                            (-dy / (self.inner_height - self.container_height)).clamp(0.0, 1.0);
                        self.scroll_y.set((self.scroll_y.get() + sy).clamp(0.0, 1.0));
                    }

                    let dx = view_bounds.left() - content_bounds.left();
                    let dy = view_bounds.top() - content_bounds.top();

                    if dx < 0.0 {
                        let sx = (-dx / (self.inner_width - self.container_width)).clamp(0.0, 1.0);
                        self.scroll_x.set((self.scroll_x.get() - sx).clamp(0.0, 1.0));
                    }

                    if dy < 0.0 {
                        let sy =
                            (-dy / (self.inner_height - self.container_height)).clamp(0.0, 1.0);
                        self.scroll_y.set((self.scroll_y.get() - sy).clamp(0.0, 1.0));
                    }

                    self.sync_signals(cx.scale_factor());
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
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

                    if self.inner_width != 0.0
                        && self.inner_height != 0.0
                        && self.container_width != 0.0
                        && self.container_height != 0.0
                    {
                        let top = ((self.inner_height - self.container_height)
                            * self.scroll_y.get())
                        .round()
                            / scale_factor;
                        let left = ((self.inner_width - self.container_width)
                            * self.scroll_x.get())
                        .round()
                            / scale_factor;

                        self.container_width = bounds.width();
                        self.container_height = bounds.height();

                        if self.inner_width != self.container_width {
                            self.scroll_x.set(
                                ((left * scale_factor) / (self.inner_width - self.container_width))
                                    .clamp(0.0, 1.0),
                            );
                        } else {
                            self.scroll_x.set(0.0);
                        }

                        if self.inner_height != self.container_height {
                            self.scroll_y.set(
                                ((top * scale_factor)
                                    / (self.inner_height - self.container_height))
                                    .clamp(0.0, 1.0),
                            );
                        } else {
                            self.scroll_y.set(0.0);
                        }

                        if let Some(callback) = &self.on_scroll {
                            (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                        }

                        self.reset();
                    }

                    self.container_width = bounds.width();
                    self.container_height = bounds.height();
                    self.sync_signals(scale_factor);
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

impl Handle<'_, ScrollView> {
    /// Sets a callback which will be called when a scrollview is scrolled, either with the mouse wheel, touchpad, or using the scroll bars.
    pub fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.modify(|scrollview| scrollview.on_scroll = Some(Arc::new(callback)))
    }

    /// Sets whether the scrollbar should move to the cursor when pressed.
    pub fn scroll_to_cursor(self, scroll_to_cursor: impl Res<bool> + 'static) -> Self {
        let scroll_to_cursor = scroll_to_cursor.to_signal(self.cx);
        self.bind(scroll_to_cursor, move |handle| {
            let scroll_to_cursor = scroll_to_cursor.get();
            handle.modify(|scrollview| scrollview.scroll_to_cursor.set(scroll_to_cursor));
        })
    }

    /// Set the horizontal scroll position of the [ScrollView]. Accepts a value or signal of type an `f32` between 0 and 1.
    pub fn scroll_x(self, scrollx: impl Res<f32> + 'static) -> Self {
        let scrollx = scrollx.to_signal(self.cx);
        self.bind(scrollx, move |handle| {
            let scrollx = scrollx.get();
            let sx = scrollx;
            handle.modify(|scrollview| scrollview.scroll_x.set(sx));
        })
    }

    /// Set the vertical scroll position of the [ScrollView]. Accepts a value or signal of type an `f32` between 0 and 1.
    pub fn scroll_y(self, scrollx: impl Res<f32> + 'static) -> Self {
        let scrollx = scrollx.to_signal(self.cx);
        self.bind(scrollx, move |handle| {
            let scrolly = scrollx.get();
            let sy = scrolly;
            handle.modify(|scrollview| scrollview.scroll_y.set(sy));
        })
    }

    /// Sets whether the horizontal scrollbar should be visible.
    pub fn show_horizontal_scrollbar(self, flag: impl Res<bool> + 'static) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let show_scrollbar = flag.get();
            let s = show_scrollbar;
            handle.modify(|scrollview| scrollview.show_horizontal_scrollbar.set(s));
        })
    }

    /// Sets whether the vertical scrollbar should be visible.
    pub fn show_vertical_scrollbar(self, flag: impl Res<bool> + 'static) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let show_scrollbar = flag.get();
            let s = show_scrollbar;
            handle.modify(|scrollview| scrollview.show_vertical_scrollbar.set(s));
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
