use crate::prelude::*;
use crate::style::{Rule, Selector, SelectorRelation, StyleRule};
use vizia_id::GenerationalId;
use vizia_storage::{LayoutTreeIterator, TreeExt};

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

fn link_style_data(cx: &mut Context, entity: Entity, matched_rules: &Vec<Rule>) {
    let mut should_relayout = false;
    let mut should_redraw = false;

    // Display
    if cx.style.display.link(entity, &matched_rules) {
        println!("1");
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.visibility.link(entity, &matched_rules) {
        println!("2");
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.z_order.link(entity, &matched_rules) {
        println!("3");
        should_redraw = true;
    }

    if cx.style.overflow.link(entity, &matched_rules) {
        should_redraw = true;
    }

    // Opacity
    if cx.style.opacity.link(entity, &matched_rules) {
        println!("4");
        should_redraw = true;
    }

    if cx.style.left.link(entity, &matched_rules) {
        println!("5");
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.right.link(entity, &matched_rules) {
        println!("6");
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.top.link(entity, &matched_rules) {
        println!("7");
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.bottom.link(entity, &matched_rules) {
        println!("8");
        should_relayout = true;
        should_redraw = true;
    }

    // Size
    if cx.style.width.link(entity, &matched_rules) {
        println!("9");
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.height.link(entity, &matched_rules) {
        println!("10");
        should_relayout = true;
        should_redraw = true;
    }

    // Size Constraints
    if cx.style.max_width.link(entity, &matched_rules) {
        println!("11");
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.min_width.link(entity, &matched_rules) {
        println!("13");
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.max_height.link(entity, &matched_rules) {
        println!("14");
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.min_height.link(entity, &matched_rules) {
        println!("15");
        should_relayout = true;
        should_redraw = true;
    }

    // Border
    if cx.style.border_width.link(entity, &matched_rules) {
        println!("24");
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.border_color.link(entity, &matched_rules) {
        println!("25");
        should_redraw = true;
    }

    if cx.style.border_shape_top_left.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_shape_top_right.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_shape_bottom_left.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_shape_bottom_right.link(entity, &matched_rules) {
        should_redraw = true;
    }

    if cx.style.border_radius_top_left.link(entity, &matched_rules) {
        println!("26");
        should_redraw = true;
    }

    if cx.style.border_radius_top_right.link(entity, &matched_rules) {
        println!("27");
        should_redraw = true;
    }

    if cx.style.border_radius_bottom_left.link(entity, &matched_rules) {
        println!("28");
        should_redraw = true;
    }

    if cx.style.border_radius_bottom_right.link(entity, &matched_rules) {
        println!("29");
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
        println!("30");
        should_relayout = true;
        should_redraw = true;
    }

    if cx.style.position_type.link(entity, &matched_rules) {
        println!("30");
        should_relayout = true;
        should_redraw = true;
    }

    // Background
    if cx.style.background_color.link(entity, &matched_rules) {
        println!("41");
        should_redraw = true;
    }

    if cx.style.background_image.link(entity, &matched_rules) {
        println!("42");
        should_redraw = true;
    }

    // Font
    if cx.style.font_color.link(entity, &matched_rules) {
        println!("43");
        should_redraw = true;
    }

    if cx.style.font_size.link(entity, &matched_rules) {
        println!("44");
        should_redraw = true;
        should_relayout = true;
    }

    if cx.style.font_family.link(entity, &matched_rules) {
        println!("44");
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
        println!("45");
        should_redraw = true;
    }

    if cx.style.outer_shadow_v_offset.link(entity, &matched_rules) {
        println!("46");
        should_redraw = true;
    }

    if cx.style.outer_shadow_blur.link(entity, &matched_rules) {
        println!("47");
        should_redraw = true;
    }

    if cx.style.outer_shadow_color.link(entity, &matched_rules) {
        println!("48");
        should_redraw = true;
    }

    // Inner Shadow
    if cx.style.inner_shadow_h_offset.link(entity, &matched_rules) {
        println!("45");
        should_redraw = true;
    }

    if cx.style.inner_shadow_v_offset.link(entity, &matched_rules) {
        println!("46");
        should_redraw = true;
    }

    if cx.style.inner_shadow_blur.link(entity, &matched_rules) {
        println!("47");
        should_redraw = true;
    }

    if cx.style.inner_shadow_color.link(entity, &matched_rules) {
        println!("48");
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

    if should_relayout {
        println!("should relayout");
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

        let mut matched_rule_ids = Vec::with_capacity(100);
        let mut prev_matched_rule_ids = Vec::with_capacity(100);

        let iterator = LayoutTreeIterator::full(tree);

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
            compute_matched_rules(cx, tree, entity, &mut matched_rules);
            matched_rule_ids.extend(matched_rules.into_iter().map(|r| r.id));
            link_style_data(cx, entity, &matched_rule_ids);

            prev_matched_rule_ids.clear();
            prev_matched_rule_ids.append(&mut matched_rule_ids);
        }

        cx.style.needs_restyle = false;
    }
}
