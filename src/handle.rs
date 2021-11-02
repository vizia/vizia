use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

use morphorm::{LayoutType, PositionType, Units};

use crate::{Color, Context, Entity, Style, ViewHandler};

macro_rules! set_style {
    ($name:ident, $t:ty) => {
        pub fn $name(self, value: $t) -> Self {
            self.style.borrow_mut().$name.insert(self.entity, value);

            self
        }
    };
}


pub struct Handle<T> {
    pub entity: Entity,
    pub style: Rc<RefCell<Style>>,
    pub p: PhantomData<T>,
}

impl<T> Handle<T> {

    pub fn text(self, value: &str) -> Self {
        self.style.borrow_mut().text.insert(self.entity, value.to_owned());

        self
    }

    set_style!(background_color, Color);

    set_style!(layout_type, LayoutType);
    set_style!(position_type, PositionType);


    set_style!(left, Units);
    set_style!(right, Units);
    set_style!(top, Units);
    set_style!(bottom, Units);
    set_style!(width, Units);
    set_style!(height, Units);
    set_style!(child_left, Units);
    set_style!(child_right, Units);
    set_style!(child_top, Units);
    set_style!(child_bottom, Units);

    // pub fn bottom(self, value: Units) -> Self {
    //     self.cx.style.bottom.insert(self.entity, value);

    //     self
    // }


    
}

