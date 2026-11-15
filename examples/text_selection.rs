use vizia::prelude::*;

fn selectable_paragraph(cx: &mut Context, text: &'static str) {
    Label::new(cx, text)
        .text_selectable(true)
        .text_wrap(true)
        .width(Stretch(1.0))
        .height(Auto)
        .line_height(1.45)
        .selection_color(Color::rgba(30, 112, 168, 96));
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let editable_text = Signal::new(String::from("Focused textboxes keep their own selection."));

        VStack::new(cx, |cx| {
            Label::new(cx, "Selectable label text")
                .font_size(24.0)
                .font_weight(FontWeight(700))
                .height(Auto);

            VStack::new(cx, |cx| {
                selectable_paragraph(
                    cx,
                    "A selection can begin midway through this label and continue into the labels below. The highlight is drawn by each label behind its own glyphs.",
                );
                selectable_paragraph(
                    cx,
                    "This paragraph wraps at the container edge. Drag in either direction, double-click a word, or triple-click to select the complete label.",
                );
                selectable_paragraph(
                    cx,
                    "Unicode and bidirectional text use Parley boundaries: café, 你好世界, مرحبا بالعالم.",
                );
            })
            .width(Stretch(1.0))
            .height(Auto)
            .vertical_gap(Pixels(12.0));

            Label::new(cx, "This label is intentionally not selectable.")
                .height(Auto)
                .color(Color::rgb(112, 112, 112));

            Label::new(cx, "Keyboard selection")
                .font_size(18.0)
                .font_weight(FontWeight(600))
                .height(Auto);
            selectable_paragraph(
                cx,
                "After clicking or dragging, use Shift+Arrow to extend, Ctrl/Cmd+Shift+Left or Right for words, Shift+Home or End for line edges, Ctrl/Cmd+A to select all, Ctrl/Cmd+C to copy, and Escape to clear.",
            );

            Textbox::new(cx, editable_text)
                .width(Stretch(1.0))
                .placeholder("Textbox shortcut precedence");
        })
        .width(Stretch(1.0))
        .height(Stretch(1.0))
        .padding(Pixels(28.0))
        .vertical_gap(Pixels(18.0));
    })
    .title("Text Selection")
    .inner_size((720, 620))
    .run()
}
