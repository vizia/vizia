use crate::{modifiers::TooltipModel, prelude::*};

pub struct Tooltip {}

impl Tooltip {
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Self {}
            .build(cx, |cx| (content)(cx))
            .position_type(PositionType::SelfDirected)
            .z_index(100)
            .size(Auto)
            .top(Percentage(100.0))
            .translate((Pixels(0.0), Pixels(10.0)))
            .hoverable(false)
            .on_build(|ex| {
                ex.add_listener(move |_: &mut Tooltip, ex, event| {
                    let flag = TooltipModel::tooltip_visible.get(ex);
                    event.map(|window_event, meta| match window_event {
                        WindowEvent::MouseDown(_) => {
                            if flag && meta.origin != ex.current() {
                                ex.toggle_class("vis", false);
                            }
                        }

                        _ => {}
                    });
                });
            })
    }
}

impl View for Tooltip {
    fn element(&self) -> Option<&'static str> {
        Some("tooltip")
    }
}
