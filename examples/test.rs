use vizia::*;

fn main() {

    Application::new(|cx|{
        // VStack::new().build(cx, |cx| {
        //     Label::new("Hello").build(cx);
        //     Label::new("World").build(cx);
        // });
        VStack::new(cx, |cx|{
            HStack::new(cx, |cx|{
                Label::new(cx, "Hello");
                Label::new(cx, "World");

                VStack::new(cx, |cx|{
                    Label::new(cx, "Hello");
                    Label::new(cx, "World");
                });  

            }).width(Pixels(200.0)).height(Pixels(200.0)).background_color(Color::green()).custom_prop(3.14);       
        });

    }).background_color(Color::red()).run();
}