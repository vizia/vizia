use vizia_storage::LayoutTreeIterator;

use crate::prelude::*;
use skia_safe::Matrix;

// Computes the clip bounds of views.
pub(crate) fn clipping_system(cx: &mut Context) {
    if cx.style.reclip.is_empty() {
        return;
    }

    let mut redraw_entities = Vec::new();

    let iter = LayoutTreeIterator::full(&cx.tree);

    for entity in iter {
        if !cx.style.reclip.contains(entity) {
            continue;
        }

        let bounds = cx.cache.bounds.get(entity).copied().unwrap();

        let overflowx = cx.style.overflowx.get(entity).copied().unwrap_or_default();
        let overflowy = cx.style.overflowy.get(entity).copied().unwrap_or_default();

        let scale = cx.style.scale_factor();

        let clip_bounds = cx
            .style
            .clip_path
            .get(entity)
            .map(|clip| match clip {
                ClipPath::Auto => bounds,
                ClipPath::Shape(rect) => bounds.shrink_sides(
                    rect.3.to_pixels(bounds.w, scale),
                    rect.0.to_pixels(bounds.h, scale),
                    rect.1.to_pixels(bounds.w, scale),
                    rect.2.to_pixels(bounds.h, scale),
                ),
            })
            .unwrap_or(bounds);

        let root_bounds = BoundingBox::from_min_max(-f32::MAX, -f32::MAX, f32::MAX, f32::MAX);

        let clip_bounds = match (overflowx, overflowy) {
            (Overflow::Visible, Overflow::Visible) => root_bounds,
            (Overflow::Hidden, Overflow::Visible) => {
                let left = clip_bounds.left();
                let right = clip_bounds.right();
                let top = root_bounds.top();
                let bottom = root_bounds.bottom();
                BoundingBox::from_min_max(left, top, right, bottom)
            }
            (Overflow::Visible, Overflow::Hidden) => {
                let left = root_bounds.left();
                let right = root_bounds.right();
                let top = clip_bounds.top();
                let bottom = clip_bounds.bottom();
                BoundingBox::from_min_max(left, top, right, bottom)
            }
            (Overflow::Hidden, Overflow::Hidden) => clip_bounds,
        };

        let transform = cx.cache.transform.get(entity).copied().unwrap_or(Matrix::new_identity());

        let rect: skia_safe::Rect = clip_bounds.into();
        let clip_bounds: BoundingBox = transform.map_rect(rect).0.into();

        let parent_clip_bounds = cx
            .tree
            .get_layout_parent(entity)
            .and_then(|parent| cx.cache.clip_path.get(parent))
            .copied()
            .unwrap_or(root_bounds);

        if let Some(clip_path) = cx.cache.clip_path.get_mut(entity) {
            *clip_path = clip_bounds.intersection(&parent_clip_bounds);
        } else {
            cx.cache.clip_path.insert(entity, clip_bounds.intersection(&parent_clip_bounds));
        }

        redraw_entities.push(entity);
    }

    for entity in redraw_entities {
        cx.needs_redraw(entity);
    }

    cx.style.reclip.clear();
}
