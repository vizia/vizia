
use std::marker::PhantomData;

use keyboard_types::Code;

use crate::{
    Context, Data, Handle, Lens, Model, TreeExt, View, WindowEvent, Binding, MouseButton,
};


/// An `ItemPtr` is used to access an item from context in a list item template.
/// 
/// An `ItemPtr` is provided by the item template of a list view and can be
/// cloned and passed into content closures. To retrieve the item from the 
/// `ItemPtr`, call the `get()` method:
/// 
/// # Example
/// ```compile_fail
/// List::new(cx, AppData::list, |cx, item|{
///     let item = item.get(cx);
/// });
/// ```
///  
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
    /// Constructs a new ItemPtr from a lens and index.
    pub fn new(lens: L, index: usize, row: usize, col: usize) -> Self {
        Self { lens, index, row, col }
    }

    /// Returns the list index the ItemPtr refers to.
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

/// A view for creating a list of items from a binding to a Vec<T>
pub struct List<L, T: 'static>
where
    L: Lens<Target = Vec<T>>,
    T: Data,
{
    p: PhantomData<L>,
    increment_callback: Option<Box<dyn Fn(&mut Context)>>,
    decrement_callback: Option<Box<dyn Fn(&mut Context)>>,
    clear_callback: Option<Box<dyn Fn(&mut Context)>>,
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
            increment_callback: None,
            decrement_callback: None,
            clear_callback: None,
        }.build2(cx, move |cx|{
            // Bind to the list data
            Binding::new(cx, lens.clone(), move |cx, list|{
                // If the number of list items is different to the number of children of the ListView
                // then remove and rebuild all the children
                let list_len = list.get(cx).len();
                let children = cx.current.child_iter(&cx.tree).enumerate().filter(|(child, _)| *child != 0).collect::<Vec<_>>();
                if children.len() != list_len {
                    //cx.remove_children(cx.current);
                    for (_, child) in children {
                        cx.remove(child);
                    }
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
                        if let Some(callback) = self.increment_callback.take() {
                            (callback)(cx);
                            self.increment_callback = Some(callback);
                        }
                    }

                    Code::ArrowUp => {
                        if let Some(callback) = self.decrement_callback.take() {
                            (callback)(cx);
                            self.decrement_callback = Some(callback);
                        }
                    }

                    Code::Escape => {
                        if let Some(callback) = self.clear_callback.take() {
                            (callback)(cx);
                            self.clear_callback = Some(callback);
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }

        if let Some(WindowEvent::MouseDown(MouseButton::Left)) = event.message.downcast() {
            if !cx.focused.is_child_of(&cx.tree, cx.current) {
                cx.focused = cx.current;
            }
        }
    }
}

 impl<L: Lens<Target = Vec<T>>,T: Data> Handle<'_, List<L,T>> {
     pub fn on_increment<F>(self, callback: F) -> Self
     where F: 'static + Fn(&mut Context) {
         if let Some(list) = self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<List<L,T>>()) {
             list.increment_callback = Some(Box::new(callback));
         }

         self
     }

     pub fn on_decrement<F>(self, callback: F) -> Self
         where F: 'static + Fn(&mut Context) {
         if let Some(list) = self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<List<L,T>>()) {
             list.decrement_callback = Some(Box::new(callback));
         }

         self
     }

     pub fn on_clear<F>(self, callback: F) -> Self
         where F: 'static + Fn(&mut Context) {
         if let Some(list) = self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<List<L,T>>()) {
             list.clear_callback = Some(Box::new(callback));
         }

         self
     }
 }
