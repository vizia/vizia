use vizia::prelude::*;

use crate::DemoRegion;

pub fn markdown_panel(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("markdown")).class("panel-title");

        Divider::new(cx);

        Label::new(cx, Localized::new("markdown-rendering")).class("section-title");

        DemoRegion::new(cx, "Markdown Rendering", |cx| {
            Markdown::new(
                cx,
                r#"## Headings and Text

### Subheading

**Bold** and *italic* text, and ~~strikethrough~~.

---

## Lists

- Item one
- Item two
  - Nested item
- Item three

1. First
2. Second
3. Third

---

## Code

Inline `code` and a code block:

```rust
fn hello() -> &'static str {
    "world"
}
```

---

## Tables

| View        | Category  |
|-------------|-----------|
| Button      | Input     |
| Label       | Display   |
| VirtualList | Data      |
"#,
            );
        });
    })
    .class("panel");
}
