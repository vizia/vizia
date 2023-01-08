use crate::{
    events::ViewHandler,
    prelude::*,
    style::{Rule, Style},
};
use fnv::FnvHashMap;
// use crate::style::{Rule, Selector, SelectorRelation, StyleRule};
use vizia_id::GenerationalId;
use vizia_storage::{LayoutTreeIterator, TreeExt};
use vizia_style::{
    matches_selector_list,
    selectors::{
        attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint},
        SelectorImpl,
    },
    selectors::{matching::ElementSelectorFlags, OpaqueElement},
    Element, MatchingContext, MatchingMode, PseudoClass, QuirksMode, SelectorIdent, Selectors,
};

#[derive(Clone)]
pub struct Node<'s, 't, 'v> {
    entity: Entity,
    store: &'s Style,
    tree: &'t Tree<Entity>,
    views: &'v FnvHashMap<Entity, Box<dyn ViewHandler>>,
}

impl<'s, 't, 'v> std::fmt::Debug for Node<'s, 't, 'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.entity)
    }
}

impl<'s, 't, 'v> Element for Node<'s, 't, 'v> {
    type Impl = Selectors;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(self)
    }

    fn is_html_slot_element(&self) -> bool {
        false
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn parent_element(&self) -> Option<Self> {
        self.tree.get_parent(self.entity).map(|parent| Node {
            entity: parent,
            store: self.store,
            tree: self.tree,
            views: self.views,
        })
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.tree.get_prev_sibling(self.entity).map(|parent| Node {
            entity: parent,
            store: self.store,
            tree: self.tree,
            views: self.views,
        })
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.tree.get_next_sibling(self.entity).map(|parent| Node {
            entity: parent,
            store: self.store,
            tree: self.tree,
            views: self.views,
        })
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn is_root(&self) -> bool {
        self.entity == Entity::root()
    }

    fn is_html_element_in_html_document(&self) -> bool {
        false
    }

    fn has_local_name(&self, local_name: &SelectorIdent) -> bool {
        if let Some(element) = self.views.get(&self.entity).and_then(|view| view.element()) {
            return element == &local_name.0;
        }

        false
    }

    fn has_namespace(&self, ns: &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl) -> bool {
        false
    }

    fn is_part(&self, name: &<Self::Impl as SelectorImpl>::Identifier) -> bool {
        false
    }

    fn imported_part(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
    ) -> Option<<Self::Impl as SelectorImpl>::Identifier> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn is_same_type(&self, other: &Self) -> bool {
        self.store.elements.get(self.entity) == other.store.elements.get(self.entity)
    }

    fn is_link(&self) -> bool {
        false
    }

    fn has_id(
        &self,
        id: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: CaseSensitivity,
    ) -> bool {
        false
    }

    fn has_class(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: CaseSensitivity,
    ) -> bool {
        if let Some(classes) = self.store.classes.get(self.entity) {
            return classes.contains(&name.0);
        }

        false
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>,
        local_name: &<Self::Impl as SelectorImpl>::LocalName,
        operation: &AttrSelectorOperation<&<Self::Impl as SelectorImpl>::AttrValue>,
    ) -> bool {
        false
    }

    fn match_pseudo_element(
        &self,
        pe: &<Self::Impl as SelectorImpl>::PseudoElement,
        context: &mut MatchingContext<'_, Self::Impl>,
    ) -> bool {
        false
    }

    fn match_non_ts_pseudo_class<F>(
        &self,
        pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass,
        context: &mut MatchingContext<'_, Self::Impl>,
        flags_setter: &mut F,
    ) -> bool
    where
        F: FnMut(&Self, ElementSelectorFlags),
    {
        if let Some(psudeo_class_flag) = self.store.pseudo_classes.get(self.entity) {
            match pc {
                PseudoClass::Hover => psudeo_class_flag.contains(PseudoClassFlags::HOVER),
                PseudoClass::Active => psudeo_class_flag.contains(PseudoClassFlags::ACTIVE),
                PseudoClass::Over => psudeo_class_flag.contains(PseudoClassFlags::OVER),
                PseudoClass::Focus => psudeo_class_flag.contains(PseudoClassFlags::FOCUS),
                PseudoClass::FocusVisible => {
                    psudeo_class_flag.contains(PseudoClassFlags::FOCUS_VISIBLE)
                }
                PseudoClass::FocusWithin => todo!(),
                PseudoClass::Enabled => todo!(),
                PseudoClass::Disabled => {
                    self.store.disabled.get(self.entity).copied().unwrap_or_default()
                }
                PseudoClass::ReadOnly => todo!(),
                PseudoClass::ReadWrite => todo!(),
                PseudoClass::PlaceHolderShown => todo!(),
                PseudoClass::Default => todo!(),
                PseudoClass::Checked => psudeo_class_flag.contains(PseudoClassFlags::CHECKED),
                PseudoClass::Indeterminate => todo!(),
                PseudoClass::Blank => todo!(),
                PseudoClass::Valid => todo!(),
                PseudoClass::Invalid => todo!(),
                PseudoClass::InRange => todo!(),
                PseudoClass::OutOfRange => todo!(),
                PseudoClass::Required => todo!(),
                PseudoClass::Optional => todo!(),
                PseudoClass::UserValid => todo!(),
                PseudoClass::UserInvalid => todo!(),
                PseudoClass::Lang(_) => todo!(),
                PseudoClass::Dir(_) => todo!(),
                PseudoClass::Custom(name) => {
                    println!("{}", name);
                    todo!()
                }
            }
        } else {
            false
        }
    }
}

pub fn inline_inheritance_system(cx: &mut Context, tree: &Tree<Entity>) {
    for entity in tree.into_iter() {
        if let Some(parent) = tree.get_layout_parent(entity) {
            cx.style.disabled.inherit_inline(entity, parent);

            cx.style.font_color.inherit_inline(entity, parent);
            cx.style.font_size.inherit_inline(entity, parent);
            cx.style.font_family.inherit_inline(entity, parent);
            cx.style.font_weight.inherit_inline(entity, parent);
            cx.style.font_style.inherit_inline(entity, parent);
            cx.style.caret_color.inherit_inline(entity, parent);
            cx.style.selection_color.inherit_inline(entity, parent);
        }
    }
}

pub fn shared_inheritance_system(cx: &mut Context, tree: &Tree<Entity>) {
    for entity in tree.into_iter() {
        if let Some(parent) = tree.get_layout_parent(entity) {
            cx.style.font_color.inherit_shared(entity, parent);
            cx.style.font_size.inherit_shared(entity, parent);
            cx.style.font_family.inherit_shared(entity, parent);
            cx.style.font_weight.inherit_shared(entity, parent);
            cx.style.font_style.inherit_shared(entity, parent);
            cx.style.caret_color.inherit_shared(entity, parent);
            cx.style.selection_color.inherit_shared(entity, parent);
        }
    }
}

pub fn hoverability_system(cx: &mut Context, tree: &Tree<Entity>) {
    let mut draw_tree: Vec<Entity> = tree.into_iter().collect();
    draw_tree.sort_by_cached_key(|entity| cx.cache.get_z_index(*entity));

    for entity in draw_tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        if tree.is_ignored(entity) {
            continue;
        }

        let parent = tree.get_layout_parent(entity).unwrap();

        if !cx.cache.get_hoverability(parent) {
            cx.cache.set_hoverability(entity, false);
        } else {
            if let Some(abilities) = cx.style.abilities.get(entity) {
                cx.cache.set_hoverability(entity, abilities.contains(Abilities::HOVERABLE));
            } else {
                cx.cache.set_hoverability(entity, false);
            }
        }
    }
}

fn link_style_data(cx: &mut Context, entity: Entity, matched_rules: &Vec<Rule>) {
    let mut should_relayout = false;
    let mut should_redraw = false;

    // Display
    if cx.style.display.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.visibility.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.z_index.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.overflow.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Opacity
    if cx.style.opacity.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.left.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.right.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.top.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.bottom.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Size
    if cx.style.width.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.height.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Size Constraints
    if cx.style.max_width.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.min_width.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.max_height.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.min_height.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Border
    if cx.style.border_width.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.border_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_top_left_shape.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_top_right_shape.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_bottom_left_shape.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_bottom_right_shape.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_top_left_radius.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_top_right_radius.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_bottom_left_radius.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_bottom_right_radius.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.outline_width.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.outline_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.outline_offset.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.layout_type.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.position_type.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    // Background
    if cx.style.background_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.background_image.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Font
    if cx.style.font_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.font_size.link(entity, &matched_rules) {
        should_redraw = true;
        should_relayout = true;
    }

    if cx.style.font_family.link(entity, &matched_rules) {
        //println!("44");
        should_redraw = true;
        should_relayout = true;
    }

    if cx.style.font_weight.link(entity, &matched_rules) {
        should_redraw = true;
        should_relayout = true;
    }

    if cx.style.font_style.link(entity, &matched_rules) {
        should_redraw = true;
        should_relayout = true;
    }

    if cx.style.text_wrap.link(entity, &matched_rules) {
        should_redraw = true;
        should_relayout = true;
    }

    if cx.style.selection_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.caret_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Outer Shadow
    if cx.style.outer_shadow_h_offset.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.outer_shadow_v_offset.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.outer_shadow_blur.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.outer_shadow_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Inner Shadow
    if cx.style.inner_shadow_h_offset.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.inner_shadow_v_offset.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.inner_shadow_blur.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.inner_shadow_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.child_left.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.child_right.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.child_top.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.child_bottom.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.row_between.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.col_between.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.cursor.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.transform.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if should_relayout {
        cx.style.needs_relayout = true;
    }

    if should_redraw {
        cx.style.needs_redraw = true;
    }
}

// Iterate tree and determine the matched style rules for each entity. Link the entity to the style data.
pub fn style_system(cx: &mut Context, tree: &Tree<Entity>) {
    if cx.style.needs_restyle {
        hoverability_system(cx, tree);

        // let mut prev_entity = None;

        // let mut matched_rule_ids = Vec::with_capacity(100);
        // let mut prev_matched_rule_ids = Vec::with_capacity(100);

        let iterator = LayoutTreeIterator::full(tree);

        // Loop through all entities
        'ent: for entity in iterator {
            let element_name = cx.views.get(&entity).and_then(|view| view.element());
            // If the entity and the previous entity have the same parent and selectors then they share the same rules
            // if let Some(prev) = prev_entity {
            //     if let Some(parent) = tree.get_layout_parent(entity) {
            //         if let Some(prev_parent) = tree.get_layout_parent(prev) {
            //             if parent == prev_parent {
            //                 if entity_selector(cx, entity).same(&entity_selector(cx, prev)) {
            //                     prev_entity = Some(entity);
            //                     link_style_data(cx, entity, &prev_matched_rule_ids);
            //                     continue 'ent;
            //                 }
            //             }
            //         }
            //     }
            // }

            let mut matched_rules = Vec::with_capacity(100);
            for (rule, (specificity, selector_list)) in cx.style.selectors.iter() {
                // println!("selector_list: {:?} {}", selector_list, entity);
                let mut context =
                    MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks);
                if matches_selector_list(
                    selector_list,
                    &Node { entity, store: &cx.style, tree: &cx.tree, views: &cx.views },
                    &mut context,
                ) {
                    matched_rules.push((*rule, specificity));
                }
            }

            matched_rules.sort_by_cached_key(|(_, s)| *s);
            matched_rules.reverse();

            println!("Matched rules: {:?}", matched_rules);
            //compute_matched_rules(cx, tree, entity, &mut matched_rules);
            //matched_rule_ids.extend(matched_rules.into_iter());
            link_style_data(
                cx,
                entity,
                &matched_rules.iter().map(|(rule, _)| *rule).collect::<Vec<_>>(),
            );

            // prev_entity = Some(entity);
            // prev_matched_rule_ids.clear();
            // prev_matched_rule_ids.append(&mut matched_rule_ids);
        }

        cx.style.needs_restyle = false;
    }
}
