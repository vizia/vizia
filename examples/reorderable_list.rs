use vizia::prelude::*;
use vizia::views::{DropSource, DropTarget};

const STYLE: &str = r#"
:root {
    alignment: center;
    col-between: 16px;
}

reorderable-list {
    width: 260px;
    height: 380px;
    background-color: #f5f6f8;
    border-width: 1px;
    border-color: #d6dae0;
    child-space: 8px;
}

reorderable-list.reorder-drop-active {
    border-color: #0f766e;
}

.reorderable-list-item {
    background-color: #ffffff;
    border-width: 1px;
    border-color: #d6dae0;
    border-radius: 6px;
    child-space: 10px;
    height: 44px;
    alignment: left;
}

.reorderable-list-item.reorder-drop-target {
    border-color: #0f766e;
}

.reorderable-list-item.reorder-drag-source {
    opacity: 0.6;
}

.reorder-indicator {
    background-color: #78c5bf;
    height: 2px;
}

.reorder-indicator.reorder-indicator-active {
    background-color: #0f766e;
}
"#;

fn reorder_in_place<T>(items: &mut Vec<T>, from: usize, to: usize) {
    if from >= items.len() {
        return;
    }

    let destination = to.min(items.len().saturating_sub(1));
    if from == destination {
        return;
    }

    let item = items.remove(from);
    items.insert(destination, item);
}

fn move_between_lists<T>(source: &mut Vec<T>, target: &mut Vec<T>, from: usize, to: usize) {
    if from >= source.len() {
        return;
    }

    let item = source.remove(from);
    let destination = to.min(target.len());
    target.insert(destination, item);
}

fn compute_destination(
    drop_target: DropTarget,
    source_index: usize,
    len: usize,
    is_internal: bool,
) -> usize {
    let mut destination = match drop_target {
        DropTarget::Before(i) => i,
        DropTarget::After(i) => i.saturating_add(1),
        DropTarget::Onto(i) => i,
    }
    .min(len);

    // Remove-first semantics only apply to internal reorders.
    if is_internal && destination > source_index {
        destination = destination.saturating_sub(1);
    }

    destination
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("failed to load style");

        let left_items = Signal::new(vec![
            "Sage".to_string(),
            "River".to_string(),
            "Maple".to_string(),
            "Poppy".to_string(),
        ]);
        let right_items = Signal::new(vec![
            "Amber".to_string(),
            "Juno".to_string(),
            "Kai".to_string(),
            "Nora".to_string(),
        ]);
        let left_list_entity = Signal::new(Entity::null());
        let right_list_entity = Signal::new(Entity::null());

        VStack::new(cx, |cx| {
            Label::new(cx, "Drag rows to reorder or move between lists")
                .font_weight(FontWeightKeyword::SemiBold);

            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "Backlog");

                    let left_handle = ReorderableList::new(cx, left_items, |cx, _index, item| {
                        Label::new(cx, item);
                    })
                    .accept_external_reorder(true)
                    .on_row_drop(move |_cx, drop_source, drop_target| {
                        let right_entity = right_list_entity.get();
                        let left_len = left_items.get().len();

                        match drop_source {
                            DropSource::Internal(from) => {
                                let destination =
                                    compute_destination(drop_target, from, left_len, true);
                                left_items
                                    .update(|items| reorder_in_place(items, from, destination));
                            }
                            DropSource::External(source_list, from) => {
                                let destination =
                                    compute_destination(drop_target, from, left_len, false);
                                if source_list == right_entity {
                                    let mut source = right_items.get();
                                    let mut target = left_items.get();
                                    move_between_lists(&mut source, &mut target, from, destination);
                                    right_items.set(source);
                                    left_items.set(target);
                                }
                            }
                        }
                    });
                    left_list_entity.set(left_handle.entity());
                    let _ = left_handle;
                });

                VStack::new(cx, |cx| {
                    Label::new(cx, "In Progress");

                    let right_handle = ReorderableList::new(cx, right_items, |cx, _index, item| {
                        Label::new(cx, item);
                    })
                    .accept_external_reorder(true)
                    .on_row_drop(move |_cx, drop_source, drop_target| {
                        let left_entity = left_list_entity.get();
                        let right_len = right_items.get().len();

                        match drop_source {
                            DropSource::Internal(from) => {
                                let destination =
                                    compute_destination(drop_target, from, right_len, true);
                                right_items
                                    .update(|items| reorder_in_place(items, from, destination));
                            }
                            DropSource::External(source_list, from) => {
                                let destination =
                                    compute_destination(drop_target, from, right_len, false);
                                if source_list == left_entity {
                                    let mut source = left_items.get();
                                    let mut target = right_items.get();
                                    move_between_lists(&mut source, &mut target, from, destination);
                                    left_items.set(source);
                                    right_items.set(target);
                                }
                            }
                        }
                    });
                    right_list_entity.set(right_handle.entity());
                    let _ = right_handle;
                });
            });
        });
    })
    .title("Reorderable List")
    .run()
}
