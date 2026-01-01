mod helpers;
use helpers::*;

use vizia::icons::{ICON_BOLD, ICON_ITALIC, ICON_UNDERLINE};
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let bold = cx.state(false);
        let italic = cx.state(false);
        let underline = cx.state(false);
        let icon_bold = cx.state(ICON_BOLD);
        let icon_italic = cx.state(ICON_ITALIC);
        let icon_underline = cx.state(ICON_UNDERLINE);

        ExamplePage::vertical(cx, |cx| {
            ToggleButton::new(cx, bold, |cx| Label::static_text(cx, "Bold")).two_way();

            ButtonGroup::new(cx, |cx| {
                ToggleButton::new(cx, bold, move |cx| Svg::new(cx, icon_bold)).two_way();

                ToggleButton::new(cx, italic, move |cx| Svg::new(cx, icon_italic)).two_way();

                ToggleButton::new(cx, underline, move |cx| Svg::new(cx, icon_underline)).two_way();
            });
        });
        (cx.state("ToggleButton"), cx.state((700, 200)))
    });

    app.title(title).inner_size(size).run()
}
