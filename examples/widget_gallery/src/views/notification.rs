use vizia::prelude::*;

use crate::DemoRegion;

pub fn notification(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Notification").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic notification").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Notification::new(
                    cx,
                    "Notification Title".to_string(),
                    Some(
                        "This is some information about the notification you just got!".to_string(),
                    ),
                );
            },
            r#"Notification::new(
    cx,
    "Notification Title".to_string(),
    Some(
        "This is some information about the notification you just got!".to_string(),
    ),
);"#,
        );
    })
    .class("panel");
}
