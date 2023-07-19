use vizia::prelude::*;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use vizia_core::image::Pixel;

const STYLE: &str = r#"
    :root {
        background-color: #181818;
    }

    label.code {
        font-family: "Fira Code";
        font-size: 14.0;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).unwrap();

        // Load these once at the start of your program
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        let syntax = ps.find_syntax_by_extension("rs").unwrap();

        let mut h = HighlightLines::new(syntax, &ts.themes["base16-mocha.dark"]);

        let s = r#"Application::new(|cx|{
    Label::new(cx, "Hello World");
}).run();"#;
        let mut code = Label::new(cx, s).space(Pixels(20.0)).class("code");
        for (line, line_text) in LinesWithEndings::from(s).enumerate() {
            let ranges: Vec<(Style, &str)> = h.highlight_line(line_text, &ps).unwrap();
            let mut start = 0;
            for (style, txt) in ranges {
                let end = start + txt.len();
                code =
                    code.span(TextCursor::new(line, start), TextCursor::new(line, end), |span| {
                        span.color(Color::rgb(
                            style.foreground.r,
                            style.foreground.g,
                            style.foreground.b,
                        ))
                    });
                start = end;
            }
        }

        Label::new(cx, s)
            .color(Color::gray())
            .span(TextCursor::new(0, 0), TextCursor::new(0, 5), |span| span.color(Color::red()))
            .span(TextCursor::new(0, 3), TextCursor::new(0, 10), |span| {
                span.font_weight(FontWeightKeyword::Bold)
            })
            .space(Pixels(20.0));
    })
    .run();
}
