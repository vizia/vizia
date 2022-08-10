use crate::prelude::*;
use crate::text::{measure_text_lines, text_layout, text_paint_general};
use crate::vg::*;

// Apply this before layout
// THE GOAL OF THIS FUNCTION: set content-width and content-height
pub fn text_constraints_system(cx: &mut Context, tree: &Tree) {
    let mut draw_tree: Vec<Entity> = tree.into_iter().collect();
    draw_tree.sort_by_cached_key(|entity| cx.cache.get_z_index(*entity));

    for entity in draw_tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        if cx.cache.display.get(entity) == Some(&Display::None) {
            continue;
        }

        if tree.is_ignored(entity) {
            continue;
        }

        // content-size is only used if any dimension is auto
        if cx.style.min_width.get(entity).copied().unwrap_or_default() != Units::Auto
            && cx.style.min_height.get(entity).copied().unwrap_or_default() != Units::Auto
            && cx.style.width.get(entity).copied().unwrap_or_default() != Units::Auto
            && cx.style.height.get(entity).copied().unwrap_or_default() != Units::Auto
            && cx.style.max_width.get(entity).map_or(true, |w| w != &Units::Auto)
            && cx.style.max_height.get(entity).map_or(true, |h| h != &Units::Auto)
        {
            continue;
        }

        let desired_width = cx.style.width.get(entity).cloned().unwrap_or_default();
        let desired_height = cx.style.height.get(entity).cloned().unwrap_or_default();
        let style = &cx.style;
        let text = style.text.get(entity);
        let image = style.image.get(entity);

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
            let mut child_space_x = 0.0;
            let mut child_space_y = 0.0;

            let align = match child_left {
                Units::Pixels(val) => {
                    child_space_x += val;
                    match child_right {
                        Units::Stretch(_) => {
                            x += val + border_width;
                            Align::Left
                        }

                        _ => Align::Left,
                    }
                }

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
            match child_right {
                Units::Pixels(px) => child_space_x += px,
                _ => {}
            }

            let baseline = match child_top {
                Units::Pixels(val) => {
                    child_space_y += val;
                    match child_bottom {
                        Units::Stretch(_) => {
                            y += val + border_width;
                            Baseline::Top
                        }

                        _ => Baseline::Top,
                    }
                }

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
            match child_bottom {
                Units::Pixels(px) => child_space_y += px,
                _ => {}
            }

            let mut content_width = 0.0;
            let mut content_height = 0.0;

            if let Some(text) = cx.style.text.get(entity).cloned() {
                let mut paint = text_paint_general(&cx.style, &cx.resource_manager, entity);
                paint.set_text_align(align);
                paint.set_text_baseline(baseline);

                let font_metrics =
                    cx.text_context.measure_font(paint).expect("Failed to read font metrics");

                if let Ok(lines) = text_layout(f32::MAX, &text, paint, &cx.text_context) {
                    let metrics = measure_text_lines(&text, paint, &lines, x, y, &cx.text_context);
                    let text_width = metrics
                        .iter()
                        .map(|m| m.width())
                        .reduce(|a, b| a.max(b))
                        .unwrap_or_default();
                    let text_height = font_metrics.height().round() * metrics.len() as f32;

                    // Add an extra pixel to account for AA
                    let text_width = text_width.round() + 1.0 + child_space_x;
                    let text_height = text_height.round() + 1.0 + child_space_y;

                    if content_width < text_width {
                        content_width = text_width;
                    }
                    if content_height < text_height {
                        content_height = text_height;
                    }
                }
            }

            if let Some(image_name) = cx.style.image.get(entity) {
                if let Some(img) = cx.resource_manager.images.get(image_name) {
                    let (image_width, image_height) = img.image.dimensions();
                    let image_width = image_width as f32;
                    let image_height = image_height as f32;

                    if content_width < image_width {
                        content_width = image_width;
                    }
                    if content_height < image_height {
                        content_height = image_height;
                    }
                }
            }

            cx.style.content_width.insert(entity, content_width);
            cx.style.content_height.insert(entity, content_height);
        }
    }
}
