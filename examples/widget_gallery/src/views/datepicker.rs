use crate::components::DemoRegion;
use chrono::Utc;
use vizia::prelude::*;

pub fn datepicker(cx: &mut Context) {
    VStack::new(cx, move |cx| {
        let date = cx.state(Utc::now().date_naive());

        Markdown::new(cx, "# Datepicker");

        Divider::new(cx);

        Markdown::new(cx, "### Basic datepicker");

        DemoRegion::new(
            cx,
            move |cx| {
                Datepicker::new(cx, date).on_select(move |cx, selected| date.set(cx, selected));
            },
            r#"let date = cx.state(Utc::now().date_naive());
Datepicker::new(cx, date)
    .on_select(move |cx, selected| date.set(cx, selected));"#,
        );
    })
    .class("panel");
}
