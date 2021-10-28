use std::marker::PhantomData;

use morphorm::{LayoutType, PositionType, Units};

use crate::{Color, Context, Entity};

macro_rules! set_style {
    ($name:ident, $t:ty) => {
        pub fn $name(self, value: $t) -> Self {
            self.cx.style.$name.insert(self.entity, value);

            self
        }
    };
}


pub struct Handle<'a,T> {
    pub entity: Entity,
    pub cx: &'a mut Context,
    pub p: PhantomData<T>,
}

impl<'a,T> Handle<'a,T> {

    pub fn text(self, value: &str) -> Self {
        self.cx.style.text.insert(self.entity, value.to_owned());

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

