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
    pub scroll_x: Signal<f32>,
    /// Progress of scroll position between 0 and 1 for the y axis
    pub scroll_y: Signal<f32>,
    /// Callback called when the scrollview is scrolled.
    pub on_scroll: Option<Arc<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,
    /// Width of the inner VStack which holds the content (typically bigger than container_width)
    pub inner_width: Signal<f32>,
    /// Height of the inner VStack which holds the content (typically bigger than container_height)
    pub inner_height: Signal<f32>,
    /// Width of the outer `ScrollView` which wraps the inner (typically smaller than inner_width)
    pub container_width: Signal<f32>,
    /// Height of the outer `ScrollView` which wraps the inner (typically smaller than inner_height)
    pub container_height: Signal<f32>,
    /// Whether the scrollbar should move to the cursor when pressed.
    pub scroll_to_cursor: Signal<bool>,
    /// Whether the horizontal scrollbar should be visible.
    pub show_horizontal_scrollbar: Signal<bool>,
    /// Whether the vertical scrollbar should be visible.
    pub show_vertical_scrollbar: Signal<bool>,
}

impl ScrollView {
    /// Creates a new [ScrollView].
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        let scroll_to_cursor = Signal::new(false);
        let scroll_x = Signal::new(0.0_f32);
        let scroll_y = Signal::new(0.0_f32);
        let inner_width = Signal::new(0.0_f32);
        let inner_height = Signal::new(0.0_f32);
        let container_width = Signal::new(0.0_f32);
        let container_height = Signal::new(0.0_f32);
        let show_horizontal_scrollbar = Signal::new(true);
        let show_vertical_scrollbar = Signal::new(true);

        let vertical_ratio: Memo<f32> = Memo::new(move |_| {
            let inner = inner_height.get();
            if inner == 0.0_f32 {
                0.0_f32
            } else {
                (container_height.get() / inner).clamp(0.0_f32, 1.0_f32)
            }
        });

        let horizontal_ratio: Memo<f32> = Memo::new(move |_| {
            let inner = inner_width.get();
            if inner == 0.0_f32 {
                0.0_f32
            } else {
                (container_width.get() / inner).clamp(0.0_f32, 1.0_f32)
            }
        });

        let has_h_scroll = Memo::new(move |_| container_width.get() < inner_width.get());
        let has_v_scroll = Memo::new(move |_| container_height.get() < inner_height.get());

        let scroll_state = Memo::new(move |_| {
            (
                scroll_x.get(),
                scroll_y.get(),
                inner_width.get(),
                inner_height.get(),
                container_width.get(),
                container_height.get(),
            )
        });
        let scroll_state_signal = scroll_state;

        Self {
            scroll_to_cursor,
            scroll_x,
            scroll_y,
            on_scroll: None,
            inner_width,
            inner_height,
            container_width,
            container_height,
            show_horizontal_scrollbar,
            show_vertical_scrollbar,
        }
        .build(cx, move |cx| {
            ScrollContent::new(cx, content);

            Binding::new(cx, show_vertical_scrollbar, move |cx| {
                if show_vertical_scrollbar.get() {
                    Scrollbar::new(
                        cx,
                        scroll_y,
                        vertical_ratio,
                        Orientation::Vertical,
                        |cx, value| {
                            cx.emit(ScrollEvent::SetY(value));
                        },
                    )
                    .position_type(PositionType::Absolute)
                    .scroll_to_cursor(scroll_to_cursor);
                }
            });

            Binding::new(cx, show_horizontal_scrollbar, move |cx| {
                if show_horizontal_scrollbar.get() {
                    Scrollbar::new(
                        cx,
                        scroll_x,
                        horizontal_ratio,
                        Orientation::Horizontal,
                        |cx, value| {
                            cx.emit(ScrollEvent::SetX(value));
                        },
                    )
                    .position_type(PositionType::Absolute)
                    .scroll_to_cursor(scroll_to_cursor);
                }
            });
        })
        .bind(scroll_state, move |mut handle| {
            let (scroll_x, scroll_y, inner_width, inner_height, container_width, container_height) =
                scroll_state_signal.get();
            let scale_factor = handle.context().scale_factor();
            let top = ((inner_height - container_height) * scroll_y).round() / scale_factor;
            let left = ((inner_width - container_width) * scroll_x).round() / scale_factor;
            handle.horizontal_scroll(-left.abs()).vertical_scroll(-top.abs());
        })
        .toggle_class("h-scroll", has_h_scroll)
        .toggle_class("v-scroll", has_v_scroll)
    }

    fn reset(&mut self) {
        if self.inner_width.get() == self.container_width.get() {
            self.scroll_x.set(0.0);
        }

        if self.inner_height.get() == self.container_height.get() {
            self.scroll_y.set(0.0);
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
                    self.scroll_x.set((self.scroll_x.get() + *f).clamp(0.0, 1.0));

                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                    }
                }

                ScrollEvent::ScrollY(f) => {
                    self.scroll_y.set((self.scroll_y.get() + *f).clamp(0.0, 1.0));
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                    }
                }

                ScrollEvent::SetX(f) => {
                    self.scroll_x.set(*f);
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                    }
                }

                ScrollEvent::SetY(f) => {
                    self.scroll_y.set(*f);
                    if let Some(callback) = &self.on_scroll {
                        (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                    }
                }

                ScrollEvent::ChildGeo(w, h) => {
                    let bounds = cx.bounds();
                    let scale_factor = cx.scale_factor();

                    let mut scroll_x = self.scroll_x.get();
                    let mut scroll_y = self.scroll_y.get();
                    let mut inner_width = self.inner_width.get();
                    let mut inner_height = self.inner_height.get();
                    let mut container_width = self.container_width.get();
                    let mut container_height = self.container_height.get();

                    if inner_width != 0.0
                        && inner_height != 0.0
                        && container_width != 0.0
                        && container_height != 0.0
                    {
                        let top =
                            ((inner_height - container_height) * scroll_y).round() / scale_factor;
                        let left =
                            ((inner_width - container_width) * scroll_x).round() / scale_factor;

                        container_width = bounds.width();
                        container_height = bounds.height();
                        inner_width = *w;
                        inner_height = *h;

                        if inner_width != container_width {
                            scroll_x = ((left * scale_factor) / (inner_width - container_width))
                                .clamp(0.0, 1.0);
                        } else {
                            scroll_x = 0.0;
                        }

                        if inner_height != container_height {
                            scroll_y = ((top * scale_factor) / (inner_height - container_height))
                                .clamp(0.0, 1.0);
                        } else {
                            scroll_y = 0.0;
                        }

                        self.scroll_x.set(scroll_x);
                        self.scroll_y.set(scroll_y);
                        self.inner_width.set(inner_width);
                        self.inner_height.set(inner_height);
                        self.container_width.set(container_width);
                        self.container_height.set(container_height);

                        if let Some(callback) = &self.on_scroll {
                            (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                        }

                        self.reset();
                    }

                    self.inner_width.set(*w);
                    self.inner_height.set(*h);
                    self.reset();
                }

                ScrollEvent::ScrollToView(entity) => {
                    let view_bounds = cx.cache.get_bounds(*entity);

                    let content_bounds = cx.bounds();

                    let dx = content_bounds.right() - view_bounds.right();
                    let dy = content_bounds.bottom() - view_bounds.bottom();

                    // Calculate the scroll position to bring the child into view.
                    if dx < 0.0 {
                        let sx = (-dx / (self.inner_width.get() - self.container_width.get()))
                            .clamp(0.0, 1.0);
                        self.scroll_x.set((self.scroll_x.get() + sx).clamp(0.0, 1.0));
                    }

                    if dy < 0.0 {
                        let sy = (-dy / (self.inner_height.get() - self.container_height.get()))
                            .clamp(0.0, 1.0);
                        self.scroll_y.set((self.scroll_y.get() + sy).clamp(0.0, 1.0));
                    }

                    let dx = view_bounds.left() - content_bounds.left();
                    let dy = view_bounds.top() - content_bounds.top();

                    if dx < 0.0 {
                        let sx = (-dx / (self.inner_width.get() - self.container_width.get()))
                            .clamp(0.0, 1.0);
                        self.scroll_x.set((self.scroll_x.get() - sx).clamp(0.0, 1.0));
                    }

                    if dy < 0.0 {
                        let sy = (-dy / (self.inner_height.get() - self.container_height.get()))
                            .clamp(0.0, 1.0);
                        self.scroll_y.set((self.scroll_y.get() - sy).clamp(0.0, 1.0));
                    }

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

                    let mut scroll_x = self.scroll_x.get();
                    let mut scroll_y = self.scroll_y.get();
                    let inner_width = self.inner_width.get();
                    let inner_height = self.inner_height.get();
                    let mut container_width = self.container_width.get();
                    let mut container_height = self.container_height.get();

                    if inner_width != 0.0
                        && inner_height != 0.0
                        && container_width != 0.0
                        && container_height != 0.0
                    {
                        let top =
                            ((inner_height - container_height) * scroll_y).round() / scale_factor;
                        let left =
                            ((inner_width - container_width) * scroll_x).round() / scale_factor;

                        container_width = bounds.width();
                        container_height = bounds.height();

                        if inner_width != container_width {
                            scroll_x = ((left * scale_factor) / (inner_width - container_width))
                                .clamp(0.0, 1.0);
                        } else {
                            scroll_x = 0.0;
                        }

                        if inner_height != container_height {
                            scroll_y = ((top * scale_factor) / (inner_height - container_height))
                                .clamp(0.0, 1.0);
                        } else {
                            scroll_y = 0.0;
                        }

                        self.scroll_x.set(scroll_x);
                        self.scroll_y.set(scroll_y);
                        self.container_width.set(container_width);
                        self.container_height.set(container_height);

                        if let Some(callback) = &self.on_scroll {
                            (callback)(cx, self.scroll_x.get(), self.scroll_y.get());
                        }

                        self.reset();
                    }

                    self.container_width.set(bounds.width());
                    self.container_height.set(bounds.height());
                }
            }

            WindowEvent::MouseScroll(x, y) => {
                cx.set_active(true);
                let (x, y) = if cx.modifiers.shift() { (-*y, -*x) } else { (-*x, -*y) };

                // What percentage of the negative space does this cross?
                if x != 0.0 && self.inner_width.get() > self.container_width.get() {
                    let negative_space = self.inner_width.get() - self.container_width.get();
                    if negative_space != 0.0 {
                        let logical_delta = x * SCROLL_SENSITIVITY / negative_space;
                        cx.emit(ScrollEvent::ScrollX(logical_delta));
                    }
                    // Prevent event propagating to ancestor scrollviews.
                    meta.consume();
                }
                if y != 0.0 && self.inner_height.get() > self.container_height.get() {
                    let negative_space = self.inner_height.get() - self.container_height.get();
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
            handle.modify(|scrollview| scrollview.scroll_to_cursor.set(scroll_to_cursor.get()));
        })
    }

    /// Set the horizontal scroll position of the [ScrollView]. Accepts a value or signal of type `f32` between 0 and 1.
    pub fn scroll_x(self, scrollx: impl Res<f32> + 'static) -> Self {
        let scrollx = scrollx.to_signal(self.cx);
        self.bind(scrollx, move |handle| {
            handle.modify(|scrollview| scrollview.scroll_x.set(scrollx.get()));
        })
    }

    /// Set the vertical scroll position of the [ScrollView]. Accepts a value or signal of type `f32` between 0 and 1.
    pub fn scroll_y(self, scrolly: impl Res<f32> + 'static) -> Self {
        let scrolly = scrolly.to_signal(self.cx);
        self.bind(scrolly, move |handle| {
            handle.modify(|scrollview| scrollview.scroll_y.set(scrolly.get()));
        })
    }

    /// Sets whether the horizontal scrollbar should be visible.
    pub fn show_horizontal_scrollbar(self, flag: impl Res<bool> + 'static) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            handle.modify(|scrollview| scrollview.show_horizontal_scrollbar.set(flag.get()));
        })
    }

    /// Sets whether the vertical scrollbar should be visible.
    pub fn show_vertical_scrollbar(self, flag: impl Res<bool> + 'static) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            handle.modify(|scrollview| scrollview.show_vertical_scrollbar.set(flag.get()));
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
