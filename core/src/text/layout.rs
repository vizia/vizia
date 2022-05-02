use unicode_bidi::{bidi_class, BidiClass};

use crate::prelude::*;
use femtovg::{ErrorKind, Paint, TextContext, TextMetrics};
use std::ops::Range;
use crate::resource::{FontOrId, ResourceManager};
use crate::style::Style;

pub(crate) fn text_paint_layout(
    style: &Style,
    resource_manager: &ResourceManager,
    entity: Entity,
) -> Paint {
    let font = style.font.get(entity).map(|s| s.as_str()).unwrap_or("");
    let font_size = style.font_size.get(entity).cloned().unwrap_or(16.0);
    // drawing and layout are always in physical space
    let font_size = font_size * style.dpi_factor as f32;

    text_paint(font, &style.default_font, resource_manager, font_size)
}

pub fn text_paint_draw(cx: &DrawContext, entity: Entity) -> Paint {
    let font = cx.font(entity).map(|s| s.as_str()).unwrap_or("");
    let font_size = cx.font_size(entity);

    text_paint(font, cx.default_font(), cx.resource_manager(), font_size)
}

pub fn text_paint_general(cx: &Context, entity: Entity) -> Paint {
    let font = cx.style.font.get(entity).map(|s| s.as_str()).unwrap_or("");
    let font_size =
        cx.style.font_size.get(entity).copied().unwrap_or(16.0) * cx.style.dpi_factor as f32;

    text_paint(font, &cx.style.default_font, &cx.resource_manager, font_size)
}

fn text_paint(
    font: &str,
    default_font: &str,
    resource_manager: &ResourceManager,
    font_size: f32,
) -> Paint {
    // TODO - This should probably be cached in cx to save look-up time
    let default_font = resource_manager
        .fonts
        .get(default_font)
        .and_then(|font| match font {
            FontOrId::Id(id) => Some(id),
            _ => None,
        })
        .expect("Failed to find default font");

    let font_id = resource_manager
        .fonts
        .get(font)
        .and_then(|font| match font {
            FontOrId::Id(id) => Some(id),
            _ => None,
        })
        .unwrap_or(default_font);

    let mut paint = Paint::default();
    paint.set_font_size(font_size);
    paint.set_font(&[font_id.clone()]);

    paint
}

pub fn text_layout(
    width: f32,
    text: &str,
    paint: Paint,
    text_context: &TextContext,
) -> Result<Vec<Range<usize>>, ErrorKind> {
    let mut lines = text_context.break_text_vec(width, text, paint)?;
    if lines.len() == 0 {
        lines.push(0..0)
    }
    let mut soft_break = false;
    for line_range in lines.iter_mut() {
        if soft_break {
            // trim start spaces
            let mut broken = false;
            for (idx, ch) in text[line_range.clone()].char_indices() {
                if !ch.is_whitespace() {
                    line_range.start = idx + line_range.start;
                    broken = true;
                    break;
                }
            }
            if !broken {
                line_range.start = line_range.end;
            }
        }

        // trim end newlines
        // also: if there's any newlines, don't trim starting space on next line
        soft_break = true;
        for (idx, ch) in text[line_range.clone()].char_indices().rev() {
            if bidi_class(ch) == BidiClass::B {
                soft_break = false;
                line_range.end = idx + line_range.start;
            } else {
                break;
            }
        }
    }

    // if the text ends with a newline, add a blank line
    if text.chars().last() == Some('\n') {
        lines.push(text.len()..text.len());
    }

    Ok(lines)
}

pub fn measure_text_lines(
    text: &str,
    paint: Paint,
    lines: &[Range<usize>],
    x: f32,
    y: f32,
    text_context: &TextContext,
) -> Vec<TextMetrics> {
    let mut metrics = vec![];
    let line_height = text_context.measure_font(paint).unwrap().height();

    for (idx, line) in lines.iter().enumerate() {
        let y = y + idx as f32 * line_height;
        if let Ok(mut res) = text_context.measure_text(x, y, &text[line.clone()], paint) {
            for glyph in res.glyphs.iter_mut() {
                glyph.byte_index += line.start;
            }
            metrics.push(res);
        } else {
            metrics.push(TextMetrics::default());
        }
    }

    metrics
}

// returns (line_no, (x_pos, y_pos))
// TODO affinity
// TODO neither this nor the next function are correct for rtl. we probably need to explicitly do
// bidi analysis the same way femtovg does
pub fn idx_to_pos<'a>(
    byte_idx: usize,
    metrics: impl Iterator<Item = &'a (Range<usize>, TextMetrics)>,
) -> (usize, (f32, f32)) {
    let mut uninit = true;
    let mut result_line = 0;
    let mut result_xpos = 0.0;
    let mut result_ypos = 0.0;
    for (line, (range, line_metrics)) in metrics.enumerate() {
        if range.start == byte_idx {
            result_line = line;
            result_xpos = line_metrics.x;
            result_ypos = line_metrics.y;
            break;
        }
        for glyph in line_metrics.glyphs.iter() {
            if uninit {
                uninit = false;
                result_line = line;
                result_xpos = glyph.x;
                result_ypos = glyph.y;
            }
            if glyph.byte_index == byte_idx {
                result_line = line;
                result_xpos = glyph.x;
                result_ypos = glyph.y;
            } else if glyph.byte_index < byte_idx {
                // if the target is after me, place the cursor after me
                result_line = line;
                result_xpos = glyph.x + glyph.advance_x;
                result_ypos = glyph.y;
            } else {
                break;
            }
        }
    }

    (result_line, (result_xpos - 1.0, result_ypos))
}

// TODO see above
pub fn pos_to_idx<'a>(
    x: f32,
    y: f32,
    cache: impl Iterator<Item = &'a (Range<usize>, TextMetrics)>,
) -> usize {
    let mut last = 0;
    // first: what line is it?
    for (line_range, line_metrics) in cache {
        if y < line_metrics.y + line_metrics.height() {
            // it's me!
            for glyph in line_metrics.glyphs.iter() {
                if x < glyph.x + glyph.advance_x / 2.0 {
                    return glyph.byte_index;
                }
            }
            return line_range.end;
        }
        last = line_range.end;
    }

    last
}
