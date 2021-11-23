use std::marker::PhantomData;

use morphorm::GeometryChanged;

use crate::{Context, Entity, Event, Handle, MouseButton, View, ViewHandler, WindowEvent};




pub struct Press<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> Press<V> {
    pub fn new<'a,F>(handle: Handle<V>, cx: &mut Context, action: F) -> Handle<Press<V>> 
    where F: 'static + Fn(&mut Context)
    {
        if let Some(mut view) = cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self {
                    view,
                    action: Some(Box::new(action)),
                    p: Default::default(),
                }; 
        
                cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(press) = view.downcast_mut::<Press<V>>() {
                    press.action = Some(Box::new(action));
                }
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
                    if event.target == cx.current {
                        if let Some(action) = self.action.take() {
                            (action)(cx);
    
                            self.action = Some(action);
                        }
                        
                        cx.captured = cx.current;
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
        if let Some(mut view) = cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self {
                    view,
                    action: Some(Box::new(action)),
                    p: Default::default(),
                }; 
        
                cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(release) = view.downcast_mut::<Release<V>>() {
                    release.action = Some(Box::new(action));
                }
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
                    if event.target == cx.current {
                        if let Some(action) = self.action.take() {
                            (action)(cx);
    
                            self.action = Some(action);
                        }

                        cx.captured = Entity::null();

                    }



                }

                _=> {}
            }
        }
    }
}

// Hover
pub struct Hover<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> Hover<V> {
    pub fn new<'a,F>(handle: Handle<V>, cx: &mut Context, action: F) -> Handle<Hover<V>> 
    where F: 'static + Fn(&mut Context)
    {
        if let Some(mut view) = cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self {
                    view,
                    action: Some(Box::new(action)),
                    p: Default::default(),
                }; 
        
                cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(hover) = view.downcast_mut::<Hover<V>>() {
                    hover.action = Some(Box::new(action));
                }
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

impl<V: View> View for Hover<V> {
    fn body<'a>(&mut self, cx: &'a mut Context) {
        self.view.body(cx);
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        self.view.event(cx, event);

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseEnter => {
                    if event.target == cx.current {
                        if let Some(action) = self.action.take() {
                            (action)(cx);
    
                            self.action = Some(action);
                        }
                    }
                }

                _=> {}
            }
        }
    }
}

// Geo
pub struct Geo<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context, GeometryChanged)>>,

    p: PhantomData<V>,
}

impl<V: View> Geo<V> {
    pub fn new<'a,F>(handle: Handle<V>, cx: &mut Context, action: F) -> Handle<Geo<V>> 
    where F: 'static + Fn(&mut Context, GeometryChanged)
    {
        if let Some(mut view) = cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self {
                    view,
                    action: Some(Box::new(action)),
                    p: Default::default(),
                }; 
        
                cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(geo) = view.downcast_mut::<Geo<V>>() {
                    geo.action = Some(Box::new(action));
                }
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

impl<V: View> View for Geo<V> {
    fn body<'a>(&mut self, cx: &'a mut Context) {
        self.view.body(cx);
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        self.view.event(cx, event);

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::GeometryChanged(geo) => {
                    if event.target == cx.current {
                        if let Some(action) = self.action.take() {
                            (action)(cx, *geo);
    
                            self.action = Some(action);
                        }
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

    fn on_hover<F>(self, cx: &mut Context, action: F) -> Handle<Hover<Self::View>>
    where F: 'static + Fn(&mut Context);

    fn on_geo_changed<F>(self, cx: &mut Context, action: F) -> Handle<Geo<Self::View>>
    where F: 'static + Fn(&mut Context, GeometryChanged);


}

impl<V: View> Actions for Handle<V> {
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

    fn on_hover<F>(self, cx: &mut Context, action: F) -> Handle<Hover<Self::View>>
    where F: 'static + Fn(&mut Context) 
    {
        Hover::new(self, cx, action)
    }

    fn on_geo_changed<F>(self, cx: &mut Context, action: F) -> Handle<Geo<Self::View>>
    where F: 'static + Fn(&mut Context, GeometryChanged)
    {
        Geo::new(self, cx, action)
    }
}


pub trait ViewModifers {
    type View: View;

    fn overlay<B>(self, cx: &mut Context, builder: B) -> Handle<Self::View>
    where B: 'static + FnOnce(&mut Context);
}

impl<V: View> ViewModifers for Handle<V> {
    type View = V;
    fn overlay<B>(self, cx: &mut Context, builder: B) -> Handle<Self::View>
    where B: 'static + FnOnce(&mut Context) {
        (builder)(cx);

        self
    }
}