use femtovg::{Align, Baseline, Paint};
use morphorm::Units;

use crate::{
    style::{Overflow, Selector, SelectorRelation},
    BoundingBox, Context, Display, Entity, FontOrId, PseudoClass, Rule, Tree, TreeExt, Visibility,
};

pub fn apply_z_ordering(cx: &mut Context, tree: &Tree) {
    for entity in tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        if tree.is_ignored(entity) {
            continue;
        }

        let parent = tree.get_layout_parent(entity).unwrap();

        if let Some(z_order) = cx.style.z_order.get(entity) {
            cx.cache.set_z_index(entity, *z_order);
        } else {
            let parent_z_order = cx.cache.get_z_index(parent);
            cx.cache.set_z_index(entity, parent_z_order);
        }
    }
}

pub fn apply_clipping(cx: &mut Context, tree: &Tree) {
    //println!("Apply Clipping");
    for entity in tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        if tree.is_ignored(entity) {
            continue;
        }

        let parent = tree.get_layout_parent(entity).unwrap();

        let parent_clip_region = cx.cache.get_clip_region(parent);
        //let parent_border_width = cx.style.border_width.get(parent).cloned().unwrap_or_default().value_or(0.0, 0.0);

        //println!("Parent border width: {}", parent_border_width);
        // parent_clip_region.x;
        // parent_clip_region.y;
        // parent_clip_region.w;
        // parent_clip_region.h;

        let root_clip_region = cx.cache.get_clip_region(Entity::root());

        let overflow = cx.style.overflow.get(entity).cloned().unwrap_or_default();

        if overflow == Overflow::Hidden {
            let clip_widget = cx.style.clip_widget.get(entity).cloned().unwrap_or(entity);
            //if let Some(clip_widget) = cx.style.clip_widget.get(entity).cloned() {
            //let clip_widget_border_width = cx.style.border_width.get(clip_widget).cloned().unwrap_or_default().value_or(0.0, 0.0);
            let clip_x = cx.cache.get_posx(clip_widget);
            let clip_y = cx.cache.get_posy(clip_widget);
            let clip_w = cx.cache.get_width(clip_widget);
            let clip_h = cx.cache.get_height(clip_widget);

            let mut intersection = BoundingBox::default();
            intersection.x = clip_x.max(parent_clip_region.x);
            intersection.y = clip_y.max(parent_clip_region.y);

            intersection.w = if clip_x + clip_w < parent_clip_region.x + parent_clip_region.w {
                clip_x + clip_w - intersection.x
            } else {
                parent_clip_region.x + parent_clip_region.w - intersection.x
            };

            intersection.h = if clip_y + clip_h < parent_clip_region.y + parent_clip_region.h {
                clip_y + clip_h - intersection.y
            } else {
                parent_clip_region.y + parent_clip_region.h - intersection.y
            };

            cx.cache.set_clip_region(entity, intersection);
            //} else {
            //    cx.cache.set_clip_region(entity, parent_clip_region);
            //}
        } else {
            cx.cache.set_clip_region(entity, root_clip_region);
        }

        //let clip_region = cx.cache.get_clip_region(entity);
        //println!("Entity: {}  Clip Region: {:?}", entity, clip_region);
    }
}

pub fn apply_visibility(cx: &mut Context, tree: &Tree) {
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

        if cx.cache.get_visibility(parent) == Visibility::Invisible {
            cx.cache.set_visibility(entity, Visibility::Invisible);
        } else {
            if let Some(visibility) = cx.style.visibility.get(entity) {
                cx.cache.set_visibility(entity, *visibility);
            } else {
                cx.cache.set_visibility(entity, Visibility::Visible);
            }
        }

        if cx.cache.get_display(parent) == Display::None {
            cx.cache.set_display(entity, Display::None);
        } else {
            if let Some(display) = cx.style.display.get(entity) {
                cx.cache.set_display(entity, *display);
            } else {
                cx.cache.set_display(entity, Display::Flex);
            }
        }

        let parent_opacity = cx.cache.get_opacity(parent);

        let opacity = cx.style.opacity.get(entity).cloned().unwrap_or_default();

        cx.cache.set_opacity(entity, opacity.0 * parent_opacity);
    }
}

// Apply this before layout
// THE GOAL OF THIS FUNCTION: set content-width and content-height
pub fn apply_text_constraints(cx: &mut Context, tree: &Tree) {
    //println!("Apply text constraints");
    let mut draw_tree: Vec<Entity> = tree.into_iter().collect();
    draw_tree.sort_by_cached_key(|entity| cx.cache.get_z_index(*entity));

    for entity in draw_tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        if tree.is_ignored(entity) {
            continue;
        }

        let desired_width = cx.style.width.get(entity).cloned().unwrap_or_default();
        let desired_height = cx.style.height.get(entity).cloned().unwrap_or_default();
        let text = cx.style.text.get(entity);
        let image = cx.style.image.get(entity);

        if (text.is_some() || image.is_some())
            && (desired_width == Units::Auto || desired_height == Units::Auto)
        {
            let parent = cx.tree.get_layout_parent(entity).expect("Failed to find parent somehow");
            let parent_width = cx.cache.get_width(parent);

            let border_width = match cx.style.border_width.get(entity).cloned().unwrap_or_default()
            {
                Units::Pixels(val) => val,
                Units::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_left = cx.style.child_left.get(entity).cloned().unwrap_or_default();
            let child_right = cx.style.child_right.get(entity).cloned().unwrap_or_default();
            let child_top = cx.style.child_top.get(entity).cloned().unwrap_or_default();
            let child_bottom = cx.style.child_bottom.get(entity).cloned().unwrap_or_default();

            let mut x = cx.cache.get_posx(entity);
            let mut y = cx.cache.get_posy(entity);
            let width = cx.cache.get_width(entity);
            let height = cx.cache.get_height(entity);

            let align = match child_left {
                Units::Pixels(val) => match child_right {
                    Units::Stretch(_) => {
                        x += val + border_width;
                        Align::Left
                    }

                    _ => Align::Left,
                },

                Units::Stretch(_) => match child_right {
                    Units::Pixels(val) => {
                        x += width - val - border_width;
                        Align::Right
                    }

                    Units::Stretch(_) => {
                        x += 0.5 * width;
                        Align::Center
                    }

                    _ => Align::Right,
                },

                _ => Align::Left,
            };

            let baseline = match child_top {
                Units::Pixels(val) => match child_bottom {
                    Units::Stretch(_) => {
                        y += val + border_width;
                        Baseline::Top
                    }

                    _ => Baseline::Top,
                },

                Units::Stretch(_) => match child_bottom {
                    Units::Pixels(val) => {
                        y += height - val - border_width;
                        Baseline::Bottom
                    }

                    Units::Stretch(_) => {
                        y += 0.5 * height;
                        Baseline::Middle
                    }

                    _ => Baseline::Top,
                },

                _ => Baseline::Top,
            };

            let mut content_width = 0.0;
            let mut content_height = 0.0;

            if let Some(text) = cx.style.text.get(entity) {
                let font = cx.style.font.get(entity).cloned().unwrap_or_default();

                // TODO - This should probably be cached in cx to save look-up time
                let default_font = cx
                    .resource_manager
                    .fonts
                    .get(&cx.style.default_font)
                    .and_then(|font| match font {
                        FontOrId::Id(id) => Some(id),
                        _ => None,
                    })
                    .expect("Failed to find default font");

                let font_id = cx
                    .resource_manager
                    .fonts
                    .get(&font)
                    .and_then(|font| match font {
                        FontOrId::Id(id) => Some(id),
                        _ => None,
                    })
                    .unwrap_or(default_font);

                let font_size = cx.style.font_size.get(entity).cloned().unwrap_or(16.0);

                let mut paint = Paint::default();
                paint.set_font_size(font_size);
                paint.set_font(&[font_id.clone()]);

                // TODO - should auto size use text height or font height?
                let font_metrics =
                    cx.text_context.measure_font(paint).expect("Failed to read font metrics");

                paint.set_text_align(align);
                paint.set_text_baseline(baseline);

                if let Ok(text_metrics) = cx.text_context.measure_text(x, y, text, paint) {
                    // Add an extra pixel to account for AA
                    let text_width = text_metrics.width().round() + 1.0;
                    let text_height = font_metrics.height().round() + 1.0;

                    if content_width < text_width {
                        content_width = text_width;
                    }
                    if content_height < text_height {
                        content_height = text_height;
                    }
                }
            }

            if let Some(image) = cx.style.image.get(entity) {
                let image = image.clone(); // ew
                let (image_width, image_height) = cx.get_image(&image).dimensions();
                let image_width = image_width as f32;
                let image_height = image_height as f32;

                if content_width < image_width {
                    content_width = image_width;
                }
                if content_height < image_height {
                    content_height = image_height;
                }
            }

            cx.style.content_width.insert(entity, content_width);
            cx.style.content_height.insert(entity, content_height);
        }
    }
}

pub fn apply_inline_inheritance(cx: &mut Context, tree: &Tree) {
    for entity in tree.into_iter() {
        if let Some(parent) = tree.get_layout_parent(entity) {
            cx.style.disabled.inherit_inline(entity, parent);

            cx.style.font_color.inherit_inline(entity, parent);
            cx.style.font_size.inherit_inline(entity, parent);
            cx.style.font.inherit_inline(entity, parent);
        }
    }
}

pub fn apply_shared_inheritance(cx: &mut Context, tree: &Tree) {
    for entity in tree.into_iter() {
        if let Some(parent) = tree.get_layout_parent(entity) {
            cx.style.font_color.inherit_shared(entity, parent);
            cx.style.font_size.inherit_shared(entity, parent);
            cx.style.font.inherit_shared(entity, parent);
        }
    }
}

// pub fn apply_abilities(cx: &mut Context, tree: &Tree) {
//     let mut draw_tree: Vec<Entity> = tree.into_iter().collect();
//     draw_tree.sort_by_cached_key(|entity| cx.cache.get_z_index(*entity));

//     for entity in draw_tree.into_iter() {

//         if entity == Entity::root() {
//             continue;
//         }

//         let parent= entity.parent(tree).unwrap();

//         let parent_abilities = cx.cache.abilities.get(parent).cloned().unwrap_or_default();

//         if !cx.style.abilities.get(parent).contains(Abilities::HOVERABLE) {
//             if let Some(abilities) = cx.style.abilities.get_mut(entity) {
//                 abilities.set(Abilities::HOVERABLE, false);
//             }
//         }

//         if cx.cache.get_visibility(parent) == Visibility::Invisible {
//             cx.cache.set_visibility(entity, Visibility::Invisible);
//         } else {
//             if let Some(visibility) = cx.style.visibility.get(entity) {
//                 cx.cache.set_visibility(entity, *visibility);
//             } else {
//                 cx.cache.set_visibility(entity, Visibility::Visible);
//             }
//         }
//     }
// }

// Returns true if the widget matches the selector
fn check_match(cx: &Context, entity: Entity, selector: &Selector) -> bool {
    // Universal selector always matches
    if selector.asterisk {
        if let Some(pseudo_classes) = cx.style.pseudo_classes.get(entity) {
            if !pseudo_classes.is_empty() && !pseudo_classes.intersects(*pseudo_classes) {
                return false;
            } else {
                return true;
            }
        } else {
            return true;
        }
    }

    // Check for ID match TODO
    // if selector.id.is_some() && selector.id != entity_selector.id {
    //     return false;
    // }

    // Check for element name match
    if let Some(selector_element) = &selector.element {
        if let Some(element) = cx.views.get(&entity).and_then(|view| view.element()) {
            if selector_element != &element {
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

    // Disabled needs to be handled separately because it can be inherited
    if let Some(disabled) = cx.style.disabled.get(entity) {
        if !selector.pseudo_classes.is_empty()
            && *disabled != selector.pseudo_classes.contains(PseudoClass::DISABLED)
        {
            return false;
        }
    }

    // Check for pseudo-class match
    if let Some(pseudo_classes) = cx.style.pseudo_classes.get(entity) {
        let mut selector_pseudo_classes = selector.pseudo_classes;
        selector_pseudo_classes.set(PseudoClass::DISABLED, false);

        if !selector_pseudo_classes.is_empty()
            && !selector_pseudo_classes.intersects(*pseudo_classes)
        {
            return false;
        }
    }

    return true;
}

pub fn apply_styles(cx: &mut Context, tree: &Tree) {
    //println!("RESTYLE");
    // Loop through all entities
    for entity in tree.into_iter() {
        // Skip the root
        if entity == Entity::root() {
            continue;
        }

        // Create a list of style rules that match this entity
        let mut matched_rules: Vec<Rule> = Vec::new();

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
            matched_rules.push(rule.id);
        }

        //println!("Entity: {}, Matched Rules: {:?}", entity, &matched_rules);

        // if matched_rules.len() == 0 {
        //     continue;
        // }

        let mut should_relayout = false;
        let mut should_redraw = false;

        // Display
        if cx.style.display.link(entity, &matched_rules) {
            //println!("1");
            should_relayout = true;
            should_redraw = true;
        }
        if cx.style.visibility.link(entity, &matched_rules) {
            //println!("2");
            should_relayout = true;
            should_redraw = true;
        }

        if cx.style.z_order.link(entity, &matched_rules) {
            //println!("3");
            should_redraw = true;
        }

        if cx.style.overflow.link(entity, &matched_rules) {
            should_redraw = true;
        }

        // Opacity
        if cx.style.opacity.link(entity, &matched_rules) {
            //println!("4");
            should_redraw = true;
        }

        if cx.style.left.link(entity, &matched_rules) {
            //println!("6");
            should_relayout = true;
            should_redraw = true;
        }

        if cx.style.right.link(entity, &matched_rules) {
            //println!("7");
            should_relayout = true;
            should_redraw = true;
        }

        if cx.style.top.link(entity, &matched_rules) {
            //println!("8");
            should_relayout = true;
            should_redraw = true;
        }

        if cx.style.bottom.link(entity, &matched_rules) {
            //println!("9");
            should_relayout = true;
            should_redraw = true;
        }

        // Size
        if cx.style.width.link(entity, &matched_rules) {
            //println!("10");
            should_relayout = true;
            should_redraw = true;
        }

        if cx.style.height.link(entity, &matched_rules) {
            //println!("11");
            should_relayout = true;
            should_redraw = true;
        }

        // Size Constraints
        if cx.style.max_width.link(entity, &matched_rules) {
            //println!("12");
            should_relayout = true;
            should_redraw = true;
        }

        if cx.style.min_width.link(entity, &matched_rules) {
            //println!("13");
            should_relayout = true;
            should_redraw = true;
        }

        if cx.style.max_height.link(entity, &matched_rules) {
            //println!("14");
            should_relayout = true;
            should_redraw = true;
        }

        if cx.style.min_height.link(entity, &matched_rules) {
            //println!("15");
            should_relayout = true;
            should_redraw = true;
        }

        // Border
        if cx.style.border_width.link(entity, &matched_rules) {
            //println!("24");
            should_relayout = true;
            should_redraw = true;
        }

        if cx.style.border_color.link(entity, &matched_rules) {
            //println!("25");
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
            //println!("26");
            should_redraw = true;
        }

        if cx.style.border_radius_top_right.link(entity, &matched_rules) {
            //println!("27");
            should_redraw = true;
        }

        if cx.style.border_radius_bottom_left.link(entity, &matched_rules) {
            //println!("28");
            should_redraw = true;
        }

        if cx.style.border_radius_bottom_right.link(entity, &matched_rules) {
            //println!("29");
            should_redraw = true;
        }

        if cx.style.layout_type.link(entity, &matched_rules) {
            //println!("30");
            should_relayout = true;
            should_redraw = true;
        }

        if cx.style.position_type.link(entity, &matched_rules) {
            //println!("30");
            should_relayout = true;
            should_redraw = true;
        }

        // Background
        if cx.style.background_color.link(entity, &matched_rules) {
            //println!("41");
            should_redraw = true;
        }

        if cx.style.background_image.link(entity, &matched_rules) {
            //println!("42");
            should_redraw = true;
        }

        // Font
        if cx.style.font_color.link(entity, &matched_rules) {
            //println!("43");
            should_redraw = true;
        }

        if cx.style.font_size.link(entity, &matched_rules) {
            //println!("44");
            should_redraw = true;
        }

        if cx.style.font.link(entity, &matched_rules) {
            //println!("44");
            should_redraw = true;
        }

        // Outer Shadow
        if cx.style.outer_shadow_h_offset.link(entity, &matched_rules) {
            //println!("45");
            should_redraw = true;
        }

        if cx.style.outer_shadow_v_offset.link(entity, &matched_rules) {
            //println!("46");
            should_redraw = true;
        }

        if cx.style.outer_shadow_blur.link(entity, &matched_rules) {
            //println!("47");
            should_redraw = true;
        }

        if cx.style.outer_shadow_color.link(entity, &matched_rules) {
            //println!("48");
            should_redraw = true;
        }

        // Inner Shadow
        if cx.style.inner_shadow_h_offset.link(entity, &matched_rules) {
            //println!("45");
            should_redraw = true;
        }

        if cx.style.inner_shadow_v_offset.link(entity, &matched_rules) {
            //println!("46");
            should_redraw = true;
        }

        if cx.style.inner_shadow_blur.link(entity, &matched_rules) {
            //println!("47");
            should_redraw = true;
        }

        if cx.style.inner_shadow_color.link(entity, &matched_rules) {
            //println!("48");
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
            cx.style.needs_relayout = true;
        }

        if should_redraw {
            cx.style.needs_redraw = true;
        }
    }
}
