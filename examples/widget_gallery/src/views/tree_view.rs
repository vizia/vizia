use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Clone, PartialEq)]
struct TreeNode {
    id: u32,
    parent_id: Option<u32>,
    name: String,
}

pub fn tree_view(cx: &mut Context) {
    let rows = Signal::new(vec![
        TreeNode { id: 1, parent_id: None, name: "Widgets".to_string() },
        TreeNode { id: 2, parent_id: Some(1), name: "Layout".to_string() },
        TreeNode { id: 3, parent_id: Some(1), name: "Data".to_string() },
        TreeNode { id: 4, parent_id: Some(2), name: "HStack".to_string() },
        TreeNode { id: 5, parent_id: Some(2), name: "VStack".to_string() },
        TreeNode { id: 6, parent_id: Some(3), name: "Table".to_string() },
        TreeNode { id: 7, parent_id: Some(3), name: "TreeTable".to_string() },
    ]);

    let selected_rows: Signal<Vec<u32>> = Signal::new(vec![]);
    let expanded_rows: Signal<Vec<u32>> = Signal::new(vec![1, 2, 3]);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("tree-view")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "TreeView", move |cx| {
            TreeView::from_hierarchy(
                cx,
                rows,
                move |rows: &Vec<TreeNode>| {
                    rows.iter().filter(|row| row.parent_id.is_none()).map(|row| row.id).collect()
                },
                move |rows: &Vec<TreeNode>, parent_id: &u32| {
                    rows.iter()
                        .filter(|row| row.parent_id == Some(*parent_id))
                        .map(|row| row.id)
                        .collect()
                },
                |_rows: &Vec<TreeNode>, _node_id: &u32| true,
                move |cx, row| {
                    let row_id = row.get().id;
                    let text = rows.map(move |rows| {
                        rows.iter()
                            .find(|node| node.id == row_id)
                            .map(|node| node.name.clone())
                            .unwrap_or_default()
                    });

                    Label::new(cx, text).hoverable(false);
                },
            )
            .selectable(Selectable::Single)
            .selected_row_ids(selected_rows)
            .expanded_row_ids(expanded_rows)
            .type_ahead_text(move |row| {
                rows.with(|rows| {
                    rows.iter()
                        .find(|node| node.id == row.id)
                        .map(|node| node.name.clone())
                })
            })
            .on_row_select(move |_cx, id| {
                selected_rows.set(vec![id]);
            })
            .on_row_toggle(move |_cx, id, expanded| {
                expanded_rows.update(|rows| {
                    if expanded {
                        if !rows.contains(&id) {
                            rows.push(id);
                        }
                    } else {
                        rows.retain(|current| *current != id);
                    }
                });
            })
            .width(Stretch(1.0))
            .height(Pixels(280.0));
        });
    })
    .class("panel");
}
