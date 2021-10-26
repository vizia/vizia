use vizia::*;

fn other_view(cx: &mut Context) {
    Label::new("One").build(cx);
}

fn custom_view(cx: &mut Context) {
    VStack::new().build(cx, |cx| {
        other_view(cx);
        Label::new("Two").build(cx);
        Label::new("Three").build(cx);
    });
}

fn main() {

    Application::new(|cx|{
        custom_view(cx);

        // Print the view tree
        for entity in cx.tree.into_iter() {
            println!("Entity: {} Parent: {:?}", entity, entity.parent(&cx.tree));
        }
    }).run();
}