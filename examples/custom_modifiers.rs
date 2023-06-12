use vizia::prelude::*;

pub trait CustomModifier {
    fn title(self) -> Self;
}

// Implement custom modifier for all views
impl<'a, V: View> CustomModifier for Handle<'a, V> {
    fn title(self) -> Self {
        self.font_size(24.0).font_weight(FontWeightKeyword::Bold)
    }
}

// Implement custom modifier just for the label view
// impl<'a> CustomModifier for Handle<'a, Label> {
//     fn title(self) -> Self {
//         self.font_size(24.0).font_weight(FontWeightKeyword::Bold)
//     }
// }

fn main() {
    Application::new(|cx| {
        Label::new(cx, "Custom Title Text").title();
    })
    .run();
}
