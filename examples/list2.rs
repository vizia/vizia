use std::marker::PhantomData;

use vizia::Lens;
use vizia::*;

fn main() {
    Application::new(|cx| {
        let list: Vec<u32> = (0..12u32).collect();
        Data { 
            list,
        }.build(cx);

        HStack::new(cx, |cx|{

            List::new(cx, Data::list, |cx, item| {
                if *item.value(cx) < 6 {
                    Binding::new(cx, ListData::selected, move |cx, selected|{
                        let item = item.clone();
                        HStack::new(cx, move |cx| {
                            Label::new(cx, "Hello");
                            Label::new(cx, "World");
                            Label::new(cx, &item.value(cx).to_string()).background_color(
                                if *item.value(cx) == 40 {
                                    Color::red()
                                } else {
                                    Color::rgba(0,0,0,0)
                                }
                            );
                        }).background_color(
                            if item.index() == *selected.get(cx) {
                                Color::green()
                            } else {
                                Color::blue()
                            }
                        )
                        .on_press(cx, move |cx| cx.emit(ListEvent::SetSelected(item.index())));
                    });
                }
            });

            List::new(cx, Data::list, |cx, item| {
                Binding::new(cx, ListData::selected, move |cx, selected|{
                    let item = item.clone();
                    HStack::new(cx, move |cx| {
                        Label::new(cx, "Hello");
                        Label::new(cx, "World");
                        Label::new(cx, &item.value(cx).to_string()).background_color(
                            if *item.value(cx) == 40 {
                                Color::red()
                            } else {
                                Color::rgba(0,0,0,0)
                            }
                        );
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

            VStack::new(cx, |cx|{
                // Change item with index 5 to value of 40
                Button::new(cx, |cx|{
                    cx.emit(DataEvent::Update(5, 40));
                }, |_|{});
                // Set all items to value of 3
                Button::new(cx, |cx|{
                    cx.emit(DataEvent::All(3));
                }, |_|{});
                // Set all items value to their index
                Button::new(cx, |cx|{
                    cx.emit(DataEvent::Enumerate);
                }, |_|{});
            }).row_between(Pixels(10.0));
        });
    })
    .run();
}

#[derive(Lens)]
pub struct Data {
    list: Vec<u32>,
}

#[derive(Debug)]
pub enum DataEvent {
    Update(usize, u32),
    All(u32),
    Enumerate,
}

impl Model for Data {
    fn event(&mut self, cx: &mut Context, event: &mut Event) -> bool {
        if let Some(data_event) = event.message.downcast() {
            match data_event {
                DataEvent::Update(index, value) => {
                    self.list[*index] = *value;
                    return true;
                }

                DataEvent::All(value) => {
                    for item in self.list.iter_mut() {
                        *item = *value;
                    }
                    return true;
                }

                DataEvent::Enumerate => {
                    for (index, item) in self.list.iter_mut().enumerate() {
                        *item = index as u32;
                    }
                    return true;
                }
            }
        }

        return false;
    }
}



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
