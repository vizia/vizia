use core::num;

use async_list_derived_lenses::visible_items;
use vizia::prelude::*;

use sha2::{Digest, Sha256};

#[derive(Lens)]
pub struct AsyncList {
    num_items: usize,
    item_height: f32,
    visible_items: Vec<usize>,
}

pub enum AsyncListEvent {
    SetNumItems(usize),
    SetScrollY(f32),
}

impl AsyncList {
    pub fn new<V: View>(
        cx: &mut Context,
        num_items: usize,
        height: f32,
        item: impl Fn(&mut Context, usize) -> Handle<V> + 'static,
    ) -> Handle<Self> {
        Self { num_items, item_height: height, visible_items: Vec::new() }.build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                VStack::new(cx, |cx| {
                    Binding::new(cx, AsyncList::visible_items, move |cx, list| {
                        for i in list.get(cx) {
                            (item)(cx, i)
                                .top(Pixels(i as f32 * height))
                                .height(Pixels(height))
                                .position_type(PositionType::SelfDirected);
                        }
                    });
                })
                .height(Pixels(num_items as f32 * height));
                // List::new(cx, AsyncList::visible_items, move |cx, index, _|{
                //     (item)(cx, index);
                // })
                // .height(Pixels(num_items as f32 * height));
            })
            .on_scroll(|cx, x, y| {
                //println!("Scroll {} {}", x, y);
                cx.emit(AsyncListEvent::SetScrollY(y));
            })
            .on_geo_changed(move |cx, geo| {
                if geo.contains(GeometryChanged::HEIGHT_CHANGED) {
                    let current = cx.current();
                    let dpi = cx.style().dpi_factor as f32;
                    let container_height = cx.cache().get_height(current);
                    let num_items = (container_height / height / dpi).ceil() as usize;
                    cx.emit(AsyncListEvent::SetNumItems(num_items));
                    //println!("Num Visible Items: {} {} {}", container_height, height, num_items);
                }
            });
        })
        //.height(Pixels(num_items as f32 * height))
    }
}

impl View for AsyncList {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|async_list_event, e| match async_list_event {
            AsyncListEvent::SetNumItems(num_items) => {
                self.visible_items.clear();
                for i in 0..*num_items {
                    self.visible_items.push(i);
                }
            }

            AsyncListEvent::SetScrollY(scrolly) => {
                let current = cx.current();
                let dpi = cx.style().dpi_factor as f32;
                let container_height = cx.cache().get_height(current) / dpi;
                let total_height = self.num_items as f32 * self.item_height;
                let offsety = ((total_height - container_height) * *scrolly).round() * dpi;
                let offset_num = (offsety / self.item_height).ceil() as usize;
                let num_visible = self.visible_items.len();
                //println!("list: {} {}", offset_num, offset_num+num_visible);
                self.visible_items.clear();
                for i in offset_num..(offset_num + num_visible) {
                    self.visible_items.push(i);
                }
            }
        });
    }
}

#[derive(Lens)]
pub struct AsyncModel {
    computed_result: Option<String>,
}

pub enum AsyncModelEvent {
    Finished(String),
}

impl AsyncModel {
    fn new(cx: &mut Context, index: usize) -> Self {
        cx.spawn(move |cx| {
            let result = compute_hash(index);
            cx.emit(AsyncModelEvent::Finished(result));
        });

        Self { computed_result: None }
    }
}

impl Model for AsyncModel {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|model_event, event| match model_event {
            AsyncModelEvent::Finished(value) => {
                self.computed_result = Some(value.clone());
                event.consume();
            }
        });
    }
}

fn compute_hash(i: usize) -> String {
    let mut s = format!("{}", i);
    for _ in 0..i {
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        let result = hasher.finalize();
        s = hex::encode(result);
    }
    s
}

fn main() {
    Application::new(|cx| {
        // for index in 10000000..10000010 {
        //     VStack::new(cx, |cx|{
        //         AsyncModel::new(cx, index).build(cx);

        //         Label::new(cx, AsyncModel::computed_result.map(|result| match result.clone() {
        //             Some(value) => {
        //                 value
        //             }

        //             None => {
        //                 String::from("Loading...")
        //             }
        //         }));
        //     })
        //     .height(Auto);
        // }

        AsyncList::new(cx, 10000, 30.0, |cx, index| {
            // Label::new(cx, index)
            //     .border_color(Color::red())
            //     .border_width(Pixels(1.0))
            VStack::new(cx, |cx| {
                AsyncModel::new(cx, index).build(cx);

                Label::new(
                    cx,
                    AsyncModel::computed_result.map(|result| match result.clone() {
                        Some(value) => value,

                        None => String::from("Loading..."),
                    }),
                );
            })
            // Create a model which performs an asynchronous computation
        });
    })
    .run();
}
