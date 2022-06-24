use std::marker::PhantomData;

use crate::events::ViewHandler;
use morphorm::GeometryChanged;

use crate::prelude::*;

// Press
pub struct Press<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut EventContext)>>,

    p: PhantomData<V>,
}

impl<V: View> Press<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Press<V>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(press) = view.downcast_mut::<Press<V>>() {
                    press.action = Some(Box::new(action));
                }
                handle.cx.views.insert(handle.entity, view);
            }
        }

        Handle { entity: handle.entity, p: Default::default(), cx: handle.cx }
    }
}

impl<V: View> View for Press<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.view.event(cx, event);

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseDown(MouseButton::Left) => {
                if cx.current() != cx.hovered()
                    && !cx.hovered().is_descendant_of(cx.tree, cx.current())
                {
                    return;
                }
                if let Some(action) = &self.action {
                    (action)(cx);
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        self.view.draw(cx, canvas);
    }
}

// Release
pub struct Release<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut EventContext)>>,

    p: PhantomData<V>,
}

impl<V: View> Release<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Release<V>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(release) = view.downcast_mut::<Release<V>>() {
                    release.action = Some(Box::new(action));
                }
                handle.cx.views.insert(handle.entity, view);
            }
        }

        Handle { entity: handle.entity, p: Default::default(), cx: handle.cx }
    }
}

impl<V: View> View for Release<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.view.event(cx, event);

        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseUp(MouseButton::Left) => {
                if meta.target == cx.current() {
                    if let Some(action) = &self.action {
                        (action)(cx);
                    }

                    cx.release();
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        self.view.draw(cx, canvas);
    }
}

// Hover
pub struct Hover<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut EventContext)>>,

    p: PhantomData<V>,
}

impl<V: View> Hover<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Hover<V>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(hover) = view.downcast_mut::<Hover<V>>() {
                    hover.action = Some(Box::new(action));
                }
                handle.cx.views.insert(handle.entity, view);
            }
        }

        Handle { entity: handle.entity, p: Default::default(), cx: handle.cx }
    }
}

impl<V: View> View for Hover<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.view.event(cx, event);

        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseEnter => {
                if meta.target == cx.current() {
                    if let Some(action) = &self.action {
                        (action)(cx);
                    }
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        self.view.draw(cx, canvas);
    }
}

// Hover
pub struct Over<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut EventContext)>>,

    p: PhantomData<V>,
}

impl<V: View> Over<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Over<V>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(over) = view.downcast_mut::<Over<V>>() {
                    over.action = Some(Box::new(action));
                }
                handle.cx.views.insert(handle.entity, view);
            }
        }

        Handle { entity: handle.entity, p: Default::default(), cx: handle.cx }
    }
}

impl<V: View> View for Over<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.view.event(cx, event);

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseOver => {
                if let Some(action) = &self.action {
                    (action)(cx);
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        self.view.draw(cx, canvas);
    }
}

// Leave
pub struct Leave<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut EventContext)>>,

    p: PhantomData<V>,
}

impl<V: View> Leave<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Leave<V>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(hover) = view.downcast_mut::<Leave<V>>() {
                    hover.action = Some(Box::new(action));
                }
                handle.cx.views.insert(handle.entity, view);
            }
        }

        Handle { entity: handle.entity, p: Default::default(), cx: handle.cx }
    }
}

impl<V: View> View for Leave<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.view.event(cx, event);

        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseLeave => {
                if meta.target == cx.current() {
                    if let Some(action) = &self.action {
                        (action)(cx);
                    }
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        self.view.draw(cx, canvas);
    }
}

// Move
pub struct Move<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut EventContext, f32, f32)>>,

    p: PhantomData<V>,
}

impl<V: View> Move<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Move<V>>
    where
        F: 'static + Fn(&mut EventContext, f32, f32),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(hover) = view.downcast_mut::<Move<V>>() {
                    hover.action = Some(Box::new(action));
                }
                handle.cx.views.insert(handle.entity, view);
            }
        }

        Handle { entity: handle.entity, p: Default::default(), cx: handle.cx }
    }
}

impl<V: View> View for Move<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.view.event(cx, event);

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseMove(x, y) => {
                if let Some(action) = &self.action {
                    (action)(cx, *x, *y);
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        self.view.draw(cx, canvas);
    }
}

// FocusIn
pub struct FocusIn<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut EventContext)>>,

    p: PhantomData<V>,
}

impl<V: View> FocusIn<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, FocusIn<V>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(view) = view.downcast_mut::<FocusIn<V>>() {
                    view.action = Some(Box::new(action));
                }
                handle.cx.views.insert(handle.entity, view);
            }
        }

        Handle { entity: handle.entity, p: Default::default(), cx: handle.cx }
    }
}

impl<V: View> View for FocusIn<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.view.event(cx, event);

        event.map(|window_event, _| match window_event {
            WindowEvent::FocusIn => {
                if let Some(action) = &self.action {
                    (action)(cx);
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        self.view.draw(cx, canvas);
    }
}

// FocusOut
pub struct FocusOut<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut EventContext)>>,

    p: PhantomData<V>,
}

impl<V: View> FocusOut<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, FocusOut<V>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(view) = view.downcast_mut::<FocusOut<V>>() {
                    view.action = Some(Box::new(action));
                }
                handle.cx.views.insert(handle.entity, view);
            }
        }

        Handle { entity: handle.entity, p: Default::default(), cx: handle.cx }
    }
}

impl<V: View> View for FocusOut<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.view.event(cx, event);

        event.map(|window_event, _| match window_event {
            WindowEvent::FocusOut => {
                if let Some(action) = &self.action {
                    (action)(cx);
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        self.view.draw(cx, canvas);
    }
}

// Geo
pub struct Geo<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut EventContext, GeometryChanged)>>,

    p: PhantomData<V>,
}

impl<V: View> Geo<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Geo<V>>
    where
        F: 'static + Fn(&mut EventContext, GeometryChanged),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(geo) = view.downcast_mut::<Geo<V>>() {
                    geo.action = Some(Box::new(action));
                }
                handle.cx.views.insert(handle.entity, view);
            }
        }

        Handle { entity: handle.entity, p: Default::default(), cx: handle.cx }
    }
}

impl<V: View> View for Geo<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.view.event(cx, event);

        event.map(|window_event, meta| match window_event {
            WindowEvent::GeometryChanged(geo) => {
                if meta.target == cx.current() {
                    if let Some(action) = &self.action {
                        (action)(cx, *geo);
                    }
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        self.view.draw(cx, canvas);
    }
}

/// Methods to add an event handler to a view.
pub trait Actions<'a> {
    type View: View;
    fn on_press<F>(self, action: F) -> Handle<'a, Press<Self::View>>
    where
        F: 'static + Fn(&mut EventContext);

    fn on_release<F>(self, action: F) -> Handle<'a, Release<Self::View>>
    where
        F: 'static + Fn(&mut EventContext);

    fn on_hover<F>(self, action: F) -> Handle<'a, Hover<Self::View>>
    where
        F: 'static + Fn(&mut EventContext);

    fn on_over<F>(self, action: F) -> Handle<'a, Over<Self::View>>
    where
        F: 'static + Fn(&mut EventContext);

    fn on_leave<F>(self, action: F) -> Handle<'a, Leave<Self::View>>
    where
        F: 'static + Fn(&mut EventContext);

    fn on_move<F>(self, action: F) -> Handle<'a, Move<Self::View>>
    where
        F: 'static + Fn(&mut EventContext, f32, f32);

    fn on_focus_in<F>(self, action: F) -> Handle<'a, FocusIn<Self::View>>
    where
        F: 'static + Fn(&mut EventContext);

    fn on_focus_out<F>(self, action: F) -> Handle<'a, FocusOut<Self::View>>
    where
        F: 'static + Fn(&mut EventContext);

    fn on_geo_changed<F>(self, action: F) -> Handle<'a, Geo<Self::View>>
    where
        F: 'static + Fn(&mut EventContext, GeometryChanged);
}

impl<'a, V: View> Actions<'a> for Handle<'a, V> {
    type View = V;
    fn on_press<F>(self, action: F) -> Handle<'a, Press<Self::View>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        Press::new(self, action)
    }

    fn on_release<F>(self, action: F) -> Handle<'a, Release<Self::View>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        Release::new(self, action)
    }

    fn on_hover<F>(self, action: F) -> Handle<'a, Hover<Self::View>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        Hover::new(self, action)
    }

    fn on_over<F>(self, action: F) -> Handle<'a, Over<Self::View>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        Over::new(self, action)
    }

    fn on_leave<F>(self, action: F) -> Handle<'a, Leave<Self::View>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        Leave::new(self, action)
    }

    fn on_move<F>(self, action: F) -> Handle<'a, Move<Self::View>>
    where
        F: 'static + Fn(&mut EventContext, f32, f32),
    {
        Move::new(self, action)
    }

    fn on_focus_in<F>(self, action: F) -> Handle<'a, FocusIn<Self::View>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        FocusIn::new(self, action)
    }

    fn on_focus_out<F>(self, action: F) -> Handle<'a, FocusOut<Self::View>>
    where
        F: 'static + Fn(&mut EventContext),
    {
        FocusOut::new(self, action)
    }

    fn on_geo_changed<F>(self, action: F) -> Handle<'a, Geo<Self::View>>
    where
        F: 'static + Fn(&mut EventContext, GeometryChanged),
    {
        Geo::new(self, action)
    }
}

// pub trait ViewModifers {
//     type View: View;

//     fn overlay<B>(self, cx: &mut EventContext, builder: B) -> Handle<Self::View>
//     where
//         B: 'static + FnOnce(&mut EventContext);
// }

// impl<'a, V: View> ViewModifers for Handle<'a, V> {
//     type View = V;
//     fn overlay<B>(self, cx: &mut EventContext, builder: B) -> Handle<Self::View>
//     where
//         B: 'static + FnOnce(&mut EventContext),
//     {
//         (builder)(cx);

//         self
//     }
// }
