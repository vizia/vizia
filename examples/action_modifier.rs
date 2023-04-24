use vizia::prelude::*;

// Example of extending the behaviour of a view
fn main() {
    Application::new(|cx| {
        Label::new(cx, "Press on me!").on_press(|_| println!("You pressed on a label!"));
        Label::new(cx, "Release on me!").on_mouse_up(|_, _| println!("You released on a label!"));
        Label::new(cx, "Hover on me!").on_hover(|_| println!("You hovered a label!"));
        Label::new(cx, "Right click on me!").on_mouse_down(|_, button| {
            if button == MouseButton::Right {
                println!("You right clicked on a label!")
            }
        });
        CustomView::new(cx);
    })
    .title("Action Modifiers")
    .run();
}

#[derive(Lens)]
pub struct CustomView {
    text: String,
}

impl CustomView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self { text: String::from("Hello World") }
            .build(cx, |cx| {
                Label::new(cx, CustomView::text).hoverable(false);
            })
            .on_press(|cx| {
                cx.modify::<Self>(|custom_view| custom_view.text = String::from("Testy Test"));
            })
            .size(Auto)
    }
}

impl View for CustomView {}
