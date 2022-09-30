use vizia_id::{
    impl_generational_id, GenerationalId, GENERATIONAL_ID_GENERATION_MASK,
    GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
};

/// A rule is an id used to get/set shared style properties in State.
///
/// Rather than having widgets own their data, all state is stored in a single database and
/// is stored and loaded using entities.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Rule(u32);

impl_generational_id!(Rule);

// impl PropSet2 for Rule {

//     // BACKGOUND

//     /// Set the background-color property for the shared style rule.
//     ///
//     /// Note: background-color is overridden by background-gradient, which is overridden by background-image.
//     ///
//     /// # Example
//     /// ```
//     /// rule.set_background_color(state, Color::red());
//     /// ```
//     /// # CSS
//     /// The background color property can be set with a color name, like 'red', or a hex value, like '#FF0000'.
//     /// ```css
//     /// background-color: color_name | #hex_value
//     /// ```
//     fn set_background_color(self, state: &mut State, color: Color) {
//         state.style.background_color.insert_rule(self, color);
//     }

//     // SPACE

//     /// Set the space for the shared style rule. This is equivalent to setting the left, right, top, and bottom properties.
//     ///
//     /// The space determines how much space the layout system will place on all sides of an entity.
//     ///
//     /// # Examples
//     /// ```
//     /// rule.set_space(state, Units(Pixels(10.0)));
//     /// ```
//     /// # CSS
//     /// The child-space property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```
//     /// child-space: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_space(self, state: &mut State, value: Units) {
//         state.style.left.insert_rule(self, value);
//         state.style.right.insert_rule(self, value);
//         state.style.top.insert_rule(self, value);
//         state.style.bottom.insert_rule(self, value);
//     }

//     /// Set the left property for the shared style rule.
//     ///
//     /// The left property determines how much space the layout system will place to the left of an entity.
//     ///
//     /// # Examples
//     /// ```
//     /// rule.set_left(state, Pixels(10.0));
//     /// ```
//     /// # CSS
//     /// The left property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```css
//     /// left: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_left(self, state: &mut State, value: Units) {
//         state.style.left.insert_rule(self, value);
//     }

//     /// Set the right property for the shared style rule.
//     ///
//     /// The right property determines how much space the layout system will place to the right of an entity.
//     ///
//     /// # Examples
//     /// ```
//     /// rule.set_right(state, Pixels(10.0));
//     /// ```
//     /// # CSS
//     /// The right property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```css
//     /// right: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_right(self, state: &mut State, value: Units) {
//         state.style.right.insert_rule(self, value);
//     }

//     /// Set the top property for the shared style rule.
//     ///
//     /// The top property determines how much space the layout system will place above an entity.
//     ///
//     /// # Examples
//     /// ```
//     /// rule.set_top(state, Pixels(10.0));
//     /// ```
//     /// # CSS
//     /// The top property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```css
//     /// top: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_top(self, state: &mut State, value: Units) {
//         state.style.top.insert_rule(self, value);
//     }

//     /// Set the bottom property for the shared style rule.
//     ///
//     /// The bottom property determines how much space the layout system will place below an entity.
//     ///
//     /// # Examples
//     /// ```
//     /// rule.set_bottom(state, Pixels(10.0));
//     /// ```
//     /// # CSS
//     /// The bottom property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```css
//     /// bottom: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_bottom(self, state: &mut State, value: Units) {
//         state.style.bottom.insert_rule(self, value);
//     }

//     // CHILD-SPACE

//     /// Set the child-space for the shared style rule. This is equivalent to setting the child-left, child-right, child-top, and child-bottom properties.
//     ///
//     /// The child-space determines how much space the layout system will place around the children of an entity,
//     /// provided that the individual left, rigth, top, and bottom properties of the child are set to auto.
//     /// This does not place any space between child entities.
//     ///
//     /// # Examples
//     /// ```
//     /// rule.set_child_space(state, Units(Pixels(10.0)));
//     /// ```
//     /// # CSS
//     /// The child-space property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```css
//     /// child-space: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_child_space(self, state: &mut State, value: Units) {
//         state.style.child_left.insert_rule(self, value);
//         state.style.child_right.insert_rule(self, value);
//         state.style.child_top.insert_rule(self, value);
//         state.style.child_bottom.insert_rule(self, value);
//     }

//     /// Set the horizontal spacing between children for the shared style rule.
//     ///
//     /// The col-between determines how much space the layout system will place between the children of an entity horizontally,
//     /// provided that the individual left and right properties of the child are set to auto.
//     /// This applies to both horizontal stacks and grids with more than one column.
//     ///
//     /// # Examples
//     /// Adds a shared col-between property to a rule:
//     /// ```
//     /// rule.set_col_between(state, Units(Pixels(10.0)));
//     /// ```
//     /// # CSS
//     /// The col-between property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```css
//     /// col-between: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_col_between(self, state: &mut State, value: Units) {
//         state.style.col_between.insert_rule(self, value);
//     }

//     /// Set the vertical spacing between children for the shared style rule.
//     ///
//     /// The row-between determines how much space the layout system will place between the children of an entity vertically,
//     /// provided that the individual top and bottom properties of the child are set to auto.
//     /// This applies to both vertical stacks and grids with more than one row.
//     ///
//     /// # Examples
//     /// Adds a shared row-between property to a rule:
//     /// ```
//     /// rule.set_row_between(state, Units(Pixels(10.0)));
//     /// ```
//     /// # CSS
//     /// The row-between property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```css
//     /// row-between: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_row_between(self, state: &mut State, value: Units) {
//         state.style.row_between.insert_rule(self, value);
//     }

//     /// Set the child-left property for the shared style rule.
//     ///
//     /// The child-left property determines how much space the layout system will place to the left of all of the children of an entity,
//     /// provided that the left property of the child is set to auto.
//     ///
//     /// # Examples
//     /// ```
//     /// rule.set_child_left(state, Pixels(10.0));
//     /// ```
//     /// # CSS
//     /// The child-left property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```css
//     /// child-left: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_child_left(self, state: &mut State, value: Units) {
//         state.style.child_left.insert_rule(self, value);
//     }

//     /// Set the child-right property for the shared style rule.
//     ///
//     /// The child-right property determines how much space the layout system will place to the right of all of the children of an entity,
//     /// provided that the right property of the child is set to auto.
//     ///
//     /// # Examples
//     /// ```
//     /// rule.set_child_right(state, Pixels(10.0));
//     /// ```
//     /// # CSS
//     /// The child-right property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```css
//     /// child-right: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_child_right(self, state: &mut State, value: Units) {
//         state.style.child_right.insert_rule(self, value);
//     }

//     /// Set the child-top property for the shared style rule.
//     ///
//     /// The child-top property determines how much space the layout system will place above all of the children of an entity,
//     /// provided that the top property of the child is set to auto.
//     ///
//     /// # Examples
//     /// ```
//     /// rule.set_child_top(state, Pixels(10.0));
//     /// ```
//     /// # CSS
//     /// The child-top property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```css
//     /// child-top: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_child_top(self, state: &mut State, value: Units) {
//         state.style.child_top.insert_rule(self, value);
//     }

//     /// Set the child-bottom property for the shared style rule.
//     ///
//     /// The child-bottom property determines how much space the layout system will place below all of the children of an entity,
//     /// provided that the bottom property of the child is set to auto.
//     ///
//     /// # Examples
//     /// ```
//     /// rule.set_child_bottom(state, Pixels(10.0));
//     /// ```
//     /// # CSS
//     /// The child-bottom property can be set with a number (in pixels), a number with px units, a percentage, a stretch value, or auto.
//     /// ```css
//     /// child-bottom: {} | {}px | {}% | {}s | auto
//     /// ```
//     fn set_child_bottom(self, state: &mut State, value: Units) {
//         state.style.child_bottom.insert_rule(self, value);
//     }

// }
