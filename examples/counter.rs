use vizia::*;

fn main() {
    Application::new(|cx|{   
        VStack::new().build(cx, |cx|{
            
            let count = 0i32.build(cx);

            Button::new(move |cx| count.set(cx, |c| *c += 1)).build(cx, |cx|{
                Label::new("Increment").build(cx);
            });
            Button::new(move |cx|  count.set(cx, |c| *c -= 1)).build(cx, |cx|{
                Label::new("Decrement").build(cx);
            });

            Label::new(&count.get(cx).to_string()).build(cx);
        });  
    }).run();
}