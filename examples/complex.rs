use vizia::*;

fn main() {

    Application::new(|cx|{ //0
        VStack::new().build(cx, |cx| { // 1 (0)
            Label::new("").build(cx); // 2 (1)
            Label::new("").build(cx); // 3 (1)
        });

        VStack::new().build(cx, |cx| { // 4 (0)
            VStack::new().build(cx, |cx| { // 5 (4)
                Label::new("").build(cx); // 6 (5)
                Label::new("").build(cx); // 7 (5)
            });
            Label::new("").build(cx); // 8 (4)
            Label::new("").build(cx); // 9 (4)
        });
    }).run();
}