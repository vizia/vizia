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
        let auto = cx.state(Auto);
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
            .height(auto)
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
            Label::with_spans(cx, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("p");
        }

        NodeValue::Heading(heading) => {
            Label::with_spans(cx, |cx| {
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
            let text = cx.state(text.clone());
            TextSpan::new(cx, text, |_| {}).class("span");
        }

        NodeValue::Emph => {
            let empty = cx.state("");
            TextSpan::new(cx, empty, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("emph");
        }

        NodeValue::Strong => {
            let empty = cx.state("");
            TextSpan::new(cx, empty, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("strong");
        }

        NodeValue::Strikethrough => {
            let empty = cx.state("");
            TextSpan::new(cx, empty, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("strikethrough");
        }

        NodeValue::List(_list) => {
            let auto = cx.state(Auto);
            let indent = cx.state(Pixels(20.0));
            VStack::new(cx, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .height(auto)
            .left(indent);
        }

        NodeValue::Item(_list) => {
            let auto = cx.state(Auto);
            HStack::new(cx, |cx| {
                let bullet = cx.state("\u{2022} ");
                Label::new(cx, bullet).width(auto);
                VStack::new(cx, |cx| {
                    for child in node.children() {
                        parse_node(cx, child, list_level + 1);
                    }
                })
                .height(auto);
            })
            .class("li")
            .height(auto);
        }

        NodeValue::Code(code) => {
            let code = cx.state(code.literal.to_owned());
            TextSpan::new(cx, code, |_| {}).class("code");
        }

        NodeValue::CodeBlock(code_block) => {
            let mut code = code_block.literal.to_owned();
            code.pop().unwrap();
            let show_vertical_scrollbar = cx.state(false);
            let auto = cx.state(Auto);
            let stretch_one = cx.state(Stretch(1.0));
            ScrollView::new(cx, |cx| {
                let code_signal = cx.state(code);
                Label::new(cx, code_signal).class("code");
            })
            .show_vertical_scrollbar(show_vertical_scrollbar)
            .height(auto)
            .width(stretch_one);
        }

        NodeValue::Link(link) => {
            let url = link.url.clone();
            let empty = cx.state("");
            let cursor_hand = cx.state(CursorIcon::Hand);
            let pointer_auto = cx.state(PointerEvents::Auto);
            TextSpan::new(cx, empty, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .cursor(cursor_hand)
            .pointer_events(pointer_auto)
            .on_press(move |_| {
                open::that(url.as_str()).unwrap();
            })
            .class("link");
        }

        NodeValue::SoftBreak => {
            let newline = cx.state("\n");
            TextSpan::new(cx, newline, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            });
        }

        NodeValue::Table(_table) => {
            let stretch_one = cx.state(Stretch(1.0));
            let auto = cx.state(Auto);
            VStack::new(cx, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("table")
            .width(stretch_one)
            .height(auto);
        }

        NodeValue::TableRow(headers) => {
            let table_headers = cx.state(*headers);
            let stretch_one = cx.state(Stretch(1.0));
            let auto = cx.state(Auto);
            HStack::new(cx, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("table-row")
            .toggle_class("table-headers", table_headers)
            .width(stretch_one)
            .height(auto);
            Divider::horizontal(cx);
        }

        NodeValue::TableCell => {
            let stretch_one = cx.state(Stretch(1.0));
            Label::with_spans(cx, |cx| {
                for child in node.children() {
                    parse_node(cx, child, list_level);
                }
            })
            .class("table-cell")
            .width(stretch_one);
        }

        _ => {}
    }
}
