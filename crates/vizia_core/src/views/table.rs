use std::marker::PhantomData;
use std::rc::Rc;

use crate::prelude::*;
use crate::state::{Index, Then};

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
        Self { p: PhantomData::default() }.build(cx, move |cx| {
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
    fn element(&self) -> Option<&'static str> {
        Some("table")
    }
}

pub struct TableColumn<R, L>
where
    R: Lens<TargetOwned = Vec<L::SourceOwned>>,
    L: Lens,
    L::SourceOwned: Data,
    L::TargetOwned: Data,
{
    p1: PhantomData<R>,
    p2: PhantomData<L>,
}

impl<R, L> TableColumn<R, L>
where
    R: Lens<TargetOwned = Vec<L::SourceOwned>>,
    L: Lens,
    L::SourceOwned: Data,
    L::TargetOwned: Data,
    L::Target: Data,
{
    pub fn new<F, Label>(
        cx: &mut Context,
        list: R,
        item: L,
        label: Label,
        content: F,
    ) -> Handle<Self>
    where
        F: 'static
            + Fn(
                &mut Context,
                usize,
                Then<
                    Then<R, Index<<R as Lens>::TargetOwned, <R as Lens>::Target, L::SourceOwned>>,
                    L,
                >,
            ),
        Label: 'static + Fn(&mut Context),
        <R as Lens>::Source: Model,
    {
        Self { p1: PhantomData::default(), p2: PhantomData::default() }.build(cx, move |cx| {
            //VStack::new(cx, move |cx|{
            (label)(cx);
            //    Element::new(cx).height(Pixels(1.0)).background_color(Color::black());
            //}).height(Pixels(30.0));

            let content = Rc::new(content);
            //let item = item.clone();
            List::new(cx, list, move |cx, index, it| {
                let content = content.clone();
                let item = item.clone();
                VStack::new(cx, move |cx| {
                    //let item = item.clone();
                    Binding::new(cx, it.then(item), move |cx, l| {
                        (content)(cx, index, l);
                    });
                });
            })
            .width(Stretch(1.0));
        })
    }
}

impl<R, L> View for TableColumn<R, L>
where
    R: Lens<TargetOwned = Vec<L::SourceOwned>>,
    L: Lens,
    L::SourceOwned: Data,
    L::TargetOwned: Data,
{
    fn element(&self) -> Option<&'static str> {
        Some("tablecolumn")
    }
}
