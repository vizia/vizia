#![cfg(feature = "markdown")]

use std::cell::RefCell;

use comrak::nodes::{Ast, NodeValue};
use comrak::{parse_document, Arena, Options};

use crate::prelude::*;

/// A view which parses and displays markdown as rich text.
pub struct Markdown {}

impl Markdown {
    /// Create a new [Markdown] view.
    pub fn new<'a>(cx: &'a mut Context, document: &str) -> Handle<'a, Self> {
        Self {}
            .build(cx, |cx| {
                // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
                let arena = Arena::new();

                let mut options = Options::default();
                options.extension.strikethrough = true;
                options.extension.table = true;

                // Parse the document into a root `AstNode`
                let root = parse_document(&arena, document, &options);

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
            Label::rich(cx, "", |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("p");
        }

        NodeValue::Heading(heading) => {
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
        }

        NodeValue::Text(text) => {
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

        NodeValue::List(_list) => {
            VStack::new(cx, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .height(Auto)
            .left(Pixels(20.0));
        }

        NodeValue::Item(_list) => {
            HStack::new(cx, |cx| {
                Label::new(cx, "\u{2022} ").width(Auto);
                VStack::new(cx, |cx| {
                    for child in node.children() {
                        parse_node(cx, child, list_level + 1);
                    }
                })
                .height(Auto);
            })
            .class("li")
            .height(Auto);
        }

        NodeValue::Code(code) => {
            TextSpan::new(cx, &code.literal.to_owned(), |_| {}).class("code");
        }

        NodeValue::CodeBlock(code_block) => {
            let mut code = code_block.literal.to_owned();
            code.pop().unwrap();
            ScrollView::new(cx, |cx| {
                Label::new(cx, code).class("code");
            })
            .show_vertical_scrollbar(false)
            .height(Auto)
            .width(Stretch(1.0));
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
                open::that(url.as_str()).unwrap();
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

        NodeValue::Table(_table) => {
            VStack::new(cx, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("table")
            .width(Stretch(1.0))
            .height(Auto);
        }

        NodeValue::TableRow(headers) => {
            HStack::new(cx, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("table-row")
            .toggle_class("table-headers", *headers)
            .width(Stretch(1.0))
            .height(Auto);
            Divider::horizontal(cx);
        }

        NodeValue::TableCell => {
            Label::rich(cx, "", |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("table-cell")
            .width(Stretch(1.0));
        }

        _ => {}
    }
}
