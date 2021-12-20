use morphorm::PositionType;

use crate::{Handle, Context, View, Data, Lens, Model, Visibility, Code, Binding, WindowEvent, style::PropGet};


#[derive(Debug, Default, Data, Lens, Clone)]
pub struct PopupData {
    is_open: bool,
}

impl Model for PopupData {
    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(popup_event) = event.message.downcast() {
            match popup_event {
                PopupEvent::Open => {
                    self.is_open = true;
                    event.consume();
                }

                PopupEvent::Close => {
                    self.is_open = false;
                    event.consume();
                }

                PopupEvent::Switch => {
                    self.is_open ^= true;
                    event.consume();
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum PopupEvent {
    Open,
    Close,
    Switch,
}

pub struct Popup {

}

impl Popup {
    pub fn new<F>(cx: &mut Context, builder: F) -> Handle<Self> 
    where F: 'static + Fn(&mut Context)
    {

        // let is_open = if let Some(popup_data) = cx.data::<PopupData>() {
        //     popup_data.is_open
        // } else {
        //     true
        // };

        Self {

        }.build2(cx, |cx|{
            Binding::new(cx, PopupData::is_open, move |cx, flag|{
                let is_open = *flag.get(cx);
                
                cx.style.borrow_mut().visibility.insert(cx.current, if is_open {Visibility::Visible} else {Visibility::Invisible});

                (builder)(cx);
            });


            cx.add_listener(|popup: &mut Self, cx, event| {
                if let Some(popup_data) = cx.data::<PopupData>() {
                    if let Some(window_event) = event.message.downcast() {
                        match window_event {
                            WindowEvent::MouseDown(_) => {
                                if popup_data.is_open {
                                    if event.origin != cx.current {
                                        if !cx.current.is_over(cx) {
                                            cx.emit(PopupEvent::Close);
                                            event.consume();
                                        } 
                                    }
                                }
                            }
        
                            WindowEvent::KeyDown(code, _) => {
                                if popup_data.is_open {
                                    if *code == Code::Escape {
                                        cx.emit(PopupEvent::Close);
                                    }
                                }
                            }
        
                            _=> {}
                        }
                    }
                }
            });

        }).position_type(PositionType::SelfDirected)
    }
}

impl View for Popup {
    fn element(&self) -> Option<String> {
        Some("popup".to_string())
    }
}