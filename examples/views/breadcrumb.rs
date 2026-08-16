mod helpers;
use helpers::*;

use std::sync::Arc;
use vizia::prelude::*;

#[derive(Debug)]
enum BreadcrumbExampleEvent {
    Select(usize),
}

struct BreadcrumbExampleModel;

impl Model for BreadcrumbExampleModel {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            BreadcrumbExampleEvent::Select(index) => {
                println!("on_item_event emitted index: {}", index);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        BreadcrumbExampleModel.build(cx);

        // Add breadcrumb styling to prevent overlap
        cx.add_stylesheet(
            r#"
            breadcrumb {
                layout-type: row;
                gap: 4px;
                width: 1s;
                height: auto;
                align-items: center;
                wrap: wrap;
            }
            
            breadcrumb .breadcrumb-item {
                white-space: nowrap;
                flex: 0 1 auto;
            }
            
            breadcrumb .breadcrumb-item.disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }
            
            breadcrumb .breadcrumb-separator {
                white-space: nowrap;
                flex: 0 0 auto;
            }

            breadcrumb .breadcrumb-overflow {
                cursor: pointer;
            }

            .breadcrumb-overflow-list {
                gap: 2px;
                padding: 6px;
            }

            .breadcrumb-overflow-item {
                height: auto;
                padding: 2px 4px;
                white-space: nowrap;
                cursor: pointer;
            }
            "#,
        )
        .expect("Failed to add breadcrumb stylesheet");

        ExamplePage::vertical(cx, |cx| {
            // Example 1: Simple static breadcrumb
            VStack::new(cx, |cx| {
                Label::new(cx, "Simple Breadcrumb").class("label");
                Breadcrumb::new(
                    cx,
                    vec![
                        BreadcrumbItem::new("Home"),
                        BreadcrumbItem::new("Products"),
                        BreadcrumbItem::new("Electronics"),
                    ],
                );
            });

            // Example 2: Breadcrumb with custom separator
            VStack::new(cx, |cx| {
                Label::new(cx, "Custom Separator").class("label");
                Breadcrumb::new(
                    cx,
                    vec![
                        BreadcrumbItem::new("Home"),
                        BreadcrumbItem::new("Products"),
                        BreadcrumbItem::new("Electronics"),
                    ],
                )
                .separator(" > ");
            });

            // Example 3: Breadcrumb with middle truncation
            VStack::new(cx, |cx| {
                Label::new(cx, "Middle Truncation (max 5)").class("label");
                Breadcrumb::new(
                    cx,
                    vec![
                        BreadcrumbItem::new("Home"),
                        BreadcrumbItem::new("Products"),
                        BreadcrumbItem::new("Electronics"),
                        BreadcrumbItem::new("Phones"),
                        BreadcrumbItem::new("Smartphones"),
                        BreadcrumbItem::new("iPhone"),
                    ],
                )
                .separator(" > ")
                .truncation(TruncationStrategy::Middle {
                    max_visible: 5,
                    ellipsis: "...".to_string(),
                });
            });

            // Example 4: Breadcrumb with start truncation
            VStack::new(cx, |cx| {
                Label::new(cx, "Start Truncation (max 4)").class("label");
                Breadcrumb::new(
                    cx,
                    vec![
                        BreadcrumbItem::new("Level1"),
                        BreadcrumbItem::new("Level2"),
                        BreadcrumbItem::new("Level3"),
                        BreadcrumbItem::new("Level4"),
                        BreadcrumbItem::new("Level5"),
                    ],
                )
                .separator(" → ")
                .truncation(TruncationStrategy::Start {
                    max_visible: 4,
                    ellipsis: "...".to_string(),
                });
            });

            // Example 5: Breadcrumb with end truncation
            VStack::new(cx, |cx| {
                Label::new(cx, "End Truncation (max 4)").class("label");
                Breadcrumb::new(
                    cx,
                    vec![
                        BreadcrumbItem::new("Home"),
                        BreadcrumbItem::new("A"),
                        BreadcrumbItem::new("B"),
                        BreadcrumbItem::new("C"),
                        BreadcrumbItem::new("D"),
                    ],
                )
                .separator(" / ")
                .truncation(TruncationStrategy::End {
                    max_visible: 4,
                    ellipsis: "...".to_string(),
                });
            });

            // Example 6: Breadcrumb with custom truncation logic
            VStack::new(cx, |cx| {
                Label::new(cx, "Custom Truncation (filter disabled)").class("label");
                let items = vec![
                    BreadcrumbItem::new("Home"),
                    BreadcrumbItem::disabled("Disabled Item"),
                    BreadcrumbItem::new("Products"),
                    BreadcrumbItem::disabled("Another Disabled"),
                    BreadcrumbItem::new("Electronics"),
                ];
                Breadcrumb::new(cx, items.clone()).separator(" | ").truncation(
                    TruncationStrategy::Custom(Arc::new(|items| {
                        items.iter().filter(|item| !item.disabled).cloned().collect()
                    })),
                );
            });

            // Example 7: Breadcrumb with click handler
            VStack::new(cx, |cx| {
                Label::new(cx, "Clickable Breadcrumb (see console for clicks)").class("label");
                Breadcrumb::new(
                    cx,
                    vec![
                        BreadcrumbItem::new("Home"),
                        BreadcrumbItem::new("Products"),
                        BreadcrumbItem::new("Electronics"),
                    ],
                )
                .on_item_click(|_cx, index| {
                    println!("Clicked breadcrumb item at index: {}", index);
                });
            });

            // Example 8: Truncated clickable breadcrumb with overflow popup items
            VStack::new(cx, |cx| {
                Label::new(cx, "Truncated + Clickable (visible + hidden items)").class("label");
                Breadcrumb::new(
                    cx,
                    vec![
                        BreadcrumbItem::new("Home"),
                        BreadcrumbItem::new("Projects"),
                        BreadcrumbItem::new("Rust"),
                        BreadcrumbItem::new("Vizia"),
                        BreadcrumbItem::new("Crates"),
                        BreadcrumbItem::new("vizia_core"),
                        BreadcrumbItem::new("views"),
                        BreadcrumbItem::new("breadcrumb.rs"),
                    ],
                )
                .separator(" / ")
                .truncation(TruncationStrategy::Middle {
                    max_visible: 5,
                    ellipsis: "...".to_string(),
                })
                .on_item_click(|_cx, index| {
                    println!("Truncated breadcrumb click at original index: {}", index);
                });
            });
        });
    })
    .title("Breadcrumb")
    .run()
}
