use std::marker::PhantomData;

use vizia::Lens;
use vizia::*;

fn main() {
    Application::new(|cx| {
        let list: Vec<u32> = (10..22u32).collect();
        Data { 
            list,
        }.build(cx);

        // List of 12 items
        List::new(cx, Data::list, |cx, item| {
            Binding::new(cx, ListData::selected, move |cx, selected|{
                let item = item.clone();
                HStack::new(cx, move |cx| {
                    Label::new(cx, "Hello");
                    Label::new(cx, "World");
                    Label::new(cx, &item.value(cx).to_string());
                    //Label::new(cx, &item.index().to_string());
                }).background_color(
                    if item.index() == *selected.get(cx) {
                        Color::green()
                    } else {
                        Color::blue()
                    }
                ).on_press(cx, move |cx| cx.emit(ListEvent::SetSelected(item.index())));
            });
        });
    })
    .run();
}

#[derive(Lens)]
pub struct Data {
    list: Vec<u32>,
}
impl Model for Data {}



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

pub trait Hoverable {
    type View;
    fn on_press<F>(self, cx: &mut Context, action: F) -> Self::View
    where F: 'static + Fn(&mut Context);
}

impl<'a,V: View> Hoverable for Handle<V> {
    type View = Handle<Press<V>>;
    fn on_press<F>(self, cx: &mut Context, action: F) -> Self::View
    where F: 'static + Fn(&mut Context) 
    {
        Press::new(self, cx, action)
    }
}
