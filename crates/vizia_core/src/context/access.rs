use accesskit::{NodeBuilder, NodeId, TextSelection};

use crate::{
    accessibility::IntoNode, cache::CachedData, prelude::*, style::Style, text::TextContext,
};

// A context used for configuring the accessibility features of a view.
pub struct AccessContext<'a> {
    pub(crate) current: Entity,
    pub(crate) style: &'a mut Style,
    pub(crate) cache: &'a CachedData,
    pub(crate) text_context: &'a mut TextContext,
}

impl<'a> AccessContext<'a> {
    // pub fn new(cx: &'a mut Context) -> Self {
    //     Self { current: cx.current, style: &mut cx.style, cache: &cx.cache }
    // }

    pub fn node_id(&self) -> NodeId {
        self.current.accesskit_id()
    }
}
