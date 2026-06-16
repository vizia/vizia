use crate::prelude::*;

/// The position of the title on the frame border.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FrameTitlePosition {
    /// Title at the top-left corner
    #[default]
    TopLeft,
    /// Title at the top-center
    TopCenter,
    /// Title at the top-right corner
    TopRight,
}

/// A container widget that groups related content with a border and optional title.
///
/// The frame displays a border around its content and supports an optional title
/// that is positioned to intersect the frame's border, similar to an HTML fieldset.
///
/// # Examples
///
/// ```no_run
/// # use vizia_core::prelude::*;
/// # let cx = &mut Context::default();
/// Frame::new(cx, |cx| {
///     Label::new(cx, "Frame content").hoverable(false);
/// }).title_position(FrameTitlePosition::TopCenter);
/// ```
pub struct Frame {
    title_position: Signal<FrameTitlePosition>,
}

impl Frame {
    /// Creates a new frame with content but no title.
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        let title_position = Signal::new(FrameTitlePosition::default());

        Self { title_position }.build(cx, |cx| {
            // Content (top layer)
            content(cx);
        })
    }

    /// Creates a new frame with a title and content.
    ///
    /// The title is positioned to intersect the frame's border.
    pub fn with_title<S: View>(
        cx: &mut Context,
        title: impl FnOnce(&mut Context) -> Handle<S>,
        content: impl FnOnce(&mut Context),
    ) -> Handle<Self> {
        let title_position = Signal::new(FrameTitlePosition::default());

        Self { title_position }.build(cx, |cx| {
            // Title - positioned absolutely with negative top offset

            title(cx)
                .class("frame-title")
                .position_type(PositionType::Absolute)
                .top(Pixels(0.0))
                .translate((Pixels(0.0), Percentage(-50.0)))
                .size(Auto)
                .bind(title_position, move |handle| {
                    let pos = title_position.get();
                    match pos {
                        FrameTitlePosition::TopLeft => {
                            handle
                                .toggle_class("left", true)
                                .toggle_class("center", false)
                                .toggle_class("right", false);
                        }
                        FrameTitlePosition::TopCenter => {
                            handle
                                .toggle_class("left", false)
                                .toggle_class("center", true)
                                .toggle_class("right", false);
                        }
                        FrameTitlePosition::TopRight => {
                            handle
                                .toggle_class("left", false)
                                .toggle_class("center", false)
                                .toggle_class("right", true);
                        }
                    }
                });

            // Content
            content(cx);
        })
    }
}

impl View for Frame {
    fn element(&self) -> Option<&'static str> {
        Some("frame")
    }
}

impl Handle<'_, Frame> {
    /// Set the position of the title on the frame border.
    pub fn title_position(self, position: FrameTitlePosition) -> Self {
        self.modify(|frame| frame.title_position.set(position))
    }
}
