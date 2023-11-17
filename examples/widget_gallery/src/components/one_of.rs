use vizia::prelude::*;

/// A view which switches between one of many other views.
#[derive(Lens)]
pub struct OneOf {
    selected_index: usize,
}

impl OneOf {
    pub fn new<L, T, F>(cx: &mut Context, lens: L, content: F) -> Handle<Self>
    where
        L: Lens,
        <L as Lens>::Target: std::ops::Deref<Target = [T]>,
        T: Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, Index<L, T>) -> PageView,
    {
        Self { selected_index: 0 }.build(cx, move |cx| {
            let content2 = content.clone();
            Binding::new(cx, OneOf::selected_index, move |cx, selected| {
                let selected = selected.get(cx);
                let l = lens.index(selected);
                ((content)(cx, l).content)(cx);
            });
        })
    }
}

impl View for OneOf {
    fn element(&self) -> Option<&'static str> {
        Some("oneof")
    }
}

pub trait OneOfModifiers {
    fn with_selected<U: Into<usize>>(self, selected: impl Res<U>) -> Self;
}

impl<'a> OneOfModifiers for Handle<'a, OneOf> {
    fn with_selected<U: Into<usize>>(self, selected: impl Res<U>) -> Self {
        self.bind(selected, |handle, selected| {
            let index = selected.get(&handle).into();
            handle.modify(|oneof| oneof.selected_index = index);
        })
    }
}

pub struct PageView {
    content: Box<dyn Fn(&mut Context)>,
}

impl PageView {
    pub fn new<C>(content: C) -> Self
    where
        C: 'static + Fn(&mut Context),
    {
        Self { content: Box::new(content) }
    }
}
