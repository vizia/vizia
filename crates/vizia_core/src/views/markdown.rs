use std::cell::RefCell;

use comrak::nodes::{Ast, NodeValue};
use comrak::{format_html, parse_document, Arena, ExtensionOptions, Options};

use crate::prelude::*;

pub struct Markdown {}

impl Markdown {
    pub fn new<'a>(cx: &'a mut Context, document: &str) -> Handle<'a, Self> {
        Self {}
            .build(cx, |cx| {
                // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
                let arena = Arena::new();

                let mut options = Options::default();
                options.extension.strikethrough = true;

                // Parse the document into a root `AstNode`
                let root = parse_document(&arena, document, &options);

                // Iterate over all the descendants of root.
                for node in root.descendants() {
                    println!("{:?}", node.data.borrow().value);
                }

                for node in root.children() {
                    parse_node(cx, node, 0);
                }
            })
            .height(Auto)
    }
}

impl View for Markdown {
    fn element(&self) -> Option<&'static str> {
        Some("markdown")
    }
}

fn parse_node<'a>(
    cx: &mut Context,
    node: &'a comrak::arena_tree::Node<'a, RefCell<Ast>>,
    list_level: usize,
) {
    match &node.data.borrow().value {
        NodeValue::Paragraph => {
            println!("p");
            let parent = node.parent().unwrap();

            // if matches!(parent.data.borrow().value, NodeValue::Item(_)) {
            //     Label::rich(cx, "", |cx| {
            //         TextSpan::new(
            //             cx,
            //             // match list_level {
            //             //     1 => "\u{2022} ",
            //             //     2 => "\u{25E6} ",
            //             //     _ => "\u{25AA} ",
            //             // },
            //             match list_level {
            //                 1 => " ",
            //                 2 => "  ",
            //                 _ => "    ",
            //             },
            //             |_| {},
            //         );
            //         for child in node.children() {
            //             println!("Child  {:?}", child.data.borrow().value);
            //             parse_node(cx, child, list_level);
            //         }
            //     })
            //     .class("p");
            // } else {
            Label::rich(cx, "", |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("p");
            // }
        }

        NodeValue::Heading(heading) => {
            let parent = node.parent().unwrap();

            // if matches!(parent.data.borrow().value, NodeValue::Item(_)) {
            //     TextSpan::new(cx, "", |cx| {
            //         for child in node.children() {
            //             parse_node(cx, child, list_level);
            //         }
            //     })
            //     .class(match heading.level {
            //         1 => "h1",
            //         2 => "h2",
            //         3 => "h3",
            //         4 => "h4",
            //         5 => "h5",
            //         6 => "h6",
            //         _ => "h6",
            //     });
            // } else {
            Label::rich(cx, "", |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class(match heading.level {
                1 => "h1",
                2 => "h2",
                3 => "h3",
                4 => "h4",
                5 => "h5",
                6 => "h6",
                _ => "h6",
            });
            // }
        }

        NodeValue::Text(text) => {
            println!("{:?}", text);

            TextSpan::new(cx, text, |_| {}).class("span");
        }

        NodeValue::Emph => {
            TextSpan::new(cx, "", |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("emph");
        }

        NodeValue::Strong => {
            TextSpan::new(cx, "", |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("strong");
        }

        NodeValue::Strikethrough => {
            TextSpan::new(cx, "", |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("strikethrough");
        }

        NodeValue::List(list) => {
            println!("List  {:?}", list);
            // Label::rich(cx, "", |cx| {
            //     TextSpan::new(cx, "\u{2022} ", |_| {});
            //     // TextSpan::new(cx, "\u{2022} ", |_|{});

            // });
            VStack::new(cx, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .height(Auto)
            .left(Pixels(20.0));
        }

        NodeValue::Item(list) => {
            println!("Item  {:?}", list);
            HStack::new(cx, |cx| {
                Label::new(cx, "\u{2022} ").width(Auto);
                VStack::new(cx, |cx| {
                    for child in node.children() {
                        parse_node(cx, child, list_level + 1);
                        // println!("Child  {:?}", child.data.borrow().value);
                    }
                })
                .height(Auto);
            })
            .class("li")
            .height(Auto);
        }

        NodeValue::CodeBlock(code_block) => {
            let mut code = code_block.literal.to_owned();
            code.pop().unwrap();
            ScrollView::new(cx, 0.0, 0.0, true, false, |cx| {
                Label::new(cx, code).class("code");
            })
            .height(Auto)
            .width(Stretch(1.0));

            // for child in node.children() {
            //     println!("Child  {:?}", child.data.borrow().value);
            // }
        }

        NodeValue::Link(link) => {
            let url = link.url.clone();
            TextSpan::new(cx, "", |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .cursor(CursorIcon::Hand)
            .pointer_events(PointerEvents::Auto)
            .on_press(move |_| {
                open::that(url.as_str());
            })
            .class("link");
        }

        NodeValue::SoftBreak => {
            TextSpan::new(cx, "\n", |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            });
        }

        t => println!("{:?}", t),
    }
}

pub struct TextSpan {}

impl TextSpan {
    pub fn new<'a>(
        cx: &'a mut Context,
        text: &str,
        children: impl Fn(&mut Context),
    ) -> Handle<'a, Self> {
        Self {}
            .build(cx, |cx| {
                children(cx);
            })
            .text(text)
            .display(Display::None)
            .pointer_events(PointerEvents::None)
    }
}

impl View for TextSpan {
    fn element(&self) -> Option<&'static str> {
        Some("text-span")
    }
}
