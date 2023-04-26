use crate::prelude::*;

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
    }
}

impl View for Tooltip {
    fn element(&self) -> Option<&'static str> {
        Some("tooltip")
    }
}
