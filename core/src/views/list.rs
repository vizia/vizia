
use std::marker::PhantomData;

use keyboard_types::Code;

use crate::{
    Context, Data, Event, Handle, Lens, Model, TreeExt, View, WindowEvent, Binding,
};


/// An ItemPtr can be used to access an item from a bound list
#[derive(Debug)]
pub struct ItemPtr<L, T>
where
    L: Lens<Target = Vec<T>>,
{
    lens: L,
    index: usize,
    row: usize,
    col: usize,
}

// Manual implementations of Clone and Copy or else the compiler complains about a Clone bound on T which isn't actually required
impl<L, T> Copy for ItemPtr<L, T> where L: Lens<Target = Vec<T>> {}

impl<L: Lens<Target = Vec<T>>, T> Clone for ItemPtr<L, T> {
    fn clone(&self) -> Self {
        Self { lens: self.lens.clone(), index: self.index, row: self.row, col: self.col }
    }
}

impl<L, T> ItemPtr<L, T>
where
    L: Lens<Target = Vec<T>>,
{
    /// Constructs a new ItemPtr from a lens and index
    pub fn new(lens: L, index: usize, row: usize, col: usize) -> Self {
        Self { lens, index, row, col }
    }

    /// Returns the list index the ItemPtr links to 
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn get<'a>(&self, cx: &'a Context) -> &'a T
    where
        <L as Lens>::Source: 'static,
    {
        self.lens
            .view(cx.data().expect("Failed to get data"))
            .get(self.index)
            .expect(&format!("Failed to get item: {}", self.index))
    }
}


pub trait DataHandle: Clone + Copy {
    type Data;
    fn get<'a>(&self, cx: &'a Context) -> &'a Self::Data;
}

impl<L, T> DataHandle for ItemPtr<L, T>
where
    L: Lens<Target = Vec<T>>,
{
    type Data = T;
    fn get<'a>(&self, cx: &'a Context) -> &'a Self::Data {
        self.get(cx)
    }
}


/// Data for tracking the selected item in a list
#[derive(Lens, Default)]
pub struct ListData {
    pub selected: usize,
    pub length: usize,
}

impl ListData {
    pub fn new(selected: usize) -> Self {
        Self { selected, length: 0 }
    }
}

/// Events for modifying the selected item
#[derive(Debug)]
pub enum ListEvent {
    IncrementSelection,
    DecrementSelection,
    SetSelected(usize),
    SetLength(usize),
}

impl Model for ListData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(list_event) = event.message.downcast() {
            match list_event {
                ListEvent::IncrementSelection => {
                    let mut new_selected = self.selected + 1;
                    new_selected = new_selected.clamp(0, self.length - 1);
                    cx.emit(ListEvent::SetSelected(new_selected));
                }

                ListEvent::DecrementSelection => {
                    let mut new_selected = self.selected as i32 - 1;
                    new_selected = new_selected.clamp(0, self.length as i32 - 1);
                    cx.emit(ListEvent::SetSelected(new_selected as usize));
                }

                ListEvent::SetSelected(index) => {
                    if *index <= 0 {
                        self.selected = 0;
                    } else if *index > self.length - 1 {
                        self.selected = self.length - 1;
                    } else {
                        self.selected = *index;
                    }
                }

                ListEvent::SetLength(length) => {
                    self.length = *length;
                }
            }
        }
    }
}

/// A view for creating a list of items from a binding to a Vec<T>
pub struct List<L, T: 'static>
where
    L: Lens<Target = Vec<T>>,
    T: Data,
{
    p: PhantomData<L>,
}

impl<L: 'static + Lens<Target = Vec<T>>, T: Data> List<L, T> {
    /// Creates a new ListView with a binding to the given lens and a template for constructing the list items
    pub fn new<F>(cx: &mut Context, lens: L, item: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, ItemPtr<L, T>),
        <L as Lens>::Source: Model,
    {
        //let item_template = Rc::new(item);
        List {
            p: PhantomData::default(),
        }.build2(cx, move |cx|{

            cx.focused = cx.current;

            // Bind to the list data
            Binding::new(cx, lens.clone(), move |cx, list|{
                // If the number of list items is different to the number of children of the ListView
                // then remove and rebuild all the children
                let list_len = list.get(cx).len();
                if cx.current.child_iter(&cx.tree).count() != list_len {
                    cx.remove_children(cx.current);
                }

                for index in 0..list_len {
                    let ptr = ItemPtr::new(lens.clone(), index, index, 0);
                    (item)(cx, ptr);
                }
            });
        })

    }
}

impl<L: 'static + Lens<Target = Vec<T>>, T: Data> View for List<L, T> {
    fn element(&self) -> Option<String> {
        Some("list".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::KeyDown(code, _) => match code {
                    Code::ArrowDown => {
                        cx.emit(ListEvent::IncrementSelection);
                    }

                    Code::ArrowUp => {
                        cx.emit(ListEvent::DecrementSelection);
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}

// impl<L: Lens<Target = Vec<T>>,T: Data> Handle<List<L,T>> {
//     pub fn with_list_data(self, cx: &mut Context, flag: bool) -> Self {
//         if let Some(list) = cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<List<L,T>>()) {
//             list.list_data = flag;
//         }

//         self
//     }
// }
