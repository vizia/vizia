use crate::{Context, Handle, TreeExt, View};
pub struct ForEach {}

impl ForEach {
    pub fn new<F>(cx: &mut Context, range: std::ops::Range<usize>, mut template: F) -> Handle<Self>
    where
        F: 'static + FnMut(&mut Context, usize),
    {
        Self {}.build2(cx, move |cx| {
            if cx.current.child_iter(&cx.tree.clone()).count() != range.len() {
                for child in cx.current.child_iter(&cx.tree.clone()) {
                    cx.remove(child);
                }

                cx.style.needs_relayout = true;
                cx.style.needs_redraw = true;
            }

            for i in range {
                (template)(cx, i);
            }
        })
    }
}

impl View for ForEach {}
