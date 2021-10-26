
use super::Property;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StyleRule {
    pub(crate) id: Rule,
    pub(crate) selectors: Vec<Selector>,
    pub(crate) properties: Vec<Property>,
}

// impl std::fmt::Display for StyleRule {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for selector in self.selectors.iter() {
//             write!(f, "{}", selector)?;
//         }

//         write!(f, " {{\n")?;

//         for property in self.properties.iter() {
//             write!(f, "    {}\n", property)?;
//         }

//         write!(f, "}}\n\n")?;

//         Ok(())
//     }
// }

impl StyleRule {
    pub(crate) fn specificity(&self) -> Specificity {
        let mut specificity = Specificity([0, 0, 0]);
        for selector in self.selectors.iter() {
            specificity += selector.specificity();
        }

        return specificity;
    }
}

/*
impl StyleRule {
    pub fn new(id: Rule) -> Self {
        StyleRule {
            id,
            selectors: Vec::new(),
            properties: Vec::new(),
        }
    }

    pub fn selector(mut self, selector: Selector) -> Self {
        self.selectors.push(selector);

        self
    }

    pub fn parent_selector(mut self, mut selector: Selector) -> Self {
        selector.relation = SelectorRelation::Parent;
        self.selectors.push(selector);

        self
    }

    // pub fn property(mut self, property: Property) -> Self {
    //     self.properties.push(property);

    //     self
    // }

    pub(crate) fn specificity(&self) -> Specificity {
        let mut specificity = Specificity([0, 0, 0]);
        for selector in self.selectors.iter() {
            specificity += selector.specificity();
        }

        return specificity;
    }

    // Property Setters

    pub fn set_display(mut self, value: Display) -> Self {
        self.properties.push(Property::Display(value));

        self
    }

    pub fn set_visibility(mut self, value: Visibility) -> Self {
        self.properties.push(Property::Visibility(value));

        self
    }

    pub fn set_overflow(mut self, value: Overflow) -> Self {
        self.properties.push(Property::Overflow(value));

        self
    }

    // Background
    pub fn set_background_color(mut self, value: Color) -> Self {
        self.properties.push(Property::BackgroundColor(value));

        self
    }

    pub fn set_background_gradient(mut self, value: LinearGradient) -> Self {
        self.properties.push(Property::BackgroundGradient(value));

        self
    }

    // Outer Shadow
    pub fn set_outer_shadow_h_offset(mut self, value: Units) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.horizontal_offset = value;

        self.properties.push(Property::OuterShadow(box_shadow));

        self
    }

    pub fn set_outer_shadow_v_offset(mut self, value: Units) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.vertical_offset = value;

        self.properties.push(Property::OuterShadow(box_shadow));

        self
    }

    pub fn set_outer_shadow_color(mut self, value: Color) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.color = value;

        self.properties.push(Property::OuterShadow(box_shadow));

        self
    }

    pub fn set_outer_shadow_blur(mut self, value: Units) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.blur_radius = value;

        self.properties.push(Property::OuterShadow(box_shadow));

        self
    }

    // Inner Shadow
    pub fn set_inner_shadow_h_offset(mut self, value: Units) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.horizontal_offset = value;

        self.properties.push(Property::InnerShadow(box_shadow));

        self
    }

    pub fn set_inner_shadow_v_offset(mut self, value: Units) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.vertical_offset = value;

        self.properties.push(Property::InnerShadow(box_shadow));

        self
    }

    pub fn set_inner_shadow_color(mut self, value: Color) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.color = value;

        self.properties.push(Property::InnerShadow(box_shadow));

        self
    }

    pub fn set_inner_shadow_blur(mut self, value: Units) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.blur_radius = value;

        self.properties.push(Property::InnerShadow(box_shadow));

        self
    }

    // Positioning

    pub fn set_space(mut self, value: Units) -> Self {
        self.properties.push(Property::Space(value));

        self
    }

    pub fn set_left(mut self, value: Units) -> Self {
        self.properties.push(Property::Left(value));

        self
    }

    pub fn set_right(mut self, value: Units) -> Self {
        self.properties.push(Property::Right(value));

        self
    }

    pub fn set_top(mut self, value: Units) -> Self {
        self.properties.push(Property::Top(value));
        self
    }

    pub fn set_bottom(mut self, value: Units) -> Self {
        self.properties.push(Property::Bottom(value));
        self
    }

    // Alignment and Justification

    // pub fn set_justification(mut self, val: Justification) -> Self {
    //     self.state.style.justification.set(self.entity, val);
    //     self
    // }

    // pub fn set_alignment(mut self, val: Alignment) -> Self {
    //     self.state.style.alignment.set(self.entity, val);
    //     self
    // }

    // Size

    pub fn set_width(mut self, value: Units) -> Self {
        self.properties.push(Property::Width(value));

        self
    }

    pub fn set_height(mut self, value: Units) -> Self {
        self.properties.push(Property::Height(value));

        self
    }

    // Size Constraints

    pub fn set_min_width(mut self, value: Units) -> Self {
        self.properties.push(Property::MinHeight(value));

        self
    }

    pub fn set_max_width(mut self, value: Units) -> Self {
        self.properties.push(Property::MaxWidth(value));

        self
    }

    pub fn set_min_height(mut self, value: Units) -> Self {
        self.properties.push(Property::MinHeight(value));

        self
    }

    pub fn set_max_height(mut self, value: Units) -> Self {
        self.properties.push(Property::MaxHeight(value));

        self
    }

    // Child Spacing
    pub fn set_child_space(mut self, value: Units) -> Self {
        self.properties.push(Property::ChildSpace(value));

        self
    }

    pub fn set_child_left(mut self, value: Units) -> Self {
        self.properties.push(Property::ChildLeft(value));

        self
    }

    pub fn set_child_right(mut self, value: Units) -> Self {
        self.properties.push(Property::ChildRight(value));

        self
    }

    pub fn set_child_top(mut self, value: Units) -> Self {
        self.properties.push(Property::ChildTop(value));

        self
    }

    pub fn set_child_bottom(mut self, value: Units) -> Self {
        self.properties.push(Property::ChildBottom(value));

        self
    }

    // Border

    pub fn set_border_color(mut self, value: Color) -> Self {
        self.properties.push(Property::BorderColor(value));

        self
    }

    pub fn set_border_width(mut self, value: Units) -> Self {
        self.properties.push(Property::BorderWidth(value));

        self
    }

    pub fn set_border_radius(mut self, value: Units) -> Self {
        self.properties.push(Property::BorderTopLeftRadius(value));

        self
    }

    pub fn set_border_radius_top_left(mut self, value: Units) -> Self {
        self.properties.push(Property::BorderTopLeftRadius(value));

        self
    }

    pub fn set_border_radius_top_right(mut self, value: Units) -> Self {
        self.properties.push(Property::BorderTopRightRadius(value));

        self
    }

    pub fn set_border_radius_bottom_left(mut self, value: Units) -> Self {
        self.properties
            .push(Property::BorderBottomLeftRadius(value));

        self
    }

    pub fn set_border_radius_bottom_right(mut self, value: Units) -> Self {
        self.properties
            .push(Property::BorderBottomRightRadius(value));

        self
    }

    pub fn set_color(mut self, value: Color) -> Self {
        self.properties.push(Property::FontColor(value));

        self
    }

    pub fn set_font_size(mut self, value: f32) -> Self {
        self.properties.push(Property::FontSize(value));

        self
    }
}
*/