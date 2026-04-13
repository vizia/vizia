use vizia::prelude::*;

use crate::DemoRegion;

pub fn markdown_panel(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Markdown
The `Markdown` view renders a subset of Markdown inline within the UI.",
        );

        Divider::new(cx);

        Markdown::new(cx, "### Markdown Rendering");

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
