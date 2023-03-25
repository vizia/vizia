use crate::layout::BoundingBox;

pub fn enforce_text_bounds(
    bounds: &BoundingBox,
    parent_bounds: &BoundingBox,
    transform: (f32, f32),
) -> (f32, f32) {
    let (mut tx, mut ty) = transform;
    let text_box = BoundingBox { x: bounds.x + tx, y: bounds.y + ty, w: bounds.w, h: bounds.h };
    if text_box.x < parent_bounds.x && text_box.x + text_box.w < parent_bounds.x + parent_bounds.w {
        tx += parent_bounds.x - text_box.x;
    }
    if text_box.x > parent_bounds.x && text_box.x + text_box.w > parent_bounds.x + parent_bounds.w {
        tx -= (text_box.x + text_box.w) - (parent_bounds.x + parent_bounds.w);
    }
    if text_box.w < parent_bounds.w {
        tx = 0.0;
    }
    if text_box.y < parent_bounds.y && text_box.y + text_box.h < parent_bounds.y + parent_bounds.h {
        ty -= (text_box.y + text_box.h) - (parent_bounds.y + parent_bounds.h);
    }
    if text_box.y > parent_bounds.y && text_box.y + text_box.h > parent_bounds.y + parent_bounds.h {
        ty += parent_bounds.y - text_box.y;
    }
    if text_box.h < parent_bounds.h {
        ty = 0.0;
    }
    (tx, ty)
}

pub fn ensure_visible(
    bounds: &BoundingBox,
    parent_bounds: &BoundingBox,
    transform: (f32, f32),
) -> (f32, f32) {
    let (mut tx, mut ty) = transform;
    let caret_box = BoundingBox { x: bounds.x + tx, y: bounds.y + ty, w: bounds.w, h: bounds.h };
    if caret_box.x < parent_bounds.x {
        tx += parent_bounds.x - caret_box.x;
    }
    if caret_box.x + caret_box.w >= parent_bounds.x + parent_bounds.w {
        tx -= caret_box.x + caret_box.w - (parent_bounds.x + parent_bounds.w);
    }
    if caret_box.y < parent_bounds.y {
        ty += parent_bounds.y - caret_box.y;
    }
    if caret_box.y + caret_box.h >= parent_bounds.y + parent_bounds.h {
        ty -= caret_box.y + caret_box.h - (parent_bounds.y + parent_bounds.h);
    }
    (tx, ty)
}
