use vizia::*;

fn main() {

    Application::new(|cx|{
        // VStack::new().build(cx, |cx| {
        //     Label::new("Hello").build(cx);
        //     Label::new("World").build(cx);
        // });
        VStack::new().build(cx, |cx|{
            NewStack::new(cx, |cx|{
                Label::new("Hello").build(cx);
                Label::new("World").build(cx);

                NewStack::new(cx, |cx|{
                    Label::new("Hello").build(cx);
                    Label::new("World").build(cx);
                });  

            }).width(Pixels(200.0)).height(Pixels(200.0)).background_color(Color::green());       
        });

    }).run();
}