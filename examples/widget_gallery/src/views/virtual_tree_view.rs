use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Clone, PartialEq)]
struct TreeNode {
    id: u32,
    parent_id: Option<u32>,
    name: String,
}

fn sample_rows() -> Vec<TreeNode> {
    let mut rows = vec![TreeNode { id: 1, parent_id: None, name: "Widgets".to_string() }];
    let mut next_id = 2u32;

    for section in ["Layout", "Display", "Input", "Data", "Feedback"] {
        let section_id = next_id;
        rows.push(TreeNode { id: section_id, parent_id: Some(1), name: section.to_string() });
        next_id += 1;

        for index in 1..=24 {
            rows.push(TreeNode {
                id: next_id,
                parent_id: Some(section_id),
                name: format!("{} item {:02}", section, index),
            });
            next_id += 1;
        }
    }

    rows
}

pub fn virtual_tree_view(cx: &mut Context) {
    let rows = Signal::new(sample_rows());
    let selected_rows: Signal<Vec<u32>> = Signal::new(vec![]);
    let expanded_rows: Signal<Vec<u32>> = Signal::new(vec![1, 2, 3, 4, 5, 6]);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("virtual-tree-view")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "VirtualTreeView", move |cx| {
            VirtualTreeView::from_hierarchy(
                cx,
                rows,
                28.0,
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
            .height(Pixels(320.0));
        });
    })
    .class("panel");
}
