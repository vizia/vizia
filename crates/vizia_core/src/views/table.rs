use std::marker::PhantomData;
use std::rc::Rc;

use crate::prelude::*;
use crate::state::{BindIndex, BindThen};

// TODO

pub struct Table<B, T: 'static>
where
    B: Bindable<Output = Vec<T>>,
    T: Data,
{
    p: PhantomData<B>,
}

impl<B: 'static + Bindable<Output = Vec<T>>, T: Data> Table<B, T> {
    pub fn new<'a, F>(cx: &'a mut Context, lens: B, list_builder: F) -> Handle<'a, Self>
    where
        F: 'static + Fn(&mut Context, B),
    {
        Self { p: PhantomData::default() }.build(cx, move |cx| {
            HStack::new(cx, move |cx| {
                (list_builder)(cx, lens);
            });
        })
    }
}

impl<B: 'static + Bindable<Output = Vec<T>>, T: Data> View for Table<B, T>
where
    B: 'static + Bindable<Output = Vec<T>>,
{
    fn element(&self) -> Option<&'static str> {
        Some("table")
    }
}

pub struct TableColumn<R, L, T, U>
where
    R: Bindable<Output = Vec<T>>,
    L: Lens<Source = T, Target = U>,
    T: Data,
    U: Data,
{
    p1: PhantomData<R>,
    p2: PhantomData<L>,
}

impl<R, L, T: Data, U: Data> TableColumn<R, L, T, U>
where
    R: Bindable<Output = Vec<T>>,
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
        F: 'static + Fn(&mut Context, usize, BindThen<BindIndex<R, T>, L, U>),
        Label: 'static + Fn(&mut Context),
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
                    let item = item.clone();
                    Binding::new(cx, it.then(item), move |cx, l| {
                        (content)(cx, index, l.clone());
                    });
                });
            })
            .width(Stretch(1.0));
        })
    }
}

impl<R, L, T: Data, U: Data> View for TableColumn<R, L, T, U>
where
    R: Bindable<Output = Vec<T>>,
    L: Lens<Source = T, Target = U>,
{
    fn element(&self) -> Option<&'static str> {
        Some("tablecolumn")
    }
}
