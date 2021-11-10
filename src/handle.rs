use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use morphorm::{LayoutType, PositionType, Units};

use crate::{Color, CursorIcon, Entity, PseudoClass, Style};

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

    pub fn cursor(self, cursor_icon: CursorIcon) -> Self {

        self.style.borrow_mut().cursor.insert(self.entity, cursor_icon);

        self
    }

    pub fn class(self, name: &str) -> Self {
        
        if let Some(class_list) = self.style.borrow_mut().classes.get_mut(self.entity) {
            class_list.insert(name.to_string());
        }

        self
    }

    pub fn checked(self, state: bool) -> Self {
        if let Some(pseudo_classes) = self.style.borrow_mut().pseudo_classes.get_mut(self.entity) {
            pseudo_classes.set(PseudoClass::CHECKED, true);
        }

        self
    }

    pub fn text(self, value: &str) -> Self {
        self.style.borrow_mut().text.insert(self.entity, value.to_owned());

        self
    }

    pub fn child_space(self, value: Units) -> Self {
        self.style.borrow_mut().child_left.insert(self.entity, value);
        self.style.borrow_mut().child_right.insert(self.entity, value);
        self.style.borrow_mut().child_top.insert(self.entity, value);
        self.style.borrow_mut().child_bottom.insert(self.entity, value);

        self
    }

    pub fn space(self, value: Units) -> Self {
        self.style.borrow_mut().left.insert(self.entity, value);
        self.style.borrow_mut().right.insert(self.entity, value);
        self.style.borrow_mut().top.insert(self.entity, value);
        self.style.borrow_mut().bottom.insert(self.entity, value);

        self
    }

    pub fn size(self, value: Units) -> Self {
        self.style.borrow_mut().width.insert(self.entity, value);
        self.style.borrow_mut().height.insert(self.entity, value);

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
    set_style!(row_between, Units);
    set_style!(col_between, Units);
    set_style!(row_index, usize);
    set_style!(row_span, usize);
    set_style!(col_index, usize);
    set_style!(col_span, usize);
    set_style!(grid_rows, Vec<Units>);
    set_style!(grid_cols, Vec<Units>);

    set_style!(border_width, Units);
    set_style!(border_color, Color);

    set_style!(font_size, f32);


    
}

