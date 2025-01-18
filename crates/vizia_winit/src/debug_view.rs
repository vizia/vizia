use vizia_core::prelude::*;

use crate::{window::Window, window_modifiers::WindowModifiers};

#[derive(Lens)]
pub struct DebugView {
    show_inspector: bool,
    hover_bounds: BoundingBox,
}

pub enum DebugViewEvent {
    InspectorClosed,
}

impl DebugView {
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Self { show_inspector: true, hover_bounds: BoundingBox::default() }
            .build(cx, |cx| {
                (content)(cx);

                Element::new(cx)
                    .hoverable(false)
                    .background_color(Color::blue())
                    .position_type(PositionType::Absolute)
                    .opacity(0.4)
                    .left(DebugView::hover_bounds.map(|bb| Pixels(bb.x)))
                    .top(DebugView::hover_bounds.map(|bb| Pixels(bb.y)))
                    .width(DebugView::hover_bounds.map(|bb| Pixels(bb.w)))
                    .height(DebugView::hover_bounds.map(|bb| Pixels(bb.h)));

                Binding::new(cx, Self::show_inspector, |cx, show_subwindow| {
                    if show_subwindow.get(cx) {
                        Window::popup(cx, false, |cx| {
                            Inspector::new(cx);
                        })
                        .on_close(|cx| {
                            cx.emit(DebugViewEvent::InspectorClosed);
                        })
                        .title("Inspector")
                        .inner_size((400, 800))
                        .position((500, 100));
                    }
                });
            })
            .id("debug-view")
            .focused(true)
    }
}

impl View for DebugView {
    fn element(&self) -> Option<&'static str> {
        Some("debug-view")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|debug_event, _| match debug_event {
            DebugViewEvent::InspectorClosed => self.show_inspector = false,
        });

        event.map(|inspector_event, _| match inspector_event {
            InspectorEvent::SetHoverRect(rect) => self.hover_bounds = *rect,
            _ => {}
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::KeyDown(code, _) => {
                if *code == Code::F12 {
                    self.show_inspector = true;
                }
            }

            _ => {}
        });
    }
}
