use morphorm::PositionType;

use crate::prelude::*;

#[derive(Debug, Default, Data, Lens, Clone)]
pub struct MousePopupData {
    pub is_open: bool,
    pub x: f32,
    pub y: f32,
}

impl Model for MousePopupData {
    fn event(&mut self, ex: &mut EventContext, event: &mut Event) {
        event.map(|popup_event, meta| match popup_event {
            MousePopupEvent::Open => {
                self.is_open = true;
                self.x = ex.mouse.cursorx;
                self.y = ex.mouse.cursory;
                meta.consume();
            }

            MousePopupEvent::Close => {
                self.is_open = false;
                meta.consume();
            }

            MousePopupEvent::Switch => {
                self.is_open ^= true;
                if self.is_open {
                    self.x = ex.mouse.cursorx;
                    self.y = ex.mouse.cursory;
                }
                meta.consume();
            }
        });
    }
}

#[derive(Debug)]
pub enum MousePopupEvent {
    Open,
    Close,
    Switch,
}

pub struct MousePopup<L, X, Y> {
    pub is_open: L,
    pub x_pos: X,
    pub y_pos: Y,
}

impl<L, X, Y> MousePopup<L, X, Y>
where
    L: Lens<Target = bool>,
    X: Lens<Target = f32>,
    Y: Lens<Target = f32>,
{
    pub fn new<F>(
        cx: &mut Context,
        lens: L,
        x_pos: X,
        y_pos: Y,
        capture_focus: bool,
        content: F,
    ) -> Handle<Popup<L>>
    where
        F: 'static + Fn(&mut Context),
    {
        Popup::new(cx, lens.clone(), capture_focus, move |cx| (content)(cx))
            .left(x_pos.map(|x| Pixels(*x)))
            .top(y_pos.map(|y| Pixels(*y)))
            .checked(lens.clone())
            .position_type(PositionType::SelfDirected)
            .z_order(100)
    }
}
