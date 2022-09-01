use crate::prelude::*;
use crate::text::{measure_text_lines, text_layout, text_paint_general};
use crate::vg::*;
use vizia_id::GenerationalId;

// Apply this before layout
// THE GOAL OF THIS FUNCTION: set content-width and content-height
pub fn text_constraints_system(cx: &mut Context, tree: &Tree<Entity>) {
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
                Units::Pixels(val) => val * cx.style.dpi_factor as f32,
                Units::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_left = cx.style.child_left.get(entity).cloned().unwrap_or_default();
            let child_right = cx.style.child_right.get(entity).cloned().unwrap_or_default();
            let child_top = cx.style.child_top.get(entity).cloned().unwrap_or_default();
            let child_bottom = cx.style.child_bottom.get(entity).cloned().unwrap_or_default();

            let mut x = cx.cache.get_posx(entity);
            let mut y = cx.cache.get_posy(entity);
            let mut w = cx.cache.get_width(entity) - border_width * 2.0;
            let mut h = cx.cache.get_height(entity) - border_width * 2.0;
            let mut child_space_x = 0.0;
            let mut child_space_y = 0.0;

            // shrink the bounding box based on pixel values
            if let Units::Pixels(val) = child_left {
                let val = val * cx.style.dpi_factor as f32;
                x += val;
                w -= val;
                child_space_x += val;
            }
            if let Units::Pixels(val) = child_right {
                let val = val * cx.style.dpi_factor as f32;
                w -= val;
                child_space_x += val;
            }
            if let Units::Pixels(val) = child_top {
                let val = val * cx.style.dpi_factor as f32;
                y += val;
                h -= val;
                child_space_y += val;
            }
            if let Units::Pixels(val) = child_bottom {
                let val = val * cx.style.dpi_factor as f32;
                h -= val;
                child_space_y += val;
            }

            // set align/baseline and move the start coordinate to the appropriate place in the box
            let align = match (child_left, child_right) {
                (Units::Stretch(_), Units::Stretch(_)) => {
                    x += w / 2.0;
                    Align::Center
                },
                (Units::Stretch(_), _) => {
                    x += w;
                    Align::Right
                },
                _ => Align::Left,
            };
            let baseline = match (child_top, child_bottom) {
                (Units::Stretch(_), Units::Stretch(_)) => {
                    y += h / 2.0;
                    Baseline::Middle
                },
                (Units::Stretch(_), _) => {
                    y += h;
                    Baseline::Bottom
                },
                _ => Baseline::Top,
            };

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

            cx.style.content_width.insert(entity, content_width / cx.style.dpi_factor as f32);
            cx.style.content_height.insert(entity, content_height / cx.style.dpi_factor as f32);
        }
    }
}
