use crate::prelude::*;
use crate::style::{FamilyOwned, GenericFontFamily};
use std::collections::HashMap;
use std::rc::Rc;
use vizia_style::{FontSlant, TextDecorationLine};

/// A label used to display text.
///
/// # Examples
///
/// ## Basic label
///
/// A label can be used to simply display some text on the screen.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// Label::new(cx, "Hello World");
/// ```
///
/// ## Label bound to data
///
/// A label can be bound to data using a signal which automatically updates the text whenever the underlying data changes.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// let count = cx.state(0i32);
/// let text = cx.derived({
///     let count = count;
///     move |s| format!("Count: {}", count.get(s))
/// });
/// Label::new(cx, text);
/// ```
///
/// ## Rich text with markdown
///
/// Labels automatically parse markdown syntax:
///
/// ```ignore
/// Label::new(cx, "**bold**, *italic*, and `code`");
/// ```
///
/// ## Rich text with bindings
///
/// Use `{name}` syntax with `.rich_bind()` for reactive content. Requires `.build_rich()`:
///
/// ```ignore
/// let count = cx.state(0);
/// Label::new(cx, "Count: {count}")
///     .rich_bind("count", count)
///     .build_rich();
/// ```
///
/// ## Rich text with conditionals and loops
///
/// ```ignore
/// Label::new(cx, "{#if show}Visible{/if} Items: {#each items as i}{i}, {/each}")
///     .cond("show", show_signal)
///     .each("items", items_signal, |i| i.clone())
///     .build_rich();
/// ```
///
/// ## Label for a button
///
/// A label can also be used inside of a button to be able to add text to it.
///
/// ```
/// # use vizia_core::prelude::*;
/// # let cx = &mut Context::default();
/// #
/// Button::new(cx, |cx| Label::new(cx, "Click me"));
/// ```
pub struct Label {
    describing: Option<String>,
    /// Raw text for rich text rebuilding
    raw_text: Option<String>,
}

impl Label {
    /// Creates a new [Label] view.
    ///
    /// Accepts either a plain value or a `Signal<T>` for reactive text.
    /// Supports rich text features via method chaining:
    /// - `.bind(name, signal)` - reactive placeholders `{name}`
    /// - `.cond(name, signal)` - conditionals `{#if name}...{/if}`
    /// - `.each(name, signal, f)` - loops `{#each name as item}...{/each}`
    /// - `.link(tag, url)` - clickable links `[tag]text[/tag]`
    /// - `.rich_style(tag, f)` - custom styles `[tag]text[/tag]`
    ///
    /// Markdown syntax is automatically parsed: `**bold**`, `*italic*`, `__underline__`,
    /// `~~strikethrough~~`, `` `code` ``
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// // Static text
    /// Label::new(cx, "Hello World");
    ///
    /// // Reactive text
    /// let text = cx.state("Text");
    /// Label::new(cx, text);
    /// ```
    pub fn new<T>(cx: &mut Context, text: impl Res<T> + Clone + 'static) -> Handle<Self>
    where
        T: ToStringLocalized + Clone + 'static,
    {
        // Get the text value
        let raw_text = text.resolve(cx).to_string_local(cx);

        // Check for tags [...] or bindings {...} which require method config
        let needs_config = raw_text.contains('[') || raw_text.contains('{');

        // Check for pure markdown (no config needed)
        let has_markdown = raw_text.contains("**")
            || raw_text.contains("__")
            || raw_text.contains("~~")
            || raw_text.contains('`')
            || (raw_text.contains('*') && !raw_text.starts_with('*'));

        // Only auto-parse if pure markdown (no tags/bindings that need config)
        if has_markdown && !needs_config {
            // Parse markdown and create rich text children
            let empty = cx.state(String::new());
            Self { describing: None, raw_text: Some(raw_text.clone()) }
                .build(cx, |cx| {
                    parse_rich(
                        cx,
                        &raw_text,
                        &HashMap::new(),
                        &HashMap::new(),
                        &HashMap::new(),
                        &HashMap::new(),
                        &HashMap::new(),
                    );
                })
                .text(empty) // Content is in children
                .role(Role::Label)
                .name(text)
        } else {
            // Plain text - no rich syntax detected
            Self { describing: None, raw_text: Some(raw_text) }
                .build(cx, |_| {})
                .text(text.clone())
                .role(Role::Label)
                .name(text)
        }
    }

    /// Internal method for building Labels with programmatic child TextSpans.
    /// Used by the Markdown view for rendering parsed markdown content.
    pub(crate) fn with_spans(cx: &mut Context, children: impl Fn(&mut Context)) -> Handle<Self> {
        let empty = cx.state(String::new());
        Self { describing: None, raw_text: None }
            .build(cx, |cx| {
                children(cx);
            })
            .text(empty)
            .role(Role::Label)
    }
}

/// Parsed segment of rich text
#[derive(Clone)]
enum RichSegment {
    Text(String),
    Styled { style: RichStyle, content: String },
    Tagged { tag: String, children: Vec<RichSegment> },
    Binding { name: String },
    Conditional { name: String, children: Vec<RichSegment> },
    Each { name: String, item_name: String, children: Vec<RichSegment> },
}

/// Built-in styles from markdown syntax
#[derive(Clone, Copy)]
enum RichStyle {
    Bold,
    Italic,
    Strikethrough,
    Code,
    Underline,
}

/// Parse rich text into segments
/// Supports: "escape", **bold**, *italic*, `code`, [tag], {binding}, {#if}, {#each}
fn parse_rich_segments(text: &str) -> Vec<RichSegment> {
    let mut segments = Vec::new();
    let mut remaining = text;

    while !remaining.is_empty() {
        // Check for escaped content "..." first (highest priority)
        let quote_pos = remaining.find('"');

        // Find markdown delimiters
        let mut md_earliest: Option<(usize, &str, &str, RichStyle)> = None;
        let delimiters = [
            ("**", "**", RichStyle::Bold),
            ("__", "__", RichStyle::Underline),
            ("~~", "~~", RichStyle::Strikethrough),
            ("*", "*", RichStyle::Italic),
            ("_", "_", RichStyle::Italic),
            ("`", "`", RichStyle::Code),
        ];
        for (open, close, style) in delimiters {
            if let Some(pos) = remaining.find(open) {
                if md_earliest.is_none() || pos < md_earliest.unwrap().0 {
                    md_earliest = Some((pos, open, close, style));
                }
            }
        }

        let tag_pos = remaining.find('[');
        let brace_pos = remaining.find('{');

        // Find earliest special syntax
        let earliest_pos = [
            quote_pos,
            md_earliest.map(|(p, _, _, _)| p),
            tag_pos,
            brace_pos,
        ]
        .into_iter()
        .flatten()
        .min();

        match earliest_pos {
            None => {
                segments.push(RichSegment::Text(remaining.to_string()));
                break;
            }

            // Escaped content "..."
            Some(pos) if Some(pos) == quote_pos => {
                if pos > 0 {
                    segments.push(RichSegment::Text(remaining[..pos].to_string()));
                }
                remaining = &remaining[pos + 1..];

                if let Some(end_quote) = remaining.find('"') {
                    // Content inside quotes is literal text
                    segments.push(RichSegment::Text(remaining[..end_quote].to_string()));
                    remaining = &remaining[end_quote + 1..];
                } else {
                    // No closing quote, treat opening quote as text
                    segments.push(RichSegment::Text("\"".to_string()));
                }
            }

            // Brace syntax: {binding}, {#if}, {#each}
            Some(pos) if Some(pos) == brace_pos => {
                if pos > 0 {
                    segments.push(RichSegment::Text(remaining[..pos].to_string()));
                }

                if let Some(close_pos) = remaining[pos..].find('}') {
                    let inner = &remaining[pos + 1..pos + close_pos];

                    if let Some(rest) = inner.strip_prefix("#if ") {
                        // Conditional: {#if name}...{/if}
                        let name = rest.trim();
                        let close_tag = "{/if}";
                        if let Some(end_pos) = remaining[pos + close_pos + 1..].find(close_tag) {
                            let content = &remaining[pos + close_pos + 1..pos + close_pos + 1 + end_pos];
                            let children = parse_rich_segments(content);
                            segments.push(RichSegment::Conditional {
                                name: name.to_string(),
                                children,
                            });
                            remaining = &remaining[pos + close_pos + 1 + end_pos + close_tag.len()..];
                        } else {
                            segments.push(RichSegment::Text(remaining[pos..pos + close_pos + 1].to_string()));
                            remaining = &remaining[pos + close_pos + 1..];
                        }
                    } else if let Some(rest) = inner.strip_prefix("#each ") {
                        // Loop: {#each name as item}...{/each}
                        let parts: Vec<&str> = rest.split(" as ").collect();
                        let (name, item_name) = if parts.len() == 2 {
                            (parts[0].trim(), parts[1].trim())
                        } else {
                            (rest.trim(), "item")
                        };
                        let close_tag = "{/each}";
                        if let Some(end_pos) = remaining[pos + close_pos + 1..].find(close_tag) {
                            let content = &remaining[pos + close_pos + 1..pos + close_pos + 1 + end_pos];
                            let children = parse_rich_segments(content);
                            segments.push(RichSegment::Each {
                                name: name.to_string(),
                                item_name: item_name.to_string(),
                                children,
                            });
                            remaining = &remaining[pos + close_pos + 1 + end_pos + close_tag.len()..];
                        } else {
                            segments.push(RichSegment::Text(remaining[pos..pos + close_pos + 1].to_string()));
                            remaining = &remaining[pos + close_pos + 1..];
                        }
                    } else if !inner.is_empty() && !inner.contains('{') && !inner.starts_with('/') {
                        // Simple binding: {name}
                        segments.push(RichSegment::Binding { name: inner.to_string() });
                        remaining = &remaining[pos + close_pos + 1..];
                    } else {
                        segments.push(RichSegment::Text("{".to_string()));
                        remaining = &remaining[pos + 1..];
                    }
                } else {
                    segments.push(RichSegment::Text("{".to_string()));
                    remaining = &remaining[pos + 1..];
                }
            }

            // Tag syntax: [tag]...[/tag]
            Some(pos) if Some(pos) == tag_pos => {
                if pos > 0 {
                    segments.push(RichSegment::Text(remaining[..pos].to_string()));
                }

                if let Some(bracket_end) = remaining[pos..].find(']') {
                    let tag_end = pos + bracket_end;
                    let tag = &remaining[pos + 1..tag_end];

                    if tag.starts_with('/') {
                        segments.push(RichSegment::Text(remaining[pos..=tag_end].to_string()));
                        remaining = &remaining[tag_end + 1..];
                        continue;
                    }

                    let close_tag = format!("[/{}]", tag);
                    if let Some(close_pos) = remaining.find(&close_tag) {
                        let content = &remaining[tag_end + 1..close_pos];
                        let children = parse_rich_segments(content);
                        segments.push(RichSegment::Tagged {
                            tag: tag.to_string(),
                            children,
                        });
                        remaining = &remaining[close_pos + close_tag.len()..];
                    } else {
                        segments.push(RichSegment::Text(remaining[pos..=tag_end].to_string()));
                        remaining = &remaining[tag_end + 1..];
                    }
                } else {
                    segments.push(RichSegment::Text(remaining.to_string()));
                    break;
                }
            }

            // Markdown syntax
            Some(pos) => {
                let (_, open, close, style) = md_earliest.unwrap();
                if pos > 0 {
                    segments.push(RichSegment::Text(remaining[..pos].to_string()));
                }
                remaining = &remaining[pos + open.len()..];

                if let Some(end_pos) = remaining.find(close) {
                    let content = &remaining[..end_pos];
                    segments.push(RichSegment::Styled {
                        style,
                        content: content.to_string(),
                    });
                    remaining = &remaining[end_pos + close.len()..];
                } else {
                    segments.push(RichSegment::Text(open.to_string()));
                }
            }
        }
    }

    segments
}

/// Apply a style to a TextSpan handle
fn apply_rich_style(handle: Handle<'_, TextSpan>, style: RichStyle) -> Handle<'_, TextSpan> {
    match style {
        RichStyle::Bold => handle.font_weight(FontWeightKeyword::Bold),
        RichStyle::Italic => handle.font_slant(FontSlant::Italic),
        RichStyle::Underline => handle.text_decoration_line(TextDecorationLine::Underline),
        RichStyle::Strikethrough => handle.text_decoration_line(TextDecorationLine::Strikethrough),
        RichStyle::Code => handle.font_family(vec![
            FamilyOwned::Named("Menlo".to_string()),           // macOS
            FamilyOwned::Named("Consolas".to_string()),        // Windows
            FamilyOwned::Named("DejaVu Sans Mono".to_string()), // Linux
            FamilyOwned::Generic(GenericFontFamily::Monospace), // Fallback
        ]),
    }
}

/// Create TextSpan elements from parsed segments
fn parse_rich(
    cx: &mut Context,
    text: &str,
    styles: &HashMap<String, Rc<dyn Fn(Handle<'_, TextSpan>) -> Handle<'_, TextSpan>>>,
    links: &HashMap<String, String>,
    bindings: &HashMap<String, Signal<String>>,
    conditionals: &HashMap<String, Signal<bool>>,
    each_bindings: &HashMap<String, Signal<Vec<String>>>,
) {
    let segments = parse_rich_segments(text);
    render_rich_segments(cx, &segments, styles, links, bindings, conditionals, each_bindings);
}

/// Extract plain text from segments if they contain only text (no nested styling).
/// Returns None if there's any styled, tagged, binding, conditional, or each content.
fn extract_plain_text(segments: &[RichSegment]) -> Option<String> {
    let mut result = String::new();
    for segment in segments {
        match segment {
            RichSegment::Text(s) => result.push_str(s),
            RichSegment::Styled { .. }
            | RichSegment::Tagged { .. }
            | RichSegment::Binding { .. }
            | RichSegment::Conditional { .. }
            | RichSegment::Each { .. } => return None,
        }
    }
    Some(result)
}

/// Render parsed segments into TextSpan views
fn render_rich_segments(
    cx: &mut Context,
    segments: &[RichSegment],
    styles: &HashMap<String, Rc<dyn Fn(Handle<'_, TextSpan>) -> Handle<'_, TextSpan>>>,
    links: &HashMap<String, String>,
    bindings: &HashMap<String, Signal<String>>,
    conditionals: &HashMap<String, Signal<bool>>,
    each_bindings: &HashMap<String, Signal<Vec<String>>>,
) {
    for segment in segments {
        match segment {
            RichSegment::Text(s) => {
                if !s.is_empty() {
                    let text_signal = cx.state(s.clone());
                    TextSpan::new(cx, text_signal, |_| {});
                }
            }
            RichSegment::Styled { style, content } => {
                let text_signal = cx.state(content.clone());
                let handle = TextSpan::new(cx, text_signal, |_| {});
                apply_rich_style(handle, *style);
            }
            RichSegment::Binding { name } => {
                // Look up the signal binding
                if let Some(signal) = bindings.get(name) {
                    let handle = TextSpan::new(cx, *signal, |_| {});
                    // Apply style if one exists for this binding name
                    if let Some(style_fn) = styles.get(name) {
                        let _ = style_fn(handle);
                    }
                } else {
                    // No binding found, render the placeholder as literal text
                    let text_signal = cx.state(format!("{{{}}}", name));
                    TextSpan::new(cx, text_signal, |_| {});
                }
            }
            RichSegment::Conditional { name, children } => {
                // Conditional rendering: only show content when signal is true
                if let Some(cond_signal) = conditionals.get(name) {
                    // Note: Currently renders based on initial value only.
                    // TextSpans must be direct children of Label for proper paragraph integration.
                    // Wrapping in Binding breaks the parent-child relationship needed for text rendering.
                    if *cond_signal.get(cx) {
                        render_rich_segments(cx, children, styles, links, bindings, conditionals, each_bindings);
                    }
                } else {
                    // No conditional found, render placeholder
                    let text_signal = cx.state(format!("{{#if {}}}", name));
                    TextSpan::new(cx, text_signal, |_| {});
                }
            }
            RichSegment::Each { name, item_name, children } => {
                // Loop rendering: repeat content for each item
                if let Some(vec_signal) = each_bindings.get(name) {
                    // Note: Currently renders based on initial value only.
                    // TextSpans must be direct children of Label for proper paragraph integration.
                    let items = vec_signal.get(cx).clone();
                    for item_value in items.iter() {
                        // For each item, substitute {item_name} with the actual value
                        let mut item_bindings = bindings.clone();
                        let item_signal = cx.state(item_value.clone());
                        item_bindings.insert(item_name.clone(), item_signal);

                        render_rich_segments(cx, children, styles, links, &item_bindings, conditionals, each_bindings);
                    }
                } else {
                    // No each binding found, render placeholder
                    let text_signal = cx.state(format!("{{#each {}}}", name));
                    TextSpan::new(cx, text_signal, |_| {});
                }
            }
            RichSegment::Tagged { tag, children } => {
                // Check if this tag is a link
                if let Some(url) = links.get(tag) {
                    let url = url.clone();
                    let empty = cx.state(String::new());
                    TextSpan::new(cx, empty, |cx| {
                        render_rich_segments(cx, children, styles, links, bindings, conditionals, each_bindings);
                    })
                    .color(Color::rgb(0x33, 0x99, 0xff))
                    .text_decoration_line(TextDecorationLine::Underline)
                    .cursor(CursorIcon::Hand)
                    .pointer_events(PointerEvents::Auto)
                    .navigable(true)
                    .on_press(move |_| {
                        let _ = open::that(&url);
                    });
                    continue;
                }

                // Check for custom style
                if let Some(style_fn) = styles.get(tag) {
                    // For simple text content, apply style directly to avoid nesting issues
                    // (background_color isn't inherited, so wrapper won't show)
                    if let Some(text) = extract_plain_text(children) {
                        let text_signal = cx.state(text);
                        let handle = TextSpan::new(cx, text_signal, |_| {});
                        let _ = style_fn(handle);
                    } else {
                        // Complex nested content needs a wrapper
                        let empty = cx.state(String::new());
                        let handle = TextSpan::new(cx, empty, |cx| {
                            render_rich_segments(cx, children, styles, links, bindings, conditionals, each_bindings);
                        });
                        let _ = style_fn(handle);
                    }
                } else {
                    // Unknown tag, just render children
                    render_rich_segments(cx, children, styles, links, bindings, conditionals, each_bindings);
                }
            }
        }
    }
}

/// Rich text configuration stored per entity
#[derive(Default, Clone)]
struct RichConfig {
    bindings: HashMap<String, Signal<String>>,
    conditionals: HashMap<String, Signal<bool>>,
    each_bindings: HashMap<String, Signal<Vec<String>>>,
    styles: HashMap<String, Rc<dyn Fn(Handle<'_, TextSpan>) -> Handle<'_, TextSpan>>>,
    links: HashMap<String, String>,
    parsed: bool, // Track if we've already parsed
}

// Thread-local storage for rich config during build
std::thread_local! {
    static RICH_CONFIGS: std::cell::RefCell<HashMap<Entity, RichConfig>> = std::cell::RefCell::new(HashMap::new());
}

impl Handle<'_, Label> {
    /// Which form element does this label describe.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// # let text = cx.state("hello");
    /// # let value = cx.state(false);
    /// Checkbox::new(cx, value)
    ///     .on_toggle(move |cx| value.upd(cx, |v| *v = !*v))
    ///     .id("checkbox_identifier");
    /// Label::new(cx, text).describing("checkbox_identifier");
    /// ```
    pub fn describing(mut self, entity_identifier: impl Into<String>) -> Self {
        let identifier = entity_identifier.into();
        if let Some(id) = self.cx.resolve_entity_identifier(&identifier) {
            self.cx.style.labelled_by.insert(id, self.entity);
        }
        let hidden = self.context().state(true);
        self.modify(|label| label.describing = Some(identifier)).class("describing").hidden(hidden)
    }

    /// Bind a signal to a placeholder `{name}` in the label text.
    ///
    /// The placeholder will be replaced with the signal's value and update reactively.
    /// Use `.rich_style("name", ...)` to style the bound text.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let count = cx.state(0i32);
    /// Label::new(cx, "Counter: {count}")
    ///     .rich_bind("count", count);
    /// ```
    pub fn rich_bind<T>(self, name: &str, signal: Signal<T>) -> Self
    where
        T: ToString + Clone + 'static,
    {
        self.rich_bind_internal(name, signal)
    }

    fn rich_bind_internal<T>(self, name: &str, signal: Signal<T>) -> Self
    where
        T: ToString + Clone + 'static,
    {
        // Create a derived string signal
        let string_signal = signal.drv(self.cx, |v, _| v.to_string());

        let mut config = self.get_rich_config();
        config.bindings.insert(name.to_string(), string_signal);
        self.set_rich_config(config);
        self
    }

    /// Bind a boolean signal for conditional rendering `{#if name}...{/if}`.
    ///
    /// Content inside the conditional block is only rendered when the signal is true.
    /// The label will automatically rebuild when the condition changes.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let show_warning = cx.state(true);
    /// Label::new(cx, "Status: {#if warning}**Warning!**{/if} OK")
    ///     .cond("warning", show_warning);
    /// ```
    pub fn cond(self, name: &str, signal: Signal<bool>) -> Self {
        let mut config = self.get_rich_config();
        config.conditionals.insert(name.to_string(), signal);
        self.set_rich_config(config);
        self
    }

    /// Bind a Vec signal for loop rendering `{#each name as item}...{/each}`.
    ///
    /// The template inside is repeated for each item. The label rebuilds when items change.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let items = cx.state(vec!["Apple", "Banana", "Cherry"]);
    /// Label::new(cx, "Fruits: {#each fruits as f}{f}, {/each}")
    ///     .each("fruits", items, |item| item.to_string());
    /// ```
    pub fn each<T, F>(self, name: &str, signal: Signal<Vec<T>>, item_fn: F) -> Self
    where
        T: Clone + 'static,
        F: Fn(&T) -> String + 'static,
    {
        // Derive a Signal<Vec<String>>
        let vec_signal = signal.drv(self.cx, move |v, _| {
            v.iter().map(|item| item_fn(item)).collect()
        });

        let mut config = self.get_rich_config();
        config.each_bindings.insert(name.to_string(), vec_signal);
        self.set_rich_config(config);
        self
    }

    /// Add a clickable link for a tag `[tag]text[/tag]`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// Label::new(cx, "Visit [docs]documentation[/docs]")
    ///     .link("docs", "https://docs.vizia.dev");
    /// ```
    pub fn link(self, tag: &str, url: impl Into<String>) -> Self {
        let mut config = self.get_rich_config();
        config.links.insert(tag.to_string(), url.into());
        self.set_rich_config(config);
        self
    }

    /// Add a custom style for a tag `[tag]text[/tag]`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// Label::new(cx, "This is [highlight]important[/highlight]")
    ///     .rich_style("highlight", |s| s.background_color(Color::yellow()));
    /// ```
    pub fn rich_style<F>(self, tag: &str, f: F) -> Self
    where
        F: Fn(Handle<'_, TextSpan>) -> Handle<'_, TextSpan> + 'static,
    {
        let mut config = self.get_rich_config();
        config.styles.insert(tag.to_string(), Rc::new(f));
        self.set_rich_config(config);
        self
    }

    /// Build the rich text label with all configured bindings, links, and styles.
    ///
    /// Call this after configuring rich text features with `.rich_bind()`, `.cond()`,
    /// `.each()`, `.link()`, or `.rich_style()`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// Label::new(cx, "Visit [docs]Docs[/docs] and [repo]GitHub[/repo]")
    ///     .link("docs", "https://docs.vizia.dev")
    ///     .link("repo", "https://github.com/vizia/vizia")
    ///     .build_rich();
    /// ```
    pub fn build_rich(self) -> Self {
        let raw_text = {
            let views = &self.cx.views;
            views.get(&self.entity)
                .and_then(|v| v.downcast_ref::<Label>())
                .and_then(|l| l.raw_text.clone())
        };

        if let Some(text) = raw_text {
            let config = self.get_rich_config();
            self.rebuild_rich(&text, config)
        } else {
            self
        }
    }

    fn get_rich_config(&self) -> RichConfig {
        // Get or create accumulated config for this entity
        RICH_CONFIGS.with(|configs| {
            configs.borrow().get(&self.entity).cloned().unwrap_or_default()
        })
    }

    fn set_rich_config(&self, config: RichConfig) {
        RICH_CONFIGS.with(|configs| {
            configs.borrow_mut().insert(self.entity, config);
        });
    }

    fn rebuild_rich(self, text: &str, config: RichConfig) -> Self {
        let entity = self.entity;
        let cx = self.cx;

        // Check if we've already parsed this label
        let already_parsed = RICH_CONFIGS.with(|configs| {
            configs.borrow().get(&entity).map(|c| c.parsed).unwrap_or(false)
        });

        if already_parsed {
            // Already parsed - skip to avoid duplicates
            return Handle {
                current: entity,
                entity,
                p: std::marker::PhantomData,
                cx,
            };
        }

        // Mark as parsed
        RICH_CONFIGS.with(|configs| {
            if let Some(c) = configs.borrow_mut().get_mut(&entity) {
                c.parsed = true;
            }
        });

        // Determine if we need reactive rebuilding (conditionals or loops)
        let has_reactive = !config.conditionals.is_empty() || !config.each_bindings.is_empty();

        if has_reactive {
            // Create version signal for reactive rebuilding
            let cond_signals: Vec<Signal<bool>> = config.conditionals.values().copied().collect();
            let each_signals: Vec<Signal<Vec<String>>> = config.each_bindings.values().copied().collect();

            let version = cx.derived({
                let cond_signals = cond_signals.clone();
                let each_signals = each_signals.clone();
                move |cx| {
                    let mut hash = 0u64;
                    for sig in &cond_signals {
                        hash = hash.wrapping_add(if *sig.get(cx) { 1 } else { 0 });
                    }
                    for sig in &each_signals {
                        hash = hash.wrapping_add(sig.get(cx).len() as u64);
                    }
                    hash
                }
            });

            let text = text.to_string();
            let styles = config.styles;
            let links = config.links;
            let bindings = config.bindings;
            let conditionals = config.conditionals;
            let each_bindings = config.each_bindings;

            // Build content inside the existing label using Binding for reactivity
            cx.with_current(entity, |cx| {
                Binding::new(cx, version, move |cx| {
                    parse_rich(cx, &text, &styles, &links, &bindings, &conditionals, &each_bindings);
                });
            });
        } else {
            // Static rich text - no reactive structure needed
            let text = text.to_string();
            let styles = config.styles;
            let links = config.links;
            let bindings = config.bindings;
            let conditionals = config.conditionals;
            let each_bindings = config.each_bindings;

            cx.with_current(entity, |cx| {
                parse_rich(cx, &text, &styles, &links, &bindings, &conditionals, &each_bindings);
            });
        }

        // Clear the plain text by setting to empty - rich content is in children
        cx.style.text.insert(entity, String::new());

        Handle {
            current: entity,
            entity,
            p: std::marker::PhantomData,
            cx,
        }
    }
}

impl View for Label {
    fn element(&self) -> Option<&'static str> {
        Some("label")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::Press { .. } | WindowEvent::PressDown { .. } => {
                if cx.current() == cx.mouse.left.pressed && meta.target == cx.current() {
                    if let Some(describing) = self
                        .describing
                        .as_ref()
                        .and_then(|identity| cx.resolve_entity_identifier(identity))
                    {
                        let old = cx.current;
                        cx.current = describing;
                        cx.focus_with_visibility(false);
                        let message = if matches!(window_event, WindowEvent::Press { .. }) {
                            WindowEvent::Press { mouse: false }
                        } else {
                            WindowEvent::PressDown { mouse: false }
                        };
                        cx.emit_to(describing, message);
                        cx.current = old;
                    }
                }
            }
            _ => {}
        });
    }
}

/// A view which represents a span of text within a label.
pub struct TextSpan {}

impl TextSpan {
    /// Create a new [TextSpan] view.
    ///
    /// Accepts either a plain value or a `Signal<T>` for reactive text.
    pub fn new<'a, T>(
        cx: &'a mut Context,
        text: impl Res<T> + 'static,
        children: impl Fn(&mut Context),
    ) -> Handle<'a, Self>
    where
        T: ToStringLocalized + Clone + 'static,
    {
        Self {}
            .build(cx, |cx| {
                cx.style.text_span.insert(cx.current(), true);
                cx.style.display.insert(cx.current(), Display::None);
                cx.style.pointer_events.insert(cx.current(), PointerEvents::None);
                children(cx);
            })
            .text(text)
    }
}

impl View for TextSpan {
    fn element(&self) -> Option<&'static str> {
        Some("text-span")
    }
}
