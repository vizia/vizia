use crate::{Context, Handle, TreeExt, View};

type Template<T> = Option<Box<dyn Fn(&mut Context, T)>>;

pub struct ForEach {}

impl ForEach {
    pub fn new<F>(cx: &mut Context, range: std::ops::Range<usize>, template: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, usize),
    {
        Self {}.build2(cx, move |cx| {
            if cx.current.child_iter(&cx.tree.clone()).count() != range.len() {
                for child in cx.current.child_iter(&cx.tree.clone()) {
                    cx.remove(child);
                }

                cx.style.borrow_mut().needs_relayout = true;
                cx.style.borrow_mut().needs_redraw = true;
            }

            let prev_count = cx.count;
            cx.count = 0;
            for i in range {
                (template)(cx, i);
            }
            cx.count = prev_count;
        })
    }
}

impl View for ForEach {}
