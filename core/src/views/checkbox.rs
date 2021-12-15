use crate::{Color, Context, Handle, MouseButton, Units::*, View, WindowEvent, PseudoClass};

const ICON_CHECK: &str = "\u{2713}";

pub struct Checkbox {
    pub checked: bool,
    on_checked: Option<Box<dyn Fn(&mut Context)>>,
    on_unchecked: Option<Box<dyn Fn(&mut Context)>>,
    icon_checked: Option<String>,
    icon_unchecked: Option<String>,
}

impl Checkbox {
    pub fn new(cx: &mut Context, checked: bool) -> Handle<Self> {
        Self {
            checked,
            on_checked: None,
            on_unchecked: None,
            icon_checked: None,
            icon_unchecked: None,
        }.build(cx)
        .width(Pixels(20.0))
        .height(Pixels(20.0))
        .text(
            if checked {
                ICON_CHECK
            } else {
                ""
            }
        )
        .checked(checked)
    }

    pub fn with_icons(cx: &mut Context, checked: bool, icon_checked: &str, icon_unchecked: &str) -> Handle<Self> {
        Self {
            checked,
            on_checked: None,
            on_unchecked: None,
            icon_checked: Some(icon_checked.to_owned()),
            icon_unchecked: Some(icon_unchecked.to_owned()),
        }.build(cx)
        .width(Pixels(20.0))
        .height(Pixels(20.0))
        .text(
            if checked {
                icon_checked
            } else {
                icon_unchecked
            }
        )
        .checked(checked)
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

    pub fn icon_checked(self, cx: &mut Context, icon: &str) -> Self {
        self
    }

    pub fn icon_unchecked(self, cx: &mut Context, icon: &str) -> Self {
        self
    }

}

impl View for Checkbox {
    fn element(&self) -> Option<String> {
        Some("checkbox".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if event.target == cx.current {
                        if self.checked {
                            self.checked = false;
                            if let Some(pseudo_classes) = cx.style.borrow_mut().pseudo_classes.get_mut(cx.current) {
                                pseudo_classes.set(PseudoClass::CHECKED, false);
                            }

                            if let Some(icon_unchecked) = &self.icon_unchecked {
                                cx.style.borrow_mut().text.insert(cx.current, icon_unchecked.to_owned());
                            } else {
                                cx.style.borrow_mut().text.insert(cx.current, "".to_string());
                            }

                            if let Some(callback) = self.on_unchecked.take() {
                                (callback)(cx);

                                self.on_unchecked = Some(callback);
                            }

                        } else {
                            self.checked = true;
                            if let Some(pseudo_classes) = cx.style.borrow_mut().pseudo_classes.get_mut(cx.current) {
                                pseudo_classes.set(PseudoClass::CHECKED, true);
                            }

                            if let Some(icon_checked) = &self.icon_checked {
                                cx.style.borrow_mut().text.insert(cx.current, icon_checked.to_owned());
                            } else {
                                cx.style.borrow_mut().text.insert(cx.current, ICON_CHECK.to_string());
                            }

                            if let Some(callback) = self.on_checked.take() {
                                (callback)(cx);

                                self.on_checked = Some(callback);
                            }
                        }
                    }
                }

                _=> {}
            }
        }
    }
}