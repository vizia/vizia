use morphorm::Node;

use crate::prelude::*;
use crate::style::SystemFlags;

pub(crate) fn layout_system(cx: &mut Context) {
    // text_constraints_system(cx);

    if cx.style.system_flags.contains(SystemFlags::RELAYOUT) {
        // layout(&mut cx.cache, &cx.tree, &cx.style, &mut cx.text_context);
        Entity::root().layout(&mut cx.cache, &cx.tree, &cx.style, &mut cx.text_context);

        // If layout has changed then redraw
        cx.style.system_flags.set(SystemFlags::REDRAW, true);

        for entity in cx.tree.into_iter() {
            if cx.text_context.has_buffer(entity) {
                let auto_width = cx.style.width.get(entity).copied().unwrap_or_default().is_auto();
                let auto_height =
                    cx.style.height.get(entity).copied().unwrap_or_default().is_auto();
                if !auto_width && !auto_height {
                    let width = cx.cache.bounds.get(entity).unwrap().w;
                    cx.text_context.with_buffer(entity, |fs, buf| {
                        buf.set_size(fs, width.ceil(), f32::MAX);
                    });
                }
            }

            if let Some(parent) = cx.tree.get_layout_parent(entity) {
                let parent_bounds = cx.cache.get_bounds(parent);
                let b = cx.cache.bounds.get(entity);
                if let Some(bounds) = cx.cache.bounds.get_mut(entity) {
                    bounds.x += parent_bounds.x;
                    bounds.y += parent_bounds.y;
                }
            }
        }

        // Defer resetting the layout system flag to the geometry changed system
    }
}

// fn content_size(node: Entity, store: &Store, width: Option<f32>, height: Option<f32>) -> (f32, f32) {
//     let text = store.text.get(node).unwrap();
//     let mut paint = femtovg::Paint::color(femtovg::Color::black());
//     paint.set_font_size(48.0);
//     paint.set_text_align(femtovg::Align::Left);
//     paint.set_text_baseline(femtovg::Baseline::Top);
//     paint.set_font(&vec![store.font_id.unwrap()]);
//     // let should_wrap = store.text_wrap.get(&node).copied().unwrap_or_default();
//     let text_wrap = store.text_wrap.get(node).copied().unwrap_or_default();

//     let max_width = if let Some(width) = width {
//         width
//     } else {
//         match text_wrap {
//             TextWrap::None | TextWrap::Hard => f32::MAX,
//             TextWrap::Soft | TextWrap::All => {
//                 let mut max_word = 0.0f32;
//                 for word in text.unicode_words() {
//                     if let Ok(text_metrics) = store.text_context.measure_text(0.0, 0.0, word, &paint) {
//                         max_word = max_word.max(text_metrics.width());
//                     }
//                 }
//                 max_word.ceil()
//             }
//         }
//     };

//     let font_metrics = store.text_context.measure_font(&paint).expect("Error measuring font");
//     let (text_width, text_height) = if let Ok(text_lines) = store.text_context.break_text_vec(max_width, text, &paint) {
//         let text_height = font_metrics.height() * text_lines.len() as f32;
//         let mut text_width = 0.0f32;
//         for line in text_lines {
//             let line_text = &text[line];
//             if let Ok(text_metrics) = store.text_context.measure_text(0.0, 0.0, line_text, &paint) {
//                 text_width = text_width.max(text_metrics.width());
//             }
//         }
//         (text_width, text_height)
//     } else {
//         (0.0, 0.0)
//     };

//     let height = if let Some(height) = height { height } else { text_height };

//     (text_width, height)
// }
