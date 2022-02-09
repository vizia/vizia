use std::marker::PhantomData;
use std::rc::Rc;

use crate::{Binding, Data, Index, LensExt, List, Then, VStack};
use crate::{Context, HStack, Handle, Lens, Model, View};

// TODO

pub struct Table<L, T: 'static>
where
    L: Lens<Target = Vec<T>>,
    T: Data,
{
    p: PhantomData<L>,
}

impl<L: 'static + Lens<Target = Vec<T>>, T: Data> Table<L, T> {
    pub fn new<'a, F>(cx: &'a mut Context, lens: L, list_builder: F) -> Handle<'a, Self>
    where
        F: 'static + Fn(&mut Context, L),
        <L as Lens>::Source: Model,
    {
        Self { p: PhantomData::default() }.build2(cx, move |cx| {
            HStack::new(cx, move |cx| {
                (list_builder)(cx, lens);
            });
        })
    }
}

impl<L: 'static + Lens<Target = Vec<T>>, T: Data> View for Table<L, T>
where
    L: 'static + Lens<Target = Vec<T>>,
{
    fn element(&self) -> Option<String> {
        Some("table".to_string())
    }
}

pub struct TableColumn<R, L, T, U>
where
    R: Lens<Target = Vec<T>>,
    L: Lens<Source = T, Target = U>,
    T: Data,
    U: Data,
{
    p1: PhantomData<R>,
    p2: PhantomData<L>,
}

impl<R, L, T: Data, U: Data> TableColumn<R, L, T, U>
where
    R: Lens<Target = Vec<T>>,
    L: Lens<Source = T, Target = U>,
{
    pub fn new<F, Label>(
        cx: &mut Context,
        list: R,
        item: L,
        label: Label,
        content: F,
    ) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, usize, Then<Index<R, T>, L>),
        Label: 'static + Fn(&mut Context),
    {
        Self { p1: PhantomData::default(), p2: PhantomData::default() }.build2(cx, move |cx| {
            (label)(cx);

            let content = Rc::new(content);
            List::new(cx, list, move |cx, it| {
                let content = content.clone();
                VStack::new(cx, move |cx| {
                    Binding::new(cx, it.then(item), move |cx, l| {
                        (content)(cx, it.idx(), l);
                    });
                });
            });
        })
    }
}

impl<R, L, T: Data, U: Data> View for TableColumn<R, L, T, U>
where
    R: Lens<Target = Vec<T>>,
    L: Lens<Source = T, Target = U>,
{
}
