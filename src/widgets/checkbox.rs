use crate::{Color, Context, Handle, Units::*, View};



pub struct Checkbox {
    pub checked: bool,
    on_checked: Option<Box<dyn Fn(&mut Context)>>,
    on_unchecked: Option<Box<dyn Fn(&mut Context)>>,
}

impl Checkbox {
    pub fn new(cx: &mut Context, checked: bool) -> Handle<Self> {
        Self {
            checked,
            on_checked: None,
            on_unchecked: None,
        }.build(cx)
        .width(Pixels(20.0))
        .height(Pixels(20.0))
        .background_color(if checked {
            Color::green()
        } else {
            Color::red()
        })
    }
}

impl Handle<Checkbox> {
    pub fn on_checked<F>(self, cx: &mut Context, callback: F) -> Self 
    where F: 'static + Fn(&mut Context),
    {
        if let Some(view) = cx.views.get_mut(&self.entity) {
            if let Some(checkbox) = view.downcast_mut::<Checkbox>() {
                checkbox.on_checked = Some(Box::new(callback));
            }
        }

        self
    }

    pub fn on_unchecked<F>(self, cx: &mut Context, callback: F) -> Self 
    where F: 'static + Fn(&mut Context),
    {
        if let Some(view) = cx.views.get_mut(&self.entity) {
            if let Some(checkbox) = view.downcast_mut::<Checkbox>() {
                checkbox.on_unchecked = Some(Box::new(callback));
            }
        }

        self
    }
}

impl View for Checkbox {
    fn element(&self) -> Option<String> {
        Some("checkbox".to_string())
    }
}