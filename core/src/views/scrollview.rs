

use std::rc::Rc;

use crate::{Context, View, Handle, VStack, Model, Lens, Binding, Data, Color, Units::*, WindowEvent, PropSet};



#[derive(Clone, Data, Lens)]
pub struct ScrollData {
    pub height_ratio: f32,
    pub container_height: f32,
    pub width_ratio: f32,
    pub container_width: f32,
}

#[derive(Debug)]
pub enum ScrollEvent {
    SetHeightRatio(f32),
    SetWidthRatio(f32),
    SetContainerHeight(f32),
    SetContainerWidth(f32),
}

impl Model for ScrollData {
    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(scroll_event) = event.message.downcast() {
            match scroll_event {
                ScrollEvent::SetHeightRatio(value) => {
                    self.height_ratio = *value;
                }

                ScrollEvent::SetWidthRatio(value) => {
                    self.width_ratio = *value;
                }

                ScrollEvent::SetContainerHeight(value) => {
                    self.container_height = *value;
                }

                ScrollEvent::SetContainerWidth(value) => {
                    self.container_width = *value;
                }
            }
        }
    }
}

pub struct ScrollView {
    scroll_pos: f32,
    scroll_size: f32,
    overflow: f32,
}

impl ScrollView {
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self> 
    where 
        F: 'static + Fn(&mut Context),
    {
        Self {
            scroll_pos: 0.0,
            scroll_size: 0.0,
            overflow: 0.0,
        }.build2(cx, move |cx|{
            let content = Rc::new(content);
            Binding::new(cx, ScrollData::root, move |cx, scroll_data|{
                //println!("{}", scroll_data.get(cx).something);
                let height = cx.cache.get_height(cx.current);
                let width = cx.cache.get_width(cx.current);
                let container_height = scroll_data.get(cx).container_height;
                let container_width = scroll_data.get(cx).container_width;
                let height_ratio = scroll_data.get(cx).height_ratio;
                let width_ratio = scroll_data.get(cx).width_ratio;
                let h = (container_height - height) * height_ratio;
                let w = (container_width - width) * width_ratio;
                println!("{} {}", height, container_height);
                let content = content.clone();
                VStack::new(cx, move |cx|{
                    (content)(cx);
                }).size(Auto).background_color(Color::green()).child_space(Pixels(5.0)).top(Pixels(-h.floor())).left(Pixels(-w.floor())); 
            });
        })
    }
}

impl View for ScrollView {
    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::GeometryChanged(geo_changed) => {
                    
                    //println!("Geo Changed: {}", event.target);
                    // The container is the second child because the binding is the first
                    // TODO - Move bindings out of tree
                    let container = cx.tree.get_child(cx.current, 1).unwrap();
                    if event.target == cx.current || event.target == container {
                        //println!("entity: {} container: {} {}", cx.cache.get_height(cx.current), container, cx.cache.get_height(container));
                        let scroll_size = cx.cache.get_height(cx.current) / cx.cache.get_height(container);
                        //println!("Scroll Size: {}", scroll_size);
                        let container_height = cx.cache.get_height(container);
                        let container_width = cx.cache.get_width(container);
                        cx.emit(ScrollEvent::SetContainerHeight(container_height));
                        cx.emit(ScrollEvent::SetContainerWidth(container_width));

                    }
                }

                WindowEvent::MouseScroll(_, y) => {
                    let container = cx.tree.get_child(cx.current, 1).unwrap();

                }

                _=> {}
            }
        }
    }
}


pub struct ScrollBar {
    is_dragging: bool,
}

impl ScrollBar {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            is_dragging: false,
        }.build2(cx, |cx|{
            
        })
    }
}

impl View for ScrollBar {
    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        
    }
}