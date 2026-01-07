//! Rich Text Example
//!
//! Demonstrates the rich text API using `Label::new` with method chaining:
//! - Markdown syntax: **bold**, *italic*, _italic_, __underline__, ~~strikethrough~~, `code`
//! - Custom tags: [tag]content[/tag] - use with .link() or .rich_style() + .build_rich()
//! - Bindings: {name} - use with .rich_bind() + .build_rich() (reactive!)
//! - Conditionals: {#if name}...{/if} - use with .cond() + .build_rich() (reactive!)
//! - Loops: {#each name as item}...{/each} - use with .each() + .build_rich() (reactive!)
//! - Escaping: "**not bold**" - literal text in quotes
//!
//! Run with: `cargo run --example rich_text`

use vizia::prelude::*;

struct RichTextApp {
    count: Signal<i32>,
    show_warning: Signal<bool>,
    items: Signal<Vec<String>>,
}

impl App for RichTextApp {
    fn app_name() -> &'static str {
        "Rich Text"
    }

    fn new(cx: &mut Context) -> Self {
        Self {
            count: cx.state(0),
            show_warning: cx.state(true),
            items: cx.state(vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string()]),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        VStack::new(cx, |cx| {
            // Section: Markdown syntax (automatic - no .build_rich() needed!)
            Label::new(cx, "Label::new - markdown syntax")
                .font_size(18.0)
                .font_weight(FontWeightKeyword::Bold);

            // Markdown is automatically parsed in Label::new
            Label::new(cx, "Text with markdown: **bold**, *italic*, and __underlined__ words.");
            Label::new(cx, "Also ~~strikethrough~~ and `monospace code`.");

            Element::new(cx).height(Pixels(12.0));

            // Section: Links and custom styles (need .build_rich())
            Label::new(cx, "Links & custom styles")
                .font_size(18.0)
                .font_weight(FontWeightKeyword::Bold);

            // Clickable links with custom tags
            Label::new(cx, "Visit [docs]Vizia Docs[/docs] or [repo]GitHub[/repo].")
                .link("docs", "https://docs.vizia.dev")
                .link("repo", "https://github.com/vizia/vizia")
                .build_rich();

            // Custom styles on tagged content
            Label::new(cx, "This has a [highlight]highlighted[/highlight] section.")
                .rich_style("highlight", |s| s.background_color(Color::rgba(255, 255, 0, 128)))
                .build_rich();

            Element::new(cx).height(Pixels(12.0));

            // Section: Reactive bindings (need .build_rich())
            Label::new(cx, "Reactive bindings - {name} syntax")
                .font_size(18.0)
                .font_weight(FontWeightKeyword::Bold);

            let count = self.count;
            Label::new(cx, "Counter: {count} *(reactive)*")
                .rich_bind("count", count)
                .rich_style("count", |s| {
                    s.font_weight(FontWeightKeyword::Bold).color(Color::rgb(51, 153, 225))
                })
                .build_rich();

            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::new(cx, "-"))
                    .on_press(move |cx| count.upd(cx, |v| *v -= 1));
                Button::new(cx, |cx| Label::new(cx, "+"))
                    .on_press(move |cx| count.upd(cx, |v| *v += 1));
            })
            .gap(Pixels(8.0))
            .height(Auto);

            Element::new(cx).height(Pixels(12.0));

            // Section: Conditionals (need .build_rich())
            Label::new(cx, "Conditionals - {#if}...{/if} (reactive)")
                .font_size(18.0)
                .font_weight(FontWeightKeyword::Bold);

            let show_warning = self.show_warning;
            Label::new(cx, "Status: {#if warn}**Warning!** {/if}All systems go.")
                .cond("warn", show_warning)
                .build_rich();

            Button::new(cx, |cx| Label::new(cx, "Toggle Warning"))
                .on_press(move |cx| show_warning.upd(cx, |v| *v = !*v));

            Element::new(cx).height(Pixels(12.0));

            // Section: Loops (need .build_rich())
            Label::new(cx, "Loops - {#each}...{/each} (reactive)")
                .font_size(18.0)
                .font_weight(FontWeightKeyword::Bold);

            let items = self.items;
            Label::new(cx, "Fruits: {#each fruits as f}{f}, {/each}")
                .each("fruits", items, |item| item.clone())
                .build_rich();

            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::new(cx, "Add Durian"))
                    .on_press(move |cx| items.upd(cx, |v| v.push("Durian".to_string())));
                Button::new(cx, |cx| Label::new(cx, "Remove Last")).on_press(move |cx| {
                    items.upd(cx, |v| {
                        v.pop();
                    })
                });
            })
            .gap(Pixels(8.0))
            .height(Auto);

            Element::new(cx).height(Pixels(12.0));

            // Section: Escaping (automatic - no .build_rich() needed)
            Label::new(cx, "Escaping - quote syntax")
                .font_size(18.0)
                .font_weight(FontWeightKeyword::Bold);

            Label::new(cx, "Literal: \"**not bold**\" vs rendered: **bold**");
        })
        .padding(Pixels(20.0))
        .gap(Pixels(6.0));

        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((625, 560)))
    }
}

fn main() -> Result<(), ApplicationError> {
    RichTextApp::run()
}
