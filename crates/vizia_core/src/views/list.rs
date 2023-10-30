use crate::binding::Index;
use crate::prelude::*;
use std::marker::PhantomData;
use vizia_input::Code;

/// A view for creating a list of items from a binding to a `Vec<T>`
pub struct List<L, T: 'static>
where
    L: Lens,
    <L as Lens>::Target: std::ops::Deref<Target = [T]>,
{
    p: PhantomData<L>,
    increment_callback: Option<Box<dyn Fn(&mut EventContext)>>,
    decrement_callback: Option<Box<dyn Fn(&mut EventContext)>>,
    clear_callback: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl<L: Lens, T: Clone> List<L, T>
where
    <L as Lens>::Target: std::ops::Deref<Target = [T]>,
{
    /// Creates a new List view with a binding to the given lens and a template for constructing the list items
    pub fn new<F>(cx: &mut Context, lens: L, item: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, usize, Index<L, T>),
    {
        //let item_template = Rc::new(item);
        List {
            p: PhantomData,
            increment_callback: None,
            decrement_callback: None,
            clear_callback: None,
        }
        .build(cx, move |cx| {
            // Bind to the list data
            Binding::new(cx, lens.map(|lst| lst.len()), move |cx, list_len| {
                // If the number of list items is different to the number of children of the ListView
                // then remove and rebuild all the children
                let list_len = list_len.get_fallible(cx).map_or(0, |d| d);

                for index in 0..list_len {
                    let ptr = lens.index(index);
                    (item)(cx, index, ptr);
                }
            });
        })
    }
}

impl<L: Lens, T> View for List<L, T>
where
    <L as Lens>::Target: std::ops::Deref<Target = [T]>,
{
    fn element(&self) -> Option<&'static str> {
        Some("list")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::KeyDown(code, _) => match code {
                Code::ArrowDown => {
                    if let Some(callback) = &self.increment_callback {
                        (callback)(cx);
                    }
                }

                Code::ArrowUp => {
                    if let Some(callback) = &self.decrement_callback {
                        (callback)(cx);
                    }
                }

                Code::Escape => {
                    if let Some(callback) = &self.clear_callback {
                        (callback)(cx);
                    }
                }

                _ => {}
            },

            _ => {}
        });

        // event.map(|window_event, _| match window_event {
        //     WindowEvent::MouseDown(MouseButton::Left) => {
        //         if !cx.focused.is_child_of(&cx.tree, cx.current) {
        //             cx.focused = cx.current;
        //         }
        //     }
        //     _ => {}
        // });
    }
}

impl<L: Lens<Target = Vec<T>>, T: Data> Handle<'_, List<L, T>> {
    pub fn on_increment<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext),
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
        F: 'static + Fn(&mut EventContext),
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
        F: 'static + Fn(&mut EventContext),
    {
        if let Some(list) =
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<List<L, T>>())
        {
            list.clear_callback = Some(Box::new(callback));
        }

        self
    }
}
