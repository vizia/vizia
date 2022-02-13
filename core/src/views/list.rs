use std::marker::PhantomData;

use keyboard_types::Code;

use crate::{
    Binding, Context, Data, Handle, Index, Lens, LensExt, Model, TreeExt, View, WindowEvent, Then,
};

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
        F: 'static + Fn(&mut Context, usize, Then<L,Index<Vec<T>, usize>>),
        <L as Lens>::Source: Model,
    {
        //let item_template = Rc::new(item);
        List {
            p: PhantomData::default(),
            increment_callback: None,
            decrement_callback: None,
            clear_callback: None,
        }
        .build2(cx, move |cx| {
            //let list_lens = lens.clone();
            // Bind to the list data
            Binding::new(cx, lens.clone(), move |cx, list| {
                // If the number of list items is different to the number of children of the ListView
                // then remove and rebuild all the children
                let list_len = list.get(cx).len();
                let children = cx
                    .current
                    .child_iter(&cx.tree)
                    .enumerate()
                    .filter(|(child, _)| *child != 0)
                    .collect::<Vec<_>>();
                if children.len() != list_len {
                    //println!("Remove children");
                    //cx.remove_children(cx.current);
                    for (_, child) in children {
                        cx.remove(child);
                    }
                }

                for index in 0..list_len {
                    let ptr = list.clone().index(index);
                    (item)(cx, index, ptr);
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

        // if let Some(WindowEvent::MouseDown(MouseButton::Left)) = event.message.downcast() {
        //     if !cx.focused.is_child_of(&cx.tree, cx.current) {
        //         cx.focused = cx.current;
        //     }
        // }
    }
}

impl<L: Lens<Target = Vec<T>>, T: Data> Handle<'_, List<L, T>> {
    pub fn on_increment<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        if let Some(list) =
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<List<L, T>>())
        {
            list.increment_callback = Some(Box::new(callback));
        }

        self
    }

    pub fn on_decrement<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        if let Some(list) =
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<List<L, T>>())
        {
            list.decrement_callback = Some(Box::new(callback));
        }

        self
    }

    pub fn on_clear<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        if let Some(list) =
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<List<L, T>>())
        {
            list.clear_callback = Some(Box::new(callback));
        }

        self
    }
}
