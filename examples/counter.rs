use vizia::*;

fn main() {
    Application::new(|cx|{   
        HStack::new(cx, |cx|{
            
            let count = 0i32.build(cx);

            Button::new(cx, move |cx| {count.set(cx, |val| *val += 1)}, |cx|{
                Label::new(cx, "Increment");
            });
            Button::new(cx, move |cx|  count.set(cx, |val| *val -= 1), |cx|{
                Label::new(cx, "Decrement");
            });

            Label::new(cx, &count.get(cx).to_string());
        });  
    }).run();
}