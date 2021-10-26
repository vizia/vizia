
use morphorm::Units;

use crate::{Color, State, GenerationalId};

/// Trait which provides methods for setting layout and style properties on entities, rules, and animations
pub trait PropSet2 
where 
    Self: GenerationalId, 
{
    // BACKGROUND

    /// Set the background color property for the entity or rule.
    ///
    /// # Examples
    /// Adds an inline background-color property to an entity:
    /// ```
    /// entity.set_background_color(state, Color::red());
    /// ```
    /// Adds a shared background-color property to a rule:
    /// ```
    /// rule.set_background_color(state, Color::red());
    /// ```
    fn set_background_color(self, state: &mut State, color: Color);

    // SPACE

    /// Set the space for the entity or rule. This is equivalent to setting the left, right, top, and bottom properties.
    ///
    /// The space determines how much space the layout system will place on all sides of an entity.
    ///
    /// # Examples
    /// Adds inline left, right, top, and bottom properties to an entity:
    /// ```
    /// entity.set_space(state, Units(Pixels(10.0)));
    /// ```
    /// Adds shared left, right, top, and bottom properties to a rule:
    /// ```
    /// rule.set_space(state, Units(Pixels(10.0)));
    /// ```
    fn set_space(self, state: &mut State, value: Units);

    /// Set the left property for the entity or rule.
    ///
    /// The left property determines how much space the layout system will place to the left of an entity.
    /// 
    /// # Examples
    /// Adds an inline left property to an entity:
    /// ```
    /// entity.set_left(state, Units(Pixels(10.0)));
    /// ```
    /// Adds a shared left property to a rule:
    /// ```
    /// rule.set_left(state, Units(Pixels(10.0)));
    /// ```
    fn set_left(self, state: &mut State, value: Units);

    /// Set the right property for the entity or rule.
    ///
    /// The right property determines how much space the layout system will place to the right of an entity.
    /// 
    /// # Examples
    /// Adds an inline right property to an entity:
    /// ```
    /// entity.set_right(state, Units(Pixels(10.0)));
    /// ```
    /// Adds a shared right property to a rule:
    /// ```
    /// rule.set_right(state, Units(Pixels(10.0)));
    /// ```
    fn set_right(self, state: &mut State, value: Units);

    /// Set the top property for the entity or rule.
    ///
    /// The top property determines how much space the layout system will place above an entity.
    /// 
    /// # Examples
    /// Adds an inline top property to an entity:
    /// ```
    /// entity.set_top(state, Units(Pixels(10.0)));
    /// ```
    /// Adds a shared top property to a rule:
    /// ```
    /// rule.set_top(state, Units(Pixels(10.0)));
    /// ```
    fn set_top(self, state: &mut State, value: Units);

    /// Set the bottom property for the entity or rule.
    ///
    /// The bottom property determines how much space the layout system will place below an entity.
    /// 
    /// # Examples
    /// Adds an inline bottom property to an entity:
    /// ```
    /// entity.set_bottom(state, Units(Pixels(10.0)));
    /// ```
    /// Adds a shared bottom property to a rule:
    /// ```
    /// rule.set_bottom(state, Units(Pixels(10.0)));
    /// ```
    fn set_bottom(self, state: &mut State, value: Units);


    // CHILD-SPACE

    /// Set the child-space for the entity or rule. This is equivalent to setting the child-left, child-right, child-top, and child-bottom properties.
    ///
    /// The child-space determines how much space the layout system will place around the children of an entity,
    /// provided that the individual left, rigth, top, and bottom properties of the child are set to auto.
    /// This does not place any space between child entities.
    ///
    /// # Examples
    /// Adds inline left, right, top, and bottom properties to an entity:
    /// ```
    /// entity.set_child_space(state, Units(Pixels(10.0)));
    /// ```
    /// Adds shared left, right, top, and bottom properties to a rule:
    /// ```
    /// rule.set_child_space(state, Units(Pixels(10.0)));
    /// ```
    fn set_child_space(self, state: &mut State, value: Units);


    /// Set the horizontal spacing between children for the entity or rule.
    ///
    /// The col-between determines how much space the layout system will place between the children of an entity horizontally,
    /// provided that the individual left and right properties of the child are set to auto.
    /// This applies to both horizontal stacks and grids with more than one column.
    ///
    /// # Examples
    /// Adds inline left, right, top, and bottom properties to an entity:
    /// ```
    /// entity.set_col_between(state, Units(Pixels(10.0)));
    /// ```
    /// Adds shared left, right, top, and bottom properties to a rule:
    /// ```
    /// rule.set_col_between(state, Units(Pixels(10.0)));
    /// ```
    fn set_col_between(self, state: &mut State, value: Units);

    /// Set the vertical spacing between children for the entity or rule.
    ///
    /// The row-between determines how much space the layout system will place between the children of an entity vertically,
    /// provided that the individual top and bottom properties of the child are set to auto.
    /// This applies to both vertical stacks and grids with more than one row.
    ///
    /// # Examples
    /// Adds inline left, right, top, and bottom properties to an entity:
    /// ```
    /// entity.set_row_between(state, Units(Pixels(10.0)));
    /// ```
    /// Adds shared left, right, top, and bottom properties to a rule:
    /// ```
    /// rule.set_row_between(state, Units(Pixels(10.0)));
    /// ```
    fn set_row_between(self, state: &mut State, value: Units);

    /// Set the child-left property for the entity or rule.
    ///
    /// The child-left property determines how much space the layout system will place to the left of all of the children of an entity,
    /// provided that the left property of the child is set to auto.
    ///
    /// # Examples
    /// Adds an inline child-left property to an entity:
    /// ```
    /// entity.set_child_left(state, Units(Pixels(10.0)));
    /// ```
    /// Adds a shared child-left property to a rule:
    /// ```
    /// rule.set_child_left(state, Units(Pixels(10.0)));
    /// ```
    fn set_child_left(self, state: &mut State, value: Units);

    /// Set the child-right property for the entity or rule.
    ///
    /// The child-right property determines how much space the layout system will place to the right of all of the children of an entity,
    /// provided that the right property of the child is set to auto.
    ///
    /// # Examples
    /// Adds an inline child-right property to an entity:
    /// ```
    /// entity.set_child_right(state, Units(Pixels(10.0)));
    /// ```
    /// Adds a shared child-right property to a rule:
    /// ```
    /// rule.set_child_right(state, Units(Pixels(10.0)));
    /// ```
    fn set_child_right(self, state: &mut State, value: Units);

    /// Set the child-top property for the entity or rule.
    ///
    /// The child-top property determines how much space the layout system will place above all of the children of an entity,
    /// provided that the top property of the child is set to auto.
    /// 
    /// # Examples
    /// Adds an inline child-top property to an entity:
    /// ```
    /// entity.set_child_top(state, Units(Pixels(10.0)));
    /// ```
    /// Adds a shared child-top property to a rule:
    /// ```
    /// rule.set_child_top(state, Units(Pixels(10.0)));
    /// ```
    fn set_child_top(self, state: &mut State, value: Units);

    /// Set the child-bottom property for the entity or rule.
    ///
    /// The child-bottom property determines how much space the layout system will place below all of the children of an entity,
    /// provided that the bottom property of the child is set to auto.
    ///
    /// # Examples
    /// Adds an inline child-bottom property to an entity:
    /// ```
    /// entity.set_child_bottom(state, Units(Pixels(10.0)));
    /// ```
    /// Adds a shared child-bottom property to a rule:
    /// ```
    /// rule.set_child_bottom(state, Units(Pixels(10.0)));
    /// ```
    fn set_child_bottom(self, state: &mut State, value: Units);
}
