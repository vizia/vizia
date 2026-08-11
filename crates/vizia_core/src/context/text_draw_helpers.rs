use skia_safe::{Paint, TextBlob, TextBlobBuilder};
use vizia_style::TextAlign;

use crate::text::shaped_text::{PreShapedRun, PreShapedText};

#[derive(Clone)]
pub(super) struct DrawTextRun {
    pub(super) blob: TextBlob,
    pub(super) x_offset: f32,
    pub(super) run_advance: f32,
    pub(super) paint: Paint,
    pub(super) underline: bool,
    pub(super) strikethrough: bool,
    pub(super) decoration_paint: Paint,
    pub(super) baseline_y: f32,
}

pub(super) struct RunClusterAdvances {
    cluster_bytes: Vec<usize>,
    prefix_advances: Vec<f32>,
}

pub(super) fn compute_line_x_offset(
    align: TextAlign,
    base_rtl: bool,
    line_width: f32,
    constraint_width: f32,
) -> f32 {
    let effective_align = if base_rtl {
        match align {
            TextAlign::Left => TextAlign::Right,
            TextAlign::Right => TextAlign::Left,
            other => other,
        }
    } else {
        align
    };

    match effective_align {
        TextAlign::Right => (constraint_width - line_width).max(0.0),
        TextAlign::Center => ((constraint_width - line_width) * 0.5).max(0.0),
        _ => 0.0,
    }
}

pub(super) fn build_line_runs(
    pre: &PreShapedText,
    run_cluster_advances: &[RunClusterAdvances],
    line_start: usize,
    line_end: usize,
    baseline_y: f32,
    line_x_offset: f32,
) -> Vec<DrawTextRun> {
    let mut runs: Vec<DrawTextRun> = Vec::new();
    let mut cursor_x = line_x_offset;

    for (run_index, pre_run) in pre.runs.iter().enumerate() {
        let overlap_start = pre_run.byte_range.start.max(line_start);
        let overlap_end = pre_run.byte_range.end.min(line_end);
        if overlap_start >= overlap_end {
            continue;
        }

        let line_glyphs: Vec<_> = pre_run
            .glyphs
            .iter()
            .filter(|g| g.cluster_byte >= overlap_start && g.cluster_byte < overlap_end)
            .collect();

        if line_glyphs.is_empty() {
            continue;
        }

        let x_origin = line_glyphs.iter().map(|g| g.x).fold(f32::INFINITY, f32::min);

        let n = line_glyphs.len();
        let mut builder = TextBlobBuilder::new();
        let (glyph_ids, x_positions) = builder.alloc_run_pos_h(&pre_run.font, n, baseline_y, None);

        for (i, g) in line_glyphs.iter().enumerate() {
            glyph_ids[i] = g.glyph_id;
            x_positions[i] = g.x - x_origin;
        }

        let Some(blob) = builder.make() else { continue };

        let slice_advance = compute_slice_advance(
            &run_cluster_advances[run_index],
            overlap_start,
            overlap_end,
        );

        runs.push(DrawTextRun {
            blob,
            x_offset: cursor_x,
            run_advance: slice_advance,
            paint: pre_run.paint.fill.clone(),
            underline: pre_run.paint.underline,
            strikethrough: pre_run.paint.strikethrough,
            decoration_paint: pre_run.paint.decoration_paint.clone(),
            baseline_y,
        });

        cursor_x += slice_advance;
    }

    runs
}

pub(super) fn build_run_cluster_advances(pre_run: &PreShapedRun) -> RunClusterAdvances {
    if pre_run.glyphs.is_empty() {
        return RunClusterAdvances {
            cluster_bytes: Vec::new(),
            prefix_advances: vec![0.0],
        };
    }

    let mut raw: Vec<(usize, f32)> = pre_run
        .glyphs
        .iter()
        .map(|glyph| (glyph.cluster_byte, glyph.x))
        .collect();
    raw.sort_unstable_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    });

    let mut by_byte: Vec<(usize, f32)> = Vec::with_capacity(raw.len());
    for (cluster_byte, x) in raw {
        if let Some((last_cluster_byte, last_x)) = by_byte.last_mut()
            && *last_cluster_byte == cluster_byte
        {
            if x < *last_x {
                *last_x = x;
            }
            continue;
        }
        by_byte.push((cluster_byte, x));
    }

    if by_byte.is_empty() {
        return RunClusterAdvances {
            cluster_bytes: Vec::new(),
            prefix_advances: vec![0.0],
        };
    }

    let mut visual_order: Vec<usize> = (0..by_byte.len()).collect();
    visual_order.sort_by(|&lhs, &rhs| {
        by_byte[lhs]
            .1
            .partial_cmp(&by_byte[rhs].1)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut advances = vec![0.0f32; by_byte.len()];
    for (visual_rank, &cluster_idx) in visual_order.iter().enumerate() {
        let x = by_byte[cluster_idx].1;
        let next_x = if visual_rank + 1 < visual_order.len() {
            by_byte[visual_order[visual_rank + 1]].1
        } else {
            pre_run.total_advance
        };
        advances[cluster_idx] = (next_x - x).max(0.0);
    }

    let mut cluster_bytes = Vec::with_capacity(by_byte.len());
    let mut prefix_advances = Vec::with_capacity(by_byte.len() + 1);
    prefix_advances.push(0.0);

    for (idx, (cluster_byte, _)) in by_byte.iter().enumerate() {
        let advance = advances[idx];
        cluster_bytes.push(*cluster_byte);
        let next_prefix = prefix_advances.last().copied().unwrap_or(0.0) + advance;
        prefix_advances.push(next_prefix);
    }

    RunClusterAdvances {
        cluster_bytes,
        prefix_advances,
    }
}

fn compute_slice_advance(
    run_cluster_advances: &RunClusterAdvances,
    overlap_start: usize,
    overlap_end: usize,
) -> f32 {
    if overlap_start >= overlap_end || run_cluster_advances.cluster_bytes.is_empty() {
        return 0.0;
    }

    let start_idx = run_cluster_advances
        .cluster_bytes
        .partition_point(|cluster_byte| *cluster_byte < overlap_start);
    let end_idx = run_cluster_advances
        .cluster_bytes
        .partition_point(|cluster_byte| *cluster_byte < overlap_end);

    run_cluster_advances.prefix_advances[end_idx] - run_cluster_advances.prefix_advances[start_idx]
}
