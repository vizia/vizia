use std::any::TypeId;

use crate::{Context, Handle, Lens, Store, View};

pub struct List<L, T: 'static>
where
    L: Lens<Target = Vec<T>>,
{
    lens: L,
    builder: Option<Box<dyn Fn(&mut Context, ItemPtr<L, T>)>>,
}

pub struct ItemPtr<L, T>
where
    L: Lens<Target = Vec<T>>,
{
    lens: L,
    index: usize,
}

impl<L, T> ItemPtr<L, T>
where
    L: Lens<Target = Vec<T>>,
{
    pub fn new(lens: L, index: usize) -> Self {
        Self { lens, index }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn value<'a>(&self, cx: &'a Context) -> &'a T
    where
        <L as Lens>::Source: 'static,
    {
        self.lens.view(cx.data().unwrap()).get(self.index).unwrap()
    }
}

impl<L: 'static + Lens<Target = Vec<T>>, T> List<L, T> {
    pub fn new<F>(cx: &mut Context, lens: L, item: F) -> Handle<'_, Self>
    where
        F: 'static + Fn(&mut Context, ItemPtr<L, T>),
    {
        Self {
            lens,
            builder: Some(Box::new(item)),
        }
        .build(cx)
    }
}

impl<L: 'static + Lens<Target = Vec<T>>, T> View for List<L, T> {
    fn body(&mut self, cx: &mut Context) {
        let builder = self.builder.take().unwrap();
        let store = cx
            .data
            .get(&TypeId::of::<L::Source>())
            .and_then(|model| model.downcast_ref::<Store<L::Source>>());
        if let Some(store) = store {
            let len = self.lens.view(&store.data).len();
            for index in 0..len {
                let ptr = ItemPtr::new(self.lens.clone(), index);
                (builder)(cx, ptr);
            }
        }
        self.builder = Some(builder);
    }
}
