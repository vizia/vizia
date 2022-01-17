use crate::{Color, Context, Handle, LocalizedStringKey, Res, View};

pub struct Label;

impl Label {
    pub fn new<'a>(cx: &mut Context, text: impl LocalizedStringKey<'a>) -> Handle<Self> {
        // Get the enviroment data
        // Check the local
        // Replace the string

        let handle = Self {}
            .build(cx)
            //.width(Pixels(120.0))
            //.height(Pixels(50.0))
            //.child_space(Stretch(1.0))
            //.child_left(Pixels(5.0))
            //.background_color(Color::blue())
            .text(text.key());

        // if let Some(message) = cx.enviroment.bundle.get_message(text.key()) {
        //     let pattern = message.value().expect("Message has no value.");
        //     let mut errors = vec![];
        //     let value = cx.enviroment.bundle.format_pattern(&pattern, None, &mut errors);
        //     cx.style.text.insert(handle.entity, value.to_string());
        // }

        handle
    }

    pub fn test(&mut self, test: &str) {
        println!("{}", test);
    }
}

impl View for Label {
    fn element(&self) -> Option<String> {
        Some("label".to_string())
    }
}

impl<'a> Handle<'a, Label> {
    pub fn custom(self, test: impl Res<Color>) -> Self {
        // if let Some(view) = self.cx.views.get_mut(&self.entity) {
        //     if let Some(label) = view.downcast_mut::<Label>() {
        //         label.test(test);
        //     }
        // }

        self.cx.style.background_color.insert(self.entity, *test.get(self.cx));

        self
    }
}
