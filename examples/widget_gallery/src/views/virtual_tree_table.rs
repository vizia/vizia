use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Clone, PartialEq)]
struct TreeNode {
    id: u32,
    parent_id: Option<u32>,
    name: String,
    kind: String,
    status: String,
}

fn sample_rows() -> Vec<TreeNode> {
    let mut rows = vec![TreeNode {
        id: 1,
        parent_id: None,
        name: "Widgets".to_string(),
        kind: "Root".to_string(),
        status: "Ready".to_string(),
    }];

    let mut next_id = 2u32;
    for section in ["Layout", "Display", "Input", "Data", "Feedback"] {
        let section_id = next_id;
        rows.push(TreeNode {
            id: section_id,
            parent_id: Some(1),
            name: section.to_string(),
            kind: "Category".to_string(),
            status: "Ready".to_string(),
        });
        next_id += 1;

        for index in 1..=30 {
            rows.push(TreeNode {
                id: next_id,
                parent_id: Some(section_id),
                name: format!("{} item {:02}", section, index),
                kind: "Widget".to_string(),
                status: if index % 3 == 0 { "New".to_string() } else { "Stable".to_string() },
            });
            next_id += 1;
        }
    }

    rows
}

pub fn virtual_tree_table(cx: &mut Context) {
    let rows = Signal::new(sample_rows());
    let expanded_rows: Signal<Vec<u32>> = Signal::new(vec![1, 2, 3, 4, 5, 6]);

    let columns: Signal<Vec<TreeTableColumn<TreeNodeRow<u32>, u32, TableHeader<String>>>> =
        Signal::new(vec![
            TreeTableColumn::new(
                "name",
                |cx, sort_dir| TableHeader::new(cx, "name", "Name", sort_dir),
                move |cx, row| {
                    let row_for_cell: TreeTableRow<TreeNodeRow<u32>, u32> = row.get();

                    TreeTableFirstCell::new(cx, row_for_cell.clone(), move |cx, row| {
                        let row_id = row.id;
                        let name = rows.map(move |rows| {
                            rows.iter()
                                .find(|node| node.id == row_id)
                                .map(|node| node.name.clone())
                                .unwrap_or_default()
                        });
                        Label::new(cx, name).class("table-cell-text");
                    });
                },
            )
            .width(Pixels(240.0))
            .min_width(140.0)
            .resizable(true),
            TreeTableColumn::new(
                "kind",
                |cx, sort_dir| TableHeader::new(cx, "kind", "Kind", sort_dir),
                move |cx, row| {
                    let row_for_cell: TreeTableRow<TreeNodeRow<u32>, u32> = row.get();
                    let kind = rows.map(move |rows| {
                        rows.iter()
                            .find(|node| node.id == row_for_cell.row.id)
                            .map(|node| node.kind.clone())
                            .unwrap_or_default()
                    });
                    Label::new(cx, kind).class("table-cell-text");
                },
            )
            .width(Pixels(140.0))
            .min_width(100.0)
            .resizable(true),
            TreeTableColumn::new(
                "status",
                |cx, sort_dir| TableHeader::new(cx, "status", "Status", sort_dir),
                move |cx, row| {
                    let row_for_cell: TreeTableRow<TreeNodeRow<u32>, u32> = row.get();
                    let status = rows.map(move |rows| {
                        rows.iter()
                            .find(|node| node.id == row_for_cell.row.id)
                            .map(|node| node.status.clone())
                            .unwrap_or_default()
                    });
                    Label::new(cx, status).class("table-cell-text");
                },
            )
            .width(Pixels(120.0))
            .min_width(90.0)
            .resizable(true),
        ]);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("virtual-tree-table")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "VirtualTreeTable", move |cx| {
            VirtualTreeTable::from_hierarchy(
                cx,
                rows,
                columns,
                30.0,
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
            )
            .expanded_row_ids(expanded_rows)
            .resizable_columns(true)
            .width(Stretch(1.0))
            .height(Pixels(340.0))
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
            });
        });
    })
    .class("panel");
}
