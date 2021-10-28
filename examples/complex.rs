use vizia::*;

fn main() {

    Application::new(|cx|{ //0
        VStack::new(cx, |cx| { // 1 (0)
            Label::new(cx, ""); // 2 (1)
            Label::new(cx, ""); // 3 (1)
        });

        VStack::new(cx, |cx| { // 4 (0)
            VStack::new(cx, |cx| { // 5 (4)
                Label::new(cx, ""); // 6 (5)
                Label::new(cx, ""); // 7 (5)
            });
            Label::new(cx, ""); // 8 (4)
            Label::new(cx, ""); // 9 (4)
        });
    }).run();
}