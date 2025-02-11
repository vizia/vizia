use crate::{icons::ICON_CHEVRON_DOWN, prelude::*};

/// Events that can be triggered by the collapsible view.
pub enum CollapsibleEvent {
    ToggleOpen,
}

/// A collapsible view that can be opened or closed to hide content.
///
/// # Example
/// ```no_run
/// Collapsible::new(
///     cx,
///     |cx| {
///         Label::new(cx, "Click me to collapse the content").hoverable(false);
///     },
///     |cx| {
///         Label::new(cx, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5").hoverable(false);
///     },
/// )
/// .width(Pixels(300.0));
/// ```
#[derive(Lens)]
pub struct Collapsible {
    is_open: bool,
}

impl Collapsible {
    /// Create a new collapsible view with a header and content.
    pub fn new(
        cx: &mut Context,
        header: impl Fn(&mut Context),
        content: impl Fn(&mut Context),
    ) -> Handle<Self> {
        Self { is_open: false }
            .build(cx, |cx| {
                // Header
                HStack::new(cx, |cx| {
                    header(cx);
                    Svg::new(cx, ICON_CHEVRON_DOWN)
                        .class("expand-icon")
                        .on_press(|cx| cx.emit(CollapsibleEvent::ToggleOpen));
                })
                .class("header")
                .on_press(|cx| cx.emit(CollapsibleEvent::ToggleOpen));

                // Content
                VStack::new(cx, |cx| {
                    content(cx);
                })
                .class("content");
            })
            .toggle_class("open", Collapsible::is_open)
    }
}

impl View for Collapsible {
    fn element(&self) -> Option<&'static str> {
        Some("collapsible")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|collapsible_event, _| match collapsible_event {
            CollapsibleEvent::ToggleOpen => {
                self.is_open = !self.is_open;
            }
        });
    }
}

impl Handle<'_, Collapsible> {
    /// Set the open state of the collapsible view.
    pub fn open(self, open: impl Res<bool>) -> Self {
        self.bind(open, |handle, open| {
            let open = open.get(&handle);
            handle.modify(|collapsible| collapsible.is_open = open);
        })
    }
}
