use crate::layout::BoundingBox;

pub(crate) fn enforce_text_bounds(
    text_bounds: &BoundingBox,
    bounds: &BoundingBox,
    transform: (f32, f32),
) -> (f32, f32) {
    let (mut tx, mut ty) = transform;
    let text_box = BoundingBox {
        x: text_bounds.x + tx,
        y: text_bounds.y + ty,
        w: text_bounds.w,
        h: text_bounds.h,
    };

    if text_box.right() < bounds.right() {
        tx += bounds.right() - text_box.right();
    }
    if text_box.left() > bounds.left() {
        tx -= text_box.left() - bounds.left();
    }
    if text_box.width() < bounds.width() {
        tx = 0.0;
    }
    if text_box.bottom() < bounds.bottom() {
        ty += bounds.bottom() - text_box.bottom();
    }
    if text_box.top() > bounds.top() {
        ty -= text_box.top() - bounds.top();
    }
    if text_box.height() < bounds.height() {
        ty = 0.0;
    }
    (tx, ty)
}

pub(crate) fn ensure_visible(
    text_bounds: &BoundingBox,
    bounds: &BoundingBox,
    transform: (f32, f32),
) -> (f32, f32) {
    let (mut tx, mut ty) = transform;
    let caret_box = BoundingBox {
        x: text_bounds.x + tx,
        y: text_bounds.y + ty,
        w: text_bounds.w,
        h: text_bounds.h,
    };
    if caret_box.left() < bounds.left() {
        tx += bounds.left() - caret_box.left();
    }
    if caret_box.right() > bounds.right() {
        tx -= caret_box.right() - bounds.right();
    }
    if caret_box.top() < bounds.top() {
        ty += bounds.top() - caret_box.top();
    }
    if caret_box.bottom() > bounds.bottom() {
        ty -= caret_box.bottom() - bounds.bottom();
    }
    (tx, ty)
}
