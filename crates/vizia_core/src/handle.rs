use std::marker::PhantomData;

use morphorm::{LayoutType, PositionType, Units};

use vizia_id::GenerationalId;

use crate::prelude::*;
use crate::style::{Abilities, PseudoClass};
use crate::text::Selection;

macro_rules! set_style {
    ($name:ident, $t:ty) => {
        pub fn $name(self, value: impl Res<$t>) -> Self {
            value.set_or_bind(self.cx, self.entity, |cx, entity, v| {
                cx.style.$name.insert(entity, v.into());

                // TODO - Split this out
                cx.need_relayout();
                cx.need_redraw();
            });

            // self.cx.style().$name.insert(self.entity, value.get_val(self.cx).into());

            // // TODO - Split this out
            // self.cx.need_relayout();
            // self.cx.need_redraw();

            self
        }
    };
}

/// A handle to a view which has been already built into the tree.
///
/// This type is part of the prelude.
pub struct Handle<'a, T> {
    pub entity: Entity,
    pub p: PhantomData<T>,
    pub cx: &'a mut Context,
}

impl<'a, T> Handle<'a, T> {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn ignore(self) -> Self {
        self.cx.tree.set_ignored(self.entity, true);
        self.focusable(false)
    }

    /// Stop the user from tabbing out of a subtree, which is useful for modal dialogs.
    pub fn lock_focus_to_within(self) -> Self {
        self.cx.tree.set_lock_focus_within(self.entity, true);
        self.cx.focus_stack.push(self.cx.focused);
        if !self.cx.focused.is_descendant_of(&self.cx.tree, self.entity) {
            let new_focus = vizia_storage::TreeIterator::subtree(&self.cx.tree, self.entity)
                .filter(|node| crate::tree::is_focusable(self.cx, *node))
                .next()
                .unwrap_or(Entity::root());
            self.cx.with_current(new_focus, |cx| cx.focus());
        }
        self
    }

    pub fn modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut T),
        T: 'static,
    {
        if let Some(view) = self
            .cx
            .views
            .get_mut(&self.entity)
            .and_then(|view_handler| view_handler.downcast_mut::<T>())
        {
            (f)(view);
        }

        self
    }

    /// Callback which is run when the view is built/rebuilt
    pub fn on_build<F>(self, callback: F) -> Self
    where
        F: Fn(&mut EventContext),
    {
        let mut event_context = EventContext::new(self.cx);
        event_context.current = self.entity;
        (callback)(&mut event_context);

        self
    }

    pub fn bind<L, F>(self, lens: L, closure: F) -> Self
    where
        L: Lens,
        <L as Lens>::Target: Data,
        F: 'static + Fn(Handle<'_, T>, L),
    {
        let entity = self.entity();
        Binding::new(self.cx, lens, move |cx, data| {
            let new_handle = Handle { entity, p: Default::default(), cx };

            new_handle.cx.set_current(new_handle.entity);
            (closure)(new_handle, data);
        });
        self
    }

    pub fn id(self, id: impl Into<String>) -> Self {
        let id = id.into();
        self.cx.style.ids.insert(self.entity, id.clone()).expect("Could not insert id");
        self.cx.need_restyle();

        self.cx.entity_identifiers.insert(id, self.entity);

        self
    }

    pub fn cursor(self, cursor_icon: CursorIcon) -> Self {
        self.cx.style.cursor.insert(self.entity, cursor_icon);

        self.cx.need_redraw();

        self
    }

    pub fn class(self, name: &str) -> Self {
        if let Some(class_list) = self.cx.style.classes.get_mut(self.entity) {
            class_list.insert(name.to_string());
        }

        self.cx.need_restyle();

        self
    }

    pub fn toggle_class(self, name: &str, applied: impl Res<bool>) -> Self {
        let name = name.to_owned();
        applied.set_or_bind(self.cx, self.entity, move |cx, entity, applied| {
            if let Some(class_list) = cx.style.classes.get_mut(entity) {
                if applied {
                    class_list.insert(name.clone());
                } else {
                    class_list.remove(&name);
                }
            }

            cx.need_restyle();
        });

        self
    }

    pub fn font(self, font_name: &str) -> Self {
        self.cx.style.font.insert(self.entity, font_name.to_owned());

        self.cx.need_redraw();

        self
    }

    pub fn checked(self, state: impl Res<bool>) -> Self {
        state.set_or_bind(self.cx, self.entity, |cx, entity, val| {
            if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClass::CHECKED, val);
            } else {
                let mut pseudoclass = PseudoClass::empty();
                pseudoclass.set(PseudoClass::CHECKED, val);
                cx.style.pseudo_classes.insert(entity, pseudoclass).unwrap();
            }

            cx.need_restyle();
        });

        // let state = state.get_val(self.cx);
        // if let Some(pseudo_classes) = self.cx.style().pseudo_classes.get_mut(self.entity) {
        //     pseudo_classes.set(PseudoClass::CHECKED, state);
        // } else {
        //     let mut pseudoclass = PseudoClass::empty();
        //     pseudoclass.set(PseudoClass::CHECKED, state);
        //     self.cx.style().pseudo_classes.insert(self.entity, pseudoclass).unwrap();
        // }

        // self.cx.need_restyle();

        self
    }

    pub fn disabled(self, state: impl Res<bool>) -> Self {
        state.set_or_bind(self.cx, self.entity, |cx, entity, val| {
            cx.style.disabled.insert(entity, val);
            cx.need_restyle();
        });

        self
    }

    pub fn text<U: ToString>(self, value: impl Res<U>) -> Self {
        value.set_or_bind(self.cx, self.entity, |cx, entity, val| {
            if let Some(prev_data) = cx.style.text.get(entity) {
                if prev_data != &val.to_string() {
                    cx.style.text.insert(entity, val.to_string());

                    cx.need_relayout();
                    cx.need_redraw();
                }
            } else {
                cx.style.text.insert(entity, val.to_string());

                cx.need_relayout();
                cx.need_redraw();
            }
        });

        self
    }

    pub fn image<U: ToString>(self, value: impl Res<U>) -> Self {
        value.set_or_bind(self.cx, self.entity, |cx, entity, val| {
            let val = val.to_string();
            if let Some(prev_data) = cx.style.image.get(entity) {
                if prev_data != &val {
                    cx.style.image.insert(entity, val);

                    cx.need_redraw();
                }
            } else {
                cx.style.image.insert(entity, val);

                cx.need_redraw();
            }
        });

        self
    }

    pub fn z_order(self, value: i32) -> Self {
        self.cx.style.z_order.insert(self.entity, value);

        self.cx.need_redraw();

        self
    }

    pub fn overflow(self, value: Overflow) -> Self {
        self.cx.style.overflow.insert(self.entity, value);

        self.cx.need_redraw();

        self
    }

    pub fn display<U: Clone + Into<Display>>(self, value: impl Res<U>) -> Self {
        value.set_or_bind(self.cx, self.entity, |cx, entity, val| {
            cx.style.display.insert(entity, val.into());

            cx.need_relayout();
            cx.need_redraw();
        });

        self
    }

    pub fn visibility<U: Clone + Into<Visibility>>(self, value: impl Res<U>) -> Self {
        value.set_or_bind(self.cx, self.entity, move |cx, entity, v| {
            cx.style.visibility.insert(entity, v.into());

            cx.need_redraw();
        });

        self
    }

    // Abilities
    pub fn hoverable(self, state: bool) -> Self {
        if let Some(abilities) = self.cx.style.abilities.get_mut(self.entity) {
            abilities.set(Abilities::HOVERABLE, state);
        }

        self.cx.need_restyle();

        self
    }

    pub fn focusable(self, state: bool) -> Self {
        if let Some(abilities) = self.cx.style.abilities.get_mut(self.entity) {
            abilities.set(Abilities::FOCUSABLE, state);
            // If an element is not focusable then it can't be keyboard navigatable
            if !state {
                abilities.set(Abilities::KEYBOARD_NAVIGATABLE, false);
            }
        }

        self.cx.need_restyle();

        self
    }

    pub fn keyboard_navigatable(self, state: bool) -> Self {
        if let Some(abilities) = self.cx.style.abilities.get_mut(self.entity) {
            abilities.set(Abilities::KEYBOARD_NAVIGATABLE, state);
            if state {
                // If an element is keyboard navigatable then it must be focusable
                abilities.set(Abilities::FOCUSABLE, state);
            }
        }

        self.cx.need_restyle();

        self
    }

    pub fn child_space(self, value: Units) -> Self {
        self.cx.style.child_left.insert(self.entity, value);
        self.cx.style.child_right.insert(self.entity, value);
        self.cx.style.child_top.insert(self.entity, value);
        self.cx.style.child_bottom.insert(self.entity, value);

        self.cx.need_relayout();
        self.cx.need_redraw();

        self
    }

    pub fn border_radius(self, value: Units) -> Self {
        self.cx.style.border_radius_top_left.insert(self.entity, value);
        self.cx.style.border_radius_top_right.insert(self.entity, value);
        self.cx.style.border_radius_bottom_left.insert(self.entity, value);
        self.cx.style.border_radius_bottom_right.insert(self.entity, value);

        self.cx.need_redraw();

        self
    }

    pub fn space(self, value: Units) -> Self {
        self.cx.style.left.insert(self.entity, value);
        self.cx.style.right.insert(self.entity, value);
        self.cx.style.top.insert(self.entity, value);
        self.cx.style.bottom.insert(self.entity, value);

        self.cx.need_relayout();
        self.cx.need_redraw();

        self
    }

    pub fn size(self, value: Units) -> Self {
        self.cx.style.width.insert(self.entity, value);
        self.cx.style.height.insert(self.entity, value);

        self.cx.need_relayout();
        self.cx.need_redraw();

        self
    }

    pub fn min_size(self, value: Units) -> Self {
        self.cx.style.min_width.insert(self.entity, value);
        self.cx.style.min_height.insert(self.entity, value);

        self.cx.need_relayout();
        self.cx.need_redraw();

        self
    }

    pub fn max_size(self, value: Units) -> Self {
        self.cx.style.max_width.insert(self.entity, value);
        self.cx.style.max_height.insert(self.entity, value);

        self.cx.need_relayout();
        self.cx.need_redraw();

        self
    }

    pub fn color(self, color: Color) -> Self {
        self.cx.style.font_color.insert(self.entity, color);

        self
    }

    pub fn grid_rows(self, rows: Vec<Units>) -> Self {
        self.cx.style.grid_rows.insert(self.entity, rows);

        self
    }

    pub fn grid_cols(self, cols: Vec<Units>) -> Self {
        self.cx.style.grid_cols.insert(self.entity, cols);

        self
    }

    set_style!(background_color, Color);
    set_style!(background_image, String);

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

    set_style!(border_width, Units);
    set_style!(border_color, Color);

    set_style!(font_size, f32);
    set_style!(text_selection, Selection);
    set_style!(caret_color, Color);
    set_style!(selection_color, Color);
    set_style!(text_wrap, bool);

    //set_style!(display, Display);
    //set_style!(visibility, Visibility);

    set_style!(rotate, f32);
    set_style!(translate, (f32, f32));
    set_style!(scale, (f32, f32));

    set_style!(border_shape_top_left, BorderCornerShape);
    set_style!(border_shape_top_right, BorderCornerShape);
    set_style!(border_shape_bottom_left, BorderCornerShape);
    set_style!(border_shape_bottom_right, BorderCornerShape);

    set_style!(border_radius_top_left, Units);
    set_style!(border_radius_top_right, Units);
    set_style!(border_radius_bottom_left, Units);
    set_style!(border_radius_bottom_right, Units);

    set_style!(outline_width, Units);
    set_style!(outline_color, Color);
    set_style!(outline_offset, Units);
}
