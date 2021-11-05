use crate::{Context, Entity, Handle, LocalizedStringKey, View};
use crate::Units::*;



pub struct Label;

impl Label {
    pub fn new<'a>(cx: &mut Context, text: impl LocalizedStringKey<'a>) -> Handle<Self> {

        // Get the enviroment data
        // Check the local
        // Replace the string



        let handle = Self{}.build(cx)
             .width(Pixels(120.0))
             .height(Pixels(50.0))
             .child_space(Stretch(1.0))
             .child_left(Pixels(5.0))
             //.background_color(Color::blue())
             .text(text.key());

        if let Some(message) = cx.enviroment.bundle.get_message(text.key()) {
            let pattern = message.value().expect("Message has no value.");
            let mut errors = vec![];
            let value = cx.enviroment.bundle.format_pattern(&pattern, None, &mut errors);
            cx.style.borrow_mut().text.insert(handle.entity, value.to_string());
        }

        handle
        
    }
}

impl View for Label {
    fn debug(&self, entity: Entity) -> String {
        format!("{} Label", entity)
    }
}