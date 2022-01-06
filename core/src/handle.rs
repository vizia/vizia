use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use morphorm::{LayoutType, PositionType, Units};

use crate::{
    style::Overflow, Abilities, Color, CursorIcon, Display, Entity, PseudoClass, Style, Visibility, BorderCornerShape,
};

macro_rules! set_style {
    ($name:ident, $t:ty) => {
        pub fn $name(self, value: $t) -> Self {
            self.style.borrow_mut().$name.insert(self.entity, value);

            // TODO - Split this out
            self.style.borrow_mut().needs_relayout = true;
            self.style.borrow_mut().needs_redraw = true;

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
    pub fn null() -> Self {
        Self { entity: Entity::null(), style: Rc::default(), p: PhantomData::default() }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn cursor(self, cursor_icon: CursorIcon) -> Self {
        self.style.borrow_mut().cursor.insert(self.entity, cursor_icon);

        self.style.borrow_mut().needs_redraw = true;

        self
    }

    pub fn class(self, name: &str) -> Self {
        if let Some(class_list) = self.style.borrow_mut().classes.get_mut(self.entity) {
            class_list.insert(name.to_string());
        }

        self.style.borrow_mut().needs_restyle = true;

        self
    }

    pub fn font(self, font_name: &str) -> Self {
        self.style.borrow_mut().font.insert(self.entity, font_name.to_owned());

        self.style.borrow_mut().needs_redraw = true;

        self
    }

    pub fn checked(self, state: bool) -> Self {
        let style = self.style.clone();
        let mut borrow = style.borrow_mut();
        if let Some(pseudo_classes) = borrow.pseudo_classes.get_mut(self.entity) {
            pseudo_classes.set(PseudoClass::CHECKED, state);
        } else {
            let mut pseudoclass = PseudoClass::empty();
            pseudoclass.set(PseudoClass::CHECKED, state);
            borrow.pseudo_classes.insert(self.entity, pseudoclass);
        }
        
        borrow.needs_restyle = true;

        self
    }

    pub fn disabled(self, state: bool) -> Self {

        self.style.borrow_mut().disabled.insert(self.entity, state);
        self.style.borrow_mut().needs_restyle = true;
        // let style = self.style.clone();
        // let mut borrow = style.borrow_mut();
        // if let Some(pseudo_classes) = borrow.pseudo_classes.get_mut(self.entity) {
        //     pseudo_classes.set(PseudoClass::DISABLED, state);
        // } else {
        //     let mut pseudoclass = PseudoClass::empty();
        //     pseudoclass.set(PseudoClass::DISABLED, state);
        //     borrow.pseudo_classes.insert(self.entity, pseudoclass);
        // }
        
        // borrow.needs_restyle = true;

        self
    }

    pub fn text(self, value: &str) -> Self {
        self.style.borrow_mut().text.insert(self.entity, value.to_owned());

        self.style.borrow_mut().needs_redraw = true;

        self
    }

    pub fn z_order(self, value: i32) -> Self {
        self.style.borrow_mut().z_order.insert(self.entity, value);

        self.style.borrow_mut().needs_redraw = true;

        self
    }

    pub fn overflow(self, value: Overflow) -> Self {
        self.style.borrow_mut().overflow.insert(self.entity, value);

        self.style.borrow_mut().needs_redraw = true;

        self
    }

    // Abilities
    pub fn hoverable(self, state: bool) -> Self {
        if let Some(abilities) = self.style.borrow_mut().abilities.get_mut(self.entity) {
            abilities.set(Abilities::HOVERABLE, state);
        }

        self.style.borrow_mut().needs_restyle = true;

        self
    }

    pub fn child_space(self, value: Units) -> Self {
        self.style.borrow_mut().child_left.insert(self.entity, value);
        self.style.borrow_mut().child_right.insert(self.entity, value);
        self.style.borrow_mut().child_top.insert(self.entity, value);
        self.style.borrow_mut().child_bottom.insert(self.entity, value);

        self.style.borrow_mut().needs_relayout = true;
        self.style.borrow_mut().needs_redraw = true;

        self
    }

    pub fn border_radius(self, value: Units) -> Self {
        self.style.borrow_mut().border_radius_top_left.insert(self.entity, value);
        self.style.borrow_mut().border_radius_top_right.insert(self.entity, value);
        self.style.borrow_mut().border_radius_bottom_left.insert(self.entity, value);
        self.style.borrow_mut().border_radius_bottom_right.insert(self.entity, value);

        self.style.borrow_mut().needs_redraw = true;

        self
    }

    pub fn space(self, value: Units) -> Self {
        self.style.borrow_mut().left.insert(self.entity, value);
        self.style.borrow_mut().right.insert(self.entity, value);
        self.style.borrow_mut().top.insert(self.entity, value);
        self.style.borrow_mut().bottom.insert(self.entity, value);

        self.style.borrow_mut().needs_relayout = true;
        self.style.borrow_mut().needs_redraw = true;

        self
    }

    pub fn size(self, value: Units) -> Self {
        self.style.borrow_mut().width.insert(self.entity, value);
        self.style.borrow_mut().height.insert(self.entity, value);

        self.style.borrow_mut().needs_relayout = true;
        self.style.borrow_mut().needs_redraw = true;

        self
    }

    pub fn min_size(self, value: Units) -> Self {
        self.style.borrow_mut().min_width.insert(self.entity, value);
        self.style.borrow_mut().min_height.insert(self.entity, value);

        self.style.borrow_mut().needs_relayout = true;
        self.style.borrow_mut().needs_redraw = true;

        self
    }

    pub fn max_size(self, value: Units) -> Self {
        self.style.borrow_mut().max_width.insert(self.entity, value);
        self.style.borrow_mut().max_height.insert(self.entity, value);

        self.style.borrow_mut().needs_relayout = true;
        self.style.borrow_mut().needs_redraw = true;

        self
    }

    pub fn color(self, color: Color) -> Self {
        self.style.borrow_mut().font_color.insert(self.entity, color);

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

    set_style!(min_width, Units);
    set_style!(max_width, Units);
    set_style!(min_height, Units);
    set_style!(max_height, Units);

    set_style!(min_left, Units);
    set_style!(max_left, Units);
    set_style!(min_right, Units);
    set_style!(max_right, Units);
    set_style!(min_top, Units);
    set_style!(max_top, Units);
    set_style!(min_bottom, Units);
    set_style!(max_bottom, Units);

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

    set_style!(display, Display);
    set_style!(visibility, Visibility);

    set_style!(rotate, f32);
    set_style!(translate, (f32, f32));

    set_style!(border_shape_top_left, BorderCornerShape);
    set_style!(border_shape_top_right, BorderCornerShape);
    set_style!(border_shape_bottom_left, BorderCornerShape);
    set_style!(border_shape_bottom_right, BorderCornerShape);

    set_style!(border_radius_top_left, Units);
    set_style!(border_radius_top_right, Units);
    set_style!(border_radius_bottom_left, Units);
    set_style!(border_radius_bottom_right, Units);


}
