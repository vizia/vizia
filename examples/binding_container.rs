

use vizia::*;

fn main() {
    Application::new(|cx|{
        Data {
            list: vec![5; 5],
        }.build(cx);

        CustomView::new(cx);
    }).run();
}

pub struct CustomView {

}

impl CustomView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {

        }.build(cx)
    }
}

impl View for CustomView {
    fn body(&mut self, cx: &mut Context) {
        // for child in cx.current.child_iter(&cx.tree.clone()) {
        //     cx.remove(child);
        // }

        let children: Vec<_> = cx.current.child_iter(&cx.tree).collect();
        for child in children {
            cx.remove(child);
        }

        Binding::new(cx, Data::list, |cx, item|{
            Label::new(cx, &format!("{}",item.get(cx).get(0).unwrap()));
        });

        Button::new(cx, |cx| cx.emit(CustomEvent::Reset), |_|{});
    }
}

#[derive(Lens)]
pub struct Data {
    list: Vec<i32>,
}

#[derive(Debug)]
pub enum CustomEvent {
    Reset,
}

impl Model for Data {
    fn event(&mut self, cx: &mut Context, event: &mut Event) -> bool {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::Reset => {
                    self.list = vec![3;3];
                    return true;
                }
            }
        }

        return false;
    }
}