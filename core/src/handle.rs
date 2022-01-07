use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use morphorm::{LayoutType, PositionType, Units};

use crate::{
    style::Overflow, Abilities, Color, CursorIcon, Display, Entity, PseudoClass, Style, Visibility, Context, Res,
};

macro_rules! set_style {
    ($name:ident, $t:ty) => {
        pub fn $name(self, value: impl Res<$t>) -> Self {
            self.cx.style.$name.insert(self.entity, value.get(self.cx).clone().into());

            // TODO - Split this out
            self.cx.style.needs_relayout = true;
            self.cx.style.needs_redraw = true;

            self
        }
    };
}

pub struct Handle<'a, T> {
    pub entity: Entity,
    pub p: PhantomData<T>,
    pub cx: &'a mut Context, 
}

impl<'a,T> Handle<'a,T> {
    // pub fn null() -> Self {
    //     Self { entity: Entity::null(), style: Rc::default(), p: PhantomData::default() }
    // }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn cursor(self, cursor_icon: CursorIcon) -> Self {
        self.cx.style.cursor.insert(self.entity, cursor_icon);

        self.cx.style.needs_redraw = true;

        self
    }

    pub fn class(self, name: &str) -> Self {
        if let Some(class_list) = self.cx.style.classes.get_mut(self.entity) {
            class_list.insert(name.to_string());
        }

        self.cx.style.needs_restyle = true;

        self
    }

    pub fn font(self, font_name: &str) -> Self {
        self.cx.style.font.insert(self.entity, font_name.to_owned());

        self.cx.style.needs_redraw = true;

        self
    }

    pub fn checked(self, state: bool) -> Self {
        if let Some(pseudo_classes) = self.cx.style.pseudo_classes.get_mut(self.entity) {
            pseudo_classes.set(PseudoClass::CHECKED, state);
        }

        self.cx.style.needs_restyle = true;

        self
    }

    pub fn text(self, value: &str) -> Self {
        self.cx.style.text.insert(self.entity, value.to_owned());

        self.cx.style.needs_redraw = true;

        self
    }

    pub fn z_order(self, value: i32) -> Self {
        self.cx.style.z_order.insert(self.entity, value);

        self.cx.style.needs_redraw = true;

        self
    }

    pub fn overflow(self, value: Overflow) -> Self {
        self.cx.style.overflow.insert(self.entity, value);

        self.cx.style.needs_redraw = true;

        self
    }

    pub fn visibility<U: Clone + Into<Visibility>>(self, value: impl Res<U>) -> Self {
        self.cx.style.visibility.insert(self.entity, value.get(self.cx).clone().into());

        self.cx.style.needs_redraw = true;

        self
    }

    // Abilities
    pub fn hoverable(self, state: bool) -> Self {
        if let Some(abilities) = self.cx.style.abilities.get_mut(self.entity) {
            abilities.set(Abilities::HOVERABLE, state);
        }

        self.cx.style.needs_restyle = true;

        self
    }

    pub fn child_space(self, value: Units) -> Self {
        self.cx.style.child_left.insert(self.entity, value);
        self.cx.style.child_right.insert(self.entity, value);
        self.cx.style.child_top.insert(self.entity, value);
        self.cx.style.child_bottom.insert(self.entity, value);

        self.cx.style.needs_relayout = true;
        self.cx.style.needs_redraw = true;

        self
    }

    pub fn space(self, value: Units) -> Self {
        self.cx.style.left.insert(self.entity, value);
        self.cx.style.right.insert(self.entity, value);
        self.cx.style.top.insert(self.entity, value);
        self.cx.style.bottom.insert(self.entity, value);

        self.cx.style.needs_relayout = true;
        self.cx.style.needs_redraw = true;

        self
    }

    pub fn size(self, value: Units) -> Self {
        self.cx.style.width.insert(self.entity, value);
        self.cx.style.height.insert(self.entity, value);

        self.cx.style.needs_relayout = true;
        self.cx.style.needs_redraw = true;

        self
    }

    pub fn min_size(self, value: Units) -> Self {
        self.cx.style.min_width.insert(self.entity, value);
        self.cx.style.min_height.insert(self.entity, value);

        self.cx.style.needs_relayout = true;
        self.cx.style.needs_redraw = true;

        self
    }

    pub fn max_size(self, value: Units) -> Self {
        self.cx.style.max_width.insert(self.entity, value);
        self.cx.style.max_height.insert(self.entity, value);

        self.cx.style.needs_relayout = true;
        self.cx.style.needs_redraw = true;

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
    //set_style!(visibility, Visibility);



    set_style!(rotate, f32);
    set_style!(translate, (f32, f32));
}
