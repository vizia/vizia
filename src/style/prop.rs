
use crate::{Message, State, Widget};
use crate::{Entity, Propagation};
use crate::{AsEntity, style::*};

use crate::{Event, WindowEvent};

use crate::tree::{TreeExt};

use morphorm::{LayoutType, PositionType, Units};

use std::rc::Rc;


/// To be replaced by [PropSet2]
pub trait PropSet: AsEntity + Sized {

    /// Helper method for sending an event to self with upward propagation
    ///
    /// # Example
    /// Adds an event with a `WindowEvent::Close` message to the event queue to be sent up the tree
    /// ```
    /// entity.emit(state, WindowEvent::Close);
    /// ``` 
    fn emit(&self, state: &mut State, message: impl Message) -> Entity
    where
        Self: 'static,
    {
        state.insert_event(Event::new(message).target(self.entity()).origin(self.entity()).propagate(Propagation::Up));

        self.entity()
    }

    /// Helper method for sending an event to target with direct propagation
    ///
    /// # Example
    /// Adds an event with a `WindowEvent::Close` message to the event queue to be sent directly to the `target` entity
    /// ```
    /// entity.emit_to(state, target, WindowEvent::Close);
    /// ```
    fn emit_to(&self, state: &mut State, target: Entity, message: impl Message) -> Entity {
        state.insert_event(Event::new(message).target(target).origin(self.entity()).propagate(Propagation::Direct));

        self.entity()
    }

    /// Adds an event listener to an entity
    ///
    /// An event listener is a callback which is called before normal event handling takes place.
    /// This allows entities with listeners to intercept events when they might normally be unable to.
    /// For example, a popup uses a listener to respond to mouse press events outside of its bounds to
    /// close the popup.
    /// 
    /// # Example
    /// Add a listener to a button which changes its background color to red when the mouse enters its bounds
    /// ```
    /// entity.add_listener(state, |button: &mut Button, state, entity, event|{
    ///     if let Some(window_event) = event.message.downcast() {
    ///         match window_event {
    ///             WindowEvent::MouseEnter => {
    ///                 entity.set_background_color(state, Color::red());
    ///             }
    ///
    ///             _=> {}
    ///         }
    ///     }   
    /// });
    /// ```
    fn add_listener<F,W>(&self, state: &mut State, listener: F) -> Entity
    where 
        W: Widget, 
        F: 'static + Fn(&mut W, &mut State, Entity, &mut Event)
    {  
        state.listeners.insert(self.entity(), Box::new(move |event_handler, state, entity, event|{
            if let Some(widget) = event_handler.downcast::<W>() {
                (listener)(widget, state, entity, event);
            }
        }));

        self.entity()
    }

    /// Force a restyle
    ///
    /// Sends a `WindowEvent::Restyle` message to the root window.
    ///
    /// # Example
    /// ```
    /// entity.restyle(state);
    /// ```
    fn restyle(&self, state: &mut State) {
        state.insert_event(Event::new(WindowEvent::Restyle).target(self.entity()).origin(self.entity()).unique());
    }

    /// Force a relayout
    ///
    /// Sends a `WindowEvent::Relayout` message to the root window.
    ///
    /// # Example
    /// ```
    /// entity.relayout(state);
    /// ```
    fn relayout(&self, state: &mut State) {
        state.insert_event(Event::new(WindowEvent::Relayout).target(self.entity()).origin(self.entity()).unique());
    }

    /// Force a redraw
    ///
    /// Sends a `WindowEvent::Redraw` message to the root window.
    ///
    /// # Example
    /// ```
    /// entity.redraw(state);
    /// ```
    fn redraw(&self, state: &mut State) {
        state.insert_event(Event::new(WindowEvent::Redraw).target(self.entity()).origin(self.entity()).unique());
    }

    // TODO
    fn set_name(self, state: &mut State, name: &str) -> Entity {
        state.style.name.insert(self.entity(), name.to_string());

        self.entity()
    }

    /// Add a class name to an entity
    ///
    /// Class names are used by the style system to assign style properties to entities.
    /// An entity can have mutiple assigned unique class names with repeated calls of this function.
    /// These class names can be referred to in css selectors, for example:
    /// ```css
    /// .foo {
    ///     background-color: red;
    /// }
    /// ```
    /// This style rule will apply a red background color to any entities with a class name `foo`.
    ///
    /// # Examples
    /// Adds a class name `foo` and to an entity:
    /// ```
    /// entity.class("foo");
    /// ```
    ///
    /// Adds a class name `foo` and a class name `bar` to an entity:
    /// ```
    /// entity.class(state, "foo").class(state, "bar");
    /// ```
    fn class(self, state: &mut State, class_name: &str) -> Entity {
        if let Some(class_list) = state.style.classes.get_mut(self.entity()) {
            class_list.insert(class_name.to_string());
        } else {
            let mut class_list = HashSet::new();
            class_list.insert(class_name.to_string());
            state.style.classes.insert(self.entity(), class_list);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    // TODO move to PropGet
    fn get_parent(self, state: &mut State) -> Option<Entity> {
        self.entity().parent(&state.tree)
    }

    // Pseudoclass

    /// Sets the entities disbaled state to the given flag.
    ///
    /// A flag value of true will set the entity to disabled, while a flag value of false will set the entity to not disabled.
    /// The `disabled` PseudoClass in css can be used to select entities in a disabled state, for example:
    /// ```css
    /// button:disabled {
    ///     background-color: red;   
    /// }
    /// ```
    /// This style rule will apply a red background to any disabled buttons.
    /// While css has an `enabled` pseudoclass, this is not used in tuix.
    ///
    /// # Example
    /// Sets the entity to disabled:
    /// ```
    /// entity.set_disabled(state, true);
    /// ```
    fn set_disabled(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClass::DISABLED, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        ////flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Sets the entities checked state to the given flag.
    ///
    /// A flag value of true will set the entity to checked, while a flag value of false will set the entity to not checked.
    /// The `checked` PseudoClass in css can be used to select entities in a checked state, for example:
    /// ```css
    /// checkbox:checked {
    ///     background-color: red;   
    /// }
    /// ```
    /// This style rule will apply a red background to any checked checkboxes.
    ///
    /// # Example
    /// Sets the entity to checked:
    /// ```
    /// entity.set_checked(state, true);
    /// ```
    fn set_checked(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClass::CHECKED, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_over(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClass::OVER, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_active(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClass::ACTIVE, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_hover(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClass::HOVER, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_focus(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClass::FOCUS, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    // Style
    /// Sets the element name of the entity.
    ///
    /// The element name can be used in css to select entities of a particular type, for example:
    /// ```css
    /// button {
    ///     background-color: red;   
    /// }
    /// ```
    /// This style rule will set the background color of all buttons to red.
    /// Element names are unique, so calling this method again will replace the previous element name.
    /// The element name is supposed to be unique to a widget type, e.g. a button, but this is not guaranteed
    /// by this function and so this function should be called once within the `on_build` method of a [Widget].
    ///
    /// # Example
    /// Sets the element name to `foo`:
    /// ```
    /// entity.set_element(state, "foo");
    /// ```
    fn set_element(self, state: &mut State, value: &str) -> Entity {

        state.style.elements.insert(self.entity(), value.to_string());

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    // TODO
    fn set_id(self, state: &mut State, value: &str) -> Entity {
        self.entity()
    }

    /// Sets the visibility of an entity.
    ///
    /// Visibility determines whether an entity will be rendered. Invisible entities are still acted on by the layout system.
    /// To make an entity invisible to both the rendering and layout systems, use `set_display()`.
    ///
    /// # Examples
    /// Sets the entity to be invisible:
    /// ```
    /// entity.set_visibility(state, Visibility::Invisible);
    /// ``` 
    fn set_visibility(self, state: &mut State, value: Visibility) -> Entity {
        state.style.visibility.insert(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    /// Sets whether the entity can be hovered.
    ///
    /// Entities which are *not* hoverable will not receive mouse events and cannot be selected in css
    /// with the `:hover` pseudoclass.
    ///
    /// # Example
    /// ```
    /// entity.set_hoverable(state, false);
    /// ```
    fn set_hoverable(self, state: &mut State, value: bool) -> Entity {
        state.data.set_hoverable(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    
    /// Sets whether the entity can be checked.
    ///
    /// Entities which are *not* checkable will not receive checkbox events and cannot be selected in css
    /// with the `:checked` pseudoclass.
    ///
    /// # Example
    /// ```
    /// entity.set_checkable(state, false);
    /// ```
    fn set_checkable(self, state: &mut State, value: bool) -> Entity {
        state.data.set_checkable(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    /// Sets whether the entity can be selected in a list.
    ///
    /// Entities which are *not* selectable cannot be selected in css with the `:selected` pseudoclass.
    ///
    /// # Example
    /// ```
    /// entity.set_selectable(state, false);
    /// ```
    fn set_selectable(self, state: &mut State, value: bool) -> Entity {
        state.data.set_selectable(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    /// Sets whether the entity can be focused.
    ///
    /// Entities which are *not* focusable will not receive keyboard events and cannot be selected in css
    /// with the `:focus` pseudoclass.
    ///
    /// # Example
    /// ```
    /// entity.set_focusable(state, false);
    /// ```
    fn set_focusable(self, state: &mut State, value: bool) -> Entity {
        state.data.set_focusable(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    // Overflow
    // TODO
    fn set_overflow(self, state: &mut State, value: Overflow) -> Entity {
        state.style.overflow.insert(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    // Display
    /// Sets whether the entity should be displayed.
    /// 
    /// The display property of an entity can be set to either `Display::Flex` or `Display::None`.
    /// A non-displayed entity will not be rendered or acted on by the layout system. To make an entity
    /// invisible but remain part of layout, use `set_visibility()`.
    ///
    /// # Example
    /// ```
    /// entity.set_display(state, Display::None);
    /// ``` 
    fn set_display(self, state: &mut State, value: Display) -> Entity {
        state.style.display.insert(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Sets the opacity of an entity.
    ///
    ///
    fn set_opacity(self, state: &mut State, value: f32) -> Entity {
        state.style.opacity.insert(self.entity(), Opacity(value));

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    /// Rotate the entity by a given number of degrees.
    /// 
    /// 
    fn set_rotate(self, state: &mut State, value: f32) -> Entity {
        state.style.rotate.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    /// Translate the entity by an amount in (x, y)
    ///
    /// To position an entity, use the layout properties.
    fn set_translate(self, state: &mut State, value: (f32, f32)) -> Entity {
        state.style.translate.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_scale(self, state: &mut State, value: f32) -> Entity {
        state.style.scale.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the positon type of the entity.
    ///
    /// The position type determines whether an entity will be placed in a stack or grid with its siblings (`PositionType::ParentDirected`),
    /// or will ignore its siblings when positioned (`PositionType::SelfDirected`). A self-directed child in similar to absolute positioning in
    /// css but is relative to the parents top-left corner.
    ///
    /// # Example
    /// Set the entity to be self-directed, ignroing the size and positioning of its siblings:
    /// ```
    /// entity.set_position_type(state, PositionType::SelfDirected);
    /// ```
    ///
    /// # CSS
    /// ```css
    /// position-type: parent-directed (default) | self-directed  
    /// ```
    fn set_position_type(self, state: &mut State, value: PositionType) -> Entity {
        state.style.positioning_type.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the space on all sides of an entity.
    ///
    /// This is equivalent to setting the `left`, `right`, `top`, and `bottom` properties.
    /// Space can be specified as pixels, percentage, stretch, or auto, and can be thought of like adding margins. 
    /// Space is set to auto by default and so is controlled by the parent `child-space`.
    /// 
    ///
    /// Examples:
    /// Position a solo entity in the center of its parent by adding stretch space to all sides:
    /// ```
    /// entity.set_space(state, Stratch(1.0));
    /// ``` 
    /// 
    /// # CSS
    /// ```css
    /// space: {}px | {}% | {}s | auto
    /// ```
    fn set_space(self, state: &mut State, value: Units) -> Entity {
        state.style.left.insert(self.entity(), value);
        state.style.right.insert(self.entity(), value);
        state.style.top.insert(self.entity(), value);
        state.style.bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());
        self.entity()
    } 

    /// Set the space on the left side of an entity.
    ///
    ///
    ///
    /// # Examples
    /// Position an entity 5 pixels from the left edge of its parent
    /// ```
    /// entity.set_left(state, Pixels(5.0));
    /// ```
    /// 
    /// Center the entity horizontally by adding stretch space to the left and right sides. 
    /// ```
    /// entity.set_left(state, Stratch(1.0)).set_right(state, Stretch(1.0))
    /// ```
    fn set_left(self, state: &mut State, value: Units) -> Entity {
        state.style.left.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the space on the right side of an entity.
    ///
    /// For a fixed width entity (not stretch), left spacing will override right spacing when both in pixels.
    /// So if left is 5 px and right is 5 px, the entity will be positioned 5 pixels from the left edge.
    /// Set left to stretch to position from the right edge. 
    ///
    /// # Examples
    /// Position an entity 5 pixels from the right edge of its parent. Notice that left space must be set to stretch.
    /// ```
    /// entity.set_right(state, Pixels(5.0)).set_left(state, Stretch(1.0));
    /// ```
    /// 
    /// Center the entity horizontally by adding stretch space to the left and right sides. 
    /// ```
    /// entity.set_left(state, Stratch(1.0)).set_right(state, Stretch(1.0))
    /// ```
    fn set_right(self, state: &mut State, value: Units) -> Entity {
        state.style.right.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_top(self, state: &mut State, value: Units) -> Entity {
        state.style.top.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_bottom(self, state: &mut State, value: Units) -> Entity {
        state.style.bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Set the minimum space to the left of an entity.
    fn set_min_left(self, state: &mut State, value: Units) -> Entity {
        state.style.min_left.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Set the maximum space to the left of the entity.
    fn set_max_left(self, state: &mut State, value: Units) -> Entity {
        state.style.max_left.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Set the mimimum space to the right of the entity.
    fn set_min_right(self, state: &mut State, value: Units) -> Entity {
        state.style.min_right.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Set the maximum space to the right of the entity.
    fn set_max_right(self, state: &mut State, value: Units) -> Entity {
        state.style.max_right.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Set the mimimum space above the entity.
    fn set_min_top(self, state: &mut State, value: Units) -> Entity {
        state.style.min_top.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Set the maximum space above the entity.
    fn set_max_top(self, state: &mut State, value: Units) -> Entity {
        state.style.max_top.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Set the minimum space below the entity.
    fn set_min_bottom(self, state: &mut State, value: Units) -> Entity {
        state.style.min_bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Set the maximum space below the entity.
    fn set_max_bottom(self, state: &mut State, value: Units) -> Entity {
        state.style.max_bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Set the desired width of the entity.
    ///
    ///
    fn set_width(self, state: &mut State, value: Units) -> Entity {
        
        state.style.width.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the desired height of the entity.
    ///
    ///
    fn set_height(self, state: &mut State, value: Units) -> Entity {
        state.style.height.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    // Size Constraints
    fn set_min_width(self, state: &mut State, value: Units) -> Entity {
        state.style.min_width.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_max_width(self, state: &mut State, value: Units) -> Entity {
        state.style.max_width.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_min_height(self, state: &mut State, value: Units) -> Entity {
        state.style.min_height.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_max_height(self, state: &mut State, value: Units) -> Entity {
        state.style.max_height.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    /// Set text that will be displayed within the entity.
    ///
    /// Text within an entity can be positioned with the `child-space` propeties.
    ///
    /// # Example
    /// Set the entity to display the text `Hello World`.
    /// ```
    /// entity.set_text(state, "Hello World");
    /// ```
    fn set_text(self, state: &mut State, text: &str) -> Entity {
        state.style.text.insert(self.entity(), text.to_owned());

        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the font of the text displayed within the entity.
    /// 
    /// Fonts are identified by a string key which is specified when adding a font with `state.add_font_mem()`.
    /// There are 3 built-in fonts which can be used without having to add any font data:
    ///  1. `roboto` - Roboto-Regular.ttf (Default)
    ///  2. `roboto-bold` - Roboto-Bold.ttf
    ///  3. `icon` - entypo.ttf
    /// 
    /// # Example
    /// Sets the font to the icon font (entypo) for the text displayed within the entity:
    /// ```
    /// entity.set_font("icon");
    /// ```
    fn set_font(self, state: &mut State, font: &str) -> Entity {
        state.style.font.insert(self.entity(), font.to_owned());

        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the size of the font for the text displayed within the entity.
    ///
    /// # Example
    /// ```
    /// entity.set_font_size(state, 20.0)
    /// ```
    /// 
    /// # CSS
    /// ```css
    /// font-size: {} | {}px | {}% | xx-small | x-small | small | medium | large | x-large | xx-large
    /// ```
    fn set_font_size(self, state: &mut State, value: f32) -> Entity {
        state.style.font_size.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the font color for the text diaplyed within the entity.
    ///
    /// # Example
    /// Set the font color to red:
    /// ```
    /// entity.set_color(state, Color::red());
    /// ```
    /// 
    /// # CSS
    /// ```css
    /// color: color_name | #hex_code
    /// ```
    fn set_color(self, state: &mut State, value: Color) -> Entity {
        state.style.font_color.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    // Tooltip
    fn set_tooltip(self, state: &mut State, text: &str) -> Entity {
        state.style.tooltip.insert(self.entity(), text.to_owned());

        Entity::root().redraw(state);

        self.entity()
    }

    /// Sets the background color of the entity.
    ///
    /// Background color can be specified with an alpha component but the opacity property will apply as well.
    /// So a background color with an alpha of 0.5 and an opacity property value of 0.5 is equivalent to
    /// an entity with a background alpha of 0.25.
    /// Background color is overridden by background gradient.
    ///
    /// # Examples
    /// Set the background color of the entity to red:
    /// ```
    /// entity.set_background_color(Color::red());
    /// ```
    /// Set the background color of the entity with individual red, green, and blue components:
    /// ```
    /// entity.set_background_color(Color::rgb(255, 50, 50));
    /// ```
    ///
    /// # CSS
    /// ```css
    /// background-color: color_name | #hex_code
    /// ```
    fn set_background_color(self, state: &mut State, value: Color) -> Entity {
        state.style.background_color.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_background_gradient(self, state: &mut State, value: LinearGradient) -> Entity {
        state
            .style
            .background_gradient
            .insert(self.entity(), value);

        self.entity()
    }

    // TODO
    fn set_background_image(self, state: &mut State, value: Rc<()>) -> Entity {
        state.style.background_image.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the border width of the entity.
    ///
    /// The border width applies to all sides of the entity shape, including beveled and rounded corners.
    /// A border may not be visible after setting the width due to the default border color having 0 alpha.
    /// Border width uses the same units as size and space but only the pixels and percentage variants do anything.
    /// 
    ///
    /// # Example
    /// Set the border width of the entity to 2 pixels and set the border color to black:
    /// ```
    /// entity.set_border_width(state, Units::Pixels(2.0)).set_border_color(Color::black());
    /// ```
    /// 
    /// # CSS
    /// ```css
    /// border-width: {}px | {}%
    /// ```
    fn set_border_width(self, state: &mut State, value: Units) -> Entity {
        state.style.border_width.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the border color of the entity.
    ///
    /// By default the border-width is 0 pixels, so make sure to set both the color and width of the border to see a result.
    ///
    /// # Example
    /// Set the border width of the entity to 2 pixels and set the border color to black:
    /// ```
    /// entity.set_border_width(state, Units::Pixels(2.0)).set_border_color(Color::black());
    /// ```
    /// 
    /// # CSS
    /// ```css
    /// border-color: color_name | #hex_code
    /// ```
    fn set_border_color(self, state: &mut State, value: Color) -> Entity {
        state.style.border_color.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the corner shape of the entity for all four corners.
    ///
    /// Border corners can be rounded (`BorderCornerShape::Round`), which is the default, or bevelled (`BorderCornerShape::Bevel`).
    /// The corner shape will only be visible with a non-zero border-radius in the corresponding corner.
    ///
    /// # Example
    /// Sets the border corner shape to bevelled witn a radius of 10 pixels
    /// ```
    /// entity.set_border_corner_shape(state, BorderCornerShape::Bevel).set_border_radius(state, Pixels(10.0));
    /// ```
    /// 
    /// # CSS
    /// ```css
    /// border-corner-shape: round | bevel
    /// ```
    fn set_border_corner_shape(self, state: &mut State, value: BorderCornerShape) -> Entity {
        state.style.border_shape_top_left.insert(self.entity(), value);
        state.style.border_shape_top_right.insert(self.entity(), value);
        state.style.border_shape_bottom_left.insert(self.entity(), value);
        state.style.border_shape_bottom_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the border corner shape for the top left corner of the entity.
    ///
    fn set_border_top_left_shape(self, state: &mut State, value: BorderCornerShape) -> Entity {
        state.style.border_shape_top_left.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the border corner shape for the top right corner of the entity.
    ///
    fn set_border_top_right_shape(self, state: &mut State, value: BorderCornerShape) -> Entity {
        state.style.border_shape_top_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the border corner shape for the bottom left corner of the entity.
    ///
    fn set_border_bottom_left_shape(self, state: &mut State, value: BorderCornerShape) -> Entity {
        state.style.border_shape_bottom_left.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    /// Set the border corner shape for the bottom right corner of the entity.
    ///
    fn set_border_bottom_right_shape(self, state: &mut State, value: BorderCornerShape) -> Entity {
        state.style.border_shape_bottom_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }


    /// Set the border radius of the entity for all four corners.
    ///
    ///
    fn set_border_radius(self, state: &mut State, value: Units) -> Entity {
        state.style.border_radius_top_left.insert(self.entity(), value);
        state.style.border_radius_top_right.insert(self.entity(), value);
        state.style.border_radius_bottom_left.insert(self.entity(), value);
        state.style.border_radius_bottom_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }


    fn set_border_radius_top_left(self, state: &mut State, value: Units) -> Entity {
        state.style.border_radius_top_left.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_radius_top_right(self, state: &mut State, value: Units) -> Entity {
        state.style.border_radius_top_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_radius_bottom_left(self, state: &mut State, value: Units) -> Entity {
        state.style.border_radius_bottom_left.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_radius_bottom_right(self, state: &mut State, value: Units) -> Entity {
        state.style.border_radius_bottom_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    // Outer Shadow
    fn set_outer_shadow_h_offset(mut self, state: &mut State, value: Units) -> Self {
        state
            .style
            .outer_shadow_h_offset
            .insert(self.entity(), value);

        self
    }

    fn set_outer_shadow_v_offset(mut self, state: &mut State, value: Units) -> Self {
        state
            .style
            .outer_shadow_v_offset
            .insert(self.entity(), value);

        self
    }

    fn set_outer_shadow_color(mut self, state: &mut State, value: Color) -> Self {
        state.style.outer_shadow_color.insert(self.entity(), value);

        self
    }

    fn set_outer_shadow_blur(mut self, state: &mut State, value: Units) -> Self {
        state.style.outer_shadow_blur.insert(self.entity(), value);

        self
    }

    // Clipping
    fn set_clip_widget(self, state: &mut State, value: Entity) -> Entity {
        state.style.clip_widget.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_z_order(self, state: &mut State, value: i32) -> Entity {
        state.style.z_order.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_next_focus(self, state: &mut State, value: Entity) -> Entity {
        if let Some(entity) = state.style.focus_order.get_mut(self.entity()) {
            entity.next = value;
        } else {
            state.style.focus_order.insert(
                self.entity(),
                FocusOrder {
                    next: value,
                    ..Default::default()
                },
            );
        }

        self.entity()
    }

    fn set_prev_focus(self, state: &mut State, value: Entity) -> Entity {
        if let Some(focus_order) = state.style.focus_order.get_mut(self.entity()) {
            focus_order.prev = value;
        } else {
            state.style.focus_order.insert(
                self.entity(),
                FocusOrder {
                    prev: value,
                    ..Default::default()
                },
            );
        }

        self.entity()
    }

    fn set_focus_order(self, state: &mut State, prev: Entity, next: Entity) -> Entity {
        if let Some(focus_order) = state.style.focus_order.get_mut(self.entity()) {
            focus_order.prev = prev;
            focus_order.next = next;
        } else {
            state.style.focus_order.insert(
                self.entity(),
                FocusOrder {
                    prev,
                    next,
                },
            );
        }

        self.entity()
    }

    /// Set the layout type of the entity.
    ///
    /// Layout type determines how child entities which are parent-directed will be positioned.
    /// The layout type can be `row`, `column`, or `grid`.
    ///
    /// # Exmaples
    /// Position children into a vertical stack:
    /// ```
    /// entity.set_layout_type(state, LayoutType::Column);
    /// ```
    /// 
    /// # CSS
    /// ```css
    /// layout-type: row | column | grid
    /// ```
    fn set_layout_type(&self, state: &mut State, value: LayoutType) -> Entity {
        state.style.layout_type.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_child_space(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_left.insert(self.entity(), value);
        state.style.child_right.insert(self.entity(), value);
        state.style.child_top.insert(self.entity(), value);
        state.style.child_bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_child_left(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_left.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_row_between(&self, state: &mut State, value: Units) -> Entity {
        state.style.row_between.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_col_between(&self, state: &mut State, value: Units) -> Entity {
        state.style.col_between.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_child_right(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_right.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_child_top(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_top.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_child_bottom(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_grid_rows(&self, state: &mut State, value: Vec<Units>) -> Entity {
        state.style.grid_rows.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_grid_cols(&self, state: &mut State, value: Vec<Units>) -> Entity {
        state.style.grid_cols.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_row_index(&self, state: &mut State, value: usize) -> Entity {
        state.style.row_index.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_col_index(&self, state: &mut State, value: usize) -> Entity {
        state.style.col_index.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_row_span(&self, state: &mut State, value: usize) -> Entity {
        state.style.row_span.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_col_span(mut self, state: &mut State, value: usize) -> Self {
        state.style.col_span.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);
        
        self
    }

}

// Implement PropSet for all types which implement AsEntity
impl<T: AsEntity> PropSet for T {

}
pub trait PropGet: Sized + AsEntity {


    fn name(&self, state: &mut State) -> String {
        state.style.name.get(self.entity()).cloned().unwrap_or_default()
    }

    fn element(&self, state: &mut State) -> String {
        state.style.elements.get(self.entity()).cloned().unwrap_or_default()
    }

    fn is_disabled(self, state: &mut State) -> bool;
    fn is_checked(self, state: &mut State) -> bool;
    fn is_over(self, state: &mut State) -> bool;
    fn is_active(self, state: &mut State) -> bool;
    fn is_focused(self, state: &mut State) -> bool;
    fn is_selected(self, state: &mut State) -> bool;
    fn is_hovered(self, state: &mut State) -> bool;


    fn is_hoverable(self, state: &mut State) -> bool {
        state.data.get_hoverable(self.entity())
    }
    fn is_focusable(self, state: &mut State) -> bool {
        state.data.get_focusable(self.entity())
    }
    fn is_checkable(self, state: &mut State) -> bool {
        state.data.get_checkable(self.entity())
    }
    fn is_selectable(self, state: &mut State) -> bool {
        state.data.get_selectable(self.entity())
    }

    fn is_visible(self, state: &mut State) -> bool {
        state.data.get_visibility(self.entity()) == Visibility::Visible
    }

    //
    fn get_overflow(&self, state: &mut State) -> Overflow;

    // Display
    fn get_display(&self, state: &mut State) -> Display;

    fn get_layout_type(&self, state: &mut State) -> LayoutType {
        state
            .style
            .layout_type
            .get(self.entity())
            .cloned()
            .unwrap_or_default()
    }

    // Background Color
    fn get_background_color(&self, state: &mut State) -> Color {
        state.style.background_color.get(self.entity()).cloned().unwrap_or_default()
    }

    // Position
    fn get_left(&self, state: &mut State) -> Units;
    fn get_right(&self, state: &mut State) -> Units;
    fn get_top(&self, state: &mut State) -> Units;
    fn get_bottom(&self, state: &mut State) -> Units;

    // Size
    fn get_width(&self, state: &mut State) -> Units;
    fn get_height(&self, state: &mut State) -> Units;

    // Size Constraints
    fn get_min_width(&self, state: &mut State) -> Units;
    fn get_max_width(&self, state: &mut State) -> Units;
    fn get_min_height(&self, state: &mut State) -> Units;
    fn get_max_height(&self, state: &mut State) -> Units;

    // Border
    fn get_border_width(&self, state: &mut State) -> Units;

    // Tooltip
    fn get_tooltip(&self, state: &mut State) -> String;

    // Text
    fn get_text(&self, state: &mut State) -> String;
    fn get_font(&self, state: &mut State) -> String;
}

impl PropGet for Entity {
    fn is_disabled(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClass::DISABLED)
        } else {
            false
        }
    }
    fn is_hovered(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClass::HOVER)
        } else {
            false
        }
    }
    fn is_selected(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClass::SELECTED)
        } else {
            false
        }
    }
    fn is_checked(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClass::CHECKED)
        } else {
            false
        }
    }
    fn is_over(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClass::OVER)
        } else {
            false
        }
    }
    fn is_active(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClass::ACTIVE)
        } else {
            false
        }
    }
    fn is_focused(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClass::FOCUS)
        } else {
            false
        }
    }

    fn get_overflow(&self, state: &mut State) -> Overflow {
        state.style.overflow.get(*self).cloned().unwrap_or_default()
    }

    // Display
    fn get_display(&self, state: &mut State) -> Display {
        state.style.display.get(*self).cloned().unwrap_or_default()
    }

    // Position
    fn get_left(&self, state: &mut State) -> Units {
        state.style.left.get(*self).cloned().unwrap_or_default()
    }
    fn get_right(&self, state: &mut State) -> Units {
        state.style.right.get(*self).cloned().unwrap_or_default()
    }
    fn get_top(&self, state: &mut State) -> Units {
        state.style.top.get(*self).cloned().unwrap_or_default()
    }
    fn get_bottom(&self, state: &mut State) -> Units {
        state.style.bottom.get(*self).cloned().unwrap_or_default()
    }

    // Size
    fn get_width(&self, state: &mut State) -> Units {
        state.style.width.get(*self).cloned().unwrap_or_default()
    }

    fn get_height(&self, state: &mut State) -> Units {
        state.style.height.get(*self).cloned().unwrap_or_default()
    }

    // Size Constraints
    fn get_min_width(&self, state: &mut State) -> Units {
        state
            .style
            .min_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_max_width(&self, state: &mut State) -> Units {
        state
            .style
            .max_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_min_height(&self, state: &mut State) -> Units {
        state
            .style
            .min_height
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_max_height(&self, state: &mut State) -> Units {
        state
            .style
            .max_height
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Border
    fn get_border_width(&self, state: &mut State) -> Units {
        state
            .style
            .border_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Tooltip
    fn get_tooltip(&self, state: &mut State) -> String {
        state.style.tooltip.get(*self).cloned().unwrap_or_default()
    }

    // Text
    fn get_text(&self, state: &mut State) -> String {
        state.style.text.get(*self).cloned().unwrap_or_default()
    }

    fn get_font(&self, state: &mut State) -> String {
        state.style.font.get(*self).cloned().unwrap_or_default()
    }
}
