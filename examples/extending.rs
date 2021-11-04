use std::marker::PhantomData;

use vizia::*;

/// Example currently broken due to changes in [Handle]. 
fn main() {
    Application::new(|cx| {
        Button::new(cx, |_| println!("Pressed"), |cx|{
            Label::new(cx, "Press Me!");
        }).on_hover(cx, |cx| println!("Hover"));
    }).run();
}

pub struct Hover<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut EventCtx)>>,

    p: PhantomData<V>,
}

impl<V: View> Hover<V> {
    pub fn new<'a,F>(handle: Handle<V>, cx: &mut Context, action: F) -> Handle<Hover<V>> 
    where F: 'static + Fn(&mut EventCtx)
    {
        let view = cx.views.remove(&handle.entity).unwrap();
        let item = Self {
            view,
            action: Some(Box::new(action)),
            p: Default::default(),
        }; 

        cx.views.insert(handle.entity, Box::new(item));

        Handle {
            entity: handle.entity,
            style: handle.style.clone(),
            p: Default::default(),
        }
    }
}

impl<V: View> View for Hover<V> {
    fn body<'a>(&mut self, cx: &'a mut Context) {
        self.view.body(cx);
    }

    fn event(&mut self, cx: &mut EventCtx, event: &mut Event) {
        self.view.event(cx, event);

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseEnter => {
                    if let Some(action) = self.action.take() {
                        (action)(cx);

                        self.action = Some(action);
                    }
                }

                _=> {}
            }
        }
    }
}

pub trait Hoverable {
    type View;
    fn on_hover<F>(self, cx: &mut Context, action: F) -> Self::View
    where F: 'static + Fn(&mut EventCtx);
}

impl<'a,V: View> Hoverable for Handle<V> {
    type View = Handle<Hover<V>>;
    fn on_hover<F>(self, cx: &mut Context, action: F) -> Self::View
    where F: 'static + Fn(&mut EventCtx) 
    {
        Hover::new(self, cx, action)
    }
}