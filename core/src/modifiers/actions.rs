use std::marker::PhantomData;

use crate::events::ViewHandler;
use morphorm::GeometryChanged;

use crate::prelude::*;

// Press
#[doc(hidden)]
pub struct Press<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> Press<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Press<V>>
    where
        F: 'static + Fn(&mut Context),
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

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        self.view.event(cx, event);

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseDown(MouseButton::Left) => {
                if cx.current() != cx.hovered()
                    && !cx.hovered().is_descendant_of(cx.tree_ref(), cx.current())
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
#[doc(hidden)]
pub struct Release<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> Release<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Release<V>>
    where
        F: 'static + Fn(&mut Context),
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

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
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
#[doc(hidden)]
pub struct Hover<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> Hover<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Hover<V>>
    where
        F: 'static + Fn(&mut Context),
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

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
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

// Hover Out
#[doc(hidden)]
pub struct HoverOut<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> HoverOut<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, HoverOut<V>>
    where
        F: 'static + Fn(&mut Context),
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

impl<V: View> View for HoverOut<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
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

// Over
#[doc(hidden)]
pub struct Over<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> Over<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Over<V>>
    where
        F: 'static + Fn(&mut Context),
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

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
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

// Over
#[doc(hidden)]
pub struct OverOut<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> OverOut<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, OverOut<V>>
    where
        F: 'static + Fn(&mut Context),
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

impl<V: View> View for OverOut<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        self.view.event(cx, event);

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseOut => {
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

// Mouse Move
#[doc(hidden)]
pub struct MouseMove<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context, f32, f32)>>,

    p: PhantomData<V>,
}

impl<V: View> MouseMove<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, MouseMove<V>>
    where
        F: 'static + Fn(&mut Context, f32, f32),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(hover) = view.downcast_mut::<MouseMove<V>>() {
                    hover.action = Some(Box::new(action));
                }
                handle.cx.views.insert(handle.entity, view);
            }
        }

        Handle { entity: handle.entity, p: Default::default(), cx: handle.cx }
    }
}

impl<V: View> View for MouseMove<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
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

// Focus
#[doc(hidden)]
pub struct Focus<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> Focus<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Focus<V>>
    where
        F: 'static + Fn(&mut Context),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(view) = view.downcast_mut::<Focus<V>>() {
                    view.action = Some(Box::new(action));
                }
                handle.cx.views.insert(handle.entity, view);
            }
        }

        Handle { entity: handle.entity, p: Default::default(), cx: handle.cx }
    }
}

impl<V: View> View for Focus<V> {
    fn element(&self) -> Option<&'static str> {
        self.view.element()
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
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

// Focus Out
#[doc(hidden)]
pub struct FocusOut<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context)>>,

    p: PhantomData<V>,
}

impl<V: View> FocusOut<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, FocusOut<V>>
    where
        F: 'static + Fn(&mut Context),
    {
        if let Some(mut view) = handle.cx.views.remove(&handle.entity) {
            if view.downcast_ref::<V>().is_some() {
                let item = Self { view, action: Some(Box::new(action)), p: Default::default() };

                handle.cx.views.insert(handle.entity, Box::new(item));
            } else {
                if let Some(view) = view.downcast_mut::<Focus<V>>() {
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

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
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
#[doc(hidden)]
pub struct Geo<V: View> {
    view: Box<dyn ViewHandler>,
    action: Option<Box<dyn Fn(&mut Context, GeometryChanged)>>,

    p: PhantomData<V>,
}

impl<V: View> Geo<V> {
    pub fn new<'a, F>(handle: Handle<'a, V>, action: F) -> Handle<'a, Geo<V>>
    where
        F: 'static + Fn(&mut Context, GeometryChanged),
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

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
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

/// Modifiers which add an action callback to a view.
pub trait ActionModifiers<'a, V: View> {
    /// Adds a callback which is performed when the view is pressed on with the left mouse button.
    fn on_press<F>(self, action: F) -> Handle<'a, Press<V>>
    where
        F: 'static + Fn(&mut Context);

    /// Adds a callback which is performed when the left mouse button is released on a view after being pressed.
    fn on_release<F>(self, action: F) -> Handle<'a, Release<V>>
    where
        F: 'static + Fn(&mut Context);

    /// Adds a callback which is performed when the mouse pointer moves over or away from a view.
    fn on_hover<F>(self, action: F) -> Handle<'a, Hover<V>>
    where
        F: 'static + Fn(&mut Context);

    /// Adds a callback which is performed when the mouse pointer moves over or away from a view.
    fn on_hover_out<F>(self, action: F) -> Handle<'a, HoverOut<V>>
    where
        F: 'static + Fn(&mut Context);

    /// Adds a callback which is performed when the mouse pointer moves over or away from the bounds of a view.
    fn on_over<F>(self, action: F) -> Handle<'a, Over<V>>
    where
        F: 'static + Fn(&mut Context);

    /// Adds a callback which is performed when the mouse pointer moves over or away from the bounds of a view.
    fn on_over_out<F>(self, action: F) -> Handle<'a, OverOut<V>>
    where
        F: 'static + Fn(&mut Context);

    /// Adds a callback which is performed when the mouse pointer moves.
    fn on_mouse_move<F>(self, action: F) -> Handle<'a, MouseMove<V>>
    where
        F: 'static + Fn(&mut Context, f32, f32);

    /// Adds a callback which is performed when the view gains or loses keyboard focus.
    fn on_focus<F>(self, action: F) -> Handle<'a, Focus<V>>
    where
        F: 'static + Fn(&mut Context);

    /// Adds a callback which is performed when the view gains or loses keyboard focus.
    fn on_focus_out<F>(self, action: F) -> Handle<'a, FocusOut<V>>
    where
        F: 'static + Fn(&mut Context);

    /// Adds a callback which is performed when the the view changes size or position after layout.
    fn on_geo_changed<F>(self, action: F) -> Handle<'a, Geo<V>>
    where
        F: 'static + Fn(&mut Context, GeometryChanged);
}

impl<'a, V: View> ActionModifiers<'a, V> for Handle<'a, V> {
    fn on_press<F>(self, action: F) -> Handle<'a, Press<V>>
    where
        F: 'static + Fn(&mut Context),
    {
        Press::new(self, action)
    }

    fn on_release<F>(self, action: F) -> Handle<'a, Release<V>>
    where
        F: 'static + Fn(&mut Context),
    {
        Release::new(self, action)
    }

    fn on_hover<F>(self, action: F) -> Handle<'a, Hover<V>>
    where
        F: 'static + Fn(&mut Context),
    {
        Hover::new(self, action)
    }

    fn on_hover_out<F>(self, action: F) -> Handle<'a, HoverOut<V>>
    where
        F: 'static + Fn(&mut Context),
    {
        HoverOut::new(self, action)
    }

    fn on_over<F>(self, action: F) -> Handle<'a, Over<V>>
    where
        F: 'static + Fn(&mut Context),
    {
        Over::new(self, action)
    }

    fn on_over_out<F>(self, action: F) -> Handle<'a, OverOut<V>>
    where
        F: 'static + Fn(&mut Context),
    {
        OverOut::new(self, action)
    }

    fn on_mouse_move<F>(self, action: F) -> Handle<'a, MouseMove<V>>
    where
        F: 'static + Fn(&mut Context, f32, f32),
    {
        MouseMove::new(self, action)
    }

    fn on_focus<F>(self, action: F) -> Handle<'a, Focus<V>>
    where
        F: 'static + Fn(&mut Context),
    {
        Focus::new(self, action)
    }

    fn on_focus_out<F>(self, action: F) -> Handle<'a, FocusOut<V>>
    where
        F: 'static + Fn(&mut Context),
    {
        FocusOut::new(self, action)
    }

    fn on_geo_changed<F>(self, action: F) -> Handle<'a, Geo<V>>
    where
        F: 'static + Fn(&mut Context, GeometryChanged),
    {
        Geo::new(self, action)
    }
}
