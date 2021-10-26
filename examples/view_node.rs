use vizia::*;

struct CustomView;

impl CustomView {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Node for CustomView {
    fn body(&mut self, cx: &mut Context) {
        VStack::new().build(cx, |cx| {
            Label::new("").build(cx);
            Label::new("").build(cx);
        });
    }
}

fn main() {

    Application::new(|cx|{
        CustomView::new().build(cx);
    }).run();
}