use crate::prelude::*;

/// A sidebar container with optional header, scrollable content area, and optional footer.
///
/// The sidebar is composed as:
/// - `sidebar-header`
/// - `sidebar-content`
/// - `sidebar-footer`
pub struct Sidebar {
    width: Signal<Units>,
}

impl Sidebar {
    /// Creates a new [Sidebar] with custom header, content and footer builders.
    pub fn new<H, C, F>(cx: &mut Context, header: H, content: C, footer: F) -> Handle<Self>
    where
        H: 'static + Fn(&mut Context),
        C: 'static + Fn(&mut Context),
        F: 'static + Fn(&mut Context),
    {
        let width: Signal<Units> = Signal::new(Pixels(200.0));
        Self { width }.build(cx, move |cx| {
            Resizable::new(
                cx,
                width,
                ResizeStackDirection::Right,
                move |_cx, new_size| width.set(Pixels(new_size)),
                move |cx| {
                    VStack::new(cx, |cx| {
                        (header)(cx);
                    })
                    .class("sidebar-header")
                    .height(Auto);

                    Divider::new(cx).class("sidebar-divider");
                    ScrollView::new(cx, move |cx| {
                        (content)(cx);
                    })
                    .class("sidebar-content")
                    .height(Stretch(1.0));

                    Divider::new(cx).class("sidebar-divider");

                    VStack::new(cx, |cx| {
                        (footer)(cx);
                    })
                    .class("sidebar-footer")
                    .height(Auto);
                },
            );
        })
    }
}

impl View for Sidebar {
    fn element(&self) -> Option<&'static str> {
        Some("sidebar")
    }
}
