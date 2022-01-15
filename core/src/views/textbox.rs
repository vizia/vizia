use keyboard_types::Code;

use crate::{Context, Handle, MouseButton, View, WindowEvent, Selection, Label, ZStack, Binding, Lens, Model, Element, Units::*, Color};


#[derive(Lens)]
pub struct TextData {
    text: String,
    selection: Selection,
}

impl Model for TextData {

}



pub struct Textbox {
    edit: bool,
    on_submit: Option<Box<dyn Fn(&mut Context, &Self)>>,
}

impl Textbox {
    pub fn new<'a>(cx: &'a mut Context, placeholder: &'static str) -> Handle<'a, Self> {
        Self { 
            edit: false, 
            on_submit: None 
        }
        .build2(cx, move |cx|{



            TextData {
                text: placeholder.to_owned(),
                selection: Selection::new(0, placeholder.len()),
            }.build(cx);
            
            ZStack::new(cx, move |cx|{
                Binding::new(cx, TextData::selection, |cx, selection|{
                    // Position the element according to the selection
                    Element::new(cx).background_color(Color::rgba(100,100,200,120));
                });
            }).size(Auto).text(placeholder);

        })
    }
}

impl<'a> Handle<'a, Textbox> {
    pub fn on_submit<F>(self, cx: &mut Context, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context, &Textbox),
    {
        if let Some(view) = cx.views.get_mut(&self.entity) {
            if let Some(textbox) = view.downcast_mut::<Textbox>() {
                textbox.on_submit = Some(Box::new(callback));
            }
        }

        self
    }
}

impl View for Textbox {


    fn element(&self) -> Option<String> {
        Some("textbox".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if !self.edit {
                        self.edit = true;
                        cx.focused = cx.current;
                    }

                    // Hit test
                    if self.edit {
                        
                    }
                }

                WindowEvent::CharInput(c) => {
                    println!("Input character: {}", c);
                    // Get the selection
                    // Replace the selected range
                    // Set the new selection
                }

                WindowEvent::KeyDown(code, key) => match code {
                    Code::Enter => {
                        // Finish editing
                        self.edit = false;
                    }

                    Code::ArrowLeft => {
                        // Determine grapheme or word movement
                        // Move the cursor
                    }

                    Code::ArrowRight => {
                        // Determine grapheme or word movement
                        // Move the cursor
                    }

                    // TODO
                    Code::ArrowUp => {

                    }

                    // TODO
                    Code::ArrowDown => {

                    }

                    Code::Backspace => {
                        // Determine grapheme or word deletion (upstream)
                        // Delete the text
                    }

                    Code::Delete => {
                        // Determine grapheme or word deletion (downstream)
                        // Delete the text
                    }

                    // TODO
                    Code::Home => {

                    }

                    // TODO
                    Code::End => {

                    }

                    // TODO
                    Code::PageUp => {

                    }

                    // TODO
                    Code::PageDown => {

                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}
