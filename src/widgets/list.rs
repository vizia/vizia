

use keyboard_types::Code;

use crate::{Color, Context, Event, Handle, Lens, Model, MouseButton, Store, TreeExt, View, WindowEvent};
use crate::Units::*;
#[derive(Debug, Copy)]
pub struct ItemPtr<L, T>
where
    L: Lens<Target = Vec<T>>,
{
    lens: L,
    index: usize,
    row: usize,
    col: usize,
}

impl<L: Lens<Target = Vec<T>>,T> Clone for ItemPtr<L,T> {
    fn clone(&self) -> Self {
        Self {
            lens: self.lens.clone(),
            index: self.index,
            row: self.row,
            col: self.col,
        }
    }
}

impl<L, T> ItemPtr<L, T>
where
    L: Lens<Target = Vec<T>>,
{
    pub fn new(lens: L, index: usize, row: usize, col: usize) -> Self {
        Self { lens, index, row, col }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn value<'a>(&self, cx: &'a Context) -> &'a T
    where
        <L as Lens>::Source: 'static,
    {
        self.lens.view(cx.data().expect("Failed to get data")).get(self.index).expect("Failed to get item")
    }
}

#[derive(Lens)]
pub struct ListData {
    pub selected: usize,
    pub length: usize,
}

#[derive(Debug)]
pub enum ListEvent {
    IncrementSelection,
    DecrementSelection,
    SetSelected(usize),
}

impl Model for ListData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) -> bool {
        if let Some(list_event) = event.message.downcast() {
            match list_event {
                ListEvent::IncrementSelection => {
                    self.selected += 1;
                    self.selected = self.selected.clamp(0, self.length-1);
                    
                    return true;
                }

                ListEvent::DecrementSelection => {
                    if self.selected <= 1 {
                        self.selected = 0;
                    } else {
                        self.selected -= 1;
                    }

                    return true;
                }

                ListEvent::SetSelected(index) => {
                    if *index <= 0 {
                        self.selected = 0;
                    } else if *index > self.length - 1 {
                        self.selected = self.length - 1;
                    } else {
                        self.selected = *index;
                    }
                    return true;
                }
            }
        }

        false
    }
}

pub struct List<L, T: 'static>
where
    L: Lens<Target = Vec<T>>,
{
    lens: L,
    builder: Option<Box<dyn Fn(&mut Context, ItemPtr<L, T>)>>,
}

impl<L: 'static + Lens<Target = Vec<T>>, T> List<L, T> {
    pub fn new<F>(cx: &mut Context, lens: L, item: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, ItemPtr<L, T>),
        <L as Lens>::Source: Model,
    {

        let parent = cx.current;

        let handle = Self {
            lens,
            builder: Some(Box::new(item)),
        }
        .build(cx)
        .height(Auto)
        .width(Auto)
        .background_color(Color::rgb(50,70,90));

        for entity in parent.parent_iter(&cx.tree) {
            if let Some(model_list) = cx.data.model_data.get_mut(entity) {
                for model in model_list.iter_mut() {
                    if let Some(store) = model.downcast::<Store<L::Source>>() {
                        store.observers.insert(handle.entity);
                    }
                }
            }
        }

        cx.focused = handle.entity;

        handle
    }
}

impl<L: 'static + Lens<Target = Vec<T>>, T> View for List<L, T> {
    fn body(&mut self, cx: &mut Context) {

        for child in cx.current.child_iter(&cx.tree.clone()) {
            cx.remove(child);
        }


        let builder = self.builder.take().unwrap();

        let mut found_store = None;

        'tree: for entity in cx.current.parent_iter(&cx.tree.clone()) {
            if let Some(model_list) = cx.data.model_data.get(entity) {
                for model in model_list.iter() {
                    if let Some(store) = model.downcast_ref::<Store<L::Source>>() {
                        found_store = Some(store); 
                        break 'tree;
                    }
                }
            }
        };

        if let Some(store) = found_store {
            
            let len = self.lens.view(&store.data).len();
            
            ListData {
                selected: 3,
                length: len,
            }.build(cx);

            for index in 0..len {
                let ptr = ItemPtr::new(self.lens.clone(), index, index, 0);
                (builder)(cx, ptr);
                cx.count += 1;
            }
        }

        // let store = cx
        //     .data
        //     .model_data
        //     .get(&TypeId::of::<L::Source>())
        //     .and_then(|model| model.downcast_ref::<Store<L::Source>>());

        // if let Some(store) = store {
        //     let len = self.lens.view(&store.data).len();
        //     for index in 0..len {
        //         let ptr = ItemPtr::new(self.lens.clone(), index);
        //         (builder)(cx, ptr);
        //     }
        // }
        self.builder = Some(builder);
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                // WindowEvent::MouseDown(button) => {
                //     if *button == MouseButton::Left {
                //         cx.emit(ListEvent::IncrementSelection);
                //     }

                //     if *button == MouseButton::Right {
                //         cx.emit(ListEvent::DecrementSelection);
                //     }
                // }

                WindowEvent::KeyDown(code, key) => {
                    match code {
                        Code::ArrowDown => {
                            cx.emit(ListEvent::IncrementSelection);
                        }

                        Code::ArrowUp => {
                            cx.emit(ListEvent::DecrementSelection);
                        }

                        _=> {}
                    }
                }

                _=> {}
            }
        }
    }
}
