use std::ops::Range;

use skia_safe::Matrix;
use vizia_storage::TreeExt;

use crate::prelude::*;
use crate::text::resolved_text_direction;
#[cfg(feature = "clipboard")]
use crate::text::selection::selected_text;
use crate::text::selection::{TextSelectionPoint, selected_ranges};

fn selectable_labels(cx: &EventContext<'_>, window: Entity) -> Vec<(Entity, usize)> {
    window
        .branch_iter(cx.tree)
        .filter(|entity| {
            cx.text_context.selectable_labels.contains(*entity)
                && cx.style.display.get(*entity).copied().unwrap_or_default() != Display::None
        })
        .filter_map(|entity| {
            cx.text_context.text_shaped.get(entity).map(|shaped| (entity, shaped.selectable_len()))
        })
        .collect()
}

fn text_local_point(cx: &EventContext<'_>, entity: Entity, x: f32, y: f32) -> (f32, f32) {
    let bounds = cx.cache.get_bounds(entity);
    let point = cx
        .cache
        .transform
        .get(entity)
        .copied()
        .unwrap_or_else(Matrix::new_identity)
        .invert()
        .map(|inverse| inverse.map_point((x, y)))
        .unwrap_or_else(|| (x, y).into());

    let logical_width = cx.physical_to_logical(bounds.w);
    let logical_height = cx.physical_to_logical(bounds.h);
    let mut padding_left = cx
        .style
        .padding_left
        .get_resolved(entity, &cx.style.custom_units_props)
        .unwrap_or_default()
        .to_px(logical_width, 0.0)
        * cx.scale_factor();
    let mut padding_right = cx
        .style
        .padding_right
        .get_resolved(entity, &cx.style.custom_units_props)
        .unwrap_or_default()
        .to_px(logical_width, 0.0)
        * cx.scale_factor();
    let padding_top = cx
        .style
        .padding_top
        .get_resolved(entity, &cx.style.custom_units_props)
        .unwrap_or_default()
        .to_px(logical_height, 0.0)
        * cx.scale_factor();
    let padding_bottom = cx
        .style
        .padding_bottom
        .get_resolved(entity, &cx.style.custom_units_props)
        .unwrap_or_default()
        .to_px(logical_height, 0.0)
        * cx.scale_factor();

    if resolved_text_direction(cx.style, entity) == Direction::RightToLeft {
        std::mem::swap(&mut padding_left, &mut padding_right);
    }

    let text_height =
        cx.text_context.text_shaped.get(entity).map(|shaped| shaped.height()).unwrap_or(0.0);
    let mut top = match cx.style.alignment.get(entity).copied().unwrap_or_default() {
        Alignment::TopLeft | Alignment::TopCenter | Alignment::TopRight => 0.0,
        Alignment::Left | Alignment::Center | Alignment::Right => 0.5,
        Alignment::BottomLeft | Alignment::BottomCenter | Alignment::BottomRight => 1.0,
    };
    top *= bounds.height() - padding_top - padding_bottom - text_height;

    (point.x - bounds.x - padding_left, point.y - bounds.y - padding_top - top)
}

fn nearest_selectable_label(
    cx: &EventContext<'_>,
    labels: &[(Entity, usize)],
    x: f32,
    y: f32,
) -> Option<Entity> {
    let hovered = *cx.hovered;
    if let Some(entity) = hovered
        .parent_iter(cx.tree)
        .find(|entity| labels.iter().any(|(candidate, _)| candidate == entity))
    {
        return Some(entity);
    }

    labels
        .iter()
        .min_by(|(left, _), (right, _)| {
            let distance = |entity| {
                let bounds = cx.transformed_bounds(entity);
                let dx = if x < bounds.left() {
                    bounds.left() - x
                } else if x > bounds.right() {
                    x - bounds.right()
                } else {
                    0.0
                };
                let dy = if y < bounds.top() {
                    bounds.top() - y
                } else if y > bounds.bottom() {
                    y - bounds.bottom()
                } else {
                    0.0
                };
                dx * dx + dy * dy
            };
            distance(*left).total_cmp(&distance(*right))
        })
        .map(|(entity, _)| *entity)
}

fn point_from_window(
    cx: &EventContext<'_>,
    labels: &[(Entity, usize)],
    x: f32,
    y: f32,
) -> Option<TextSelectionPoint> {
    let entity = nearest_selectable_label(cx, labels, x, y)?;
    let (local_x, local_y) = text_local_point(cx, entity, x, y);
    let shaped = cx.text_context.text_shaped.get(entity)?;
    let (byte, affinity) = shaped.point_at(local_x, local_y);
    Some(TextSelectionPoint::with_affinity(entity, byte, affinity))
}

fn refresh_ranges(cx: &mut EventContext<'_>, window: Entity, labels: &[(Entity, usize)]) {
    for (entity, _) in labels {
        cx.text_context.selected_ranges.remove(*entity);
    }

    let points = cx.text_context.selections.get(&window).and_then(|selection| selection.points());
    if let Some((anchor, focus)) = points {
        for (entity, range) in selected_ranges(labels, anchor, focus) {
            cx.text_context.selected_ranges.insert(entity, range);
        }
    }

    for (entity, _) in labels {
        cx.with_current(*entity, |cx| cx.needs_redraw());
    }
}

pub(crate) fn begin_selection(cx: &mut EventContext<'_>, extend: bool) {
    let entity = cx.current();
    let window = cx.tree.get_parent_window(entity).unwrap_or(Entity::root());
    let labels = selectable_labels(cx, window);
    let Some(point) = point_from_window(cx, &labels, cx.mouse.cursor_x, cx.mouse.cursor_y) else {
        return;
    };

    let selection = cx.text_context.selections.entry(window).or_default();
    if !extend || selection.anchor.is_none() {
        selection.anchor = Some(point);
    }
    selection.focus = Some(point);
    selection.preferred_x = None;
    selection.dragging = true;
    selection.active = true;
    refresh_ranges(cx, window, &labels);
}

pub(crate) fn extend_selection(cx: &mut EventContext<'_>) {
    let entity = cx.current();
    let window = cx.tree.get_parent_window(entity).unwrap_or(Entity::root());
    let labels = selectable_labels(cx, window);
    let Some(point) = point_from_window(cx, &labels, cx.mouse.cursor_x, cx.mouse.cursor_y) else {
        return;
    };
    let Some(selection) = cx.text_context.selections.get_mut(&window) else { return };
    if !selection.dragging {
        return;
    }
    selection.focus = Some(point);
    selection.preferred_x = None;
    refresh_ranges(cx, window, &labels);
}

pub(crate) fn end_selection(cx: &mut EventContext<'_>) {
    let window = cx.tree.get_parent_window(cx.current()).unwrap_or(Entity::root());
    if let Some(selection) = cx.text_context.selections.get_mut(&window) {
        selection.dragging = false;
    }
}

pub(crate) fn select_word(cx: &mut EventContext<'_>) {
    let entity = cx.current();
    let window = cx.tree.get_parent_window(entity).unwrap_or(Entity::root());
    let labels = selectable_labels(cx, window);
    let (x, y) = text_local_point(cx, entity, cx.mouse.cursor_x, cx.mouse.cursor_y);
    let Some(shaped) = cx.text_context.text_shaped.get(entity) else { return };
    let Range { start, end } = shaped.word_at(x, y);
    let selection = cx.text_context.selections.entry(window).or_default();
    selection.anchor = Some(TextSelectionPoint::new(entity, start));
    selection.focus = Some(TextSelectionPoint::new(entity, end));
    selection.active = true;
    selection.dragging = false;
    refresh_ranges(cx, window, &labels);
}

pub(crate) fn select_label(cx: &mut EventContext<'_>) {
    let entity = cx.current();
    let window = cx.tree.get_parent_window(entity).unwrap_or(Entity::root());
    let labels = selectable_labels(cx, window);
    let Some((_, len)) = labels.iter().find(|(candidate, _)| *candidate == entity) else {
        return;
    };
    let selection = cx.text_context.selections.entry(window).or_default();
    selection.anchor = Some(TextSelectionPoint::new(entity, 0));
    selection.focus = Some(TextSelectionPoint::new(entity, *len));
    selection.active = true;
    selection.dragging = false;
    refresh_ranges(cx, window, &labels);
}

fn select_all(cx: &mut EventContext<'_>, window: Entity) {
    let labels = selectable_labels(cx, window);
    let Some((first, _)) = labels.first().copied() else { return };
    let Some((last, last_len)) = labels.last().copied() else { return };
    let selection = cx.text_context.selections.entry(window).or_default();
    selection.anchor = Some(TextSelectionPoint::new(first, 0));
    selection.focus = Some(TextSelectionPoint::new(last, last_len));
    selection.preferred_x = None;
    selection.active = true;
    selection.dragging = false;
    refresh_ranges(cx, window, &labels);
}

#[cfg(feature = "clipboard")]
fn copy_selection(cx: &mut EventContext<'_>, window: Entity) {
    let labels = selectable_labels(cx, window);
    let Some((anchor, focus)) =
        cx.text_context.selections.get(&window).and_then(|selection| selection.points())
    else {
        return;
    };
    let ranges = selected_ranges(&labels, anchor, focus);
    let text = selected_text(&ranges, |entity| {
        cx.text_context.text_shaped.get(entity).map(|shaped| shaped.selectable_text())
    });
    if !text.is_empty() {
        let _ = cx.set_clipboard(text);
    }
}

#[derive(Clone, Copy)]
enum KeyboardMovement {
    Visual { forward: bool, word: bool },
    Line { down: bool },
    LineEdge { end: bool },
}

fn move_focus(cx: &mut EventContext<'_>, window: Entity, movement: KeyboardMovement) {
    let labels = selectable_labels(cx, window);
    let Some(selection) = cx.text_context.selections.get(&window) else { return };
    let Some(mut focus) = selection.focus else { return };
    let preferred_x = selection.preferred_x;
    let Some(index) = labels.iter().position(|(entity, _)| *entity == focus.entity) else {
        return;
    };

    let mut next_preferred_x = None;
    let mut crossed_boundary = false;
    if let Some(shaped) = cx.text_context.text_shaped.get(focus.entity) {
        let (byte, affinity) = match movement {
            KeyboardMovement::Visual { forward, word } => {
                let moved = shaped.move_visual(focus.byte, focus.affinity, forward, word);
                crossed_boundary = moved == (focus.byte, focus.affinity);
                moved
            }
            KeyboardMovement::Line { down } => {
                let (moved, horizontal, boundary) =
                    shaped.move_line(focus.byte, focus.affinity, down, preferred_x);
                next_preferred_x = Some(horizontal);
                crossed_boundary = boundary;
                moved
            }
            KeyboardMovement::LineEdge { end } => {
                shaped.move_line_edge(focus.byte, focus.affinity, end)
            }
        };
        focus.byte = byte;
        focus.affinity = affinity;
    }

    if crossed_boundary {
        let forward = match movement {
            KeyboardMovement::Visual { forward, .. } => forward,
            KeyboardMovement::Line { down } => down,
            KeyboardMovement::LineEdge { .. } => false,
        };
        let adjacent = if forward { index.checked_add(1) } else { index.checked_sub(1) };
        if let Some((entity, len)) = adjacent.and_then(|index| labels.get(index)).copied() {
            focus = if forward {
                TextSelectionPoint::new(entity, 0)
            } else {
                TextSelectionPoint::with_affinity(entity, len, parley::Affinity::Upstream)
            };
            if let KeyboardMovement::Line { down } = movement
                && let Some(shaped) = cx.text_context.text_shaped.get(entity)
            {
                let y = if down { 0.0 } else { shaped.height() };
                let (byte, affinity) = shaped.point_at(next_preferred_x.unwrap_or(0.0), y);
                focus = TextSelectionPoint::with_affinity(entity, byte, affinity);
            }
        }
    }

    if let Some(selection) = cx.text_context.selections.get_mut(&window) {
        selection.focus = Some(focus);
        selection.preferred_x = next_preferred_x;
    }
    refresh_ranges(cx, window, &labels);
}

fn clear_selection(cx: &mut EventContext<'_>, window: Entity) {
    let labels = selectable_labels(cx, window);
    if let Some(selection) = cx.text_context.selections.get_mut(&window) {
        selection.clear();
    }
    refresh_ranges(cx, window, &labels);
}

pub(crate) fn handle_global_event(cx: &mut EventContext<'_>, event: &mut Event) {
    event.map(|window_event, meta| {
        let window = meta.origin;
        let active = cx
            .text_context
            .selections
            .get(&window)
            .map(|selection| selection.active)
            .unwrap_or(false);
        if active && matches!(window_event, WindowEvent::MouseDown(MouseButton::Left)) {
            let hovered_is_selectable = cx
                .hovered
                .parent_iter(cx.tree)
                .any(|entity| cx.text_context.selectable_labels.contains(entity));
            if !hovered_is_selectable {
                clear_selection(cx, window);
            }
            return;
        }
        if !active || cx.text_context.plain_editors.contains(*cx.focused) {
            return;
        }

        let command =
            if cfg!(target_os = "macos") { cx.modifiers.logo() } else { cx.modifiers.ctrl() };
        let handled = match window_event {
            WindowEvent::KeyDown(Code::KeyA, _) if command && !cx.modifiers.shift() => {
                select_all(cx, window);
                true
            }
            WindowEvent::KeyDown(Code::KeyC, _) if command && !cx.modifiers.shift() => {
                #[cfg(feature = "clipboard")]
                copy_selection(cx, window);
                true
            }
            WindowEvent::KeyDown(Code::ArrowLeft, _) if cx.modifiers.shift() => {
                move_focus(cx, window, KeyboardMovement::Visual { forward: false, word: command });
                true
            }
            WindowEvent::KeyDown(Code::ArrowRight, _) if cx.modifiers.shift() => {
                move_focus(cx, window, KeyboardMovement::Visual { forward: true, word: command });
                true
            }
            WindowEvent::KeyDown(Code::ArrowUp, _) if cx.modifiers.shift() && !command => {
                move_focus(cx, window, KeyboardMovement::Line { down: false });
                true
            }
            WindowEvent::KeyDown(Code::ArrowDown, _) if cx.modifiers.shift() && !command => {
                move_focus(cx, window, KeyboardMovement::Line { down: true });
                true
            }
            WindowEvent::KeyDown(Code::Home, _) if cx.modifiers.shift() => {
                move_focus(cx, window, KeyboardMovement::LineEdge { end: false });
                true
            }
            WindowEvent::KeyDown(Code::End, _) if cx.modifiers.shift() => {
                move_focus(cx, window, KeyboardMovement::LineEdge { end: true });
                true
            }
            WindowEvent::KeyDown(Code::Escape, _) => {
                clear_selection(cx, window);
                true
            }
            _ => false,
        };
        if handled {
            meta.consume();
        }
    });
}
