use crate::prelude::*;

use crate::style::SystemFlags;
use crate::style::{Abilities, PseudoClass, Rule, Selector, SelectorRelation, StyleRule};
use vizia_id::GenerationalId;
use vizia_storage::{DrawIterator, LayoutTreeIterator, TreeExt};

pub fn inline_inheritance_system(cx: &mut Context) {
    for entity in cx.tree.into_iter() {
        if let Some(parent) = cx.tree.get_layout_parent(entity) {
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

pub fn shared_inheritance_system(cx: &mut Context) {
    for entity in cx.tree.into_iter() {
        if let Some(parent) = cx.tree.get_layout_parent(entity) {
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

pub fn hoverability_system(cx: &mut Context) {
    let draw_tree = DrawIterator::full(&cx.tree);

    for entity in draw_tree {
        if entity == Entity::root() {
            continue;
        }

        let parent = cx.tree.get_layout_parent(entity).unwrap();

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

// Returns the selector of an entity
#[allow(unused)] // can be used for a potential optimization where styling is cached between
                 // similar siblings. needs some work.
fn entity_selector(cx: &Context, entity: Entity) -> Selector {
    Selector {
        asterisk: false,
        id: cx.style.ids.get(entity).cloned(),
        element: cx
            .views
            .get(&entity)
            .and_then(|view| view.element())
            .map(|element| element.to_owned()),
        classes: cx.style.classes.get(entity).cloned().unwrap_or_default(),
        pseudo_classes: {
            let mut pseudo_classes =
                cx.style.pseudo_classes.get(entity).cloned().unwrap_or_default();
            if let Some(disabled) = cx.style.disabled.get(entity) {
                pseudo_classes.set(PseudoClass::DISABLED, *disabled);
            }
            pseudo_classes
        },
        relation: SelectorRelation::None,
    }
}

// Returns true if the widget matches the selector
fn check_match(cx: &Context, entity: Entity, selector: &Selector) -> bool {
    // Universal selector always matches
    if selector.asterisk {
        if let Some(mut pseudo_classes) = cx.style.pseudo_classes.get(entity).cloned() {
            if let Some(disabled) = cx.style.disabled.get(entity) {
                pseudo_classes.set(PseudoClass::DISABLED, *disabled);
            }
            let selector_pseudo_classes = selector.pseudo_classes;
            if !pseudo_classes.is_empty() && !pseudo_classes.contains(selector_pseudo_classes) {
                return false;
            } else {
                return true;
            }
        } else {
            return true;
        }
    }

    // If there's an id in the selector, it must match the entity's id
    if let Some(id) = &selector.id {
        if Some(id) != cx.style.ids.get(entity) {
            return false;
        }
    }

    // Check for element name match
    if let Some(selector_element) = &selector.element {
        if let Some(element) = cx.views.get(&entity).and_then(|view| view.element()) {
            if selector_element != &element {
                return false;
            }
        } else if entity == Entity::root() {
            if selector_element != "root" {
                return false;
            }
        } else {
            return false;
        }
    }

    // Check for classes match
    if let Some(classes) = cx.style.classes.get(entity) {
        if !selector.classes.is_subset(classes) {
            return false;
        }
    } else if !selector.classes.is_empty() {
        return false;
    }

    // Check for pseudo-class match
    if let Some(mut pseudo_classes) = cx.style.pseudo_classes.get(entity).cloned() {
        if let Some(disabled) = cx.style.disabled.get(entity) {
            pseudo_classes.set(PseudoClass::DISABLED, *disabled);
        }
        let selector_pseudo_classes = selector.pseudo_classes;

        if !selector_pseudo_classes.is_empty() && !pseudo_classes.contains(selector_pseudo_classes)
        {
            return false;
        }
    }

    return true;
}

pub(crate) fn compute_matched_rules<'a>(
    cx: &'a Context,
    tree: &Tree<Entity>,
    entity: Entity,
    matched_rules: &mut Vec<&'a StyleRule>,
) {
    // Loop through all of the style rules
    'rule_loop: for rule in cx.style.rules.iter() {
        let mut relation_entity = entity;
        // Loop through selectors (Should be from right to left)
        // All the selectors need to match for the rule to apply
        'selector_loop: for rule_selector in rule.selectors.iter().rev() {
            // Get the relation of the selector
            match rule_selector.relation {
                SelectorRelation::None => {
                    if !check_match(cx, entity, rule_selector) {
                        continue 'rule_loop;
                    }
                }

                SelectorRelation::Parent => {
                    // Get the parent
                    // Contrust the selector for the parent
                    // Check if the parent selector matches the rule_seletor
                    if let Some(parent) = tree.get_layout_parent(relation_entity) {
                        if !check_match(cx, parent, rule_selector) {
                            continue 'rule_loop;
                        }

                        relation_entity = parent;
                    } else {
                        continue 'rule_loop;
                    }
                }

                SelectorRelation::Ancestor => {
                    // Walk up the tree
                    // Check if each entity matches the selector
                    // If any of them match, move on to the next selector
                    // If none of them do, move on to the next rule
                    for ancestor in relation_entity.parent_iter(tree) {
                        if ancestor == relation_entity {
                            continue;
                        }
                        if tree.is_ignored(ancestor) {
                            continue;
                        }

                        if check_match(cx, ancestor, rule_selector) {
                            relation_entity = ancestor;

                            continue 'selector_loop;
                        }
                    }

                    continue 'rule_loop;
                }
            }
        }

        // If all the selectors match then add the rule to the matched rules list
        matched_rules.push(rule);
    }
}

fn link_style_data(style: &mut Style, entity: Entity, matched_rules: &Vec<Rule>) {
    let mut should_relayout = false;
    let mut should_redraw = false;
    let mut should_reorder = false;
    let mut should_reclip = false;
    let mut should_rehide = false;

    // Display
    if style.display.link(entity, &matched_rules) {
        should_rehide = true;
    }

    if style.visibility.link(entity, &matched_rules) {
        should_rehide = true;
    }

    if style.z_order.link(entity, &matched_rules) {
        should_reorder = true;
    }

    if style.overflow.link(entity, &matched_rules) {
        should_reclip = true;
    }

    // Opacity
    if style.opacity.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.left.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.right.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.top.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.bottom.link(entity, &matched_rules) {
        should_relayout = true;
    }

    // Size
    if style.width.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.height.link(entity, &matched_rules) {
        should_relayout = true;
    }

    // Size Constraints
    if style.max_width.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.min_width.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.max_height.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.min_height.link(entity, &matched_rules) {
        should_relayout = true;
    }

    // Border
    if style.border_width.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.border_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_shape_top_left.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_shape_top_right.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_shape_bottom_left.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_shape_bottom_right.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_radius_top_left.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_radius_top_right.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_radius_bottom_left.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.border_radius_bottom_right.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.outline_width.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.outline_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.outline_offset.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.layout_type.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.position_type.link(entity, &matched_rules) {
        should_relayout = true;
    }

    // Background
    if style.background_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.background_image.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Font
    if style.font_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.font_size.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.font_family.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.font_weight.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.font_style.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.text_wrap.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.selection_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.caret_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Outer Shadow
    if style.outer_shadow_h_offset.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.outer_shadow_v_offset.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.outer_shadow_blur.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.outer_shadow_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Inner Shadow
    if style.inner_shadow_h_offset.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.inner_shadow_v_offset.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.inner_shadow_blur.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.inner_shadow_color.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if style.child_left.link(entity, &matched_rules) {
        should_relayout = true;
        should_redraw = true;
    }

    if style.child_right.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.child_top.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.child_bottom.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.row_between.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.col_between.link(entity, &matched_rules) {
        should_relayout = true;
    }

    if style.cursor.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if should_relayout {
        style.system_flags.set(SystemFlags::RELAYOUT, true);
    }

    if should_redraw {
        style.system_flags.set(SystemFlags::REDRAW, true);
    }

    if should_reorder {
        style.system_flags.set(SystemFlags::REORDER, true);
    }

    if should_reclip {
        style.system_flags.set(SystemFlags::RECLIP, true);
    }

    if should_rehide {
        style.system_flags.set(SystemFlags::REHIDE, true);
    }
}

// Iterate tree and determine the matched style rules for each entity. Link the entity to the style data.
pub fn style_system(cx: &mut Context) {
    if cx.style.system_flags.contains(SystemFlags::RESTYLE) {
        hoverability_system(cx);

        let mut matched_rule_ids = Vec::with_capacity(100);
        let mut prev_matched_rule_ids = Vec::with_capacity(100);

        let iterator = LayoutTreeIterator::full(&cx.tree);

        // Loop through all entities
        for entity in iterator {
            // If the entity and the previous entity have the same parent and selectors then they share the same rules
            //if let Some(prev) = prev_entity {
            //    if let Some(parent) = tree.get_layout_parent(entity) {
            //        if let Some(prev_parent) = tree.get_layout_parent(prev) {
            //            if parent == prev_parent {
            //                if entity_selector(cx, entity).same(&entity_selector(cx, prev)) {
            //                    prev_entity = Some(entity);
            //                    link_style_data(cx, entity, &prev_matched_rule_ids);
            //                    continue 'ent;
            //                }
            //            }
            //        }
            //    }
            //}

            let mut matched_rules = Vec::with_capacity(100);
            compute_matched_rules(cx, &cx.tree, entity, &mut matched_rules);
            matched_rule_ids.extend(matched_rules.into_iter().map(|r| r.id));
            link_style_data(&mut cx.style, entity, &matched_rule_ids);

            prev_matched_rule_ids.clear();
            prev_matched_rule_ids.append(&mut matched_rule_ids);
        }

        cx.style.system_flags.set(SystemFlags::RESTYLE, false);
    }
}
