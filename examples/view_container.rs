use vizia::*;

struct CustomView;

impl CustomView {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Container for CustomView {
    fn body<F>(&mut self, cx: &mut Context, f: F) 
    where F: 'static + Fn(&mut Context)
    {
        VStack::new().build(cx, move |cx| {
            (f)(cx);
            (f)(cx);
            Label::new("Three").build(cx);
            Label::new("Four").build(cx);
        });
    }
}

fn main() {

    Application::new(|cx|{
        CustomView::new().build(cx, |cx|{
            VStack::new().build(cx, |cx| {
                Label::new("One").build(cx);
                Label::new("Two").build(cx);
            });
        });
    }).run();
}