use std::marker::PhantomData;

use crate::{Context, Event, Handle, MouseButton, View, ViewHandler, WindowEvent};




pub struct Press<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> Press<V> {
    pub fn new<'a,F>(handle: Handle<V>, cx: &mut Context, action: F) -> Handle<Press<V>> 
    where F: 'static + Fn(&mut Context)
    {
        if let Some(view) = cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self {
                    view,
                    action: Some(Box::new(action)),
                    p: Default::default(),
                }; 
        
                cx.views.insert(handle.entity, Box::new(item));
            } else {
                cx.views.insert(handle.entity, view);
            }

        }


        Handle {
            entity: handle.entity,
            style: handle.style.clone(),
            p: Default::default(),
        }
    }
}

impl<V: View> View for Press<V> {
    fn body<'a>(&mut self, cx: &'a mut Context) {
        self.view.body(cx);
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        self.view.event(cx, event);

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
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

pub struct Release<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> Release<V> {
    pub fn new<'a,F>(handle: Handle<V>, cx: &mut Context, action: F) -> Handle<Release<V>> 
    where F: 'static + Fn(&mut Context)
    {
        if let Some(view) = cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self {
                    view,
                    action: Some(Box::new(action)),
                    p: Default::default(),
                }; 
        
                cx.views.insert(handle.entity, Box::new(item));
            } else {
                cx.views.insert(handle.entity, view);
            }

        }


        Handle {
            entity: handle.entity,
            style: handle.style.clone(),
            p: Default::default(),
        }
    }
}

impl<V: View> View for Release<V> {
    fn body<'a>(&mut self, cx: &'a mut Context) {
        self.view.body(cx);
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        self.view.event(cx, event);

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
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

pub trait Actions {
    type View: View;
    fn on_press<F>(self, cx: &mut Context, action: F) -> Handle<Press<Self::View>>
    where F: 'static + Fn(&mut Context);

    fn on_release<F>(self, cx: &mut Context, action: F) -> Handle<Release<Self::View>>
    where F: 'static + Fn(&mut Context);


}

impl<'a,V: View> Actions for Handle<V> {
    type View = V;
    fn on_press<F>(self, cx: &mut Context, action: F) -> Handle<Press<Self::View>>
    where F: 'static + Fn(&mut Context) 
    {
        Press::new(self, cx, action)
    }

    fn on_release<F>(self, cx: &mut Context, action: F) -> Handle<Release<Self::View>>
    where F: 'static + Fn(&mut Context) 
    {
        Release::new(self, cx, action)
    }


}
